use std::collections::BTreeSet;
use proconio::input;

fn main() {
    input! {
        h : usize,
        w : usize,
        a : [[usize; w]; h]
    }
    let mut ans = 0;
    let mut dfs = Vec::new();
    dfs.push((0usize, 0usize, BTreeSet::new()));
    while !dfs.is_empty() {
        let mut v = dfs.pop().unwrap();
        if v.2.insert(a[v.0][v.1]) {
            if v.0 + 1 < h {
                dfs.push((v.0+1, v.1, v.2.clone()));
            }
            if v.1 + 1 < w {
                dfs.push((v.0, v.1+1, v.2.clone()));
            }
            if v.0 == h - 1 && v.1 == w - 1 {
                // goal
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

