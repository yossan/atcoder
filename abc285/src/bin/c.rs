// abc285_brutmhyhiizp
use proconio::*;

fn main() {
    input! {
        s: marker::Chars
    }
    let n = s.len();
    let mut cnt = 0;
    for i in 0 .. n {
        cnt += (s[n - 1 - i] as u8 - ('A' as u8 - 1)) as i64 * 26i64.pow(i as u32);
    }
    println!("{}", cnt)
}


