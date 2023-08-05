// 薬の飲む量は単調減少していくので二分探索で解を求めることが出来る
use proconio::*;

fn main() {
    input! {
        n: usize,
        k: i64,
        ab: [(i64, i64); n],
    }
    let mut ok: i64 = 1_000_000_000 * 3 * 100_000;
    let mut ng: i64 = 0;
    while (ok - ng).abs() > 1 { // 整数以下となれば終了
        let mid = (ok + ng) / 2;
        // mid日目に飲む薬の量
        let mut sum = 0;
        for i in 0..n {
            if ab[i].0 >= mid {
                sum += ab[i].1;
            }
        }
        if sum <= k {
            // 解の探索を左側に移す
            ok = mid;
        } else {
            // 回の探索を右側に移す
            ng = mid;
        }
    }
    println!("{}", ok);
}
