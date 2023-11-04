/// 解説参照
/// https://atcoder.jp/contests/abc326/editorial/7540
use proconio::*;
use itertools::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		N: usize, // NxNマス, RとCの文字列数
		R: marker::Chars,
		C: marker::Chars,
	}
	let AA = (0..N).permutations(N);
	let BB = (0..N).permutations(N);
	let CC = (0..N).permutations(N);
	for a in AA {
		for b in BB.clone() {
			'c:for c in CC.clone() {
				if any(&a, &b, &c, |a, b, c| {
					a == b || b == c || a == c
				}) {
					continue;
				}
				let mut S = vec![vec!['.'; N]; N];
				for (i, &j) in a.iter().enumerate() {
					S[i][j] = 'A';
				}
				for (i, &j) in b.iter().enumerate() {
					S[i][j] = 'B';
				}
				for (i, &j) in c.iter().enumerate() {
					S[i][j] = 'C';
				}
				// i 行目に書かれた文字の中で最も左にある文字は R の i 文字目と一致する
				for i in 0..N {
					// i行の各列の先頭文字を取り出す
					for j in 0..N {
						if S[i][j] == '.' {
							continue;
						}
						if S[i][j] != R[i] {
							// 先頭文字がR[i]と異なる
							break 'c;
						}
						break;
					}
				}
				// j 列目に書かれた文字の中で最も上にある文字は C の j 文字目と一致する
				for j in 0..N {
					// j列の各行の先頭文字を取り出す
					for i in 0..N {
						if S[i][j] == '.' {
							continue;
						}
						if S[i][j] != C[j] {
							// 先頭文字がC[i]と異なる
							break 'c;
						}
						break;
					}
				}

				println!("Yes");
				for i in 0..N {
					for j in 0..N {
						print!("{} ", S[i][j]);
					}
					println!("");
				}
				return;
			}
		}
	}
	println!("No");
}

fn any<F>(a: &[usize], b: &[usize], c: &[usize], f: F) -> bool 
where F: Fn(usize, usize, usize) -> bool,
{
	for i in 0..a.len().min(b.len().min(c.len())) {
		if f(a[i], b[i], c[i]) {
			return true;
		}
	}
	false
}


