use proconio::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [marker::Chars; n],
        t: [marker::Chars; m],
    }
    let mut count = 0;
    for i in 0..n {
        for j in 0..m{
            let si = &s[i][3..6];
            let tj = &t[j][0..3];
            if si == tj {
                count += 1;
                break;
            }
        }
    }
    println!("{}", count);
}
