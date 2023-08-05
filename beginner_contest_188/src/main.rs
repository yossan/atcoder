use std::io::prelude::*;
use std::io::stdin;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();
    let _ = stdin.read_line(&mut input);
    input.trim().parse::<T>().ok().unwrap()
}

const d: usize = 5;
fn main() {
    const N: usize = read();
    let a = [0i32; N];
    let A = read::<String>().split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let B = read::<String>().split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let sum: i32 = A.iter().zip(B.iter()).map(|(a, b)| a * b).sum();
    if sum == 0 {
        println!("Yes");
    } else {
        println!("No");
    }

}
