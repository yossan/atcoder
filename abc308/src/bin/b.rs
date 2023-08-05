use proconio::*;
fn main() {
    input! {
        n: usize,
        m: usize,
        cc: [marker::Chars; n], // i皿目の色
        dd: [marker::Chars; m], // diの色はpiの価格に対応
        pp: [usize; m+1], // diの価格に対応ただしd0はその他の皿の値
    }
    let mut ans = 0;
    for i in 0..n {
        let c = &cc[i];
        let mut j = -1;
        for (k, d) in dd.iter().enumerate() {
            if c == d {
                j = k as isize + 1;
                break;
            }
        }
        let price = if j != -1 {
            pp[j as usize]
        } else {
            pp[0]
        };
        ans += price;
    }
    println!("{}", ans);
}
