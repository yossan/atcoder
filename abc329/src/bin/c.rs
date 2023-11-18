use proconio::*;
use std::collections::HashSet;

#[allow(non_snake_case)]
fn main() {
	input! {
		n: usize,
		s: marker::Chars,
	}
	let mut set: HashSet<&[char]> = HashSet::new();
	let mut i = 0;
	let mut j = i;
	while i <= n {
		if s[i] == s[j] {
			set.insert(&s[i..=j]);
			j += 1;
		} else {
			i = j;
			j = i;
		}
		if j >= n {
			break;
		}
	}
	println!("{}", set.len());
}
