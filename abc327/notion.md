abc327  
https://atcoder.jp/contests/abc327/tasks

# 結果と概要

```
{o} A - 隣接するab
{x} B - A^A = Bを満たすAがあるかどうか
{o} C - 与えられた9x9マスが数独を満たすかどうか
{x} D - 0,1からなる数列Xが存在するかどうか (二部グラフ問題)
```

# B

## 問題概要

整数Bが与えられる。
この時、 $A^A = B$ を満たすAが存在するかどうか
[制約]
* 1 <= B <= 10^18
* Bは整数

## 解説

与えられたBの範囲 `1_000_000_000_000_000_000` から、
Aの範囲を限ることができる。

参照

* i64: `-9_223_372_036_854_775_808` ~ `9_223_372_036_854_775_808`

* 15^15=`437_893_890_380_859_375`

* 16^16 > i64,u64の範囲を超える

```rust
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		mut b: i64,
	}

	for a in 1..=15 {
		let mut x = 1;
		for _ in 0..a {
			x *= a;
		}
		if x == b {
			println!("{a}");
			return;
		}
	}
	println!("-1");
}
```

無限ループを使って解く方法もある。

```rust
for a in 1.. {
    if a.pow(a as u32) == b {
        p!(a);
        return;
    }
    if a.pow(a as u32) > b {
        p!(-1);
        return;
    }
}
```

## 敗因

素因数分解からのアプローチを行ってしまった。

```素因数分解コード.rs
let mut nums = Vec::new();
let mut p = 2;
while p * p <= b {
    if b % p != 0 {
        p += 1;
        continue;
    }

    let mut e = 0;
    while b % p == 0 {
        e += 1;
        b /= p;
    }
    nums.push((p, e));
}
// 最後に残ったのを加える
if b != 1 {
    nums.push((b, 1));
}
```

# C

## 問題概要

与えられた9x9マスが数独条件を満たすかどうか。

## 解説

以下回答したACコード。
3x3のブロックを取り出すコードが良かったと思う。

```rust
use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		a: [[marker::Usize1; 9]; 9],
	}
	let mut ng = false;

	let mut cols = vec![vec![false; 9]; 9];
	let mut rows = vec![vec![false; 9]; 9];
	let mut blks = vec![vec![false; 9]; 9];
	'i:for i in 0..9 {
		let col = &mut cols[i];
		let row = &mut rows[i];
		for j in 0..9 {
			if !col[a[i][j]] {
				col[a[i][j]] = true;
			} else {
				ng = true;
				break 'i;
			}

			if !row[a[j][i]] {
				row[a[j][i]] = true;
			} else {
				ng = true;
				break 'i;
			}

			let k = if i < 3 {
				if j < 3 {
					0
				} else if j < 6 {
					1
				} else {
					2
				}
			} else if i < 6 {
				if j < 3 {
					3
				} else if j < 6 {
					4
				} else {
					5
				}
			} else {
				if j < 3 {
					6
				} else if j < 6 {
					7
				} else {
					8
				}
			};

			let blk = &mut blks[k];
			if !blk[a[i][j]] {
				blk[a[i][j]] = true;
			} else {
				ng = true;
				break 'i;
			}
		}
	}
	if !ng {
		println!("Yes");
	} else {
		println!("No");
	}
}
```

# D - Good Tuple Problem

## 問題概要

N以下の正整数から長さMの数列A,Bを与える。
このA,Bが良いパートナーかどうか。
[良いパートナーとは]
0, 1からなる長さNの数列X = (X1, X2,..,X3)において、 $X_A[i] ≠ X_B[i]$ (i=1,2,3,..M)となるXが存在する時、数列AとBは良いパートナーという。

## 解説

まさかの二部グラフ問題だった。。。

問題文の条件を満たすような数列Xが存在するとは、

辺で結ばれた頂点同士に異なる数字が書き込まれる

