use proconio::input;
use itertools::Itertools;
use std::f64::consts::PI;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn length(&self, other: &Point) -> f64 {
        f64::sqrt((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0))
    }
}

fn main() {
    input! {
        n: usize,
        points: [(f64, f64); n],
    }
    println!("start");

    let angle_c = |a:f64, b:f64, c:f64| {
        let sita = (a.powf(2.0) + b.powf(2.0) - c.powf(2.0)) / (2.0 * a * b);
        sita.acos() * 180.0 / PI
    };
    let mut max_angle = 0.0f64;
    for tri in points.iter().combinations(3) {
        let mut w = tri.windows(3);
        while let Some(&[p1, p2, p3]) = w.next() {
            let p1 = Point { x: p1.0, y: p1.1 };
            let p2 = Point { x: p2.0, y: p2.1 };
            let p3 = Point { x: p3.0, y: p3.1 };
            let a = p1.length(&p2);
            let b = p2.length(&p3);
            let c = p3.length(&p1);
            let angle = 
                if a > b && a > c {
                    angle_c(b, c, a)
                } else if b > a && b > c {
                    angle_c(a, c, b)
                } else {
                    angle_c(a, b, c)
                };
            max_angle = angle.max(max_angle);
        }
    }
    println!("{}", max_angle);
}
