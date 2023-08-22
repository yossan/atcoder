// Passing ★5
use proconio::*;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};


struct Edge {
    node: usize,
    cost: usize,
}

type AdjList = BTreeMap<usize, Vec<Edge>>;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }
    let mut adj = AdjList::new();
    for (a, b, c) in abc {
        adj.entry(a).or_default().push ( Edge { node: b, cost: c } );
        adj.entry(b).or_default().push ( Edge { node: a, cost: c } );
    }
    let from_1 = shortest_path(&adj, 1, n);
    let from_n = shortest_path(&adj, n, n);
    // k番目を経由した最短経路を出力する
    for k in 1..=n {
        println!("{}", from_1[k] + from_n[k]);
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


// Dijkstra's shortest path algorithm.

fn shortest_path(adj_list: &AdjList, from: usize, n: usize) -> Vec<usize> {
    let mut dist = vec![std::usize::MAX; n+1];

    let mut heap = BinaryHeap::new();

    dist[from] = 0;

    heap.push(State { cost: 0, position: from });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] { continue; }

        let edges = adj_list.get(&position).unwrap();
        for edge in edges {
            let next = State { cost: cost + edge.cost, position: edge.node };
            // コストを更新する
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }

    }
    dist
}
