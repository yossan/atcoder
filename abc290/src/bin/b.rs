fn main() {
    proconio::input! {
        n: usize,
        mut k: usize,
        s: proconio::marker::Chars,
    }

    for i in 0..n {
        if k > 0 && s[i] == 'o' {
            print!("o");
            k -= 1;
        } else {
            print!("x");
        }
    }
    println!("");
}
