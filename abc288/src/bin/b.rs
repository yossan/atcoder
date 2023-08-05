use proconio::*;
use itertools::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [marker::Chars; n],
    }
    let ans = s.into_iter().take(k).sorted().collect::<Vec<Vec<char>>>();
    for i in 0..k {
        let s = ans[i].iter().collect::<String>();
        println!("{}", s);
    }
}

