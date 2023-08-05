use proconio::*;

fn main() {
    input! {
        n: usize,
        students: [(usize, usize); n],
        q: usize,
        queries: [(usize, usize); q],
    }
    // 累積和
    let mut sum = vec![vec![0usize; n+1]; 2];
    for i in 1..=n {
        let std = students[i-1];
        if std.0 == 1 {
            sum[0][i] = sum[0][i-1] + std.1;
            sum[1][i] = sum[1][i-1];
        } else {
            sum[0][i] = sum[0][i-1];
            sum[1][i] = sum[1][i-1] + std.1;
        }
    }
    for q in queries {
        let ans0 = sum[0][q.1] - sum[0][q.0 - 1];
        let ans1 = sum[1][q.1] - sum[1][q.0 - 1];
        println!("{} {}", ans0, ans1);
    }
}
