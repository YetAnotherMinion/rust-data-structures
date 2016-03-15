use core::ops::{Add, Sub, Mul, Div};
use numeric::{Zero, One};

pub trait Field<T> : Sized + Zero<T> + One<T> + Add + Sub + Mul + Div {}

impl<T> Field<T> for T where T: Sized + Zero<T> + One<T> + Add + Sub + Mul + Div {}

