use proconio::*;
// . = 透明ます
// # = 黒います

fn sharp_count(ss: &Vec<Vec<char>>) -> usize {
    let mut cnt = 0;
    for s in ss {
        cnt += s.iter().filter(|t| { **t == '#' }).count();
    }
    cnt
}

fn main() {
    input! {
        ha: usize,
        wa: usize,
        sa: [marker::Chars; ha],
        hb: usize,
        wb: usize,
        sb: [marker::Chars; hb],
        hx: usize,
        wx: usize,
        sx: [marker::Chars; hx],
    }
    let a = sharp_count(&sa);
    let b = sharp_count(&sb);
    let x = sharp_count(&sx);
    if x < a || x < b {
        println!("No");
        return;
    }

    for ia in 0..(10-ha) {
        for ja in 0..(10-wa) {
            for ib in 0..(10-hb) {
                for jb in 0..(10-wb) {
                    let mut sz = vec!["..........".chars().collect::<Vec<char>>(); 10];
                    for i in 0..ha {
                        for j in 0..wa {
                            let a = sa[i][j];
                            if a == '#' {
                                sz[ia+i][ja+j] = '#';
                            }
                        }
                    }

                    for i in 0..hb {
                        for j in 0..wb {
                            let b = sb[i][j];
                            if b == '#' {
                                sz[ib+i][jb+j] = '#';
                            }
                        }
                    }

                    for xi in 0..(10-hx) {
                        for xj in 0..(10-wx) {
                            let mut hit = true;
                            'i:for i in 0..hx {
                                for j in 0..wx {
                                    let c = sz[xi+i][xj+j];
                                    let x = sx[i][j];
                                    if c != x {
                                        hit = false;
                                        break 'i;
                                    }

                                }
                            }
                            if hit {
                                println!("Yes");
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}



