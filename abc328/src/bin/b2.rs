/// ゾロ目を作り出す方法で解いてみる
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		n: usize,
		d: [usize; n], // i月目はdi日
	}
	let mut cnt = 0;
	for i in 1..10 {
		// i月i日が入っているかどうかを確認
		if i <= n && i <= d[i-1] /*i月の日数を取り出す*/ {
			cnt += 1;
		}
		// i月11*i日が入っているかどうか確認
		if i <= n && 11 * i <= d[i-1] {
			cnt += 1;
		}
		// 11*i月i日が入っているかどうか確認
		if 11 * i <= n && i <= d[11*i-1] /* i*11月の日数を取り出す*/ {
			cnt += 1;
		}
		if 11 * i <= n && 11 * i <= d[11*i-1] {
			cnt += 1;
		}
	}
	println!("{cnt}");
}
