use proconio::*;

fn main() {
    input! {
        h: isize,
        w: isize,
        s: [marker::Chars; h],
    }
    // 進む方向リスト
    let dir = [(-1, -1), (-1, 0), (-1, 1), (0, -1,), (0, 1), (1, -1), (1, 0), (1, 1)];
    for i in 0..h {
        for j in 0..w {
            for d in &dir {
                let mut str = Vec::new();
                for t in 0..5 {
                    let x = i + t * d.0;
                    let y = j + t * d.1;
                    if x < 0 || x >= h || y < 0 || y >= w {
                        break;
                    }
                    str.push(s[x as usize][y as usize]);
                }
                if str == ['s', 'n', 'u', 'k', 'e'] {
                    for t in 0..5 {
                        let x = i + t * d.0;
                        let y = j + t * d.1;
                        println!("{} {}", x+1, y+1);
                    }
                }
            }
        }
    }
}

