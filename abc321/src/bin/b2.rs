use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		N: usize,
		X: usize,
		A: [usize; N-1],
	}
	// Aを昇順で並べた際に2番目からN-1番目までの和がX以上であるかどうか
	for p in 0..=100 {
		let mut S = A.clone();
		S.push(p);
		S.sort();
		if S[1..N-1].iter().sum::<usize>() >= X {
			println!("{p}");
			return;
		}
	}
	println!("-1");
}
