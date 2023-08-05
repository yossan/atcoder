use std::collections::BTreeSet;
use proconio::input;

fn main() {
    input! {
        h : usize,
        w : usize,
        a : [[usize; w]; h]
    }

    let mut set = BTreeSet::new();
    let ans = dfs(h, w, 0, 0, &mut set, &a);
    println!("{}", ans);
}
/*
3 3
1 2 3
2 4 3
3 3 6
 */
fn dfs(h:usize, w:usize, i:usize, j:usize, set:&mut BTreeSet<usize>, a:&Vec<Vec<usize>>) -> usize {
    if !set.insert(a[i][j]) { return 0; }


    let mut res = if i+1 == h && j+1 == w { 1 } else { 0 };
    if i + 1 < h {
        res += dfs(h, w, i+1, j, set, a);
    }

    if j + 1 < w {
        res += dfs(h, w, i, j+1, set, a);
    }

    set.remove(&a[i][j]);
    res
}
