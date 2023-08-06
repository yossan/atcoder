use proconio::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut visited = vec![false; n];
    visited[ab[0].0-1] = true;
    for &(a, b) in &ab {
        if visited[a-1] {
            visited[b-1] = true;
        } else {
            break;
        }
    }
    if visited.iter().all(|&e| e) {
        println!("{}", ab[0].0);
    } else {
        println!("-1");
    }
}

