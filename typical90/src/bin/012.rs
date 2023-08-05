use proconio::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut squares = vec![vec![false; 2000]; 2000]; 

    for qi in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    ri: usize,
                    ci: usize,
                }
                squares[ri][ci] = true;
            },
            2 => {
                input! {
                    rai: usize,
                    cai: usize,
                    rbi: usize,
                    cbi: usize,
                }
                if !squares[rai][cai] || !squares[rbi][cbi] {
                    println!("No");
                    continue;
                }

                let dy = rbi - rai;
                let dx = cbi - cai;
            }

            _ => { panic!() }
        }
    }
}
