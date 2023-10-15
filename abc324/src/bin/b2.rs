use proconio::*;

fn main() {
	input! {
		mut n: usize, // n = 2^x3^yを満たすx,yがあるかどうか
	}
	while n % 2 == 0 {
		n /= 2;
	}
	while n % 3 == 0 {
		n /= 3;
	}
	if n == 1 {
		println!("Yes");
	} else {
		println!("No");
	}
}
