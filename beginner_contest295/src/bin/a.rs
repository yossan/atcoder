fn main() {
    proconio::input! {
        n: usize,
        ww: [String; n],
    }

    let a = ww.iter().any(|w| { ["and","not", "that", "the", "you"].contains(&w.as_str()) });
    if a {
        println!("Yes");
    } else {
        println!("No");
    }
    /*
    for w in ww {
        // and, not, that, the, you
        match &w as &str {
            "and" | "not" | "that" | "the" | "you"  => {
                println!("Yes");
                return;
            }
            _ => {
            }
        }
    }
    println!("No");
    */
}
