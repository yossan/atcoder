use proconio::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n], // a日間, 毎日b錠ずつ
    }
    ab.sort_by(|(a1, _), (a2, _)| {
        a1.cmp(a2)
    });
    let mut sum = 0usize;
    for i in 0..n {
        sum += ab[i].1;
    }
    if sum <= k {
        println!("1");
        return;
    }

    for i in 0..n {
        if sum <= k {
            println!("{:?}", ab[i-1].0 + 1);
            return;
        }
        sum -= ab[i].1;
    }
    println!("{:?}", ab.last().unwrap().0 + 1);
}
