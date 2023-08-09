// Approximate Equalization 2 (おおよそな平衝)
// 平衝化
// Aiを+1, Ajを-1することで各要素の差の最大を1にする
use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n], // 配列aの各要素を平衝化するには何回かかるか
    }
    let sum = a.iter().sum::<usize>();
    // 平衝化に必要な回数を求めれば良いので、昇順で並べ替え
    a.sort();

    // 配列aを平衝化した配列bを作る
    // 平均で初期化
    let mut b = vec![sum / n; n];
    // 余りを後ろ側に配置
    for i in 0..(sum % n) {
        b[n - 1 - i] += 1;
    }
    // 各要素の差分の和を取る
    let mut ans = 0;
    for i in 0..n {
        let a = a[i] as i64;
        let b = b[i] as i64;
        ans += (a - b).abs();
    }
    // 平衝化回数はその1/2
    println!("{}", ans / 2);
}


