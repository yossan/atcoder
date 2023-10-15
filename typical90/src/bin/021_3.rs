#![allow(non_snake_case, unused)]

use proconio::input;

mod scc {
use std::collections::BTreeMap;
type Adj = BTreeMap<usize, Vec<usize>>;
pub struct SccGraph {
	n: usize,
	adj: Adj,
	adj_rev: Adj,
}
impl SccGraph {
	pub fn new(n: usize) -> Self {
		SccGraph {
			n,
			adj: Adj::new(),
			adj_rev: Adj::new(),
		}
	}
	pub fn add_edge(&mut self, a: usize, b: usize) {
		self.adj.entry(a).or_default().push(b);
		self.adj_rev.entry(b).or_default().push(a);
	}

	pub fn scc(&self) -> Vec<Vec<usize>> {
		let mut stack = Vec::new();
		let mut visited = vec![false; self.n+1];
		for i in 1..=self.n {
			self.dfs(i, &self.adj, &mut visited, &mut stack);
		}
		// strong connected componentごとにグループ化する
		let mut scc: Vec<Vec<usize>> = Vec::new();
		let mut visited = vec![false; self.n+1];
		for &i in stack.iter().rev() {
			if !visited[i] {
				let mut g = Vec::new();
				self.dfs(i, &self.adj_rev, &mut visited, &mut g);
				scc.push(g);
			}
		}
		scc
	}

	fn dfs(&self, v: usize, adj: &Adj, visited: &mut [bool], stack: &mut Vec<usize>) {
		if visited[v] { return;}
		visited[v] = true;
		if let Some(list) = adj.get(&v) {
			for &next in list {
				if !visited[next] {
					self.dfs(next, adj, visited, stack);
				}
			}
		}
		stack.push(v);
	}
}
}

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(usize,usize);m]
    }
    let mut g = scc::SccGraph::new(n);
    for i in 0..m {
        let (a, b) = ab[i];
        g.add_edge(a, b);
    }
    println!(
        "{}",
        g.scc()
            .iter()
            .fold(0, |acc, scc| acc + scc.len() * (scc.len() - 1) / 2)
    );
}
