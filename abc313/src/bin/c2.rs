use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    solve(n, &mut a);
}

fn solve(n: usize, a: &mut [usize]) {
    let sum: usize = a.iter().sum();
    a.sort();
    let mut b = vec![sum / n; n];
    for i in 0..(sum % n) {
        b[n - 1 - i] += 1;
    }
    let mut ans = 0;
    for i in 0.. n {
        let a = a[i] as i64;
        let b = b[i] as i64;
        ans += i64::abs(a - b);
    }
    println!("{}", ans / 2);
}

