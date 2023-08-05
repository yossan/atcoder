fn main() {
    proconio::input! {
        _n: usize,
        s: proconio::marker::Chars,
    }
    let mut p = &s[0];
    for c in s.iter().skip(1) {
        if c == p { 
            println!("No");
            return;
        }
        p = c;
    }
    println!("Yes");
}
