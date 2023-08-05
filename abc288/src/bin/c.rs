use proconio::*;
use std::collections::BTreeMap;

type AdjList = BTreeMap<usize, Vec<usize>>;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut adj: AdjList = BTreeMap::new();
    for _ in 0..m {
        input! {
            ab: (usize, usize),
        }
        let (a, b) = ab;
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }
    let mut record = vec![false; n];
    // 連結成分の数を数える
    let mut s = 0;
    for i in 1..=n {
        if !record[i-1] {
            s += 1;
            // 連結成分をrecordに記録する
            dfs(&adj, &mut record, i);
        }
    }
    let (m, n, s) = (m as isize, n as isize, s as isize);
    println!("{}", m - n + s);
}
fn dfs(adj: &AdjList, record: &mut Vec<bool>, pos: usize) {
    if let None = adj.get(&pos) {
        return;
    }
    let nexts = adj.get(&pos).unwrap();
    for &next in nexts {
        if !record[next-1] {
            record[next-1] = true;
            dfs(adj, record, next);
        }
    }
}
