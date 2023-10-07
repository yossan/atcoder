// 参考 sansen 
// https://atcoder.jp/contests/abc323/submissions/46288823

/*
N 人のプレイヤーが参加するプログラミングコンテスト World Tour Finals が行われており、競技時間の半分が過ぎました。
このコンテストでは M 問の問題が出題されており、問題 i の点数 Aiは 500 以上 2500 以下の 100 の倍数です。

各 i=1,…,N について、プレイヤー i がどの問題を既に解いたかを表す文字列 Siが与えられます。 
Siは o, x からなる長さ M の文字列で、Siの j 文字目が o のときプレイヤー i は問題 j を既に解いており、x のときまだ解いていません。 
ただし、どのプレイヤーもまだ全ての問題を解いてはいません。

プレイヤー i の総合得点は、解いた問題の点数の合計に、ボーナス点 i 点を加えた点数として計算されます。
さて、各 i=1,…,N について以下の質問に答えてください。

* プレイヤー i がまだ解いていない問題を少なくとも何問解くことで、プレイヤー i の総合得点が他のプレイヤー全員の現在の総合得点を上回ることができますか？

なお、問題文中の条件と制約から、プレイヤー i が全ての問題を解くことで、他のプレイヤー全員の現在の総合得点を上回ることができることが証明できます。 このことから、答えは常に定義されることに注意してください。
*/
use proconio::*;

fn main() {
	input! {
		n: usize, // プレーヤー数
		m: usize, // 問題数
		a: [usize; m], // 各問題のスコア
		s: [marker::Chars; n], // 各プレーヤーの結果ox
	}

	let scores = s
		.iter()
		.enumerate()
		.map(|(i, s)| {
			i + 1
				+ s.iter()
				.zip(a.iter())
				.filter(|p| *p.0 == 'o')
				.map(|p| *p.1)
				.sum::<usize>()
		})
	.collect::<Vec<_>>();

	let max = *scores.iter().max().unwrap();

	// 各問題のスコアの順序
	let mut order = Vec::from_iter(0..m);
	// sort_byではない
	order.sort_by_key(|x| {
		a[*x]
	});

	for i in 0..n {
		let mut score = scores[i];
		let mut ans = 0;
		// スコアが大きい順に取り出す
		for &x in order.iter().rev() {
			if score >= max {
				break;
			}
			// 解いたことがない問題かどうか
			if s[i][x] == 'x' {
				score += a[x];
				ans += 1;
			}
		}
		println!("{ans}");
	}

}
