/// D - Minimum Width
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		N: u64, // 単語数
		M: u64, // M行で表示された
		L: [u64; N], // i番目の単語の横幅
	}
	// ウィンドウの横幅としてあり得る最小の幅

	// 横幅widthでの行数
	let check = |width: u64| {
		let mut length = L[0];
		let mut line = 1;
		for i in 1..N as usize {
			let lw = L[i];
			length += lw + 1;
			if length > width {
				// 改行する
				line += 1;
				// 先頭に追加
				length = lw;
			}
		}
		// M行より大きいかどうか
		line > M
	};

	// 二分探索でminimumを探していく
	// 答えは max(L[i])以下
	let mut lower = L.iter().max().unwrap() - 1;
	// 答えは sum(L[i]) + N(スペース) 以下 
	// (N-1ではないので、先頭にスペースが有ることになる)
	let mut upper = L.iter().sum::<u64>() + N;
	while lower + 1 < upper {
		let middle = (lower + upper ) / 2;
		if check(middle) {
			// M行より大きい
			lower = middle;
		} else {
			// M行以下
			upper = middle;
		}
	}
	println!("{}", upper);
}
