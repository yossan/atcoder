// 解説参照
// https://atcoder.jp/contests/abc329/editorial/7719
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		n: usize,
		s: marker::Chars,
	}
	// ランレングス圧縮
	let mut l = 0;
	// 文字マップを用意する
	let mut mx = vec![0; 26];
	while l < n {
		let mut r = l + 1;
		while r < n && s[l] == s[r] {
			r += 1
		}
		let c = (s[l] as u8 - 'a' as u8) as usize;
		// 文字マップに最大長を格納する
		mx[c] = mx[c].max(r - l);
		l = r;
	}
	let mut ans = 0;
	for i in 0..26 {
		ans += mx[i];
	}
	println!("{ans}");
}
