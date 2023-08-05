use std::collections::BTreeSet;

fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        aa: [[usize; w]; h],
    }
    let mut ans = 0;

    // ルート数
    let l = h + w - 2;
    for p in 0..(1<<l) {
        if cnt_one_bit_numbers(p, l) != h-1 { continue };
        let mut x = 0;
        let mut y = 0;
        let mut s = BTreeSet::new();
        s.insert(aa[0][0]);
        for i in 0..l {
            if (p >> i) % 2 == 1 {
                y += 1;
            } else {
                x += 1;
            }
            s.insert(aa[y][x]);
        }
        // 重複が存在するかどうか
        if s.len() == h + w - 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn cnt_one_bit_numbers(p: usize, l: usize) -> usize {
    let mut cnt = 0;
    for i in 0..l {
        if (p >> i) % 2 == 1 {
            cnt += 1;
        }
    }
    cnt
}
