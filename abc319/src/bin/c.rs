use proconio::*;
use itertools::*;

fn main() {
	input! {
		s: [[usize; 3]; 3],
	}
	let mut gakkari = 0;
	let mut cnt = 0;
	for path in (0..9).permutations(9) {
		let mut i = 0;
		let mut es = [(0usize, 0usize, 0usize); 3];
		cnt += 1;
		for e in path {
			let c = e / 3;
			let r = e % 3;
			es[i%3] = (c, r, s[c][r]);
			if i >= 2 {
				let [a, b, c] = es;
				let (a1, a2, a3) = a;
				let (b1, b2, b3) = b;
				let (c1, c2, c3) = c;

				if a3 == b3 && a3 != c3 {
					gakkari += 1;
					break;
				}
			}
			i += 1;
		}
	}
	println!("{}", ((cnt - gakkari) as f64) / (cnt as f64));
}
