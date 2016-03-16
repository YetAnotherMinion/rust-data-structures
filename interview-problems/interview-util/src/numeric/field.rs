use core::ops::{Add, Sub, Mul, Div};
use numeric::{Zero, One};

pub trait Field : Sized + Zero + One + Add + Sub + Mul + Div {}

//impl Field for T where T: Sized + Zero + One + Add + Sub + Mul + Div {}

