// Sandwich Number
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        s: Chars,
    }
    if s.len() != 8 {
        println!("No");
        return;
    }
    if s[0] < 'A' || s[0] > 'Z' {
        println!("No");
        return;
    }
    if s[7] < 'A' || s[7] > 'Z' {
        println!("No");
        return;
    }

    let st = s[1..=6].iter().collect::<String>();
    if let Ok(num) = st.parse::<isize>() {
        if num >= 100_000 && num <= 999_999 {
            println!("Yes");
            return;
        }
    }
    println!("No");
    return;
}

