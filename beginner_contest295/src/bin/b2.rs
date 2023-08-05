fn main() {
    proconio::input! {
        r: usize,
        c: usize,
        b: [proconio::marker::Chars; r],
    }

    let mut ans = b.clone();
    for h in 0..r {
        for w in 0..c {
            if b[i][j] == '.' || b[i][j] == '#' {
                continue;
            }
            let bomb = (b[h][w] as u8 - '0' as u8) as isize;
            for i in bomb ..= bomb {
            }
        }
    }
}
