use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		n: usize,
		mut a: [usize; n],
	}
	a.sort();
	let max_val = a[n-1];
	let mut ans = 0;
	
	for &num in &a {
		if num == max_val {
			continue;
		}
		ans = ans.max(num);
	}
	println!("{ans}");
}
