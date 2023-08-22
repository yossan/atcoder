// Circular Playlist
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    }

    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
    }
    let mut pt = t % sum;
    let mut ans = (0, 0);
    for i in 0 ..n {
        if a[i] >= pt {
            ans = (i + 1, pt);
            break;
        }
        pt -= a[i];
    }
    println!("{} {}", ans.0, ans.1);
}

