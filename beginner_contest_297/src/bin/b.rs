/*
 * 長さ 8 の文字列 S が与えられます。S には K, Q がちょうど 1 文字ずつ、R, B, N がちょうど 2 文字ずつ含まれます。 S が以下の条件を全て満たしているか判定してください。

S において左から x,y (x<y) 文字目が B であるとする。このとき x と y の偶奇が異なる。

K は 2 つの R の間にある。より形式的には、S において左から x,y (x<y) 文字目が R であり、 z 文字目が K であるとする。このとき、 x<z<y が成り立つ。
*/
fn main() {
    // K,Qがちょうど一文字ずつ
    // R,B,Nが二文字すつ
    // 条件1
    // 2つのBは、偶奇に配置される
    // 条件2
    // Kは2つのRの間にある - R*K*R
    proconio::input! {
        s: String,
    }
    // bの位置の和をポイントとする
    let mut b_point = 0;
    // k(-1)とr(x2)の位置をポイントにする
    let mut r_point = 1;

    for (i, c) in s.chars().enumerate() {
        match c {
            'B' => {
                b_point += i;
            }
            'R' => {
                r_point *= 2;
            }
            'K' => {
                r_point -= 1;
            }
            _ => {
            }
        }
    }
    if b_point % 2 != 0 && r_point == 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
