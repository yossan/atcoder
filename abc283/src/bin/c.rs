use proconio::*;

fn main() {
    input! {
        s: marker::Chars
    }
    let mut s = s.iter().collect::<String>();
    let mut n = s.len();
    let mut cnt = 0;
    while n > 0 {
        if n >= 2 {
            if &s[0..2] == "00" {
                cnt += 1;
                n -= 2;
                s.remove(0);
                s.remove(0);
                continue;
            }
        }
        cnt += 1;
        n -= 1;
        s.remove(0);
    }
    println!("{}", cnt);
}

