use proconio::*;

fn main() {
	input! {
		n: usize,
	}

	let mut s: String = String::new();
	for i in 0..=n {
		for j in 1..=9 {
			if n % j == 0 { 
				let m = n / j;
				if i % m == 0 {
					s.push_str(format!("{}", j).as_str());
					break;
				}
			}
		}
		if s.len() <= i {
			s.push_str("-");
		}
	}
	println!("{s}");
}
