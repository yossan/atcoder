use proconio::*;
use itertools::*;

fn main() {
    input! {
        n: usize,
        s: [marker::Chars; n],
    }
    for ss in s.iter().combinations(2) {
        let mut v = Vec::new();
        v.extend(ss[0].clone());
        v.extend(ss[1].clone());
        let len = v.len();
        let mut hit = true;
        for i in 0..len {
            if v[i] != v[len-1-i] {
                hit = false;
                break;
            }
        }
        if hit {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
