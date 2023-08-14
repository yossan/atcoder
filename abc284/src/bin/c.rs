// Count Connected Components
use proconio::*;
use std::collections::BTreeMap;

type Adj = BTreeMap<usize, Vec<usize>>;

fn main() {
    input! {
        n: usize, // 頂点が1~n
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut adj = Adj::new();
    for (u, v) in uv {
        adj.entry(u).or_default().push(v);
        adj.entry(v).or_default().push(u);
    }
    let mut cnt = 0;
    let mut visited = vec![false; n+1];
    for i in 1..=n {
        if visited[i] {
            continue;
        }
        cnt += 1;
        dfs(i, &adj, &mut visited);
    }
    println!("{}", cnt);
}

fn dfs(pos: usize, adj: &Adj, visited: &mut [bool]) {
    visited[pos] = true;
    if let Some(next_list) = adj.get(&pos) {
        for &next in next_list {
            if !visited[next] {
                dfs(next, adj, visited);
            }
        }
    }
}

