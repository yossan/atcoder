use proconio::*;
use proconio::marker::*;
use std::collections::{BTreeMap, BinaryHeap};
use std::cmp::Ordering;

type AdjList = BTreeMap<usize, Vec<Edge>>;

#[derive(Debug, PartialEq, Eq, Ord)]
struct Edge {
    node: usize,
    cost: usize,
}

impl PartialOrd for Edge {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cost.cmp(&other.cost))
	}
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }
    let mut adj = AdjList::new();
    for (a, b, c) in abc {
        adj.entry(a).or_default().push ( Edge { node: b, cost: c } );
        adj.entry(b).or_default().push ( Edge { node: a, cost: c } );
    }

	let mut dist = vec![0usize; n];
	let mut stack = BinaryHeap::new();
	stack.push(Edge { node: 0, cost: 0 });
	while let Some(Edge { node, cost } ) = stack.pop() {
		println!("kakutei = {}", node);
		if let Some(list) = adj.get(&node) {
			for next in list {
				if dist[next.node] < next.cost + cost {
					dist[next.node] = next.cost + cost;
					stack.push(Edge { node: next.node, cost: dist[next.node] });
				}
			}
		}
	}
}
