use proconio::*;

fn main() {
    input! {
        s: marker::Chars,
    }
    for c in s {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u'  => (),
            _ => {
                print!("{}", c);
            }
        }
    }
    println!("");
}
