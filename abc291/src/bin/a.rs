fn main() {
    proconio::input! {
        ss: proconio::marker::Chars,
    }
    for (i, &s) in ss.iter().enumerate() {
        if (s as u8) >= 'A' as u8 && (s as u8) <= 'Z' as u8 {
            println!("{}", i+1);
            break;
        }
    }
}
