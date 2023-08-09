use proconio::*;

fn main() {
    input! {
        n: usize,
        s: marker::Chars,
    }
    let mut i = 0;
    while i < n {
        if i < n-1 && s[i..=i+1] == ['n', 'a'] {
            print!("nya");
            i += 2;
        } else {
            print!("{}", s[i]);
            i += 1;
        }
    }
    println!("");
}

