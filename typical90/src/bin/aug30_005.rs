use proconio::*;

// MOD
const MOD: usize = 1_000_000_007;

fn main() {
	input! {
		n: usize,
		b: usize,
		k: usize,
		c: [usize; k],
	}

	let mut dp = vec![vec![0usize; b]; n];

	// 桁数
	for i in 0..n {
		println!("i = {}", i);
		// 数字種類
		for k in 0..k {
			println!("k = {}", k);
			if i == 0 {
				dp[i][c[k] % b] += 1;
			} else {
				let p = (c[k] * (10usize.pow(i as u32) % MOD)) % b;
				// 余り
				for j in 0..b {
					if dp[i-1][j] != 0 {
						dp[i][(j+p) % b] += dp[i-1][j];
						dp[i][(j+p) % b] %= MOD;
					}
				}
			}
		}
	}
	// bで割り切れる数の個数
	println!("{}", dp[n-1][0]);
	/*
	for i in 0..n {
		for j in 0..b {
			print!("{} ", dp[i][j]);
		}
		println!("");
	}
	*/

}

