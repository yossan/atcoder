use proconio::*;

fn main() {
    input! {
        n: usize,
        a: usize, // 文字列回転操作コスト
        b: usize, // 差し替え操作コスト
        s: marker::Chars,
    }
    // a円支払う操作の回数を全探索する
    // 回転操作ごとの差し替えコストを計算し、
    // 最小値を求める
    let mut ans = std::u64::MAX;

    // 文字列の回転操作 s += s;
    let s = s.iter().chain(&s).collect::<Vec<&char>>();
    for i in 0..n {
        let mut tmp: u64 = (a as u64) * (i as u64);
        for j in 0..(n / 2) {
            let l = i + j;
            let r = i + n - 1 - j;
            if s[l] != s[r] { 
                tmp += b as u64;
            }
        }
        // i回回転時のコスト
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}

