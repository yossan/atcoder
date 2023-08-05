use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        s: [marker::Chars; n],
    }
    let mut ans = 0;
    let mut set: BTreeSet<String> = BTreeSet::new();
    for i in 0..n {
        let si = s[i].clone().iter().collect::<String>();
        if !set.contains(&si) {
            ans += 1;
        }
        set.insert(si.clone());
        let rev = si.chars().rev().collect::<String>();
        set.insert(rev);
    }
    println!("{}", ans);
}
