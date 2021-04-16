fn incr(a: &mut u64, b: &mut u64) {
	*a += *b;
}

fn main () {
	let x = &mut 4;
	while (*x < 10) {
		* x += 1;
	}
}
