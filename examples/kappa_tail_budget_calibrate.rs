//! Known-answer calibration of the actual tail accountant, no ant fitting.
use turmite::field_age::Tails;
use turmite::fixed_area::{State,Geom};
use turmite::walkers::splitmix_next;

fn geometric(state: &mut u64, denominator: u64) -> u64 {
    let mut n=1;
    while splitmix_next(state)&(denominator-1)!=0 {n+=1;}
    n
}

fn main() {
    println!("family,seed,cycles,known_mean,measured_mean,relative_bias,known_occupation,measured_occupation,accounting_ok");
    for (family,known,serial) in [("geometric",4.0,false),("mixture",11.5,false),("serial_mixture",11.5,true)] {
        for seed in 2026101561..2026101577 {
            let mut rng=seed;
            let mut runs=Vec::new();
            let (mut d,mut interior)=(0,0);
            for i in 0..20000 {
                if !serial || i%4==0 {
                    d=geometric(&mut rng,2);
                    let denominator=if family!="geometric" && splitmix_next(&mut rng)&7==0 {64} else {4};
                    interior=geometric(&mut rng,denominator);
                }
                runs.push((d,interior));
            }
            let total=runs.iter().map(|(d,t)|d+t).sum::<u64>();
            let mut acc=Tails::new(&[(2,total+1)]);
            let mut t=0;
            for &(d,i) in &runs {for (z,len) in [(true,d),(false,i)] {for _ in 0..len {
                acc.feed(State{t,z,area:t+1,perimeter:1,geom:Geom::default()}); t+=1;
            }}}
            acc.feed(State{t,z:true,area:t+1,perimeter:1,geom:Geom::default()});
            let out=acc.finish().remove(0);
            let steps=out.entered_steps.iter().sum::<u64>();
            let count=out.entered_n.iter().sum::<u64>();
            assert_eq!(count,20000);
            assert_eq!(steps,runs.iter().map(|r|r.1).sum::<u64>());
            assert_eq!(out.overlap_steps,steps);
            assert_eq!(out.unresolved_overlap,0);
            let mean=steps as f64/count as f64;
            let b=runs.iter().map(|r|r.0).sum::<u64>();
            println!("{family},{seed},20000,{known},{mean},{},{},{},true",mean/known-1.0,2.0/(2.0+known),b as f64/total as f64);
        }
    }
}
