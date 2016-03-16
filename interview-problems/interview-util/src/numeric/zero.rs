pub trait Zero {
    fn zero() -> Self;
}

impl Zero for u8 {
    fn zero() -> u8 {
        0u8
    }
}

impl Zero for u16 {
    fn zero() -> u16 {
        0u16
    }
}

impl Zero for u32 {
    fn zero() -> u32 {
        0u32
    }
}

impl Zero for u64 {
    fn zero() -> u64 {
        0u64
    }
}

impl Zero for usize {
    fn zero() -> usize {
        0usize
    }
}

impl Zero for i8 {
    fn zero() -> i8 {
        0i8
    }
}

impl Zero for i16 {
    fn zero() -> i16 {
        0i16
    }
}

impl Zero for i32 {
    fn zero() -> i32 {
        0i32
    }
}

impl Zero for i64 {
    fn zero() -> i64 {
        0i64
    }
}

impl Zero for isize {
    fn zero() -> isize {
        0isize
    }
}

impl Zero for f32 {
    fn zero() -> f32 {
        0f32
    }
}

impl Zero for f64 {
    fn zero() -> f64 {
        0f64
    }
}

