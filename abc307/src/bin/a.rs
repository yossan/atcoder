use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; 7*n],
    }

    let mut steps = 0;
    for i in 1..=n * 7 {
        steps += a[i-1];
        if i % 7 == 0 {
            print!("{} ", steps);
            steps = 0;
        }
    }
    println!("");
}
