// 参照 aliyome + 解説
// https://atcoder.jp/contests/abc330/submissions/47897165
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		d: i64,
	}
	let mut result = d;
	let mut x = 0;
	while x * x <= d {
		let c = x * x - d;
		if c < 0 {
			let y = ((-1 * c) as f64).sqrt() as i64;
			result = result.min((x * x + y * y - d).abs());
			let y = ((-1 * c) as f64).sqrt().ceil() as i64;
			result = result.min((x * x + y * y - d).abs());
		} else {
			let y = 0;
			result = result.min((x * x + y * y - d).abs());
		}
		x += 1;
	}
	println!("{result}");
}
