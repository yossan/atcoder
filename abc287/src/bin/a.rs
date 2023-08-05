use proconio::*;

fn main() {
    input! {
        n: usize,
        s: [marker::Chars; n],
    }
    let mut ans: isize = 0;
    for i in 0..n {
        let ss = s[i].iter().collect::<String>();
        match &*ss {
            "For" => ans += 1,
            "Against" => ans -= 1,
            _ => (),
        }
    }
    println!("{}", if ans > 0 { "Yes" } else { "No" });
}
