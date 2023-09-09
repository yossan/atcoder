use proconio::*;

fn main() {
    input! {
		n: usize,
		s: [isize; n],
    }
	let mut a = Vec::new();
	a.push(s[0]);
	for i in 0..n-1 {
		a.push(s[i+1] - s[i]);
	}
	for i in 0..n {
		print!("{} ", a[i]);
	}
	println!("");
}
