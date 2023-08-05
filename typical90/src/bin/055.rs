// https://atcoder.jp/contests/typical90/tasks/typical90_bc
use proconio::*;
use itertools::*;

fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n],
    }
    let mut cnt = 0;
    for nums in a.iter().combinations(5) {
        let mut windows = nums.windows(5);
        while let Some(&[a1, a2, a3, a4, a5]) = windows.next() {
            if a1 * a2 % p * a3 % p * a4 % p * a5 % p == q {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
