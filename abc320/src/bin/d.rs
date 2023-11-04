/// D - Relative Position
use proconio::*;
use std::collections::BTreeMap;

type Adj = BTreeMap<usize, Vec<(usize, isize, isize)>>;

#[allow(non_snake_case)]
fn main() {
	input! {
		N: usize, // 座標平面上の人の数
		M: usize, // 与えられる情報数
		ABXY: [(usize, usize, isize, isize); M], // 人AiからBiを見たときのX,Y位置
	}
	let mut adj: Adj = Adj::new();
	for (a, b, x, y) in ABXY {
		adj.entry(a).or_default().push((b, x, y));
		adj.entry(b).or_default().push((a, -1 * x, -1 *y));
	}
	let mut visited = vec![None; N+1];
	// 人1は原点にいる
	visited[1] = Some((0, 0));
	dfs(1, &adj, &mut visited);
	for i in 1..=N {
		if let Some((x, y)) = visited[i] {
			println!("{x} {y}");
		} else {
			println!("undecidable");
		}
	}
}

fn dfs(from: usize, adj: &Adj, visited: &mut [Option<(isize, isize)>]) {
	let Some(list) = adj.get(&from) else { return; };
	for &next in list {
		if visited[next.0] == None {
			// 原点からの距離をvisitedに格納する
			let abs_x = next.1 + visited[from].unwrap().0;
			let abs_y = next.2 + visited[from].unwrap().1;
			visited[next.0] = Some((abs_x, abs_y));
			dfs(next.0, adj, visited);
		}
	}
}
