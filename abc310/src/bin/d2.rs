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
    let teams = vec![0usize; t];

    let mut queue = Vec::new();
    queue.push((0, teams));
    let mut ans = 0;
    while !queue.is_empty() {
        let (k, mut teams) = queue.pop().unwrap();
        if k == n {
            // チームの中に0が一つでもあればNG
            if teams.iter().all(|&x| x != 0) {
                ans += 1;
            }
        } else {
            for i in 0..t {
                if (teams[i] & p[k]) == 0 {
                    teams[i] ^= 1 << k;
                    // 次の選手を追加
                    queue.push((k + 1, teams.clone()));
                    // 次へは行かない
                    teams[i] ^= 1 << k;
                    if teams[i] == 0 &&
                        i + 1 < t && teams[i+1] == 0 {
                        break;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
