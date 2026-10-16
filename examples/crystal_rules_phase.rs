//! Paired online ray interventions; registered protocol in crystal-rules README.
use turmite::{Rule, DIRS};

#[derive(Clone)]
struct Board { size: i32, cells: Vec<u8>, pos: (i32,i32), heading: u8, rule: Rule }
impl Board {
    fn new(rule: Rule, size: i32) -> Self {
        Self { size, cells: vec![0; (size*size) as usize], pos:(0,0), heading:0, rule }
    }
    fn index(&self, p:(i32,i32)) -> usize {
        let (x,y)=(p.0+self.size/2,p.1+self.size/2);
        assert!(x>=0 && y>=0 && x<self.size && y<self.size, "board exit");
        (y*self.size+x) as usize
    }
    fn step(&mut self) -> u8 {
        let i=self.index(self.pos); let c=self.cells[i];
        let turn=self.rule.0[c as usize].delta();
        self.heading=(self.heading+turn)&3;
        self.cells[i]=(c+1)%self.rule.len() as u8;
        let d=DIRS[self.heading as usize]; self.pos=(self.pos.0+d.0,self.pos.1+d.1); turn
    }
    fn reset(&mut self, phase:u8, radius:i32) -> usize {
        let mut n=0;
        for y in self.pos.1-radius..=self.pos.1+radius {
            for x in self.pos.0-radius..=self.pos.0+radius {
                let i=self.index((x,y));
                if self.cells[i]==2 && phase!=2 { self.cells[i]=phase; n+=1; }
            }
        } n
    }
}
fn diagonal(a:(i32,i32), b:(i32,i32)) -> Option<(i32,i32)> {
    let (x,y)=(b.0-a.0,b.1-a.1);
    (x.abs()>=16 && (x.abs()-y.abs()).abs()<=2 && y!=0).then_some((x.signum(),y.signum()))
}
#[derive(Debug, PartialEq)]
struct Metrics { alt:usize, early:usize, end:usize, censored:bool, outside:usize }
fn metrics(path:&[(i32,i32)], turns:&[u8], h:(i32,i32), radius:i32) -> Metrics {
    let n=turns.len(); let origin=path[0];
    let end=(128..=n).find(|&t|diagonal(path[t-128],path[t])!=Some(h));
    Metrics { alt:turns.windows(2).filter(|w|w[0]!=w[1]).count(),
        early:turns[..128].windows(2).filter(|w|w[0]!=w[1]).count(),
        end:end.unwrap_or(n), censored:end.is_none(),
        outside:path[1..].iter().filter(|p|(p.0-origin.0).abs()>radius || (p.1-origin.1).abs()>radius).count() }
}
fn replay(mut b:Board) -> (Vec<(i32,i32)>,Vec<u8>) {
    let mut p=vec![b.pos]; let mut c=vec![];
    for _ in 0..2048 {c.push(b.step());p.push(b.pos);} (p,c)
}
fn main() {
    let a:Vec<String>=std::env::args().collect();
    assert_eq!(a.len(),2,"RULE");
    let mut b=Board::new(a[1].parse().unwrap(),8192);
    let mut ring=[(0,0);129]; let mut last=None; let mut streak=0;
    let mut selected=0; let mut prev=0;
    println!("rule,trial,t,hx,hy,phase,edited,alt_fraction,early_alt_fraction,termination,censored,prefix,outside");
    for t in 1..=1u64<<25 {
        b.step(); ring[t as usize%129]=b.pos;
        if t<128 {continue;}
        let h=diagonal(ring[(t as usize-128)%129],b.pos);
        if h.is_some() && h==last {streak+=1;} else {streak=usize::from(h.is_some());} last=h;
        if t<1<<18 || t-prev<8192 || streak<256 {continue;}
        let h=h.unwrap(); selected+=1; prev=t;
        let (base,_) = replay(b.clone());
        for phase in [2,0,3] {
            let mut arm=b.clone(); let edited=arm.reset(phase,128);
            let (path,turns)=replay(arm); let m=metrics(&path,&turns,h,128);
            let prefix=path.iter().zip(&base).skip(1).take_while(|(x,y)|x==y).count();
            println!("{},{selected},{t},{},{},{phase},{edited},{:.9},{:.9},{},{},{prefix},{}",a[1],h.0,h.1,m.alt as f64/2047.,m.early as f64/127.,m.end,m.censored,m.outside);
        }
        if selected==16 {break;}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    #[test]
    fn calibrate_metrics() {
        let p:Vec<_>=(0..=2048).map(|t|((t+1)/2,t/2)).collect();
        let c:Vec<_>=(0..2048).map(|t|if t%2==0 {1}else{3}).collect();
        let m=metrics(&p,&c,(1,1),128);
        assert_eq!(m.alt,2047); assert!(m.censored); assert_eq!(m.end,2048);
        let axial:Vec<_>=(0..=2048).map(|t|(t,0)).collect();
        let m=metrics(&axial,&vec![1;2048],(1,1),128);
        assert_eq!(m.alt,0); assert_eq!(m.end,128); assert!(!m.censored);
    }
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(32))]
        #[test]
        fn edited_board_matches_oracle(bits in 0u32..4096,n in 4usize..13,phase in prop_oneof![Just(0u8),Just(2),Just(3)]) {
            let rule=Rule::from_bits(bits,n);
            let mut b=Board::new(rule.clone(),128);
            let mut o=turmite::reference::Ant::new(rule);
            // A deterministic heterogeneous medium exercises every phase, including colour 2.
            for y in -24..=24 {for x in -24..=24 {
                let c=((x*x+3*y*y+7) as usize%n) as u8;
                let i=b.index((x,y));b.cells[i]=c;o.cells.insert((x,y),c);
            }}
            let before=b.clone();b.reset(phase,12);
            for y in -63i32..63 {for x in -63i32..63 {
                if x.abs()<=12 && y.abs()<=12 && o.colour_at((x,y))==2 {o.cells.insert((x,y),phase);}
                prop_assert_eq!(b.cells[b.index((x,y))],o.colour_at((x,y)));
                if phase==2 {prop_assert_eq!(b.cells[b.index((x,y))],before.cells[before.index((x,y))]);}
            }}
            for _ in 0..512 {
                let p=b.pos;b.step();o.step();prop_assert_eq!(b.pos,o.pos);prop_assert_eq!(b.heading,o.heading);
                prop_assert_eq!(b.cells[b.index(p)],o.colour_at(p));
            }
        }
    }
}
