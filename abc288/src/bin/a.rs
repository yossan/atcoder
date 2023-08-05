use proconio::*;

fn main() {
    input! {
        n: usize,
    }
    for _ in 0..n {
        input! {
            ab: (isize, isize),
        }
        let (a, b) = ab;
        println!("{}", a + b);
    }
}
