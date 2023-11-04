use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		mut b: u64,
	}
	if b == 1 {
		println!("-1");
		return;
	}
	// 素因数分解
	let mut nums = Vec::new();
	let mut p = 2;
	while p * p <= b {
		if b % p != 0 {
			p += 1;
			continue;
		}

		let mut e = 0;
		while b % p == 0 {
			e += 1;
			b /= p;
		}
		nums.push((p, e));
	}
	if b != 1 {
		nums.push((b, 1));
	}

	println!("nums = {:?}", nums);

}
