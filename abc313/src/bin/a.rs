use proconio::*;

fn main() {
    input! {
        n: usize,
        pp: [isize; n],
    }
    let mut ans = -1;
    for i in 1..n {
        if pp[i] >= pp[0] {
            ans = ans.max(pp[i] - pp[0]);
        }
    }
    if ans == -1 {
        println!("0");
    } else {
        println!("{}", ans+1);
    }
}
