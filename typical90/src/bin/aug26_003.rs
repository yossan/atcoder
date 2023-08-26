use proconio::*;
use proconio::marker::*;
use std::collections::BTreeMap;

type Adj = BTreeMap<usize, Vec<usize>>;

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }
    
    let mut adj = Adj::new();
    for (a, b) in ab {
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }
    // 木の距離の導出
    // 1. 任意の点uからの最も遠い点vを求める
    // 2. 点vから最も遠い距離dを求める = 木の距離

    let dist_u = dfs(0, n, &adj);

    let v = dist_u.iter()
        .enumerate()
        .max_by(|(_, a), (_, b) | a.cmp(b)).unwrap().0;
    
    let dist_v = dfs(v, n, &adj);
    let d = dist_v.iter().max().unwrap();
    println!("{}", d + 1);
}

// 各頂点の距離を計算する
fn dfs(start: usize, n: usize, adj: &Adj) -> Vec<usize> {
    let mut visited = vec![std::usize::MAX; n];
    visited[start] = 0;
    let mut stack = Vec::new();
    stack.push(start);
    while !stack.is_empty() {
        let pos = stack.pop().unwrap();
        if let Some(list) = adj.get(&pos) {
            for &next in list {
                if visited[next] == std::usize::MAX {
                    visited[next] = visited[pos] + 1;
                    stack.push(next);
                }
            }
        }
    }
    visited
}
