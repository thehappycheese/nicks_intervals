use std::cmp;
use std::ops;

#[derive(Debug)]
pub enum BoundDirection{
	PartOfLeft,
	PartOfRight
}

#[derive(Debug)]
pub enum Bound{
	FiniteBound{
		value:f64,
		direction:BoundDirection
	},
	NegativeInfinity,
	PositiveInfinity
}

impl ops:: for Bound{
	
}