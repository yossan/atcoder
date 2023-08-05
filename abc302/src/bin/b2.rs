use proconio::*;
use std::collections::{BTreeMap, BTreeSet};

type Adjacent = BTreeMap<usize, BTreeSet<usize>>;

fn main() {
    input! {
        h: isize,
        w: isize,
        s: [marker::Chars; h],
    }

    let dir = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut g: Adjacent = BTreeMap::new();
    for i in 0..h {
        for j in 0..w {
            let pos = (i*h + j) as usize;
            for d in dir {
                let x = i + d.0;
                let y = j + d.1;
                if x < 0 || x >= h || y < 0 || y >= w {
                    continue;
                }
                let c = s[x as usize][y as usize];
                if "snuke".contains(c) {
                    let nex = x*h + y;
                    g.entry(pos).or_default().insert(nex as usize);
                }
            }
        }
    }
}

fn dfs(pos: usize, g: &Adjacent, visited: &mut [bool], h: usize, w: usize, ss: &Vec<Vec<char>>) {

    if let Some(nexts) = g.get(&pos) {
        print!("{}: ", pos);
        for &next in nexts {
            let x = next / h;
            let y = next % h;
            print!("{} ", ss[x][y]);
        }
        println!("")
    }
}
