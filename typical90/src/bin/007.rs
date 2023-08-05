use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n], 
        q: usize,
    }

    a.sort();

    for _ in 0..q {
        input! { b: usize };
        let j = lower_bound(&a, &b);
        
        let mut res = std::usize::MAX;
        if j > 0 { res = b - a[j-1] };
        if j < n { res = res.min(a[j] - b) };
        println!("{}", res);
    }
}

fn lower_bound(a: &[usize], b: &usize) -> usize {
    match a.binary_search(b) {
        Ok(p) => p,
        Err(p) => p
    }
}
