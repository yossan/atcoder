fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        events: [(usize, usize); q],
    }
    // 0, 1, 2
    let mut players = vec![0usize; n];
    for (q, x) in events {
        match q {
            1 => players[x-1] += 1,
            2 => players[x-1] += 2,
            3 => {
                if players[x-1] >= 2 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            },
            _ => (),
        }

    }
}
