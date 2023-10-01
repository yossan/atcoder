use proconio::*;

fn main() {
	input! {
		n: usize,
		s: marker::Chars,
	}
	let str = s.iter().collect::<String>();
	if let Some(ind) = str.find("ABC") {
		println!("{}", ind + 1);
	} else {
		println!("-1");
	}
}
