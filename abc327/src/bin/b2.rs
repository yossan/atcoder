use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		mut b: i64,
	}

	for a in 1..=15 {
		let mut x = 1;
		for _ in 0..a {
			x *= a;
		}
		if x == b {
			println!("{a}");
			return;
		}
	}
	println!("-1");
}
