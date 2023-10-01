use proconio::*;

fn main() {
	input! {
		mut k: usize,
	}

	for i in 1..usize::MAX {
		if check(i) {
			k -= 1;
			if k == 0 {
				println!("{i}");
				return;
			}
		}
	}
}

fn check(n: usize) -> bool {
	let mut n = n;
	let len = f64::log10(n as f64) as usize + 1;
	let mut z = Vec::new();
	for _ in 0..len {
		z.push(n % 10);
		n /= 10;
	}
	for i in 0..len-1 {
		if z[i] >= z[i+1] {
			return false;
		}
	}
	true
}
