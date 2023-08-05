fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [usize; n], 
        b: [usize; m], 
    }
    let mut c = Vec::new();
    for ai in &a {
        c.push(ai);
    }
    for bi in &b {
        c.push(bi);
    }
    c.sort_unstable();

    for ai in &a {
        let i = c.binary_search(&ai).unwrap();
        print!("{} ", i+1);
    }
    println!("");
    for bi in &b {
        let i = c.binary_search(&bi).unwrap();
        print!("{} ", i+1);
    }
    println!("");
}
