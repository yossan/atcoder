// グリッドにはいくつかのバツ印があります。バツ印を構成するマス以外に # は書かれていません
fn main() {
    proconio::input!{
        h: usize,
        w: usize,
        c: [proconio::marker::Chars; h],
    }
    let n = std::cmp::min(h, w);
    let mut ans = vec![0usize; n];
    let mut visit = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                let mut count = 1;
                let mut ei = i;
                let mut ej = j;
                for k in 1..n {
                    if i + k >= h || j + k >= w {
                        break;
                    }
                    if c[i+k][j+k] == '#' {
                        count += 1;
                        ei = i+k;
                        ej = j+k;
                    } else {
                        break;
                    }
                }
                if count >= 3 && !visit[ei][ej] {
                    visit[ei][ej] = true;
                    ans[(count / 2) - 1] += 1;
                }
            }
        }
    }
    for (i, a) in ans.iter().enumerate() {
        print!("{} ", a);
    }
    println!("");
}
