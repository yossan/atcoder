use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut cc = Vec::new();
    for _ in 0..m {
        input! { 
            _n: usize,
            c: [usize; _n],
        };
        cc.push(c);
    }

    let mut cnt = 0; 
    // m bitのbit全探索
    for i in 0..1<<m {
        let mut set = vec![false; n];
        for j in 0..m {
            if (i >> j) % 2 == 1 {
                let c = &cc[j];
                for &ci in c {
                    set[ci-1] = true;
                }
            }
        }
        cnt += if set.iter().fold(true, |acc, &b|{ acc && b }) { 1 } else { 0 };
    }
    println!("{}", cnt);
}
