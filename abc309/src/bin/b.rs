// Rotate
use proconio::*;

fn main() {
    input! {
        n: usize,
        aa: [marker::Chars; n],
    }
    let mut a = vec![vec![0u8; n]; n];
    for i in 0..n {
        let s = &aa[i];
        for j in 0..n {
            a[i][j] = s[j] as u8 - '0' as u8;
        }
    }

    let mut prev = a[1][0];
    // 一列目
    for j in 0..n {
        let tmp = a[0][j];
        a[0][j] = prev;
        prev = tmp;
    }
    // n列目
    for i in 1..n {
        let tmp = a[i][n-1];
        a[i][n-1] = prev;
        prev = tmp;
    }
    // n行目
    for j in (0..n-1).rev() {
        let tmp = a[n-1][j];
        a[n-1][j] = prev;
        prev = tmp;
    }
    // 1列目
    for i in (1..n-1).rev() {
        let tmp = a[i][0];
        a[i][0] = prev;
        prev = tmp;
    }
    for i in 0..n {
        for j in 0..n {
            print!("{}", a[i][j]);
        }
        println!("");
    }
}
