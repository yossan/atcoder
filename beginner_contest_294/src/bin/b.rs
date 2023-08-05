fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        aa: [[usize; w]; h],
    }
    for a in aa {
        for s in a {
            if s == 0 {
                print!(".");
            } else {
                print!("{}", ('A' as u8 + s as u8 -1) as char);
            } 
        }
        println!("");
    }
}
