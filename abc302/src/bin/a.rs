use proconio::*;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    if a % b == 0 {
        println!("{}", a / b);
    } else {
        println!("{}", a / b + 1);
    }
}
