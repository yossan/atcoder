use proconio::*;
use std::collections::{BTreeMap, BinaryHeap};

type Adj = BTreeMap<usize, BinaryHeap<usize>>;

fn main() {
    input! {
        n: usize,
        fs: [(usize, usize); n],
    }

    // 解説では
    // max_sではなく、同じ味同士
    let mut max_s = (0, 0);
    let mut adj = Adj::new();
    for &(f, s) in &fs {
        if max_s.1 < s {
            max_s = (f, s);
        }
        adj.entry(f).or_default().push(s);
    }

    let mut max = 0;
    let ss = adj.get_mut(&max_s.0).unwrap();
    if ss.len() > 1 {
        ss.pop();
        let t = ss.pop().unwrap();
        max = max_s.1 + t / 2;
    }

    for f in adj.keys() {
        if *f == max_s.0 { continue }
        let t = adj.get(f).unwrap().peek().unwrap();
        let score = *t + max_s.1;
        max = max.max(score);
    }
    println!("{max}");
}

