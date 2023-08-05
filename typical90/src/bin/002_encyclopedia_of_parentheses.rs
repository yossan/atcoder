use proconio::*;
use std::iter::repeat;

fn main() {
    input! {
        n: usize,
    }
    if n % 2 != 0 {
        return;
    }

    let mut s = repeat('(')
        .take(n / 2)
        .chain(repeat(')').take(n / 2))
        .collect::<Vec<_>>();
    loop {
        println!("{}", s.iter().collect::<String>());
        match s.windows(3).rposition(|v| v == &['(', ')', ')']) {
            None => break,
            Some(i) => {
                s.swap(i, i + 1);
                s[i + 1..].sort();
            }
        };
    }
}
