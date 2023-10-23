/// C - Rotate Colored Subsequence
/*
英小文字からなる長さNの文字列Sが与えられる。
色1, 色2,..色Mのうち、i番目の文字には色Ciが塗られている。
各色ごとに次の操作をi=1,2,3..Mで順番に行い、最終的な文字列を出力せよ
[操作]
Sの色iで塗られた文字からなる部分を右に1つ巡回シフトする

制約
* 1≤M≤N≤2×10^5
* 1≤Ci≤M
* N,M,Ciはすべて整数
* S は英小文字からなる長さ N の文字列
* 任意の整数 1≤i≤M に対して、ある整数 1≤j≤N が存在して Cj=i が成り立つ
*/
use proconio::*;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
	input! {
		N: usize, // 文字列の長さ
		M: usize, // 色の数
		S: marker::Chars, // 文字列
		C: [Usize1; N], // i番目の文字の色
	}
	// 1からMの色に対して操作(右に1つ巡回シフト)を行い、
	// 最終的に文字列を出力する

	// 色ごとに隣接リストを作る
	type Adj = Vec<Vec<usize>>;
	let mut c_map: Adj = vec![Vec::new(); M];
	for i in 0..N {
		c_map[C[i]].push(i);
	}

	let mut SS = vec![' '; N];

	// 色ごとに右に1つ巡回シフトし、SSに移し替えていく
	for i in 0..M {
		let s = &c_map[i];
		let k = s.len();
		for j in 0..k {
			SS[s[(j+1)%k]] = S[s[j]];
		}
	}
	println!("{}", SS.iter().collect::<String>());
}
