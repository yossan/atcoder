/*
1 から N までの番号が付いた N 人のプレイヤーが総当たり戦をしました。この総当たり戦で行われた試合全てについて、二人の一方が勝ち、もう一方が負けました。

総当たり戦の結果は N 個の長さ N の文字列 S1 ,S2,…,SNによって以下の形式で与えられます。

i !=j のとき、Siの j 文字目は o, x のいずれかであり、o のときプレイヤー i がプレイヤー j に勝ったことを、x のときプレイヤー i がプレイヤー j に負けたことを意味する。

i=j のとき、Siの j 文字目は - である。

総当たり戦で勝った試合数が多いほうが順位が上であり、勝った試合数が同じ場合は、プレイヤーの番号が小さいほうが順位が上となります。 N 人のプレイヤーの番号を順位が高い順に答えてください。

制約
2≤N≤100
N は整数
Siは o, x, - からなる長さ N の文字列
S1,…,SNは問題文中の形式を満たす
*/
use proconio::*;
use std::cmp::Reverse;
use std::cmp::Ordering::*;

fn main() {
	input! {
		n: usize,
		s: [marker::Chars; n],
	}
	// (oの数, プレーヤー番号)
	let mut map = vec![(0usize, 0usize); n];
	for i in 0..n {
		map[i] = s[i].iter().fold((0, 0), |acc, &c| { 
			if c == 'o' { (acc.0 + 1, i) } else { (acc.0, i) }
		});
	}
	map.sort_by(|a, b| {
		match Reverse(a.0).cmp(&Reverse(b.0)) {
			Equal => {
				a.1.cmp(&b.1)
			},
			x => {
				x
			}
		}
	});
	for i in 0..n {
		print!("{} ", map[i].1 + 1);
	}
	println!("");
}
