use proconio::*;

#[allow(non_snake_case)]
fn main() {
	input! {
		n: usize,
		d: [usize; n], // i月目はdi日
	}

	let mut cnt = 0;

	let div = |num: usize, comps: &mut Vec<usize>| {
		let mut j = num;
		while j != 0 {
			comps.push(j % 10);
			j /= 10;
		}
	};

	for i in 1..=n {
		let mut comps = Vec::new();
		div(i, &mut comps);
		for d in 1..=d[i-1] {
			let mut comps2 = comps.clone();
			div(d, &mut comps2);
			let f = comps2[0];
			let  mut zorome = true;
			for &d in &comps2[1..] {
				if d != f {
					zorome = false;
					break;
				}
			}
			if zorome {
				cnt += 1;
			}
		}
	}
	println!("{cnt}");
}
