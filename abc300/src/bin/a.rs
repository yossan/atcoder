fn main() {
    proconio::input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n],
    }
    for i in 0..c.len() {
        if c[i] == a + b {
            println!("{}", i+1);
        }
    }
}
