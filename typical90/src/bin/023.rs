// 023 - Avoid War（★7）
// HxWの白黒マスをビット全探索してキングが置けるかどうかカウントしていく
use proconio::*;

fn main() {
	input! {
		h: usize,
		w: usize,
		grid: [marker::Chars; h], // 白黒マス 黒マス = '#', 白マス = '.'
	}
	let len = w * h;
	let mut cnt = 0;
	for n in 0..(1 << len) {
		let mut embed: Vec<(usize, usize)> = Vec::new();
		let mut is_ok = true;
		'c:for k in 0..len {
			if (n >> k) % 2 == 1 {
				// キングが置けるかどうか
				let i = k / w;
				let j = k % w;
				if grid[i][j] == '.' {
					// 配置済みとの確認
					for e in &embed {
						if e.0.abs_diff(i) <= 1 &&
							e.1.abs_diff(j) <= 1 {
								is_ok = false;
								break 'c;
						} 
					}
					embed.push((i, j));
				} else {
					is_ok = false;
					break 'c;
				}
			}
		}
		if is_ok {
			cnt += 1;
		}
	}
	println!("{cnt}");
}
