fn main() {
    proconio::input! {
        s: proconio::marker::Chars,
    }
    for c in s {
        match c {
            '0' => {
                print!("1");
            },
            '1' => {
                print!("0");
            },
            _ => (),
        }
    }
    println!("");
}
