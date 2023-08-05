fn main() {
    proconio::input! {
        n: usize,
        aa: [usize; n],
    }
    aa.iter().for_each(|a| { if a % 2 == 0 { print!("{} ", a); }});
    println!("");
}
