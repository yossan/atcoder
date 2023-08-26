// 006 - Smallest Subsequence (★5)
use proconio::*;

fn main() {
	input! {
		n: usize,
		k: usize,
		s: marker::Chars,
	}
	let mut substr = String::new();
	// k文字の英単語を
	// 辞書順に出力していく
	// a b c d e f g h i j k l m n o p q r s t u v w x y z
	let mut cur = 0;
	for i in 0..k {
		for c in 'a' as u8..='z' as u8 {
			let c = c as char;
			if let Some(p) = s[cur..].iter().position(|&a| a == c) {
				let m = n - cur - p;
				if m >= k - i {
					cur += p + 1;
					substr.push(c);
					break;
				}
			}
		}
	}
	println!("{}", substr);
}
