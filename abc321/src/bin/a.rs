use proconio::*;

fn main() {
	input! {
		mut n: isize,
	}
	let len = f64::log10(n as f64) as usize + 1;
	let mut z = Vec::new();
	for _ in 0..len {
		z.push(n % 10);
		n /= 10;
	}
	for i in 0..len-1 {
		if z[i] >= z[i+1] {
			println!("No");
			return;
		}
	}
	println!("Yes");
}
