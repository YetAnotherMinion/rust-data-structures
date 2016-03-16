use core::ops::{Add, Sub, Mul, Div};
use numeric::{Zero, One};

pub trait Field : Sized + Zero + One + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + PartialOrd + Eq{}

impl Field for u8 {}
impl Field for u16 {}
impl Field for u32 {}
impl Field for u64 {}
impl Field for usize {}
impl Field for i8 {}
impl Field for i16 {}
impl Field for i32 {}
impl Field for i64 {}
impl Field for isize {}
