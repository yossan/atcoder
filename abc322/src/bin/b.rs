use proconio::*;

fn main() {
	input! {
		n: usize,
		m: usize,
		s: marker::Chars,
		t: marker::Chars,
	}
	let s = s.iter().collect::<String>();
	let t = t.iter().collect::<String>();
	let is_prefix = t.starts_with(&s);
	let is_surfix = t[t.len()-s.len()..].starts_with(&s);
	let ans = if is_prefix && is_surfix {
		0
	} else if is_prefix && !is_surfix {
		1
	} else if !is_prefix && is_surfix {
		2
	} else {
		3
	};
	println!("{ans}");
}

