// 021 - Come Back in One Piece（★5）
use proconio::*;
use std::collections::BTreeMap;

type Adj = BTreeMap<usize, Vec<usize>>;

fn main() {
	input! {
		n: usize, // 頂点数
		m: usize, // 辺の数
		ab: [(usize, usize); m],
	}
	let mut adj = Adj::new();
	let mut adj_rev = Adj::new();
	for (a, b) in ab {
		adj.entry(a).or_default().push(b);
		adj_rev.entry(b).or_default().push(a);
	}
	let mut stack = Vec::new();
	let mut visited = vec![false; n+1];
	for i in 1..=n {
		dfs(i, &adj, &mut visited, &mut stack);
	}
	// strong connected componentごとにグループ化する
	let mut scc: Vec<Vec<usize>> = Vec::new();
	let mut visited = vec![false; n+1];
	stack.reverse();
	for i in stack {
		if !visited[i] {
			let mut g = Vec::new();
			dfs(i, &adj_rev, &mut visited, &mut g);
			scc.push(g);
		}
	}
	// 各グループごとにペアの数を計算する nC2
	let mut cnt = 0;
	for g in scc {
		let n = g.len();
		cnt += n * (n-1) / 2;
	}
	println!("{cnt}");
}

// 帰りがけに頂点をスタックに格納する
fn dfs(v: usize, adj: &Adj, visited: &mut [bool], stack: &mut Vec<usize>) {
	if visited[v] { return;}
	visited[v] = true;
	if let Some(list) = adj.get(&v) {
		for &next in list {
			if !visited[next] {
				dfs(next, adj, visited, stack);
			}
		}
	}
	stack.push(v);
}
