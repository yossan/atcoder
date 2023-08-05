#![allow(unused_imports, dead_code, non_snake_case)]

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m]
    }
    for (i, &val) in a.iter().enumerate() {
        let x = i;
        let y = match b.binary_search(&val) {
            Ok(num) => num,
            Err(num) => num,
        };
        print!("{} ", x + y + 1);
    }
    print!("\n");
    for (i, &val) in b.iter().enumerate() {
        let x = match a.binary_search(&val) {
            Ok(num) => num,
            Err(num) => num,
        };
        let y = i;
        print!("{} ", x + y + 1);
    }
    print!("\n");
}

