use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		n: usize,
		q: usize,
		s: marker::Chars,
	}
	// i文字までの隣り合う文字の数をカウントする
	let mut z = vec![0usize; n];
	for i in 1..n {
		if s[i] == s[i-1] {
			z[i] = z[i-1] + 1;
		} else {
			z[i] = z[i-1];
		}
	}
	for _ in 0..q {
		input! {
			l: marker::Usize1,
			r: marker::Usize1,
		}
		println!("{}", z[r] - z[l]);
	}
}
