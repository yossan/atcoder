fn main() {
    proconio::input! {
        mut s: proconio::marker::Chars,
    }

    for i in 0..s.len()/2 {
        s.swap(2*i, 2*i+1);
    }
    println!("{}", s.iter().collect::<String>());
}
