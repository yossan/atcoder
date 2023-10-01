// 016 - Minimum Coins（★3）

use proconio::*;

fn main() {
	input! {
		n: usize,
		a: usize,
		b: usize,
		c: usize,
	}

	let mut ans = usize::MAX;
	for z in (0..=9999).rev() {
		for y in (0..=9999).rev() {
			let r = z  * c + y * b;
			if r > n {
				continue;
			}

			if (n - r) % a == 0 {
				let x = (n - r) / a;
				if ans > x + y + z {
					ans = x + y + z;
				}
			}
		}
	}
	println!("{ans}");
}

