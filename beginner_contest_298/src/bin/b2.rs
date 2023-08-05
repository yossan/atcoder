// 配列をコピーしないでマトリックスの回転操作ができないか
fn main() {
    proconio::input! {
        n: usize,
        a: [[usize; n]; n],
        b: [[usize; n]; n],
    }
    for t in 1..=4 {
        let mut ans = true;
        for i in 0..n {
            for j in 0..n {
                let (ei, ej) = match t {
                    1 => {
                        (j, n-i-1)
                    }
                    2 => {
                        (n-i-1, n-j-1)
                    }
                    3 => {
                        (n-j-1, i)
                    }
                    _ => {
                        (i, j)
                    }
                };
                ans &= if a[ei][ej] == 1 {
                    b[i][j] == 1
                } else {
                    true
                }
            }
        }
        if ans {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
