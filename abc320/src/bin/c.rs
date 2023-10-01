use proconio::*;
use itertools::*;

fn main() {
	input! {
		m: usize,
		s1: marker::Chars,
		s2: marker::Chars,
		s3: marker::Chars,
	}

	let mut ans = isize::MAX;
	let sss = [s1, s2, s3];
	for s in sss.iter().permutations(3) {
		let mut cnt = -1;
			'i:for i in 0..m {
				for j in (i+1)..2*m {
					if s[0][i] == s[1][j%m] {
						for k in (j+1)..3*m {
							if s[1][j%m] == s[2][k%m] {
								cnt = k as isize;
								break 'i;
							}
						}
					}
				}
			}
			if cnt > 0 {
				ans = ans.min(cnt as isize);
			}
	}
	if ans != isize::MAX {
		println!("{ans}");
	} else {
		println!("-1");
	}
}
