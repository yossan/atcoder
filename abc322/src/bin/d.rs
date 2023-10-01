use proconio::*;

fn main() {
	input! {
		mut p1: [marker::Chars; 4],
		mut p2: [marker::Chars; 4],
		mut p3: [marker::Chars; 4],
	}
	let put_to_rect = |rect: &Vec<Vec<char>>, grid: &Vec<Vec<char>>, tx: isize, ty: isize| {
		let mut new_rect = rect.clone();
		for x in 0..4 {
			for y in 0..4 {
				// グリッドからポリオミノを取り出す
				if grid[x][y] == '#' {
					let dx = x as isize + tx;
					let dy = y as isize + ty;
					// 移動したポリオミノがグリッド内部にあるかどうか
					if dx >= 0 && dx < 4 && dy >= 0 && dy < 4 {
						// すでにポリオミノが配置されていないか
						if new_rect[dx as usize][dy as usize] != '#' {
							// ポリオミノを配置する
							new_rect[dx as usize][dy as usize] = '#';
						} else {
							return None
						}
					} else {
						return None
					}
				}
			}
		}
		Some(new_rect)
	};

	let rotate90 = |grid: &mut Vec<Vec<char>>| {
		let mut new_grid = vec![vec!['.'; 4]; 4];
		for y in 0..4 {
			for x in 0..4 {
				new_grid[3-y][x] = grid[x][y];
			}
		}
		*grid = new_grid;
	};

	for tx1 in -3..4 {
		for ty1 in -3..4 {
			for ro1 in 0..3 {
				if ro1 > 0 { rotate90(&mut p1); }
				for tx2 in -3..4 {
					for ty2 in -3..4 {
						for ro2 in 0..3 {
							if ro2 > 0 { rotate90(&mut p2); }
							for tx3 in -3..4 {
								for ty3 in -3..4 {
									for ro3 in 0..3 {
										if ro3 > 0 { rotate90(&mut p3); }
										let rect = vec![vec!['.'; 4];4];
										if let Some(rect2) = put_to_rect(&rect, &p1, tx1, ty1) {
											if let Some(rect3) = put_to_rect(&rect2, &p2, tx2, ty2) {
												if let Some(rect4) = put_to_rect(&rect3, &p3, tx3, ty3) {
													let mut filled = true;
													'ch:for x in 0..4 {
														for y in 0..4 {
															if rect4[x][y] != '#' {
																filled = false;
																break 'ch;
															}
														}
													}
													if filled {
														println!("Yes");
														return;
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}
	println!("No");
}
