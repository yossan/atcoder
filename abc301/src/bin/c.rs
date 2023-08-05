use proconio::*;

fn main() {
    input! {
        ss: marker::Chars,
        tt: marker::Chars,
    }

    let mut cnt = vec![vec![0isize; 256]; 2];
    for s in ss {
        cnt[0][s as usize] += 1;
    }
    for t in tt {
        cnt[1][t as usize] += 1;
    }

    let atmark = '@' as usize;
    for a in "atcoder".chars() {
        let a = a as usize;
        // 'atcoder'の各文字についてS,Tに含まれる個数を数え、少ない方の個数が多い方の個数と等しくなるように'@'を使って数を揃えていく
        let m = isize::max(cnt[0][a], cnt[1][a]);
        for j in 0..2 {
            // '@'の個数が足りているか
            if cnt[j][atmark] < m-cnt[j][a] {
                println!("No");
                return;
            }
            // '@'を消費する
            cnt[j][atmark] -= m-cnt[j][a];
            // 個数を揃える
            cnt[j][a] = m;
        }
    }

    // 全文字において個数が等しいことを確認する
    let mut ans = true;
    for i in 0..256 {
        ans &= cnt[0][i] == cnt[1][i];
    }
    println!("{}", if ans { "Yes" } else { "No" });
}

