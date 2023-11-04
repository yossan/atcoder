/// D - Set Menu
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		N: usize, // 主菜の種類
		M: usize, // 副菜の種類
		P: usize, // 定食の基本価格
		A: [usize; N], // 主菜iの価格
		mut B: [usize; M], // 副菜iの価格
	}
	// 定食メニュー (主菜1, 副菜1) = s
	// 定食価格 = min(s, P) 

	// Bをソートして累積和を求めておく
	B.sort();
	let mut ZB = vec![0; M];
	ZB[0] = B[0];
	for j in 1..M {
		ZB[j] = ZB[j-1] + B[j];
	}

	let mut sum = 0;
	for i in 0..N {
		let mut s = None;
		if P >= A[i] {
			let k = B.binary_search(&(P-A[i])).unwrap_or_else(|e| e);
			if k > 0 {
				s = Some(k * A[i] + ZB[k-1] + (M-k) * P);
			}
		}
		sum += s.unwrap_or_else(|| M * P);

	}
	println!("{sum}");
}

