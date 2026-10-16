// Shared observation runner; outputs belong to the directory passed by the
// experiment's own launcher. Last CLI argument is a launcher-computed source/config hash.
use rayon::prelude::*;
use std::path::Path;
use turmite::{Rule, field_age};
use turmite::fixed_area::Stop;
use turmite::walkers::{PatchedAnt, RandomWalk, Rotor};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    assert_eq!(a.len(), 7, "OUT CAP GRID LO-HI,... HASH THREADS");
    let out = Path::new(&a[1]);
    assert!(!out.join("../README.md").is_dir());
    let cap: u64 = a[2].parse().unwrap();
    let grid: usize = a[3].parse().unwrap();
    let bands: Vec<(u64,u64)> = a[4].split(',').map(|b| { let (l,h)=b.split_once('-').unwrap(); (l.parse().unwrap(),h.parse().unwrap()) }).collect();
    let marker = format!("instrument_v1 {} cap={cap} grid={grid} bands={bands:?}\n",a[5]);
    rayon::ThreadPoolBuilder::new().num_threads(a[6].parse().unwrap()).build_global().unwrap();
    std::fs::create_dir_all(out).unwrap();
    let specs: Vec<String> = std::io::stdin().lines().map(|s|s.unwrap()).filter(|s|!s.trim().is_empty()).collect();
    specs.par_iter().for_each(|spec| {
        let name = spec.replace([':', '@'], "_");
        let done = out.join(format!("{name}.done"));
        if std::fs::read_to_string(&done).is_ok_and(|s|s==marker) { eprintln!("skip {spec}"); return; }
        assert!(!done.exists(), "different config/source marker: {spec}; use a new output directory");
        let clock = std::time::Instant::now();
        let (r,n) = if let Some(s) = spec.strip_prefix("rotor:") {
            (field_age::run(Rotor {seed:s.parse().unwrap()},4,grid,cap,&bands),4)
        } else if let Some(s) = spec.strip_prefix("rw:") {
            (field_age::run(RandomWalk {state:s.parse().unwrap()},4,grid,cap,&bands),4)
        } else {
            let (rule,seed) = spec.split_once('@').unwrap();
            let rule: Rule = rule.parse().unwrap();
            let n=rule.len() as u64;
            (field_age::run(PatchedAnt::new(&rule,seed.parse().unwrap(),2),n,grid,cap,&bands),n)
        };
        let mut csv=String::from("spec,band,lo,hi,reached_lo,reached_hi,t_start,t_end,steps,B,I,area_end,P_end,sum_pa,b_complete,b_steps,b_clip,i_complete,i_steps,i_clip,tail_overlap,unresolved_overlap,unresolved_duration_lower\n");
        let mut bins=String::from("spec,band,label,bin,count,time_sum,entry_count,entry_time_sum\n");
        for (k,(b,tail)) in r.bands.iter().zip(&r.tails).enumerate() {
            assert_eq!(b.steps,b.b_dep+b.i_dep);
            assert_eq!(tail.overlap_steps,b.i_dep);
            assert_eq!(b.b.steps(),b.b_dep);
            assert_eq!(b.i.steps(),b.i_dep);
            csv.push_str(&format!("{spec},{k},{},{},{},{},{},{},{},{},{},{},{},{:?},{},{},{},{},{},{},{},{},{}\n",b.lo,b.hi,b.reached_lo as u8,b.reached_hi as u8,b.t_start,b.t_end,b.steps,b.b_dep,b.i_dep,b.area_end,b.perim_end,b.sum_pa,b.b.complete,b.b.complete_steps,b.b.clipped_steps,b.i.complete,b.i.complete_steps,b.i.clipped_steps,tail.overlap_steps,tail.unresolved_overlap,tail.unresolved_duration_lower));
            for bin in 0..64 {
                bins.push_str(&format!("{spec},{k},I_intersect,{bin},{},{},{},{}\n",tail.completed_n[bin],tail.completed_steps[bin],tail.entered_n[bin],tail.entered_steps[bin]));
                if bin<32 { for (label,s) in [("B_contained",&b.b),("I_contained",&b.i)] {
                    bins.push_str(&format!("{spec},{k},{label},{bin},{},{},,\n",s.bin_n[bin],s.bin_steps[bin]));
                }}
            }
        }
        let mut age=String::from("spec,band,target,t,area,P,width,height,sx,sy,sxx,rim_n,immature,median_log_age,cycle_variance\n");
        for c in &r.censuses {
            age.push_str(&format!("{spec},{},{},{},{},{},{},{},{},{},{},{},{:?},{:?},{:?}\n",c.band,c.target,c.t,c.area,c.perimeter,c.geom.width,c.geom.height,c.geom.sx,c.geom.sy,c.geom.sxx,c.rim_n,c.immature,c.median_log_age,c.cycle_variance));
        }
        let mut haz=String::from("spec,band,boundary,log_age,log_visits,heading,phase,departures,transitions\n");
        for (&(k,z,age,vis,h,p),&(d,tr)) in &r.hazards {
            haz.push_str(&format!("{spec},{k},{},{age},{vis},{h},{p},{d},{tr}\n",z as u8));
        }
        let stop=match r.stop {Stop::Sink=>"complete",Stop::Budget=>"budget",Stop::Escaped=>"escaped"};
        let status=format!("spec,stop,t_final,cells,history_bytes_per_cell,n\n{spec},{stop},{},{},{},{n}\n",r.t,r.cells,std::mem::size_of::<field_age::Cell>());
        for (suffix,body) in [("bands.csv",csv),("bins.csv",bins),("age.csv",age),("hazards.csv",haz),("status.csv",status),("done",marker.clone())] {
            let tmp=out.join(format!("{name}.{suffix}.tmp"));
            std::fs::write(&tmp,body).unwrap();
            std::fs::rename(tmp,out.join(format!("{name}.{suffix}"))).unwrap();
        }
        eprintln!("{spec} {stop} t={} cells={} seconds={:.3}",r.t,r.cells,clock.elapsed().as_secs_f64());
    });
}
