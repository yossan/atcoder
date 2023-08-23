// 014 - We Used to Sing a Song Together（★3)
use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        mut b: [isize; n],
    }
    a.sort();
    b.sort();
    let mut e = 0;
    for i in 0..n {
        e += (a[i] - b[i]).abs();
    }
    println!("{}", e);
}
