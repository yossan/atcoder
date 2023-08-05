# a問題

結果: 導出
時間: 5分未満
リンク: https://atcoder.jp/contests/abc300/tasks/abc300_a

## 問題概要: N-choice question

a+bの答えをN個の選択肢の中から回答する

# b問題

結果: 非導出
時間: 10分未満
リンク: https://atcoder.jp/contests/abc300/tasks/abc300_b

## 問題概要: Sample Map in the RPG World

マップAを操作した結果、マップBと同一かどうか

## 問題ポイント

* マップAの操作の全網羅とマップの走査の組み合わせ
* 全て一致したかどうか判定

```b.rs
for s in 0..=h {
    for t in 0..=w {
        let mut f = true;
        for i in 0..h {
            for j in 0..w {
                f &= a[(i+s)%h][(j+t)%w] == b[i][j];
            }
        }
    }
}
```

# c問題

結果: 非導出
時間: 85分

## 問題概要: Cross

* Cross判定が不要だった
  バツ印を構成するマス以外に # は書かれていません。
* バツ印のカウント
  (i, j)から内部にkで回してバツ印のカウントができた
* カウント済みかどうかの判定
  最後の'#'の位置にフラグを立てる

```c.rs
if c[i][j] == '#' {
    let mut count = 1;
    let mut ei = i;
    let mut ej = j;
    for k in 1..n {
        if i + k >= h || j + k >= w {
            break;
        }
        if c[i+k][j+k] == '#' {
            count += 1;
            ei = i+k;
            ej = j+k;
        } else {
            break;
        }
    }
    if count >= 3 && !visit[ei][ej] {
        visit[ei][ej] = true;
        ans[(count / 2) - 1] += 1;
    }
}
```

## 別解 

外枠を確認するだけでも良い
https://atcoder.jp/contests/abc300/submissions/41029575

```
for p in 0..h.min(w) {
    if i < p || j < p ||i + p >= w || j + p >= h {
        break;
    }
    if a[j - p][i - p] == b'#' &&
        a[j + p][i - p] == b'#' &&
        a[j - p][i + p] == b'#' &&
        a[j + p][i + p] == b'#' {
            size += 1;
        } else {
            break;
        }
}

if size > 0 {
    result[size - 1] += 1;
}
```

