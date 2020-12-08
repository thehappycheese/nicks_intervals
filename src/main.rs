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
use interval_tree::{Interval_Tree};

fn main(){
	let mut tr = Interval_Tree::Empty;
	if let Some(i) = Interval::closed(5.0, 5.0){
		if let Some(q) = Interval::closed(4.0, 5.0){
			tr.insert(i);
			tr.insert(q);
		}
	}

	println!("{:#?}", tr)
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