use proconio::*;

fn main() {
    input! {
        n: usize,
        p: usize, // 正規額
        q: usize, // 割引額
        d: [usize; n],
    }
    let mut min = p;
    for i in 0..n {
        let o = d[i] + q;
        if min > o {
            min = o;
        }
    }
    println!("{}", min);
}
