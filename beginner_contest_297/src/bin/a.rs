fn main() {
    proconio::input! {
        n: usize,
        d: usize,
        t: [usize; n],
    }
    let mut v = t.windows(2);
    while let Some(&[a, b]) = v.next() {
        if b - a <= d {
            println!("{}", b);
            return;
        }
    }
    println!("-1");
}
