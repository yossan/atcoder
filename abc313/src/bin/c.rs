use proconio::*;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }
    if aa.len() == 1 {
        println!("0");
        return;
    }
    let sum = aa.iter().sum::<i64>();
    let ave = sum / (n as i64);
    dbg!(&ave);
    let mut set = vec![0; n];
    for i in 0..n {
        set[i] = ave - aa[i];
    }
    set.sort();
    dbg!(&set);
    let ret = set.binary_search_by(|&a| {
        if a >= 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let ret = match ret {
        Ok(b) => b,
        Err(e) => e,
    };
    let mut m = 0;
    let mut p = 0;
    for i in 0..n {
        if i < ret {
            m += set[i];
        } else {
            p += set[i]
        }
    }
    dbg!(m);
    dbg!(p);
    
}


