use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		n: usize,
		l: usize,
		a: [usize; n],
	}
	let mut cnt = 0;
	for i in 0..n {
		if a[i] >= l {
			cnt += 1;
		}
	}
	println!("{}", cnt);
}
