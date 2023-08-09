use proconio::*;
use std::collections::BTreeMap;

type AdjList = BTreeMap<usize, Vec<usize>>;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut adj = AdjList::new();
    for &(a, b) in &ab {
        // 有向グラフ
        adj.entry(a).or_default().push(b);
    }
    // 根をiから順に試していく
    for i in 1..=n {
        let mut seen = vec![false; n];
        dfs(i, &adj, &mut seen);
        if seen.iter().all(|e| e == &true) {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}

fn dfs(v: usize, adj: &AdjList, seen: &mut Vec<bool>) {
    seen[v-1] = true;
    if let Some(list) = adj.get(&v) {
        for &v2 in list {
            if seen[v2-1] {
                continue;
            }
            dfs(v2, adj, seen);
        }
    }
}
