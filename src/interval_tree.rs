use crate::interval::{Interval};
use crate::bound::{Bound, BoundDirection};
use Bound::{FiniteBound, PositiveInfinity, NegativeInfinity};
use BoundDirection::{PartOfLeft, PartOfRight};
use std::cmp;

pub enum Interval_Tree{
	Leaf{
		value:Interval,
		max_upper_bound:Bound,
		left:Box<Interval_Tree>,
		right:Box<Interval_Tree>
	},
	Empty
}

impl Interval_Tree{
	pub fn new_empty() -> Self{
		Interval_Tree::Empty
	}
	pub fn new_leaf(value:Interval, max_upper:Bound)->Self{
		// Create a copy of the upper_bound.
		let b = value.upper_bound;
		Interval_Tree::Leaf{value:value, max_upper_bound:b, left:Box::new(Interval_Tree::Empty), right:Box::new(Interval_Tree::Empty)}
	}
	pub fn insert(&mut self, new_value:Interval, max_upper:Bound){
		match self{
			Interval_Tree::Empty=>{
				*self = Interval_Tree::new_leaf(new_value, max_upper)
			},
			Interval_Tree::Leaf{value, max_upper_bound, left, right}=>{
				max_upper_bound = std::cmp::max(&mut max_upper, max_upper_bound);
				if new_value.lower_bound < value.lower_bound {
					left.insert(new_value, *max_upper_bound);
				} 
			}
		}
	}
}