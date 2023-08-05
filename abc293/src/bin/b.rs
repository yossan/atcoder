fn main() {
    proconio::input! {
        n: usize,
        aa: [usize; n],
    }
    let mut ans = 0u64;
    for (i, a) in aa.iter().enumerate() {
        if (ans >> i) % 2 == 0 {
            ans |= 1 << a-1;
        }
    }
    let mut cnt = 0;
    for num in 0..n {
        if (ans >> num) % 2 == 0 {
            cnt += 1;
        }
    }
    println!("{}", cnt);
    for num in 0..n {
        if (ans >> num) % 2 == 0 {
            print!("{} ", num+1);
        }
    }
    println!("");
}

