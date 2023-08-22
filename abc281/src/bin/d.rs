// Max Multiple
use proconio::*;
use itertools::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    }

    let mut s = Vec::new();

    let t = a.iter().combinations(k);
    for v in t {
        let mut sum = 0;
        for i in 0..k {
            sum += v[i];
        }
        s.push(sum);
    }

    s.sort();
    let ret = s.iter().filter(|e| {
        *e % d == 0
    }).collect::<Vec<&usize>>();

    if ret.is_empty() {
        println!("-1");
    } else {
        println!("{}", ret.last().unwrap());
    }
}
