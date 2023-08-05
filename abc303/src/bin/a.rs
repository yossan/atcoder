use proconio::*;

fn main() {
    input! {
        n: usize,
        mut ss: marker::Chars,
        mut tt: marker::Chars,
    }
    replace(&mut ss);
    replace(&mut tt);
    for i in 0..n {
        if ss[i] != tt[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn replace(xx: &mut Vec<char>) {
    for x in xx {
        if x == &'1' {
            *x = 'l';
        } else if x == &'0' {
            *x = 'o';
        }
    }
}
