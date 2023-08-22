// Count Down
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
    }
    for i in 0..=n {
        println!("{}", n - i);
    }
}
