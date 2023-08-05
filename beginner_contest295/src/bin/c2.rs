use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        mut aa: [usize; n],
    }
    let mut mapping = HashMap::new();
    for v in aa {
        *mapping.entry(v).or_insert(0usize) += 1;
    }
    let result = mapping.values().map(|v| v / 2).sum::<usize>();
    println!("{}", result);
}
