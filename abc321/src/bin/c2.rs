use proconio::*;

#[allow(non_snake_case)]
fn main() {
	// K番目に小さい321-like数を求める
	// 321-like数
	// 9876543210 の各桁をbit全探索(2^10 = 1024通り)
	// 最大値を9_876_543_210 > 90億なのでNG

	let mut ans = Vec::new();
	// 必ず1つは選ぶ
	for i in 2..(1usize << 10) {
		let mut x = 0;
		// 10桁
		for j in (0usize..10).rev() {
			if (i >> j) % 2 == 1 {
				x *= 10;
				x += j;
			}
		}
		if x != 0 {
			ans.push(x);
		}
	}

	input! {
		K: usize, 
	}
	ans.sort();

	println!("{}", ans[K-1]);
}
