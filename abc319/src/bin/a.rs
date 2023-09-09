use proconio::*;
use std::collections::HashMap;

fn main() {
	let map = HashMap::from([
		("tourist", 3858),
		("ksun48", 3679),
		("Benq", 3658),
		("Um_nik", 3648),
		("apiad", 3638),
		("Stonefeang", 3630),
		("ecnerwala", 3613),
		("mnbvmar", 3555),
		("newbiedmy", 3516),
		("semiexp", 3481),
	]);
	input! {
		s: marker::Chars,
	}

	let s = s.iter().collect::<String>();
	let a = map[s.as_str()];
	println!("{}", a);
}
