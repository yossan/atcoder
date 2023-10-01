// 021 - Come Back in One Piece（★5）
use proconio::*;
use itertools::*;
use std::collections::{BTreeMap, BTreeSet};

type AdjList = BTreeMap<usize, Vec<usize>>;

fn main() {
	input! {
		n: usize,
		m: usize,
		ab: [(usize, usize); m],
	}

	let mut adj = AdjList::new();
	for (a, b) in ab {
		adj.entry(a).or_default().push(b);
		// adj.entry(b).or_default().push(a);
	}

	/*
	let mut cnt = 0;

	for nodes in (1..=n).combinations(2) {
		let mut w = nodes.windows(2);
		if let Some(&[a, b]) = w.next() {
			let mut visited = vec![false; n+1];
			// aからb
			if dfs(a, b, &adj, &mut visited) {
				let mut visited = vec![false; n+1];
				// bからa
				if dfs(b, a, &adj, &mut visited) {
					cnt += 1;
				}
			}
		}
	}
	println!("{cnt}");
	*/

	/*
	let mut visiteds = vec![BTreeSet::new(); n+1];
	for i in 1..=n {
		dfs(i, &adj, &mut visiteds[i]);
	}
	let mut cnt = 0;
	for i in 1..=n {
		let visited = &visiteds[i];
		for &j in visited {
			if j == i {
				continue;
			}
			if visiteds[j].contains(&i) {
				cnt += 1;
			}
		}
	}
	println!("{}", cnt / 2);
	*/
}

/*
// aからの閉路探索
fn dfs(a: usize, adj: &AdjList, visited: &mut BTreeSet<usize>) {
	visited.insert(a);
	if let Some(list) = adj.get(&a) {
		for &next in list {
			if !visited.contains(&next) {
				dfs(next, adj, visited);
			}
		}
	}
}
*/

