use proconio::*;

fn main() {
	input! {
		n: usize,
		x: usize,
		mut a: [usize; n-1],
	}

	a.sort();

	let mut sum = 0;

	// 最後(=最小値)と最大値を除く
	for i in 0..n-2 {
		sum += a[i];
	}
	if x >= sum && x - sum <= a[0] && sum == x {
		println!("{}", x - sum);
		return;
	}

	sum = 0;

	// 最後(=最大値)と最小値を除く
	for i in 1..n-1 {
		sum += a[i];
	}
	if x >= sum && x - sum <= 100 && x - sum >= a[n-2] && x == sum {
		println!("{}", x - sum);
		return;
	}

	sum = 0;

	// 最大値と最小値を除く
	for i in 1..n-2 {
		sum += a[i];
	}
	if x >= sum && x - sum <= 100 && x - sum <= a[n-2] && x - sum >= a[0] {
		println!("{}", x - sum);
		return;
	}
	println!("-1");

}
