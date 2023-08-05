use proconio::*;

struct Item {
    p: usize,
    c: usize,
    f: Vec<usize>
}

fn main() {
    input! {
        n: usize,
        _m: usize,
    }
    let mut items = Vec::new();
    for _ in 0..n {
        input! {
            p: usize, // 価格
            c: usize, // 機能数
            f: [usize; c]
        }
        let f = Vec::from(f);
        let item = Item {
            p,
            c,
            f,
        };
        items.push(item);
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let item_i = &items[i];
            let item_j = &items[j];
            if item_i.p >= item_j.p {
                if item_i.p > item_j.p || item_i.c < item_j.c {
                    let mut hit = true;
                    for fi in &item_i.f {
                        if !item_j.f.contains(fi) {
                            hit = false;
                            break;
                        }
                    }
                    if hit {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
