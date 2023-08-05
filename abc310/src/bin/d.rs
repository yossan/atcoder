// ycodeさんのを検証
// https://atcoder.jp/contests/abc310/submissions/43660946
use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    // p[i]のjビット目が1 => i番目の選手とj番目のの選手は相性が悪い
    let mut p = vec![0usize; n];
    for (a, b) in ab {
        p[a] |= 1 << b;
        p[b] |= 1 << a;
    }

    // tチーム作成する
    // teams[i][k] = 1 => チームiにk番目の選手が存在する
    let mut teams = vec![0usize; t];

    let ans = dfs(n, t, &p, &mut teams, 0);
    println!("{}", ans);
}

// n人, tチーム, p = 隣接リスト(ビット表現), teams = チームリスト, k番目の選手について
fn dfs(n: usize, t: usize, p: &Vec<usize>, teams: &mut Vec<usize>, k: usize) -> usize {
    // eprintln!("k = {}, teams = {:?}", k, teams);
    if k == n {
        // チームの中に0が一つでもあればNG
        let d = if teams.iter().all(|&x| x != 0) { 1 } else { 0 };
        return d;
    }

    let mut ret = 0usize;
    for i in 0..t {
        // eprintln!("t = {}", i);
        // チームiのメンバーとk番目の選手のブラックリストを比較
        if (teams[i] & p[k]) == 0 {
            // k番目の選手をチームiに追加する
            teams[i] ^= 1 << k;
            // 次の選手へ
            ret += dfs(n, t, p, teams, k + 1);
            // k番目の選手をチームiから除外する (次のチームに追加できるように)
            teams[i] ^= 1 << k;

            // チーム名が区別されないので
            // k番目の選手を追加前の状態が誰一人もいない場合はbreak
            if teams[i] == 0 {
                break;
            }
        }
    }
    ret
}

