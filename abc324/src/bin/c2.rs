// tanakh
// https://atcoder.jp/contests/abc324/submissions/46534442
#[argio(output = AtCoder)]
fn main(n: usize, t: Chars, s: [Chars; n]) {
    let mut ret = vec![];

    for i in 0..n {
        let s = &s[i];
        if check(&t, &s) {
            ret.push(i + 1);
        }
    }

    println!("{}", ret.len());
    println!(
        "{}",
        ret.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

// 編集距離が1かどうか
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
