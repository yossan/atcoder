use proconio::*;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        t: [(u64, u64); n],
    }

    let mut tmp = vec![(0usize, 0u64, 0u64); n];
    for (i, m) in t.iter().enumerate() {
        let (a, b) = m;
        tmp[i] = (i+1, *a, *a + *b);
    }
    tmp.sort_by(|a, b| {
        // 互いに分母を払って比較する
        let v1 = a.1 * b.2;
        let v2 = b.1 * a.2;
        if v1 == v2 {
            a.0.cmp(&b.0)
        } else if v1 < v2 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    for m in tmp {
        print!("{} ", m.0);
    }
    println!("");
}
