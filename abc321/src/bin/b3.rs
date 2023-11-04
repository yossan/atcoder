/// ２分探索バージョン
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		N: usize, // Nラウンド
		X: isize, // 必要ポイント
		A: [isize; N-1], // N-1ラウンドまでの得点
	}
	// Nラウンド目に必要な得点を２分探索で探す
	let mut l = 0;
	let mut r = 100;
	while l <= r  {
		let te = (l + r ) / 2;
		// Nラウンド目のスコア
		let mut S = A.clone();
		S.push(te);

		// 最大値と最小値と合計得点
		let mut min = 100;
		let mut max = 0;
		let mut sum = 0;
		for i in 0..N {
			sum += S[i];
			min = min.min(S[i]);
			max = max.max(S[i]);
		}
		// スコア = 全合計 - min - max
		let score = sum - min - max;
		if score >= X {
			r = te - 1;
		} else {
			l = te + 1;
		}
	}
	if l > 100 {
		println!("-1");
	} else {
		println!("{}", l);
	}
}
