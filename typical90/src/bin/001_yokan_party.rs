use proconio::*;

fn main() {
    input! {
        n: usize, // 切れ目の数
        l: isize, // L cm
        k: usize, // 切れ目の選択数
        a: [isize; n],
    }
    let check = |x: isize| -> bool {
        let mut num = 0; // 何個に切れたか
        let mut pre = 0; // 前回の切れ目
        for i in 0..n {
            // xを超えたら切断
            if a[i] - pre >= x {
                num += 1;
                pre = a[i];
            }
        }
        // 最後にピースがx以上なら加算
        if l - pre >= x { num += 1 };
        num >= k + 1
    };

    // 二分探索
    let (mut left, mut right) = (-1, l + 1);
    while right - left > 1 {
        let mid = (left + right) / 2;
        println!("left = {}, right = {}, mid = {}", left, right, mid);
        if check(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}
