// 004 - Cross Sum (★2)
use proconio::*; 

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let mut row = vec![0; w];
    let mut column = vec![0; h];
    for i in 0..h {
        for j in 0..w {
            column[i] += a[i][j];
            row[j] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", column[i] + row[j] - a[i][j]);
        }
        println!("");
    }
    println!("");
}
