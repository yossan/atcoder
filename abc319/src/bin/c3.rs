use proconio::*;
use itertools::*;

fn main() {
	input! {
		cells: [usize; 9],
	}
	// 確認する列(タプル)
	let row = [
		(0, 1, 2),
		(3, 4, 5),
		(6, 7, 8),
		(0, 3, 6),
		(1, 4, 7),
		(2, 5, 8),
		(0, 4, 8),
		(2, 4, 6)];
	let mut all = 0;
	let mut not_disappoint = 0;
	for order in (0..9).permutations(9) {
		all += 1;
		let mut disappoint_flag = false;
		for (a, b, c) in row {
			if cells[a] == cells[b] && order[a] < order[c] && order[b] < order[c] {
				println!("cが最後 {}, {}, {}", order[a], order[b], order[c]);
				disappoint_flag = true;
			}
			else if cells[a] == cells[c] && order[a] < order[b] && order[c] < order[b] {
				println!("bが最後 {}, {}, {}", order[a], order[b], order[c]);
				disappoint_flag = true;
			}
			else if cells[b] == cells[c] && order[b] < order[a] && order[c] < order[a] {
				println!("aが最後 {}, {}, {}", order[a], order[b], order[c]);
				disappoint_flag = true;
			}

		}
		if !disappoint_flag {
			not_disappoint += 1;
		}
	}
	println!("{}", not_disappoint as f64 / all as f64);
}
