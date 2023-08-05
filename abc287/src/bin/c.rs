use proconio::*;
use std::collections::BTreeMap;

type Adj = BTreeMap<usize, Vec<usize>>;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut adj: Adj = BTreeMap::new();
    for i in 0..m {
        let (u, v) = uv[i];
        adj.entry(u).or_default().push(v);
        adj.entry(v).or_default().push(u);
    }
    println!("ajd = {:?}", adj);
    let mut visited = vec![false; n];
    let mut deep = vec![false; n];
    let ans = dfs(1, 1, &adj, &mut visited &mut deep);
    println!("{}", if ans { "Yes" } else { "No" });
}

fn dfs(pos: usize, k: usize, adj: &Adj, visited: &mut Vec<bool>, deep: &mut Vec<bool>) {
    if deep[k-1] {
        return;
    }
    deep[k-1] = true;
    if k == deep.len() {
    }
    if !visited[pos] {
        visited[pos-1] = true;
        if let Some(nexts) = adj.get(&pos) {
            dfs(nexts[0], k+1, adj, visited, deep)
        }
    }
}
