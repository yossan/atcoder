// diniceさんのコードを検証
// https://atcoder.jp/contests/abc317/submissions/44983001
use itertools::Itertools;
use std::collections::HashMap;
fn main() {
	proconio::input! {
		n:usize,m:usize,abc:[(usize,usize,usize);m]
	}
	// リスト自体は双方向に非対応 
	// a, bの与えられた条件 1≤Ai<Bi≤N
	let hm: HashMap<_, _> = abc.into_iter().map(|(a, b, c)| ((a, b), c)).collect();
	let mut ans = 0;
	// permutationsで、「頂点リストの並び」の組み合わせを生成する
	// [1, 2, 3, 4], [2, 3, 4, 1], ...
	for path in (1..=n).permutations(n) {
		// 経路コスト
		let mut sum = 0;
		for edge in path.windows(2) {
			// 双方向であるため
			if let Some(c) = hm.get(&(edge[0].min(edge[1]), edge[1].max(edge[0]))) {
				sum += c;
			} else {
				break;
			}
		}
		ans = ans.max(sum);
	}
	println!("{}", ans);
}

