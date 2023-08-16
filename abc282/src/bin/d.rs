use proconio::*;
use std::collections::BTreeMap;

type Adj = BTreeMap<usize, Vec<usize>>;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut adj = Adj::new();
    for (u, v) in uv {
        adj.entry(u).or_default().push(v);
        adj.entry(v).or_default().push(u);
    }
    let mut color = vec![0isize; n + 1];

    // 全頂点のペアの組み合わせ数 - 既存の辺の数
    let mut ans = (n * (n - 1) / 2 - m) as isize;
    for i in 1..=n {
        if color[i] == 0 {
            let res = dfs(i, 1, &adj, &mut color);
            if res.0 == -1 {
                println!("0");
                return;
            }
            ans -= res.0 * (res.0 - 1) / 2;
            ans -= res.1 * (res.1 - 1) / 2;
        }
    }
    println!("{}", ans);
}

// v: 移動する頂点, c: 塗りつぶす色
fn dfs(v: usize, c: isize, adj: &Adj, color: &mut Vec<isize>) -> (isize, isize) {
    let mut ret = (0, 0);
    // 色付け
    color[v] = c;
    // 色のカウント
    if c == 1 {
        ret.0 += 1;
    } else {
        ret.1 += 1;
    }
    if let Some(list) = adj.get(&v) {
        for &next in list {
            if color[next] == -c {
                // すでに塗りつぶしている頂点はパス
                continue;
            }
            if color[next] == c {
                // 二部グラフでない
                return (-1, -1);
            }
            let res = dfs(next, -c, adj, color);
            if res.0 == -1 {
                // 二部グラフでない
                return (-1, -1);
            }
            ret.0 += res.0;
            ret.1 += res.1;
        }
    }
    ret
}
