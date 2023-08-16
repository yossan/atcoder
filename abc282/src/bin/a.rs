// Generalized ABC
use proconio::*;

fn main() {
    input! {
        k: usize,
    }
    for i in 0..k {
        print!("{}", ('A' as u8 + i as u8) as char);
    }
    println!("");
}
