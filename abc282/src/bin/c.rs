use proconio::*;

fn main() {
    input! {
        n: usize,
        s: marker::Chars,
    }

    let mut d_flag = false;
    for i in 0..n {
        match &s[i] {
            ',' => {
                if d_flag {
                    print!(",");
                } else {
                    print!(".");
                }
            },
            '"' => {
                d_flag = !d_flag;
                print!("\"");
            },
            a => {
                print!("{}", a);
            }
        }
    }
    println!("");
}

