// #![feature(register_tool)]
// #![register_tool(creusot)]
// #![feature(proc_macro_hygiene, stmt_expr_attributes)]

// extern crate creusot_contracts;

// use creusot_contracts::*;

fn main () {}

// #[requires(a.len() > 0usize)]
fn x (a : &[u32]) -> bool {
  a.len() > 0;
  a[0] == 0
}

fn y (a: (&mut [u32], bool)) -> bool {
  a.0[5] = 3; false
}
