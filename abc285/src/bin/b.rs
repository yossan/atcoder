use proconio::*;

fn main() {
    input! {
        n: usize,
        s: marker::Chars,
    }

    for i in 0 .. (n - 1) {
        let mut point = 0;
        for j in 0 .. (n - 1) {
            if i + j + 1 >= n { 
                break;
            }
            if s[j] != s[i + j + 1] {
                point += 1;
            } else {
                break;
            }
        }
        println!("{}", point);
    }
}

