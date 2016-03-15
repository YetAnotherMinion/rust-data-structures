pub trait One<T> {
    fn one() -> T;
}

// Implementations of One for builtin numeric types
impl One<u8> for u8 {
    fn one() -> u8 {
        1u8
    }
}

impl One<u16> for u16 {
    fn one() -> u16 {
        1u16
    }
}

impl One<u32> for u32 {
    fn one() -> u32 {
        1u32
    }
}

impl One<u64> for u64 {
    fn one() -> u64 {
        1u64
    }
}

impl One<usize> for usize {
    fn one() -> usize {
        1usize
    }
}

impl One<i8> for i8 {
    fn one() -> i8 {
        1i8
    }
}

impl One<i16> for i16 {
    fn one() -> i16 {
        1i16
    }
}

impl One<i32> for i32 {
    fn one() -> i32 {
        1i32
    }
}

impl One<i64> for i64 {
    fn one() -> i64 {
        1i64
    }
}

impl One<isize> for isize {
    fn one() -> isize {
        1isize
    }
}

impl One<f32> for f32 {
    fn one() -> f32 {
        1f32
    }
}

impl One<f64> for f64 {
    fn one() -> f64 {
        1f64
    }
}

