fn main() {
    proconio::input! {
        n: usize,
    }
    let mut ans = 0;
    for i in 1..n {
        let (p, q) = (i, n-i);
        let x = cnt_xy(p);
        let y = cnt_xy(q);
        ans += x * y;
    }
    println!("{}", ans);
}

fn cnt_xy(n: usize) -> usize {
    let mut cnt = 0;
    let mut j = 1;
    loop {
        if j*j > n {
            break;
        }
        if n % j == 0 {
            cnt += 1;
            if n != j*j {
                cnt += 1;
            }
        }
        j += 1;
    }
    cnt
}
