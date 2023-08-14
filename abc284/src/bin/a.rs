// Sequence of Strings
use proconio::*;

fn main() {
    input! {
        n: usize,
        s: [marker::Chars; n],
    }
    for i in (0..n).rev() {
        println!("{}", s[i].iter().collect::<String>());
    }
}
