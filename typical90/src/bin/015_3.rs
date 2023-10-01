// tatarabako
// https://atcoder.jp/contests/typical90/submissions/45288808

use proconio::input;
use ac_library::ModInt1000000007;

fn main() {
	input! {
		n: usize,
	};
	//let comb = Combination::new(n, ModInt1000000007::modulus() as usize);
	let comb = Combination::new(n, ModInt1000000007::modulus() as usize);
	for k in 1..=n {
		let mut rs = ModInt1000000007::default();
		println!("rs = {rs}");
		for i in 1..=(n / k + 1) {
			let s1 = n - (k - 1) * (i - 1);
			let s2 = i;
			if s2 <= s1 {
				rs += comb.get(s1, s2);
			}
		}
		println!("{rs}");
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
		assert!(x >= y);
		let a = self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo;
		println!("{x}_C_{y} = {}", a);
		a
	}

	pub fn h(&self, n: usize, r: usize) -> usize {
		self.get(n + r - 1, r)
	}
}
