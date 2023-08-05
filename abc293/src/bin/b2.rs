fn main() {
    proconio::input! {
        n: usize,
        aa: [usize; n],
    }
    let mut ans = vec![false; n];
    let mut cnt = 0;
    for (i, a) in aa.iter().enumerate() {
        if ans[i] == false {
            if ans[a-1] == false {
                ans[a-1] = true;
                cnt += 1;
            }
        }
    }
    println!("{}", n-cnt);
    for i in 0..n {
        if ans[i] == false {
            print!("{} ", i+1);
        }
    }
    println!("");
}
