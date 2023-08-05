use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [[usize; n]; m],
    }
    let mut g: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n+1];
    for a in aa {
        let mut w = a.windows(2);
        while let Some(&[a, b]) = w.next() {
            g[a].insert(b);
            g[b].insert(a);
        }
    }
    let mut ans = 0;
    g.remove(0);
    
    for x in &g {
        ans += n-x.len()-1;
    }
    println!("{}", ans/2);
}
