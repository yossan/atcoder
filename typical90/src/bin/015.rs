// 条件を満たす部分集合の数
// DFS探索バージョン
use proconio::*;

const MOD: usize = 1_000_000_000 + 7;

fn power<T>(a: T, b: T) -> T {
	let mut p = a;
	let mut ans = 1;
}

fn n_c_r(n: usize, r: usize) -> usize {
	0
}

fn main() {
	input! {
		n: usize,
	}

	let mut ans = vec![0; n];
	for k in (0..n).rev() {
		let mut visited = vec![0; n];
		for i in (0..n).rev() {
			// kが確定する
			let cnt = dfs(i, k, n, &visited);
			visited[i] = cnt;
			ans[k] += visited[i] % MOD;
		}
	}
	for i in 0..n {
		println!("{}", (ans[i] + n) % MOD);
	}
}


fn dfs(v: usize, k: usize, n:usize, visited: &Vec<usize>) -> usize {
	let mut cnt = 0;
	for i in (v+1..n).rev() {
		if i - v <= k {
			continue;
		}
		cnt += 1 + visited[i] % MOD;
	}
	cnt
}
