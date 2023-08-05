use std::collections::BTreeSet;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut set = BTreeSet::new();
    for i in a {
        set.insert(i);
    }

    for i in 0..k {
        if !set.contains(&i) {
            println!("{}", i);
            return;
        }
    }

    println!("{}", k);
}
