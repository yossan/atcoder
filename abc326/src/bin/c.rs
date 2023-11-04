/// 解説参照
/// https://atcoder.jp/contests/abc326/tasks/abc326_c
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		N: usize, // プレゼントの数
		M: usize, // 獲得範囲 x <= Ai < x + M
		mut A: [usize; N], // i番目のプレゼントの位置
	}
	A.sort();
	A.push(usize::MAX);
	
	let mut res = 0;
	let mut r = 0;
	for l in 0..N { // O(3x10^5)
		while A[r] < A[l] + M { // 
			r += 1;
		}
		res = res.max(r-l);
	}
	println!("{res}");
}
