use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; 5*n],
    }

    a.sort_unstable();

    let ans: i32 = a[n..4*n].iter().sum();

    println!("{}", ans as f64 / (3.0 * n as f64));
}

