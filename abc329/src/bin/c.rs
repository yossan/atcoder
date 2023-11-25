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
	while i < n {
		let mut j = i;
		while j < n && s[i] == s[j] {
			set.insert(&s[i..=j]);
			j += 1;
		}
		i = j;
	}
	println!("{}", set.len());
}
