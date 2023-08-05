fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [proconio::marker::Chars; h],
        b: [proconio::marker::Chars; h],
    }
    for s in 0..=h {
        for t in 0..=w {
            let mut f = true;
            for i in 0..h {
                for j in 0..w {
                    f &= a[(i+s)%h][(j+t)%w] == b[i][j];
                }
            }
            if f {
                println!("Yes");
                return;
            }

        }
        
    }
    println!("No");
}
