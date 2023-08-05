use proconio::*;
use std::collections::BTreeMap;

type Adj = BTreeMap<usize, Vec<usize>>;

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; m],
    }
    // 隣接リスト
    let mut adj: Adj = BTreeMap::new();
    for i in 1..=n {
        let v = adj.entry(i).or_default();

        for a in &aa {
            if *a == i {
                v.push(i+1);
                (*adj.entry(i+1).or_default()).push(i);
                break;
            }
        }
    }

    let mut visited = vec![false; n+1];
    let mut ans = Vec::new();
    // 深さ優先探索
    for v in adj.keys() {
        dfs(&adj, &mut visited, *v, &mut ans);
    }
    for v in ans {
        print!("{} ", v);
    }
    println!("");
}
fn dfs(g: &Adj, visited: &mut Vec<bool>, v: usize, ans: &mut Vec<usize>) {
    if visited[v] {
        return;
    }
    visited[v] = true;
    for next_v in &g[&v] {
        if !visited[*next_v] {
            dfs(&g, visited, *next_v, ans);
        }
    }
    ans.push(v);
}
