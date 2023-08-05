fn main() {
    proconio::input! {
        n: usize,
        mut x: [usize; 5*n],
    }
    x.sort_unstable();
    let mut sum = 0;
    for i in n..4*n {
        sum += x[i];
    }
    let ave: f32 = (sum as f32) / (3 * n ) as f32;
    println!("{}", ave);
}
