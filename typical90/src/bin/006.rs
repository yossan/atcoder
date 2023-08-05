use proconio::*;
use std::iter::repeat;
use itertools::*;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, // 文字列の長さN
        k: usize, // 生成する部分列の長さK
        s: marker::Chars, // 与えられる文字列
    }

    let mut set = BTreeSet::new();

    let mut ans:String = String::new();

    // k個の1を持つNビットを生成
    let n_bits = repeat(1).take(k).chain(repeat(0).take(n-k)).collect::<Vec<usize>>();
    for bits in n_bits.iter().permutations(n) {
        if !set.insert(bits.iter().join("")) {
            continue;
        }

        let mut cmp = String::new();
        for i in 0..n {
            if *bits[i] == 1 {
                cmp.push(s[i]);
            }
        }
        if ans == "" {
            ans = cmp;
        } else if ans > cmp {
            ans = cmp;
        }
    }
    println!("{}", ans);
}
