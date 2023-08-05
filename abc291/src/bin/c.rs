use std::collections::BTreeMap;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

fn main() {
    proconio::input! {
        n: usize,
        s: proconio::marker::Chars,
    }
    let mut yes_or_no = false;
    let mut map: BTreeMap<Pos, usize> = BTreeMap::new();
    let mut pos = Pos { x: 0, y: 0 };
    map.insert(pos, 1);
    for i in 0..n {
        pos = match s[i] {
            'R' => Pos { x: pos.x+1, y: pos.y },
            'L' => Pos { x: pos.x-1, y: pos.y },
            'U' => Pos { x: pos.x,   y: pos.y+1 },
            'D' => Pos { x: pos.x,   y: pos.y-1 },
            _ => pos,
        };
        let val = map.get(&pos).unwrap_or(&0);
        if *val == 0 {
            *map.entry(pos).or_default() += 1;
        } else {
            yes_or_no = true;
            break;
        }
    }
    println!("{}", ["Yes", "No"][if yes_or_no {0} else {1}]);
}
