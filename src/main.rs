#![allow(dead_code)]
#![allow(unused_variables)]
mod interval;
mod bound;

#[macro_use]
extern crate float_cmp;

use interval::{Interval};


fn main(){
	let i = Interval::closed_inf(5.0);
	println!("{:?}", i)
}