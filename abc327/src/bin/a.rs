use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		n: usize,
		s: marker::Chars,
	}
	for i in 0..n {
		if s[i] == 'a' {
			if i + 1 < n && s[i+1] == 'b' {
				println!("Yes");
				return;
			}
		} else if s[i] == 'b' {
			if i + 1 < n && s[i+1] == 'a' {
				println!("Yes");
				return;
			}
		}
	}
	println!("No");
}
