// H 個の長さ W の ".", "T" からなる文字列 S1,S2,…,SHが与えられます。
// 以下の操作が行える
// * 文字列Siにおいて、j番目とj+1番目がTの時、S[j] = T, S[j+1]=Cに置き換える
fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        ss: [proconio::marker::Chars; h],
    }
    for mut s in ss {
        let mut j = 0;
        while j < w-1 {
            if s[j+1] == 'T' && s[j] == 'T' {
                s[j] = 'P';
                s[j+1] = 'C';
                j += 2;
            } else {
                j += 1;
            }
        }
        println!("{}", s.iter().collect::<String>());
    }
}
