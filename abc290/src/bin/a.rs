fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut sum = 0;
    for num in b {
        sum += a[num-1];
    }
    println!("{}", sum);

}
