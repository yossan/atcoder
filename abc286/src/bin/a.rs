use proconio::*;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        a: [usize; n],
    }
    for i in 1..=n {
        if i >= p && i <= q {
            print!("{} ", a[r + i - p - 1]);
            continue;
        }
        if i >= r && i <= s {
            print!("{} ", a[p + i - r - 1]);
            continue;
        }
        print!("{} ", a[i - 1]);
    }
    println!("");
}
