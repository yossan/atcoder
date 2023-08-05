use proconio::*;

fn main() {
    input! {
        n: usize,
        aa: [isize; n],
    }
    print!("{} ", aa[0]);
    for i in 0..n-1 {
        let a1 = aa[i];
        let a2 = aa[i+1];
        if (a2 - a1).abs() == 1 {
            print!("{} ", a2);
        } else {
            if a2 > a1 {
                let mut x = a1 + 1;
                loop {
                    print!("{} ", x);
                    if a2 - x == 1 {
                        break;
                    }
                    x += 1;
                }
            } else {
                let mut y = a1 - 1;
                loop {
                    print!("{} ", y);
                    if y - a2 == 1 {
                        break;
                    }
                    y -= 1;
                }

            }
            print!("{} ", a2);
        }
    }
    println!("");
}

