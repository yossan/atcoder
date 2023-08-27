use proconio::*;

fn main() {
	input! {
		n: usize,
		h: usize,
		x: usize,
		p: [usize; n],
	}
	let mut ans = (0, std::usize::MAX);
	for i in 0..n {
		let score = p[i] + h;
		if score >= x {
			if ans.1 > score {
				ans = (i + 1, score);
			}
		}
	}
	println!("{}", ans.0);
}
