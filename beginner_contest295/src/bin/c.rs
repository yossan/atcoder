fn main() {
    proconio::input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort();
    let mut i = 0;
    let mut pair = 0;
    while i < n - 1 {
        if aa[i] == aa[i+1] {
            pair += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    println!("{}", pair);
}
