// 参考コード
// tatarabako
// https://atcoder.jp/contests/typical90/submissions/45288808
use proconio::*;

const MOD: usize = 1_000_000_000 + 7;

struct Combinations {
	frac: Vec<usize>,
	invfrac: Vec<usize>,
	m: usize,
}

impl Combinations {
	fn new(n: usize, m: usize) -> Combinations {
		let mut frac = vec![0usize; n+1];
		let mut inv = vec![0usize; n+1];
		let mut invfrac = vec![0usize; n+1];

		inv[0] = 1;
		inv[1] = 1;
		for i in 2..=n {
			inv[i] = inv[m % i] * (m - m / i) % m;
		}

		frac[0] = 1;
		invfrac[0] = 1;
		for i in 0..n {
			frac[i + 1] = frac[i] * (i + 1) % m;
		}

		for i in 0..n {
			invfrac[i+1] = invfrac[i] * inv[i + 1] % m;
		}

		Combinations {
			frac,
			invfrac,
			m,
		}
	}

	fn n_c_r(&self, n: usize, r:usize) -> usize {
		self.frac[n] * self.invfrac[r] % self.m  * self.invfrac[n-r] % self.m
	}
}

fn main() {
	input! {
		n: usize,
	};
	let comb = Combinations::new(n, MOD);
	for k in 1..=n {
		let mut cnt = 0;
		for r in 1..=n/k + 1 {
			let xr = n - ((k-1) * (r-1) + 1) + 1;
			if xr >= r {
				cnt += comb.n_c_r(xr, r);
				cnt %= MOD;
			}
		}
		println!("{cnt}");
	}
}
