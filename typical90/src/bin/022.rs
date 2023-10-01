// 022 - Cubic Cake（★2）
use proconio::*;

fn main() {
	input! {
		a: usize,
		b: usize,
		c: usize,
	}
	let r = gcd(a, gcd(b, c));
	println!("{}", a / r - 1 + b / r - 1 + c / r - 1);
}


fn gcd(a: usize, b: usize) -> usize {
	let (x, y) = if a >= b {
		(a, b)
	} else {
		(b, a)
	};

	if y == 0 {
		x
	} else {
		gcd(y, x % y)
	}
}
