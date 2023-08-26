// 002 - Encyclopedia of Parentheses (★3)
use proconio::*;
fn main() {
    input! {
        n: usize,
    }
    let is_parentheses = |phrase: &str| {
        let mut score = 0;
        for p in phrase.chars() {
            score += if p == '(' {
                1
            } else {
                -1
            };
            if score < 0 {
                return false;
            }
        }
        score == 0
    };
    for i in 0..(1 << n) {
        let mut s = String::new();
        for j in (0..n).rev() {
            if (i >> j) % 2 == 0 {
                s += "(";
            } else {
                s += ")";
            }
        }
        if is_parentheses(&s) {
            println!("{}", s);
        }
    }
}
