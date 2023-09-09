use proconio::*;

fn main() {
    input! {
		h: usize,
		w: usize,
		s: [marker::Chars; h],
    }

	let mut cnt = 0;
	for i in 0..h {
		for j in 0..w {
			if s[i][j] == '#' {
				cnt += 1;
			}
		}
	}
	println!("{}", cnt);
}
