use std::cmp;
use std::ops;

#[derive(Debug, PartialEq, Eq)]
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

impl cmp::PartialEq<Bound> for Bound{
	fn eq(&self, other: &Self) -> bool{
		match other{
			Bound::NegativeInfinity => {self == &Bound::NegativeInfinity}
			Bound::PositiveInfinity => {self == &Bound::PositiveInfinity}
			Bound::FiniteBound{value:other_value, direction:other_direction} =>{
				if let Bound::FiniteBound{value: self_value,direction:self_direction} = self {
					// TODO: This comparison is fraught with floating point problems:
					other_value == self_value && other_direction == self_direction
				}else{
					false
				}
			}
		}
	}
}