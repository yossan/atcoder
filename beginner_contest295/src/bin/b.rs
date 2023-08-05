fn main() {
    proconio::input! {
        r: usize, // 縦
        c: usize, // 横
        b: [proconio::marker::Chars; r],
    }
    let mut board = b.clone();
    for i in 0..r {
        for j in 0..c {
            if b[i][j] == '.' || b[i][j] == '#' {
                continue;
            }
            let bomb = (b[i][j] as u8 - '0' as u8) as isize;
            for bi in -bomb..=bomb {
                for bj in -bomb..=bomb {
                    let (h, w, row, col) = (i as isize, j as isize, r as isize, c as isize);
                    if h + bi < 0 || h + bi >= row { continue };
                    if w + bj < 0 || w + bj >= col { continue };
                    let ei = h + bi;
                    let ej = w + bj;
                    let dist = isize::abs(ei - h) + isize::abs(ej - w);
                    if dist <= bomb {
                        board[ei as usize][ej as usize] = '.';
                    }
                }
            }
        }
    }
    for i in 0..r {
        for j in 0..c {
            print!("{}", board[i][j]);
        }
        println!("");
    }
}
