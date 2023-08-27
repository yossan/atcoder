use proconio::*;
use proconio::marker::*;

type Adj = Vec<Vec<isize>>;
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, isize); m],
    }

	let mut adj = vec![vec![-1isize; n]; n];
    for (a, b, c) in abc {
		adj[a][b] = c;
		adj[b][a] = c;
    }
	let mut ans = 0;
	for i in 0..n {
		let mut visited = vec![false; n];
		let mut ret = 0;
		dfs(i, 0, &adj, &mut visited, &mut ret);
		ans = ans.max(ret);
	}
	println!("{ans}");
}

fn dfs(v: usize, sum: isize, adj: &Adj, visited: &mut Vec<bool>, ans: &mut isize) {
	visited[v] = true;
	 
	for i in 0..visited.len() {
		if adj[v][i] != -1 && // 経路が存在する
			!visited[i]       // 探索済み
		{
			// 頂点iへ
			dfs(i, sum + adj[v][i], adj, visited, ans);
			// 頂点iから一周して戻ってきたら戻す
			visited[i] = false;
			*ans = (*ans).max(sum + adj[v][i]);
		}
	}
}

