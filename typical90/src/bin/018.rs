// 018 - Statue of Chokudai（★3）
use proconio::*;
use std::f64::consts::PI;

#[allow(non_snake_case)]
fn main() {
	input! {
		T: usize, // 1周するのにかかる時間 sec/回転
		L: usize, // 観覧車の高さ
		X: usize, // 高橋直大像の位置X
		Y: usize, // 高橋直大像の位置Y
		Q: usize, // 問題数
	}

	for _ in 0..Q {
		input! {
			E: usize, // 時間
		}
		let e = E as f64;
		let t = T as f64;
		let l = L as f64;

		let tw = E / T; // 波の数
		let ti = e / t - tw as f64;

		let yi = if ti <= 0.25 { 
			-2.0 * l * ti
		} else if ti >= 0.75 {
			-2.0  * l * ti + 2.0 * l
		} else {
			2.0  * l * ti - l
		};
		let zi = if ti <= 0.5 {
			2.0 * l * ti
		} else {
			-2.0 * l * ti + 2.0 * l
		};

		println!("{ti}: ({yi}, {zi})");
		let x = X as f64;
		let y = Y as f64;
		let sita = f64::atan2(zi, (x * x + (y - yi) * (y - yi)).sqrt()) * 180.0 / PI;
		println!("{}", sita);

	}
}
