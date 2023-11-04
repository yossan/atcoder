use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		X: usize,
		Y: usize,
	}
	// 上りかどうか
	if Y >= X {
		if Y - X <= 2 {
			println!("Yes");
		} else {
			println!("No");
		}
	} else {
		if X - Y <= 3 {
			println!("Yes");
		} else {
			println!("No");
		}
	}
}
