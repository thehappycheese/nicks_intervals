//Bound::{FiniteBound, PositiveInfinity, NegativeInfinity};
//use Bound::BoundDirection::{PartOfLeft, PartOfRight};

use super::bound;

use bound::{Bound, BoundDirection};
use Bound::{FiniteBound, PositiveInfinity, NegativeInfinity};
use BoundDirection::{PartOfLeft, PartOfRight};


#[derive(Debug)]
pub struct Interval{
	lower_bound:Bound,
	upper_bound:Bound
}

impl Interval{
	
	pub fn open(lower_bound: f64, upper_bound: f64) -> Option<Interval>{
		if lower_bound < upper_bound {
			Some(Interval{
				lower_bound:FiniteBound{value:lower_bound, direction: PartOfLeft},
				upper_bound:FiniteBound{value:upper_bound, direction: PartOfRight}
			})
		}else{
			None
		}
	}
	pub fn closed(lower_bound: f64, upper_bound: f64) -> Option<Interval>{
		if lower_bound <= upper_bound {
			Some(Interval{
				lower_bound:FiniteBound{value:lower_bound, direction:PartOfRight},
				upper_bound:FiniteBound{value:upper_bound, direction:PartOfLeft}
			})
		}else{
			None
		}
	}
	
	pub fn open_closed(lower_bound: f64, upper_bound: f64) -> Option<Interval>{
		if lower_bound < upper_bound {
			Some(Interval{
				lower_bound:FiniteBound{value:lower_bound, direction:PartOfLeft},
				upper_bound:FiniteBound{value:upper_bound, direction:PartOfLeft}
			})
		}else{
			None
		}
	}
	pub fn closed_open(lower_bound: f64, upper_bound: f64) -> Option<Interval>{
		if lower_bound < upper_bound {
			Some(Interval{
				lower_bound:FiniteBound{value:lower_bound, direction:PartOfRight},
				upper_bound:FiniteBound{value:upper_bound, direction:PartOfRight}
			})
		}else{
			None
		}
	}
	pub fn open_inf(lower_bound:f64) -> Interval{
		Interval{
			lower_bound:FiniteBound{value:lower_bound, direction:PartOfLeft},
			upper_bound:PositiveInfinity
		}
	}
	pub fn closed_inf(lower_bound:f64) -> Interval{
		Interval{
			lower_bound:FiniteBound{value:lower_bound, direction:PartOfRight},
			upper_bound:PositiveInfinity
		}
	}
	pub fn inf_open(upper_bound:f64) -> Interval{
		Interval{
			lower_bound:NegativeInfinity,
			upper_bound:FiniteBound{value:upper_bound, direction:PartOfRight}
		}
	}
	pub fn inf_closed(upper_bound:f64) -> Interval{
		Interval{
			lower_bound:NegativeInfinity,
			upper_bound:FiniteBound{value:upper_bound, direction:PartOfLeft}
		}
	}
}
