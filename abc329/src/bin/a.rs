use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		s: marker::Chars,
	}
	let n = s.len();
	for (i, c) in s.iter().enumerate() {
		if i < n -1 {
			print!("{} ", c);
		} else {
			println!("{}", c);
		}
	}
}
