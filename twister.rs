#![feature(register_tool)]
#![register_tool(creusot)]
#![feature(proc_macro_hygiene, stmt_expr_attributes)]

extern crate creusot_contracts;

use creusot_contracts::*;

struct X<'a> {
	t : &'a mut u32
}

impl X<'_> {
  // #[ensures(^ self.t == *self.t + 1)]
	fn incr (&mut self) {
		*self.t += 1;
	}
}

fn main () {
	let mut a = 1;
	let mut x = X { t : &mut a };

	x.incr();
	x.incr();

	assert!(a == 2)
}
