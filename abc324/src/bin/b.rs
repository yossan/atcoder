use proconio::*;

fn main() {
	input! {
		n: usize, // n = 2^x3^y
	}
	let mx = ((n as f64).log10() / (2 as f64).log10()) as usize + 1;
	for x in 0..mx {
		let a = 2usize.pow(x as u32);
		if n % a == 0 {
			let b = n / a;
			let my = ((b as f64).log10() / (3 as f64).log10()) as usize + 1;
			for y in 0..my {
				if b == 3usize.pow(y as u32) {
					println!("Yes");
					return;
				}
			}
		}
	}
	println!("No");
}
