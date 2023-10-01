use proconio::*;
use std::cmp::Ordering::*;

fn main() {
	input! {
		n: usize,
		m: usize,
		a: [usize; m],
	}
	for i in 1..=n {
		match a.binary_search(&i) {
			Ok(_) => {
				println!("0");
			}
			Err(ind) => {
				println!("{}", a[ind] - i);
			}
		};
	}
}
