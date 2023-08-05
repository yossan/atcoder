use proconio::*;
use itertools::*;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [marker::Chars; n],
    }

    let mut sss = Vec::new();
    for s in &ss {
        sss.push(s.iter().collect::<String>());
    }
    
    let mut set = BTreeSet::new();
    for s in sss.iter().permutations(n) {
        if !set.insert(s.clone()) {
            continue;
        }
        let mut ok = true;
        for i in 0..n-1 {
            let mut diff = 0;
            let s1 = s[i].as_bytes();
            let s2 = s[i+1].as_bytes();
            for j in 0..m {
                if s1[j] != s2[j] {
                    diff += 1;
                }
            }
            if diff != 1 {
                ok = false;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

