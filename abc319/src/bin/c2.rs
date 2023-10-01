use proconio::*;
use itertools::*;

fn main() {
	input! {
		c: [[usize; 3]; 3],
	}
	// それぞれのマスに書かれている数字をランダムな順番で知る
	// がっかりせずに全てのマスに書かれた数字を知る確率
	// [がっかりする]とは
	// 縦、横、斜めの列のうち
	// * はじめに知ったほうの２マスに書かれた数字が同じ (隣り合っている必要はなし)
	// * 最後に知ったマスに書かれた数字が異なる
	let mut total = 0;
	let mut cnt_success = 0;
	for path in (0..9).permutations(9) {
		total += 1;
		let mut is_disappointed = false;
		
		for edges in path.windows(3) {
			let [e1, e2, e3] = edges else { panic!() };
			if c[e1/3][e1%3] != c[e2/3][e2%3] || c[e2/3][e2%3] == c[e3/3][e3%3] {
				continue;
			}
			// println!("がっかりか？ = ({}, {}, {})", c[e1/3][e1%3], c[e2/3][e2%3], c[e3/3][e3%3]);
			// がっかりするには、e1, e2, e3が、縦、横、斜めの関係である必要がある
			// 縦の関係であるか？
			if e1 % 3 == e2 % 3 && e2 % 3 == e3 % 3 {
				// がっかり
				// println!("がっかりだった1 ({e1}, {e2}, {e3})");
				is_disappointed = true;
				break;
			}
			// 横の関係であるか?
			if e1 / 3 == e2 / 3 && e2 / 3 == e3 / 3 {
				// がっかり
				is_disappointed = true;
				break;
			}
			// 斜めの関係であるか
			let mut edges = [e1, e2, e3];
			edges.sort();
			if edges[0] / 3  == 0 && edges[0] % 3 == 0 &&
				edges[1] / 3 == 1 && edges[1] % 3 == 1 &&
				edges[2] / 3 == 2 && edges[2] % 3 == 2 {
				// がっかり
					is_disappointed = true;
					break;
			} 
			if edges[0] / 3 == 2 && edges[0] % 3 == 2 &&
				edges[1] / 3 == 1 && edges[1] % 3 == 1 &&
				edges[2] / 3 == 0 && edges[2] % 3 == 0 {
				// がっかり
					is_disappointed = true;
					break;
			}
		}
		if !is_disappointed {
			cnt_success += 1;
		}
	}
	println!("{}, {}", cnt_success, total);
	println!("{}", cnt_success as f64 / total as f64);
}
