use std::fmt::Debug;

use crate::apaxos::greater_equal::GreaterEqual;

pub trait Time: Default + Debug + Clone + Copy + PartialEq + GreaterEqual + 'static {}

impl<T> Time for T where T: Default + Debug + Clone + Copy + PartialEq + GreaterEqual + 'static {}
