use proconio::*;

fn main() {
	input! {
		s: marker::Chars,
	}
	let n = s.len();
	let mut ans = 1;
	for i in 0..n {
		for j in (i+1)..n {
			if is_palindrome(&s[i..=j]) {
				ans = ans.max(j - i + 1);
			}
		}
	}
	println!("{ans}");
}

fn is_palindrome(s: &[char]) -> bool {
	let n = s.len();
	for i in 0..n/2 {
		if s[i] != s[n-i-1] {
			return false;
		}
	}
	true
}

