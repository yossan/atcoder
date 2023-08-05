use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        aa: [[usize; w]; h],
    }

    let l = h + w -2;
    let mut ans = 0;
    let mut pp = vec![0usize; l];
    for i in 0..l {
        if i >= w-1 {
            // 横
            pp[i] = 1;
        }
    }

    for p in pp.iter().permutations(pp.len()) {
        let mut x = 0;
        let mut y = 0;
        let mut set = BTreeSet::new();
        set.insert(aa[0][0]);
        for i in 0..l {
            if p[i] == &1 {
                y += 1;
            } else {
                x += 1;
            }
            set.insert(aa[x][y]);
        }
        // 重複があるかどうか
        if set.len() == l + 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
