// A - Nine
use proconio::*;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if (b - a == 1) && ((a % 3 == 1 && b % 3 == 2) || (a % 3 == 2 && b % 3 == 0)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
