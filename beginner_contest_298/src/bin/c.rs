/*
 * BTreeMap - キーによってソートされた状態で値が格納される
 * BTreeSet - ヒープ
 */
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    proconio::input! {
        n: usize, // n個の箱とカード
        q: usize, // クエリーの数
    }

    // 箱番号とカード番号リストを格納する
    let mut box_to_card: BTreeMap<usize, BTreeSet<(usize, usize)>> = BTreeMap::new();
    // カード番号と箱番号リストを格納する
    let mut card_to_box: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    for qi in 0..q {
        // 質問タイプを受け取る
        proconio::input!(q_type: usize);
        match q_type {
            1 => {
                proconio::input!(card_num: usize, box_num: usize);
                // 箱番号に対応したカードリストを取得して、カード番号をインサート
                box_to_card.entry(box_num).or_default().insert((card_num, qi));
                // カード番号に対応した箱リストを取得して、箱番号を格納する
                card_to_box.entry(card_num).or_default().insert(box_num);
            }
            2 => {
                proconio::input!(box_num: usize);
                // 箱番号に入っているカード番号を昇順で出力する
                let cards = &box_to_card[&box_num];
                for (card, _index) in cards {
                    print!("{} ", card);
                }
                println!("");
            }
            3 => {
                proconio::input!(card_num: usize);
                // カード番号が入っている箱番号を昇順で出力する
                let boxes = &card_to_box[&card_num];
                for b in boxes {
                    print!("{} ", b);
                }
                println!("");

            }
            _ => {
            }
        }
    }
}
