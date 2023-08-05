fn main() {
    proconio::input! {
        n: usize,
        x: i32,
        mut a: [i32; n],
    }
    a.sort();
    for j in 0..n {
        if let Ok(_ret) = a.binary_search(&(x+a[j])) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
