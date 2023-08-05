use proconio::*;

fn main() {

    input! {
        n: usize,
        k: usize,
        s: marker::Chars,
    }

    let az = "abcdefghijklmnopqrstuvwxyz";

    let mut i = 0;
    let mut res = String::new();
    'j:for j in 0..k {
        for c in az.chars() {
            if let Some(ei) = s[i..].iter().position(|t| t == &c) {
                // Sの残りの文字数がk-jを満たすかどうか
                if n-(i+ei) >= k-j {
                    res.push(c);
                    i += ei+1;
                    break;
                }
                if res.len() == k {
                    break 'j;
                }
            }
        }
    }
    println!("{}", res);
}
