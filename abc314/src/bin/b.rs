/// B - Roulette
/*
N人でルーレットをする。
ルーレットの出目は0~36までの37個の整数値。
人iは、37個の出目のうち、 C_i 個の目 A_{i,1}, A_{i,2}, .. , A{i, Ci} に掛ける。

ルーレットが回され出目は X であった。

X に賭けた人たちのうち、賭けた目の個数が最も少ない人たちの番号を昇順にすべて出力してください。

制約

* 1≤N≤100
* 1≤Ci≤37
* 0≤Ai,j≤36
* 任意の i=1,2,…,N について、Ai,1,Ai,2,…,Ai,Ciはすべて異なる。
* 0≤X≤36
* 入力はすべて整数
*/
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		N: usize,
	}
	// ルーレットの記録
	type Adj = Vec<Vec<usize>>;
	let mut roulette: Adj = vec![Vec::new(); 38];

	// プレイヤーがかけた目の個数を記録
	let mut player = vec![0usize; 101];

	for i in 1..=N {
		input! {
			C: usize, // 掛けた目の数
		}

		player[i] = C;

		for _ in 0..C {
			input! {
				A: usize,
			}
			roulette[A].push(i);
		}
	}

	input! {
		X: usize, // 出目
	}

	// 掛けた目の個数の最低値
	let mut cmin = usize::MAX;
	for &x in &roulette[X] {
		cmin = cmin.min(player[x]);
	}
	let mut wins = Vec::new();
	for &x in &roulette[X] {
		if player[x] <= cmin {
			wins.push(x);
		}
	}
	println!("{}", wins.len());
	for p in wins {
		print!("{p} ");
	}
	println!("");
}
