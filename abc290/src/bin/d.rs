use std::collections::BTreeSet;
fn main() {
    proconio::input! {
        n: usize,
        tests: [(usize, usize, usize); n],
    }


    for (n, d, mut k) in tests {
        let mut mark = BTreeSet::new();
        mark.insert(0);
        let mut x = 0;

        'test:loop {
            k -= 1;
            if k == 0 {
                println!("{}", x);
                break 'test;
            }
            x = (x+d) % n; 
            'x:loop {
                if mark.insert(x) {
                    break 'x;
                }
                x = (x+1) % n;
            }
        }
    }
}
