fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }
    let mut re = vec![0usize; n+1];
    for x in a {
        re[x] = 1;
    }
    let mut ans = Vec::new();
    let (mut i, mut j) = (1, 1);
    while i <= n {
        while re[j] != 0 { j+= 1 };
        let mut k = j;
        while k >= i {
            ans.push(k);
            k -= 1;
        }
        j += 1;
        i = j;
    }
    for i in 0..n {
        print!("{} ", ans[i]);
    }
    println!("");
}

