/*
 * 各要素が0 or 1であるNXN行列A,B
 * Aを回転させることで、Aij = Bij = 1が成り立つか判定せよ
 * 回転させるとは
 * * 全てのi,jについて同時にA_(i,j)をA_(N+1-j,i)で置き換える
 *
 * ポイント
 * * 配列のコピーと回転操作
 */
fn main() {
    proconio::input! {
        n: usize, // <= 100
        mut a: [[usize; n]; n],
        b: [[usize; n]; n],
    }

    for _ in 1..=4 {
        r(&mut a);
        if chk(&a, &b) {
            println!("Yes"); 
            return;
        }
    }
    println!("No");
}

fn r(a: &mut Vec<Vec<usize>>) {
    let n = a.len();
    let mut t: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            t[n - 1 - j][i] = a[i][j];
        }
    }
    for i in 0..n {
        for j in 0..n {
            a[i][j] = t[i][j];
        }
    }
}

fn chk(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> bool {
    let n = a.len();
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 && b[i][j] == 0 {
                return false;
            }
        }
    }
    true
}
