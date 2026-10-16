use std::collections::{HashMap, HashSet};
use proptest::prelude::*;
use turmite::{Rule, DIRS, reference, field_age};
use turmite::walkers::{PatchedAnt, Walker, Rotor, RandomWalk};
use turmite::fixed_area::{State, Geom};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(40))]
    #[test]
    fn histories_and_census_match_sparse_reference(bits in prop::collection::vec(any::<bool>(),2..14), seed in any::<u64>()) {
        let rule: Rule=bits.iter().map(|b|if *b {'R'} else {'L'}).collect::<String>().parse().unwrap();
        let init=PatchedAnt::new(&rule,seed,2);
        let mut ant=reference::Ant::new(rule.clone());
        for y in -2..=2 { for x in -2..=2 {
            let c=init.initial_cell(x,y);
            if c!=0 { ant.cells.insert((x as i32,y as i32),c); }
        }}
        let mut seen=HashSet::from([(0i32,0i32)]);
        let mut born=HashMap::from([((0i32,0i32),0u64)]);
        let mut visits=HashMap::<(i32,i32),u64>::new();
        field_age::drive(init,128,1500, |f| {
            assert_eq!(f.pos,(ant.pos.0 as i64,ant.pos.1 as i64));
            assert_eq!(f.heading,ant.heading);
            assert_eq!(f.phase,ant.colour_at(ant.pos));
            assert_eq!(f.state.area,seen.len() as u64);
            let boundary=|p:(i32,i32)| DIRS.iter().any(|&(dx,dy)|!seen.contains(&(p.0+dx,p.1+dy)));
            assert_eq!(f.state.z,boundary(ant.pos));
            assert_eq!(f.state.perimeter,seen.iter().filter(|&&p|boundary(p)).count() as u64);
            for c in f.cells {
                let p=((c.idx%128) as i32-64,(c.idx/128) as i32-64);
                assert_eq!(c.born,born[&p]);
                assert_eq!(c.visits,*visits.get(&p).unwrap_or(&0));
                assert_eq!(c.initial,PatchedAnt::new(&rule,seed,2).initial_cell(p.0 as i64,p.1 as i64));
            }
            let c=&f.cells[f.current];
            assert_eq!((c.initial as u64+c.visits)%rule.len() as u64,f.phase as u64);
            if f.state.t%100==0 {
                let cs=field_age::census(&f,0,f.state.area,rule.len() as u64);
                let mut ages=Vec::new(); let mut cycles=Vec::new(); let mut young=0;
                for &p in &seen {
                    let rim=(-5i32..=5).any(|dx|(-5i32..=5).any(|dy|dx.abs()+dy.abs()<=5 && !seen.contains(&(p.0+dx,p.1+dy))));
                    if rim {
                        ages.push((f.state.t-born[&p]) as f64);
                        let v=*visits.get(&p).unwrap_or(&0);
                        young+=usize::from(v<rule.len() as u64);
                        cycles.push((v/rule.len() as u64) as f64);
                    }
                }
                ages.sort_by(f64::total_cmp);
                let m=ages.len();
                let med=if m%2==1 {ages[m/2].ln_1p()} else {(ages[m/2-1].ln_1p()+ages[m/2].ln_1p())/2.0};
                let mu=cycles.iter().sum::<f64>()/m as f64;
                let var=cycles.iter().map(|v|(v-mu).powi(2)).sum::<f64>()/m as f64;
                assert_eq!(cs.rim_n,m);
                assert_eq!(cs.immature,young as f64/m as f64);
                assert_eq!(cs.median_log_age,med);
                assert!((cs.cycle_variance-var).abs()<1e-9);
            }
            *visits.entry(ant.pos).or_default()+=1;
            ant.step();
            seen.insert(ant.pos); born.entry(ant.pos).or_insert(ant.steps);
            false
        });
    }

    #[test]
    fn tail_fragments_match_offline(labels in prop::collection::vec(any::<bool>(),10..300)) {
        let mut acc=field_age::Tails::new(&[(3,7),(9,13)]);
        for (t,&z) in labels.iter().enumerate() {
            acc.feed(State {t:t as u64,z,area:1+t as u64/3,perimeter:1,geom:Geom::default()});
        }
        let out=acc.finish();
        let end=labels.len()-1;
        for (k,(lo,hi)) in [(6usize,18usize),(24,36)].into_iter().enumerate() {
            let mut want=field_age::Tail::default();
            let mut s=0;
            while s<end {
                let z=labels[s];
                let mut e=s+1;
                while e<labels.len() && labels[e]==z { e+=1; }
                let resolved=e<labels.len();
                e=e.min(end);
                if !z {
                    let overlap=e.min(hi).saturating_sub(s.max(lo));
                    if overlap>0 {
                        want.overlap_steps+=overlap as u64;
                        let duration=(e-s) as u64;
                        if resolved && s>0 {
                            let b=(63-duration.leading_zeros()) as usize;
                            want.completed_n[b]+=1; want.completed_steps[b]+=duration;
                            if s>=lo && s<hi {want.entered_n[b]+=1;want.entered_steps[b]+=duration;}
                        } else {
                            want.unresolved_overlap+=overlap as u64;
                            want.unresolved_duration_lower=duration;
                            if s==0 {want.left_entry_unknown+=overlap as u64;}
                        }
                    }
                }
                s=e;
            }
            assert_eq!(out[k],want);
        }
    }
}

#[test]
fn controls_match_reference_and_existing_accounting() {
    for seed in 41..45 {
        let mut rotor=reference::Rotor::new(seed);
        field_age::drive(Rotor{seed},128,3000,|f|{
            assert_eq!(f.pos,(rotor.pos.0 as i64,rotor.pos.1 as i64));
            rotor.step(); false
        });
        let mut rw=reference::RandomWalk::new(seed);
        field_age::drive(RandomWalk{state:seed},128,3000,|f|{
            assert_eq!(f.pos,(rw.pos.0 as i64,rw.pos.1 as i64));
            rw.step(); false
        });
        let bands=[(64,80),(256,300)];
        let old=turmite::fixed_area::run(Rotor{seed},256,100000,&bands);
        let new=field_age::run(Rotor{seed},4,256,100000,&bands);
        assert_eq!(new.bands,old.bands);
        for (k,b) in new.bands.iter().enumerate() {
            assert_eq!(b.i_dep,new.tails[k].overlap_steps);
            let mut departures=0;
            for (&(band,_,_,_,_,_),&(count,transitions)) in &new.hazards {
                assert!(transitions<=count);
                if band==k {departures+=count;}
            }
            assert_eq!(departures,b.steps);
        }
    }
}

#[test]
fn follows_open_tail_beyond_band_and_reports_unresolved_mass() {
    let stream=[true,true,false,false,false,false,true,true];
    let make=|cap:usize| {
        let mut t=field_age::Tails::new(&[(2,5)]);
        for (i,&z) in stream[..=cap].iter().enumerate() {t.feed(State{t:i as u64,z,area:i as u64+1,perimeter:1,geom:Geom::default()});}
        t.finish().remove(0)
    };
    let full=make(7);
    assert_eq!(full.overlap_steps,2);
    assert_eq!(full.completed_steps.iter().sum::<u64>(),4);
    assert_eq!(full.entered_steps.iter().sum::<u64>(),4);
    let cut=make(4);
    assert_eq!(cut.unresolved_overlap,2);
    assert_eq!(cut.unresolved_duration_lower,2);
}
