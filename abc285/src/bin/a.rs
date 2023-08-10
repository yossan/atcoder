// Edge Checker 2
use proconio::*;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if b == 2 * a || b == 2 * a + 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
