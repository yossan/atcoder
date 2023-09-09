use proconio::*;

fn main() {
	input! {
		n: usize,
		mut a: [usize; n],
		q: usize,
	}

	a.sort();
	let cmp = |b: usize| { move |v: &usize| {
		if v > &b {
			std::cmp::Ordering::Greater
		} else {
			std::cmp::Ordering::Less
		}
	}};

	for _ in 0..q {
		input! {
			b: usize,
		}
		let idx = match a.binary_search_by(cmp(b)) {
			Ok(idx) => idx,
			Err(idx) => idx,
		};

		if idx == 0 {
			println!("{}", a[idx] - b);
		} else if idx < n {
			println!("{}", (a[idx] - b).min(b - a[idx-1]));
		} else {
			println!("{}", b - a[n-1]);
		}
	}
}
