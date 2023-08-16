// Let's Get a Perfect Score
use proconio::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [marker::Chars; n],
    }
    let mut cnt = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            let a = &s[i];
            let b = &s[j];
            let mut matching = true;
            for k in 0..m {
                if a[k] == 'x' && b[k] == 'x' {
                    matching = false;
                    break;
                }
            }
            if matching {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}

