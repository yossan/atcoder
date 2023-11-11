use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		s: marker::Chars,
	}
	let mut stack = Vec::new();
	let n = s.len();

	for i in 0..n {
		stack.push(s[i]);
		let n2 = stack.len();
		if n2 >= 3 {
			if stack[n2-3..] == ['A', 'B', 'C'] {
				stack.drain(n2-3..);
				// stack.pop();
				// stack.pop();
				// stack.pop();
			}
		}
	}
	for c in stack {
		print!("{c}");
	}
	println!("");
}
