

//use float_cmp::assert_eq;

use std::cmp;
//use std::ops;

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
			Bound::NegativeInfinity => {*self == Bound::NegativeInfinity}
			Bound::PositiveInfinity => {*self == Bound::PositiveInfinity}
			Bound::FiniteBound{value:other_value, direction:other_direction} =>{
				if let Bound::FiniteBound{value: self_value,direction:self_direction} = self {
					approx_eq!(f64, *other_value, *self_value, epsilon=0.000_001) && *other_direction == *self_direction
				}else{
					false
				}
			}
		}
	}
}

impl cmp::PartialOrd<Bound> for Bound{
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>{
		match self{
			Bound::NegativeInfinity => {
				if let Bound::NegativeInfinity = other{
					Some(cmp::Ordering::Equal)
				}else{
					Some(cmp::Ordering::Less)
				}
			}
			Bound::PositiveInfinity =>{
				if let Bound::PositiveInfinity = other {
					Some(cmp::Ordering::Equal)
				}else{
					Some(cmp::Ordering::Greater)
				}
			}
			Bound::FiniteBound{value:self_value, direction:self_direction} =>{
				match other{
					Bound::NegativeInfinity => Some(cmp::Ordering::Greater),
					Bound::PositiveInfinity => Some(cmp::Ordering::Less),
					Bound::FiniteBound{value:other_value, direction:other_direction} => {
						// TODO: replace == comparison with isclose:
						if approx_eq!(f64, *self_value, *other_value, epsilon=0.000_001) {
						//if self_value == other_value {
							if BoundDirection::PartOfLeft == *self_direction && BoundDirection::PartOfRight == *other_direction{
								Some(cmp::Ordering::Greater)
							}else if BoundDirection::PartOfRight == *self_direction && BoundDirection::PartOfLeft == *other_direction{
								Some(cmp::Ordering::Less)
							}else{
								return Some(cmp::Ordering::Equal);
							}
						}else{
							Some(cmp::Ordering::Less)
						}
					}
				}
			}
		}
	}
}