// Kernels for the GPU ant sweep (see src/gpu/mod.rs for the host side and the design).
//
// One thread = one ant. The ant's virtual size x size grid is a directory of 64x64-byte tiles
// allocated on first touch from a global pool. Semantics mirror `fast::run`:
//   - the ant starts at (size/2, size/2), heading 0 (north = -y), on a blank (or seeded) background;
//   - per step: read colour c, turn by rule[c] (R = +1, L = +3 quarter turns), write (c+1) % n, move;
//   - escape = the step that lands outside [0,size)^2 (that step counts, its cell is not in the bounding box);
//   - the bounding box covers every in-grid position visited, including the start.

#define TILE_SHIFT 6
#define TILE_DIM 64
#define TILE_BYTES 4096

enum { ST_RUN = 0, ST_BUDGET = 1, ST_ESCAPED = 2, ST_OOM = 3, ST_IDLE = 255 };

struct Lane {
    unsigned long long steps;   // steps done
    unsigned long long budget;  // stop after this many steps
    unsigned long long bg_seed;
    unsigned job;
    unsigned status;
    int x, y;                   // absolute grid coordinates (off-grid after an escape)
    int minx, maxx, miny, maxy;
    unsigned h;                 // heading 0..3
    unsigned acc;               // headings of the current 16-step word, 2 bits each
    unsigned turn[4];           // bit c set: colour c turns R
    unsigned n;                 // colours
    unsigned has_bg;
    unsigned pad[2];
};

struct Assign {
    unsigned long long budget;
    unsigned long long bg_seed;
    unsigned lane;
    unsigned job;
    unsigned turn[4];
    unsigned n;
    unsigned has_bg;
};

__device__ __forceinline__ unsigned long long mix64(unsigned long long z) {
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL;
    return z ^ (z >> 31);
}

// Same as ext::bg_colour(seed, x, y, colours), with (x, y) relative to the start cell.
__device__ __forceinline__ unsigned bg_colour(unsigned long long seed, int x, int y, unsigned n) {
    unsigned long long key = ((unsigned long long)(unsigned)x << 32) | (unsigned long long)(unsigned)y;
    return (unsigned)(mix64(mix64(seed ^ 0x2545F4914F6CDD1DULL) ^ key) % n);
}

// Pop a tile from the pool and install it in the directory. Returns the directory value (id + 1), or 0
// if the pool is exhausted. Tiles in the pool are all-zero, so a blank background needs no fill.
__device__ unsigned alloc_tile(const Lane& s, unsigned* dir, unsigned di, int tx, int ty,
                               unsigned char* pool, const unsigned* freelist, int* free_count, int half) {
    int old = atomicAdd(free_count, -1);
    if (old <= 0) {
        atomicAdd(free_count, 1);
        return 0;
    }
    unsigned id = freelist[old - 1];
    if (s.has_bg) {
        unsigned char* t = pool + (size_t)id * TILE_BYTES;
        for (int j = 0; j < TILE_DIM; j++)
            for (int i = 0; i < TILE_DIM; i++)
                t[(j << TILE_SHIFT) | i] =
                    (unsigned char)bg_colour(s.bg_seed, (tx << TILE_SHIFT) + i - half, (ty << TILE_SHIFT) + j - half, s.n);
    }
    dir[di] = id + 1;
    return id + 1;
}

extern "C" __global__ void step_kernel(Lane* lanes, int nlanes, unsigned* dirs, unsigned char* pool,
                                       const unsigned* freelist, int* free_count, unsigned* rings,
                                       unsigned size, unsigned dir_w, unsigned ring_words,
                                       unsigned long long slice) {
    int tid = blockIdx.x * blockDim.x + threadIdx.x;
    if (tid >= nlanes) return;
    Lane s = lanes[tid];
    if (s.status != ST_RUN) return;

    unsigned* dir = dirs + (size_t)tid * dir_w * dir_w;
    unsigned* ring = rings + (size_t)tid * ring_words;
    unsigned long long left = s.budget - s.steps;
    if (left > slice) left = slice;

    int x = s.x, y = s.y;
    int minx = s.minx, maxx = s.maxx, miny = s.miny, maxy = s.maxy;
    unsigned h = s.h, acc = s.acc, n = s.n;
    unsigned long long steps = s.steps;
    const unsigned t0 = s.turn[0], t1 = s.turn[1], t2 = s.turn[2], t3 = s.turn[3];
    const int half = (int)(size / 2);
    unsigned status = ST_RUN;

    int cur_tx = -1, cur_ty = -1;
    unsigned char* tile = 0;

    for (; left; --left) {
        int tx = x >> TILE_SHIFT, ty = y >> TILE_SHIFT;
        if (tx != cur_tx || ty != cur_ty) {
            unsigned di = (unsigned)ty * dir_w + (unsigned)tx;
            unsigned e = dir[di];
            if (!e) {
                e = alloc_tile(s, dir, di, tx, ty, pool, freelist, free_count, half);
                if (!e) {
                    status = ST_OOM;
                    break;
                }
            }
            tile = pool + (size_t)(e - 1) * TILE_BYTES;
            cur_tx = tx;
            cur_ty = ty;
        }
        unsigned off = ((unsigned)(y & (TILE_DIM - 1)) << TILE_SHIFT) | (unsigned)(x & (TILE_DIM - 1));
        unsigned c = tile[off];
        unsigned sel = c >> 5;
        unsigned w = sel == 0 ? t0 : sel == 1 ? t1 : sel == 2 ? t2 : t3;
        unsigned r = (w >> (c & 31)) & 1u;
        h = (h + 3u - 2u * r) & 3u;
        unsigned nc = c + 1;
        if (nc == n) nc = 0;
        tile[off] = (unsigned char)nc;

        unsigned sh = (unsigned)(steps & 15ULL) * 2u;
        if (sh == 0) acc = 0;
        acc |= h << sh;
        steps++;
        // headings: 0 N (0,-1), 1 E (1,0), 2 S (0,1), 3 W (-1,0)
        x += (int)(h == 1) - (int)(h == 3);
        y += (int)(h == 2) - (int)(h == 0);
        if ((steps & 15ULL) == 0) ring[((steps - 1) >> 4) % ring_words] = acc;
        if ((unsigned)x >= size || (unsigned)y >= size) {
            status = ST_ESCAPED;
            break;
        }
        minx = min(minx, x);
        maxx = max(maxx, x);
        miny = min(miny, y);
        maxy = max(maxy, y);
    }
    if (status == ST_RUN && steps >= s.budget) status = ST_BUDGET;
    if (steps > s.steps) ring[((steps - 1) >> 4) % ring_words] = acc;  // partial word, rewritten next slice

    s.x = x; s.y = y;
    s.minx = minx; s.maxx = maxx; s.miny = miny; s.maxy = maxy;
    s.h = h; s.acc = acc; s.steps = steps; s.status = status;
    lanes[tid] = s;
}

// One block per finished lane: copy its heading ring out, free (and zero) its tiles, mark it idle.
extern "C" __global__ void retire_kernel(Lane* lanes, const unsigned* ids, unsigned* dirs, unsigned char* pool,
                                         unsigned* freelist, int* free_count, const unsigned* rings,
                                         unsigned* staging, unsigned dir_entries, unsigned ring_words) {
    unsigned lane = ids[blockIdx.x];
    unsigned* dir = dirs + (size_t)lane * dir_entries;
    const unsigned* ring = rings + (size_t)lane * ring_words;
    unsigned* out = staging + (size_t)blockIdx.x * ring_words;
    for (unsigned i = threadIdx.x; i < ring_words; i += blockDim.x) out[i] = ring[i];
    for (unsigned e = threadIdx.x; e < dir_entries; e += blockDim.x) {
        unsigned v = dir[e];
        if (v) {
            uint4* t = (uint4*)(pool + (size_t)(v - 1) * TILE_BYTES);
            for (int k = 0; k < TILE_BYTES / 16; k++) t[k] = make_uint4(0, 0, 0, 0);
            int pos = atomicAdd(free_count, 1);
            freelist[pos] = v - 1;
            dir[e] = 0;
        }
    }
    if (threadIdx.x == 0) lanes[lane].status = ST_IDLE;
}

extern "C" __global__ void assign_kernel(Lane* lanes, const Assign* jobs, int njobs, unsigned size) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= njobs) return;
    Assign a = jobs[i];
    Lane s;
    s.steps = 0;
    s.budget = a.budget;
    s.bg_seed = a.bg_seed;
    s.job = a.job;
    s.status = ST_RUN;
    s.x = s.minx = s.maxx = (int)(size / 2);
    s.y = s.miny = s.maxy = (int)(size / 2);
    s.h = 0;
    s.acc = 0;
    for (int k = 0; k < 4; k++) s.turn[k] = a.turn[k];
    s.n = a.n;
    s.has_bg = a.has_bg;
    s.pad[0] = s.pad[1] = 0;
    lanes[a.lane] = s;
}

extern "C" __global__ void init_freelist(unsigned* freelist, unsigned n) {
    unsigned i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < n) freelist[i] = i;
}
