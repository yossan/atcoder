use proconio::*;

fn main() {
	input! {
		mut s: usize,
	}
	for i in 0..16 {
		// 16文字目から
		if i <= 15 && i % 2 == 0 && s % 10 != 0 {
			println!("No");
			return;
		}
		s /= 10;
	}
	println!("Yes");
}
