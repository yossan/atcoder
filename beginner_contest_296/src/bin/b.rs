fn main() {
    proconio::input! {
        ss: [proconio::marker::Chars; 8],
    }
    for i in 0..8 {
        for j in 0..8 {
            if ss[i][j] == '*' {
                println!("{}{}", (97 + j as u8) as char, 8-i);
            }
        }
    }
}
