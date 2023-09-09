use proconio::*;

fn main() {
    input! {
		s: marker::Chars,
		t: marker::Chars,
    }
	for i in 0..s.len() {
		if s[i] != t[i] {
			println!("{}", i + 1);
			return;
		}
	}
	println!("{}", t.len());
}
