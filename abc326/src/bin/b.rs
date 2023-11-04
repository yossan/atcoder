use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		mut N: usize,
	}
	// 326-like数
	// 100の位と10の位の積 = 1の位
	for i in 100 ..= 919 {
		let mut n = i;
		// 1の位
		let o1 = n % 10;
		n /= 10;
		// 10の位
		let o2 = n % 10;
		n /= 10;
		// 100の位
		let o3 = n % 10;
		if o2 * o3 == o1 {
			if i >= N {
				println!("{i}");
				return;
			}
		}
	}
}
