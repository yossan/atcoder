use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(
    ha: usize,
    _wa: usize,
    a: [Chars; ha],
    hb: usize,
    _wb: usize,
    b: [Chars; hb],
    hx: usize,
    wx: usize,
    xx: [Chars; hx],
) -> bool {
    let ac = a
        .iter()
        .map(|r| r.iter().filter(|c| **c == '#').count())
        .sum::<usize>();
    let bc = b
        .iter()
        .map(|r| r.iter().filter(|c| **c == '#').count())
        .sum::<usize>();
 
    let get = |bd: &Vec<Vec<char>>, x: isize, y: isize| -> char {
        if x < 0 || y < 0 || x >= bd[0].len() as isize || y >= bd.len() as isize {
            return '.';
        }
        bd[y as usize][x as usize]
    };
 
    for oy in -20..=20_isize {
        for ox in -20..=20_isize {
            for sy in -40..=40 {
                for sx in -40..=40 {
                    let mut acnt = 0;
                    let mut bcnt = 0;
 
                    let mut ok = true;
 
                    'case: for y in 0..hx as isize {
                        for x in 0..wx as isize {
                            let ca = get(&a, x + sx, y + sy);
                            let cb = get(&b, x + sx + ox, y + sy + oy);
                            if ca == '#' {
                                acnt += 1;
                            }
                            if cb == '#' {
                                bcnt += 1;
                            }
                            if (xx[y as usize][x as usize] == '#') != (ca == '#' || cb == '#') {
                                ok = false;
                                break 'case;
                            }
                        }
                    }
 
                    if ok && acnt == ac && bcnt == bc {
                        eprintln!("{oy} {ox} {sx} {sy}");
                        return true;
                    }
                }
            }
        }
    }
 
    false
}
