/*
編集距離が1以下かどうか
* T′は、T と等しい。
* T′は、T のいずれか 1 つの位置（先頭と末尾も含む）に英小文字を 1 つ挿入して得られる文字列である。
* T′は、T からある 1 文字を削除して得られる文字列である。
* T′は、T のある 1 文字を別の英小文字に変更して得られる文字列である。
*/

use proconio::*;

fn main() {
	input! {
		n: usize,
		mut td: marker::Chars, // 青木が受け取った英小文字からなる文字列
		s: [marker::Chars; n], // 高橋が送った英小文字列Tが格納された配列
	}
	let mut ans = Vec::new();
	for k in 0..n {
		let t = &s[k];
		if check(&td, t) {
			ans.push(k + 1);
		}
	}
	println!("{}", ans.len());
	for k in ans {
		print!("{k} ");
	}
	println!("");
}

// 編集距離が1以下かどうか
fn check(td: &[char], t: &[char]) -> bool {
    if td == t {
		// t'はtと等しい (条件1)
        return true;
    }

    let mut pre = td.len().min(t.len());

	// 先頭から比較していく
    for i in 0..td.len().min(t.len()) {
        if td[i] != t[i] {
            pre = i;
            break;
        }
    }

	// 一致しなかった部分文字列
    let td = &td[pre..];
    let t = &t[pre..];

	// t'はtのいずれか1つの位置に挿入して得られる文字列(条件2)
    if td.len() > 0 && &td[1..] == t {
        return true;
    }

	// t'はtから一文字削除して得られる文字列(条件3)
    if t.len() > 0 && &t[1..] == td {
        return true;
    }

	// t'はtのある一文字を変更して得られる文字列(条件4)
    if td.len() > 0 && t.len() > 0 && td[1..] == t[1..] {
        return true;
    }

    false
}
