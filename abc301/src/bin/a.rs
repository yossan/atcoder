use proconio::*;

fn main() {
    input! {
        n: usize,
        ss: marker::Chars,
    }
    let mut t_win = 0;
    let mut a_win = 0;

    for i in 0..n {
        match ss[i] {
            'T' => {
                t_win += 1;
            },
            'A' => {
                a_win += 1;
            },
            _ => {
            }
        }
    }
    if t_win > a_win {
        println!("T");
    } else if t_win < a_win {
        println!("A");
    } else {
        if ss[n-1] == 'T' {
            println!("A");
        } else {
            println!("T");
        }
    }
}
