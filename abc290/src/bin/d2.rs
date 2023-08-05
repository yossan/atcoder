use proconio::*;

fn main() {
    input! {
        t: usize,
        ask: [(i64, i64, i64); t],
    }
    for (n, d, k) in ask {
        let g = gcd(n, d);
        let a = n / g;
        let q = (k - 1) / a;
        let r = (k - 1) % a;
        let ans = (q + r * d) % n;
        println!("{}", ans);
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut x = a.max(b);
    let mut y = a.min(b);
    while y != 0 {
        let tmp = x;
        x = y;
        y = tmp % y;
    }
    return x;
}
