use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		n: usize,
		l: usize,
		r: usize,
		a: [usize; n],
	}

	for i in 0..n {
		let ans = if a[i] >= l && a[i] <= r {
			a[i]
		} else if a[i] < l {
			l
		} else {
			r
		};
		print!("{} ", ans);
	}
	println!("");
}
