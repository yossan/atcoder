// 020 - Log Inequality（★3)
use proconio::*;

fn main() {
	input! {
		a: usize,
		b: usize,
		c: usize,
	}

	if a < c.pow(b as u32) {
		println!("Yes");
	} else {
		println!("No");
	}
}
