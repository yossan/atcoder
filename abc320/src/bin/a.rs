use proconio::*;

fn main() {
	input! {
		a: usize,
		b: usize,
	}

	let ans = power_ab(a, b) + power_ab(b, a);
	println!("{ans}");
}

fn power_ab(a: usize, b: usize) -> usize {
	// 2進数に展開する
	let mut ans = 1;
	let mut p = a;
	let bitwidth = f64::log2(b as f64) as usize + 1;
	for j in 0..bitwidth {
		if (b >> j) % 2 == 1 {
			ans *= p;
		}
		p *= p;
	}
	ans
}
