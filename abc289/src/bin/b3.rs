// tanakh
// https://atcoder.jp/contests/abc289/submissions/38785871
use competitive::{prelude::*, union_find::UnionFind};
 
#[argio(output = AtCoder)]
fn main(n: usize, m: usize, a: [Usize1; m]) {
    let mut uf = UnionFind::new(n);
 
    for i in 0..m {
        uf.union(a[i], a[i] + 1);
    }
 
    let mut mm = BTreeMap::<usize, Vec<usize>>::new();
 
    for i in 0..n {
        mm.entry(uf.find(i)).or_default().push(i);
    }
 
    let mut first = true;
 
    for (_, e) in mm.into_iter().sorted_by_key(|e| e.1[0]) {
        for e in e.into_iter().rev() {
            if first {
                first = false;
            } else {
                print!(" ");
            }
            print!("{}", e + 1);
        }
    }
    println!()
}
