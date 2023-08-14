// Multi Test Cases
use proconio::*;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        }
        let cnt = a.iter().fold(0usize, |acc, &e| { if e % 2 != 0 { acc + 1 } else { acc } });
        println!("{}", cnt);

    }
}

