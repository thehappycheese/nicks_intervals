use crate::interval::{Interval};
use crate::bound::{Bound, BoundDirection};
use Bound::{FiniteBound, PositiveInfinity, NegativeInfinity};
use BoundDirection::{PartOfLeft, PartOfRight};
use std::cmp;

#[derive(Debug)]
enum Node_Color{
	Red,
	Black
}
use Node_Color::{Red, Black};

#[derive(Debug)]
pub enum Interval_Tree{
	Node{
		value:Interval,
		color:Node_Color,
		parent:Box<Interval_Tree>,
		left:Box<Interval_Tree>,
		right:Box<Interval_Tree>
	},
	Empty
}
use Interval_Tree::{Node, Empty};

// sorted by lower bound only for now.

impl Interval_Tree{
	pub fn new() -> Self{
		Empty
	}
	pub fn insert(&mut self, new_value:Interval){
		match *self{
			Empty=>{
				// this is the root node??
				*self = Node{
					value:new_value,
					color:Black,
					parent:Box::new(Empty),
					left:Box::new(Empty),
					right:Box::new(Empty)
				};
			},
			Node{value, color, parent, left, right}=>{
				if new_value.lower_bound < value.lower_bound {
					
					left.insert(new_value);
					if let Node{parent:left_parent, ..} = *left{
						left_parent = &self;
					}
				} 
			}
		}
	}
}