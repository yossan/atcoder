use proconio::*;

fn main() {
	input! {
		n: usize,
		a: [usize; n],
	}
	let a0 = a[0];
	let mut same = true;
	for i in 1..n {
		if a0 != a[i] {
			same = false;
			break;
		}
	}
	if same {
		println!("Yes");
	} else {
		println!("No");
	}
}
