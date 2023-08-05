/*
 * 面接官N人
 * 文字列Sが与えられi文字目がi番目の面接官の評価
 * o 良
 * - 可
 * x 不可
 * 以下を満たせば合格
 * * oが1つでもある
 * * xが1つもない
 *
 * 鉄則: 巨大な数字は余りを出力する
 */
fn main() {
    proconio::input! {
        n: usize,
        s: proconio::marker::Chars,
    }
    let mut kekka = 1;
    for c in s {
        kekka *= 
            if c == 'x' {
                0
            } else if c == '-' {
                1
            } else {
                2
            };
        kekka %= 5;
    }
    if kekka >= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}

