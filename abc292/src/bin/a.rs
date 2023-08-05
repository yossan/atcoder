fn main() {
    proconio::input! {
        s: proconio::marker::Chars,
    }

    let z = 'a' as u8 - 'A' as u8;
    for c in s {
        print!("{}", (c as u8 - z) as char);
    }
    println!("");
}
