use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		a: [[marker::Usize1; 9]; 9],
	}
	let mut ng = false;

	let mut cols = vec![vec![false; 9]; 9];
	let mut rows = vec![vec![false; 9]; 9];
	let mut blks = vec![vec![false; 9]; 9];
	'i:for i in 0..9 {
		let col = &mut cols[i];
		let row = &mut rows[i];
		for j in 0..9 {
			if !col[a[i][j]] {
				col[a[i][j]] = true;
			} else {
				ng = true;
				break 'i;
			}

			if !row[a[j][i]] {
				row[a[j][i]] = true;
			} else {
				ng = true;
				break 'i;
			}

			let k = if i < 3 {
				if j < 3 {
					0
				} else if j < 6 {
					1
				} else {
					2
				}
			} else if i < 6 {
				if j < 3 {
					3
				} else if j < 6 {
					4
				} else {
					5
				}
			} else {
				if j < 3 {
					6
				} else if j < 6 {
					7
				} else {
					8
				}
			};

			let blk = &mut blks[k];
			if !blk[a[i][j]] {
				blk[a[i][j]] = true;
			} else {
				ng = true;
				break 'i;
			}
		}
	}
	if !ng {
		println!("Yes");
	} else {
		println!("No");
	}
}
