#![allow(dead_code)]
#![allow(unused_variables)]
mod interval_tree;
mod interval;
mod bound;


#[macro_use]
extern crate float_cmp;

use interval::{Interval};
use bound::{Bound, BoundDirection};
use BoundDirection::{PartOfRight, PartOfLeft};


fn main(){
	let i = Interval::closed_inf(5.0);
	let a = Bound::FiniteBound{value:10.0,direction:PartOfRight};
	let b = Bound::FiniteBound{value:10.0,direction:PartOfLeft};

	println!("{:?}", a>b)
}













#[test]
fn test_bound_order(){
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfLeft	}	<	Bound::FiniteBound{value:0.0, direction:PartOfRight	},	false);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfLeft	}	<	Bound::FiniteBound{value:0.0, direction:PartOfLeft	},	false);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfRight	}	<	Bound::FiniteBound{value:0.0, direction:PartOfRight	},	false);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfRight	}	<	Bound::FiniteBound{value:0.0, direction:PartOfLeft	},	true);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfLeft	}	==	Bound::FiniteBound{value:0.0, direction:PartOfRight	},	false);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfLeft	}	==	Bound::FiniteBound{value:0.0, direction:PartOfLeft	},	true);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfRight	}	==	Bound::FiniteBound{value:0.0, direction:PartOfRight	},	true);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfRight	}	==	Bound::FiniteBound{value:0.0, direction:PartOfLeft	},	false);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfLeft	}	>	Bound::FiniteBound{value:0.0, direction:PartOfRight	},	true);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfLeft	}	>	Bound::FiniteBound{value:0.0, direction:PartOfLeft	},	false);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfRight	}	>	Bound::FiniteBound{value:0.0, direction:PartOfRight	},	false);
	assert_eq!(Bound::FiniteBound{value:0.0, direction:PartOfRight	}	>	Bound::FiniteBound{value:0.0, direction:PartOfLeft	},	false);
}