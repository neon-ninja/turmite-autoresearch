// Kernels for the GPU probe (see src/gpu/probe.rs for the host side and the design).
//
// Semantics mirror `probe::probe_walker` with `AntWalker` / `PatchedAnt(radius 2)`:
//   - the ant starts at the board centre (c = size/2), heading 0 (north = -y); the start cell is
//     visited before step 1 and is not a "new cell";
//   - per step: (a) boundary test on the current cell, (b) turn/colour/move, (c) if the cell entered was
//     never visited, `visit` it now, (d) accumulate ant sums, (e) escape test |x| >= c-2 || |y| >= c-2
//     (relative coordinates), (f) close the window when steps == 2^(k+1);
//   - an escaping step ends the run without closing its window.
//
// A cell is 16 bits: low byte = colour, high byte = "unvisited 4-neighbours + 1" for visited cells and 0
// for never-visited cells (so an all-zero tile is blank *and* unvisited, and the pool needs no fill).

#define TILE_SHIFT 6
#define TILE_DIM 64
#define TILE_CELLS 4096
#define PATCH_R 2

enum { ST_RUN = 0, ST_BUDGET = 1, ST_ESCAPED = 2, ST_OOM = 3, ST_IDLE = 255 };

typedef unsigned long long u64;
typedef long long i64;

struct PLane {
    u64 steps;
    u64 budget;
    u64 seed;
    unsigned job;
    unsigned status;
    int x, y;                   // absolute grid coordinates
    int minx, maxx, miny, maxy; // bounding box of the visited set, relative to the start
    unsigned h;
    unsigned k;                 // current window index; also the number of closed windows
    unsigned started;
    unsigned n;                 // colours
    unsigned turn[4];           // bit c set: colour c turns R
    unsigned has_patch;
    unsigned pad;
    u64 area, perimeter;
    i64 sx, sy, sxx;            // sums over the visited set (relative coordinates)
    u64 newc, bsteps, fresh;    // window counters
    i64 ax, ay;                 // window sums of the ant's position
    u64 axx;
};

struct PRec {                   // one closed window
    u64 area, perimeter, newc, bsteps, fresh;
    i64 sx, sy, sxx, ax, ay;
    u64 axx;
    int minx, maxx, miny, maxy;
};

struct Assign {
    u64 budget;
    u64 seed;
    unsigned lane;
    unsigned job;
    unsigned turn[4];
    unsigned n;
    unsigned has_patch;
};

__device__ __forceinline__ u64 mix64(u64 z) {
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL;
    return z ^ (z >> 31);
}

// Same as PatchedAnt::initial_cell inside the patch; (x, y) relative to the start cell.
__device__ __forceinline__ unsigned patch_colour(u64 seed, int x, int y, unsigned n) {
    u64 key = ((u64)(unsigned)x << 32) | (u64)(unsigned)y;
    return (unsigned)(mix64(mix64(seed ^ 0x5851F42D4C957F2DULL) ^ key) % n);
}

struct Ctx {
    unsigned* dir;
    unsigned short* pool;
    const unsigned* freelist;
    int* free_count;
    unsigned dir_w;
    int half;
    unsigned n;
    u64 seed;
    unsigned has_patch;
};

// The tile covering tile coordinates (tx, ty); installs one from the pool if `alloc` (null: pool empty,
// or not allocated and !alloc). New tiles are all-zero except for the cells of the random patch they hold
// (the patch can straddle up to four tiles).
__device__ unsigned short* tile_of(const Ctx& c, int tx, int ty, bool alloc) {
    unsigned di = (unsigned)ty * c.dir_w + (unsigned)tx;
    unsigned e = c.dir[di];
    if (!e) {
        if (!alloc) return 0;
        int old = atomicAdd(c.free_count, -1);
        if (old <= 0) {
            atomicAdd(c.free_count, 1);
            return 0;
        }
        unsigned id = c.freelist[old - 1];
        e = id + 1;
        unsigned short* t = c.pool + (size_t)id * TILE_CELLS;
        if (c.has_patch) {
            int x0 = max(tx << TILE_SHIFT, c.half - PATCH_R), x1 = min((tx << TILE_SHIFT) + TILE_DIM - 1, c.half + PATCH_R);
            int y0 = max(ty << TILE_SHIFT, c.half - PATCH_R), y1 = min((ty << TILE_SHIFT) + TILE_DIM - 1, c.half + PATCH_R);
            for (int j = y0; j <= y1; j++)
                for (int i = x0; i <= x1; i++)
                    t[((j & (TILE_DIM - 1)) << TILE_SHIFT) | (i & (TILE_DIM - 1))] =
                        (unsigned short)patch_colour(c.seed, i - c.half, j - c.half, c.n);
        }
        c.dir[di] = e;
    }
    return c.pool + (size_t)(e - 1) * TILE_CELLS;
}

struct Region {
    u64 area, perimeter;
    i64 sx, sy, sxx;
    int minx, maxx, miny, maxy;
};

// Region::visit of src/probe.rs. `w` is the cell's word (colour, unvisited); returns the new word.
// Neighbours in unallocated tiles are unvisited by definition (a visited cell's tile exists).
__device__ unsigned visit(const Ctx& c, Region& r, unsigned short* tile, unsigned off, int x, int y, unsigned w) {
    const int dx[4] = {0, 1, 0, -1}, dy[4] = {-1, 0, 1, 0};
    unsigned u = 0;
    for (int d = 0; d < 4; d++) {
        int nx = x + dx[d], ny = y + dy[d];
        unsigned short* t = tile_of(c, nx >> TILE_SHIFT, ny >> TILE_SHIFT, false);
        if (!t) { u++; continue; }
        unsigned short* p = t + (((ny & (TILE_DIM - 1)) << TILE_SHIFT) | (nx & (TILE_DIM - 1)));
        unsigned v = *p, uv = v >> 8;
        if (!uv) { u++; continue; }
        uv--;
        *p = (unsigned short)((uv << 8) | (v & 255));
        if (uv == 1) r.perimeter--;  // its unvisited-neighbour count just reached 0
    }
    w = ((u + 1) << 8) | (w & 255);
    tile[off] = (unsigned short)w;
    if (u > 0) r.perimeter++;
    r.area++;
    int rx = x - c.half, ry = y - c.half;
    r.sx += rx;
    r.sy += ry;
    r.sxx += (i64)rx * rx + (i64)ry * ry;
    r.minx = min(r.minx, rx); r.maxx = max(r.maxx, rx);
    r.miny = min(r.miny, ry); r.maxy = max(r.maxy, ry);
    return w;
}

extern "C" __global__ void pstep_kernel(PLane* lanes, int nlanes, unsigned* dirs, unsigned short* pool,
                                        const unsigned* freelist, int* free_count, PRec* recs,
                                        unsigned size, unsigned dir_w, unsigned max_win, u64 slice) {
    int tid = blockIdx.x * blockDim.x + threadIdx.x;
    if (tid >= nlanes) return;
    PLane s = lanes[tid];
    if (s.status != ST_RUN) return;

    Ctx c;
    c.dir = dirs + (size_t)tid * dir_w * dir_w;
    c.pool = pool;
    c.freelist = freelist;
    c.free_count = free_count;
    c.dir_w = dir_w;
    c.half = (int)(size / 2);
    c.n = s.n;
    c.seed = s.seed;
    c.has_patch = s.has_patch;

    Region r;
    r.area = s.area; r.perimeter = s.perimeter; r.sx = s.sx; r.sy = s.sy; r.sxx = s.sxx;
    r.minx = s.minx; r.maxx = s.maxx; r.miny = s.miny; r.maxy = s.maxy;

    int x = s.x, y = s.y;
    unsigned h = s.h, n = s.n, k = s.k;
    u64 steps = s.steps;
    u64 newc = s.newc, bsteps = s.bsteps, fresh = s.fresh, axx = s.axx;
    i64 ax = s.ax, ay = s.ay;
    const unsigned t0 = s.turn[0], t1 = s.turn[1], t2 = s.turn[2], t3 = s.turn[3];
    const int half = c.half;
    const unsigned hi = (unsigned)(2 * half - 6);  // not escaped <=> (unsigned)(x - 3) <= hi
    unsigned status = ST_RUN;

    int cur_tx = x >> TILE_SHIFT, cur_ty = y >> TILE_SHIFT;
    unsigned short* tile = tile_of(c, cur_tx, cur_ty, true);
    unsigned off = ((unsigned)(y & (TILE_DIM - 1)) << TILE_SHIFT) | (unsigned)(x & (TILE_DIM - 1));
    unsigned w = 0;
    if (!tile) {
        status = ST_OOM;
    } else {
        w = tile[off];
        if (!s.started) {  // the start cell is visited at t = 0, before any window
            w = visit(c, r, tile, off, x, y, w);
            s.started = 1;
        }
    }

    u64 left = s.budget - s.steps;
    if (left > slice) left = slice;
    while (status == ST_RUN && left) {
        u64 win_end = 1ULL << (k + 1);
        u64 chunk = win_end - steps;
        if (chunk > left) chunk = left;
        left -= chunk;
        for (; chunk; --chunk) {
            unsigned uv = w >> 8, col = w & 255;
            unsigned unv = uv - 1;  // the current cell is always visited
            fresh += unv;
            bsteps += (unv != 0);
            unsigned sel = col >> 5;
            unsigned tw = sel == 0 ? t0 : sel == 1 ? t1 : sel == 2 ? t2 : t3;
            unsigned rt = (tw >> (col & 31)) & 1u;
            h = (h + 3u - 2u * rt) & 3u;
            unsigned nc = col + 1;
            if (nc == n) nc = 0;
            tile[off] = (unsigned short)((uv << 8) | nc);
            steps++;
            x += (int)(h == 1) - (int)(h == 3);
            y += (int)(h == 2) - (int)(h == 0);
            int rx = x - half, ry = y - half;
            ax += rx;
            ay += ry;
            axx += (u64)((i64)rx * rx + (i64)ry * ry);
            if ((unsigned)(x - 3) > hi || (unsigned)(y - 3) > hi) {
                status = ST_ESCAPED;
                break;
            }
            int tx = x >> TILE_SHIFT, ty = y >> TILE_SHIFT;
            if (tx != cur_tx || ty != cur_ty) {
                tile = tile_of(c, tx, ty, true);
                if (!tile) {
                    status = ST_OOM;
                    break;
                }
                cur_tx = tx;
                cur_ty = ty;
            }
            off = ((unsigned)(y & (TILE_DIM - 1)) << TILE_SHIFT) | (unsigned)(x & (TILE_DIM - 1));
            w = tile[off];
            if (!(w >> 8)) {
                newc++;
                w = visit(c, r, tile, off, x, y, w);
            }
        }
        if (status != ST_RUN) break;
        if (steps == win_end) {
            PRec* rec = recs + (size_t)tid * max_win + k;
            rec->area = r.area; rec->perimeter = r.perimeter; rec->newc = newc; rec->bsteps = bsteps;
            rec->fresh = fresh; rec->sx = r.sx; rec->sy = r.sy; rec->sxx = r.sxx;
            rec->ax = ax; rec->ay = ay; rec->axx = axx;
            rec->minx = r.minx; rec->maxx = r.maxx; rec->miny = r.miny; rec->maxy = r.maxy;
            newc = bsteps = fresh = axx = 0;
            ax = ay = 0;
            k++;
        }
    }
    if (status == ST_RUN && steps >= s.budget) status = ST_BUDGET;

    s.x = x; s.y = y; s.h = h; s.k = k; s.steps = steps; s.status = status;
    s.newc = newc; s.bsteps = bsteps; s.fresh = fresh; s.axx = axx; s.ax = ax; s.ay = ay;
    s.area = r.area; s.perimeter = r.perimeter; s.sx = r.sx; s.sy = r.sy; s.sxx = r.sxx;
    s.minx = r.minx; s.maxx = r.maxx; s.miny = r.miny; s.maxy = r.maxy;
    lanes[tid] = s;
}

// One block per finished lane: free (and zero) its tiles, mark it idle.
extern "C" __global__ void pretire_kernel(PLane* lanes, const unsigned* ids, unsigned* dirs, unsigned short* pool,
                                          unsigned* freelist, int* free_count, unsigned dir_entries) {
    unsigned lane = ids[blockIdx.x];
    unsigned* dir = dirs + (size_t)lane * dir_entries;
    for (unsigned e = threadIdx.x; e < dir_entries; e += blockDim.x) {
        unsigned v = dir[e];
        if (v) {
            uint4* t = (uint4*)(pool + (size_t)(v - 1) * TILE_CELLS);
            for (int j = 0; j < TILE_CELLS * 2 / 16; j++) t[j] = make_uint4(0, 0, 0, 0);
            int pos = atomicAdd(free_count, 1);
            freelist[pos] = v - 1;
            dir[e] = 0;
        }
    }
    if (threadIdx.x == 0) lanes[lane].status = ST_IDLE;
}

extern "C" __global__ void passign_kernel(PLane* lanes, const Assign* jobs, int njobs, unsigned size) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= njobs) return;
    Assign a = jobs[i];
    PLane s;
    s.steps = 0;
    s.budget = a.budget;
    s.seed = a.seed;
    s.job = a.job;
    s.status = ST_RUN;
    s.x = s.y = (int)(size / 2);
    s.minx = s.maxx = s.miny = s.maxy = 0;
    s.h = 0;
    s.k = 0;
    s.started = 0;
    s.n = a.n;
    for (int j = 0; j < 4; j++) s.turn[j] = a.turn[j];
    s.has_patch = a.has_patch;
    s.pad = 0;
    s.area = s.perimeter = 0;
    s.sx = s.sy = s.sxx = 0;
    s.newc = s.bsteps = s.fresh = 0;
    s.ax = s.ay = 0;
    s.axx = 0;
    lanes[a.lane] = s;
}

extern "C" __global__ void pinit_freelist(unsigned* freelist, unsigned n) {
    unsigned i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) freelist[i] = i;
}
