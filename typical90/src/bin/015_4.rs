use proconio::*;

//const MOD: usize = 1_000_000_000 + 7;
const MOD: usize = 524287;

fn main() {
	input! {
		n: usize,
	};
	let comb = Combination::new(n, MOD);
	for k in 1..=n {
		let mut cnt = 0;
		for r in 1..=n/k + 1 {
			let xr = n - ((k-1) * (r-1) + 1) + 1;
			if xr >= r {
				cnt += comb.get(xr, r);
			}
		}
		println!("{cnt}");
	}
}


// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/math/combination.rs
pub struct Combination {
	fact: Vec<usize>,
	inv_fact: Vec<usize>,
	modulo: usize,
}

impl Combination {
	pub fn new(max: usize, modulo: usize) -> Self {
		let mut inv = vec![0; max + 1];
		let mut fact = vec![0; max + 1];
		let mut inv_fact = vec![0; max + 1];
		inv[1] = 1;
		for i in 2..(max + 1) {
			inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
		}
		fact[0] = 1;
		inv_fact[0] = 1;
		for i in 0..max {
			fact[i + 1] = fact[i] * (i + 1) % modulo;
		}
		for i in 0..max {
			inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
		}
		Self {
			fact,
			inv_fact,
			modulo,
		}
	}

	pub fn get(&self, x: usize, y: usize) -> usize {
		let a = self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo;
		println!("{x}_C_{y} = {}", a);
		a
	}

	pub fn h(&self, n: usize, r: usize) -> usize {
		self.get(n + r - 1, r)
	}
}
