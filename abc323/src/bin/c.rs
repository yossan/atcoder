use proconio::*;
use std::cmp::Reverse;

fn main() {
	input! {
		n: usize, // プレーヤー数
		m: usize, // 問題数
		mut a: [isize; m], // 各問題のスコア
		ss: [marker::Chars; n], // 各プレーヤーの結果ox
	}
	// 各プレーヤーのスコア結果
	let mut scores = vec![0isize; n];
	let mut top_score = 0;
	for i in 0..n {
		let s = &ss[i]; 
		for j in 0..m {
			if s[j] == 'o' {
				scores[i] += a[j];
			}
		}
		scores[i] += i as isize + 1;
		if top_score < scores[i] {
			top_score = scores[i];
		}
	}

	// スコアと旧順序
	let mut na = Vec::new();
	for i in 0..m {
		na.push((a[i], i));
	}
	// スコアが高い順に並べる
	na.sort_by(|a, b| {
		Reverse(a.0).cmp(&Reverse(b.0))
	});

	for i in 0..n {
		let mut score = scores[i];
		let s = &ss[i];
		
		if score == top_score {
			println!("0");
		} else {
			let mut j = 0;
			// 各問題のスコアと旧番号
			for &(sc, num) in &na {
				// 解いていない問題
				if s[num] != 'o' {
					score += sc;
					j += 1;
				} 
				if score > top_score {
					println!("{j}");
					break;
				}
			}
		}
	}
}
