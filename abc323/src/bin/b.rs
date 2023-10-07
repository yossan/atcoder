use proconio::*;
use std::cmp::Ordering::*;

fn main() {
	input! {
		n: usize,
		s: [marker::Chars; n],
	}
	let mut map = vec![(0usize, 0usize); n];
	for i in 0..n {
		map[i] = s[i].iter().fold((0, 0), |acc, &c| { 
			if c == 'o' { (acc.0 + 1, i) } else { (acc.0, i) }
		});
	}
	map.sort_by(|a, b| {
		if a.0.cmp(&b.0) == Equal {
			if a.1.cmp(&b.1) == Less {
				Greater
			} else if a.1.cmp(&b.1) == Greater {
				Less
			} else {
				Equal
			}
		} else if a.0.cmp(&b.0) == Less {
			Less
		} else {
			Greater
		}
	});
	for i in (0..n).rev() {
		print!("{} ", map[i].1 + 1);
	}
	println!("");
}
