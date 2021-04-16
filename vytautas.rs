// SHOULD_SUCCEED: parse-print
#![feature(register_tool)]
#![register_tool(creusot)]
#![feature(proc_macro_hygiene, stmt_expr_attributes)]

extern crate creusot_contracts;

use creusot_contracts::*;

#[ensures(^ x == y)]
fn foo<'a, 'b>(x: &'b mut &'a mut u32, y: &'a mut u32) {
 * x = y;
}

fn main () {}
