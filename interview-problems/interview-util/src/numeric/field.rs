use core::ops::{Add, Sub, Mul, Div};
use numeric::{Zero, One};

pub trait Field : Sized + Zero + One + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + PartialOrd + Eq{}

//impl Field for T where T: Sized + Zero + One + Add + Sub + Mul + Div {}

