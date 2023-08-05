use std::collections::BTreeMap;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

fn main() {
    proconio::input! {
        n: usize,
        s: proconio::marker::Chars,
    }

    let mut map: BTreeMap<Pos, isize> = BTreeMap::new();
    let mut pos = Pos { x: 0, y: 0 };
    *map.entry(pos).or_default() += 1;

    let mut ans = false;
    for i in 0..n {
        pos = match s[i] {
            'R' => { pos.x += 1; pos },
            'L' => { pos.x -= 1; pos },
            'U' => { pos.y += 1; pos },
            'D' => { pos.y -= 1; pos },
            _ => pos,
        };

        if map.contains_key(&pos) {
            ans = true;
            break;
        } else {
            map.entry(pos).or_insert(1);
        }
    }
    println!("{}", ["Yes", "No"][if ans {0} else {1}]);
}

