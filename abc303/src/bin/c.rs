use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, // n回行えるか
        m: usize, // m個の体力を回復するアイテム
        mut h: i64, // 体力
        k: i64, // 体力がk未満ならば、移動した点に置かれたアイテムを消費し、体力がkとなる
        ss: marker::Chars,
        mut ii: [(i64, i64); m],
    }

    let mut set = BTreeSet::new();
    for item in &ii {
        set.insert(*item);
    }

    let mut pos = (0, 0);
    for i in 0..n {
        let s = ss[i];
        // 移動
        match s {
            'R' => {
                pos.0 += 1;
            },
            'L' => {
                pos.0 -= 1;
            },
            'U' => {
                pos.1 += 1;
            },
            'D' => {
                pos.1 -= 1;
            },
            _ => {
            },
        }

        // ヒットポイントを減らす
        h -= 1;
        if h < 0 {
            println!("No");
            return;
        }

        // 移動する前に回復する
        if h < k {
            if set.contains(&pos) {
                h = k;
                set.remove(&pos);
            }
        }
    }
    println!("Yes");
}
