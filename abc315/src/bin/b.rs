use proconio::*;

fn main() {
    input! {
        n: usize,
        d: [isize; n]
    }
    let mut z = 0;
    for i in 0..n {
        z += d[i];
    }
    let mut half = (z + 1)/ 2;
    for i in 0..n {
        half -= d[i];
        if half <= 0 {
            println!("{} {}", i + 1, d[i] + half);
            return;
        }
    }
}

