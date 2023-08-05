use proconio::*;

fn main() {
    input! {
        s: [usize; 8],
    }
    let mut prev = 0;
    for i in 0..8 {
        let num = s[i];
        
        if num < prev {
            println!("No");
            return;
        }
        prev = num;
        if num < 100 || num > 675 {
            println!("No");
            return;
        }
        if num % 25 != 0 {
            println!("No");
            return;
        }

    }
    println!("Yes");
}
