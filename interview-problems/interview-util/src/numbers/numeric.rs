
pub trait One<T> {
    fn one() -> T;
}

pub trait Zero<T> {
    fn zero() -> T;
}

impl Zero<u8> for u8 {
    fn zero() -> u8 {
        0u8
    }
}

impl Zero<u16> for u16 {
    fn zero() -> u16 {
        0u16
    }
}

impl Zero<u32> for u32 {
    fn zero() -> u32 {
        0u32
    }
}

impl Zero<u64> for u64 {
    fn zero() -> u64 {
        0u64
    }
}

impl Zero<usize> for usize {
    fn zero() -> usize {
        0usize
    }
}

impl Zero<i8> for i8 {
    fn zero() -> i8 {
        0i8
    }
}

impl Zero<i16> for i16 {
    fn zero() -> i16 {
        0i16
    }
}

impl Zero<i32> for i32 {
    fn zero() -> i32 {
        0i32
    }
}

impl Zero<i64> for i64 {
    fn zero() -> i64 {
        0i64
    }
}

impl Zero<isize> for isize {
    fn zero() -> isize {
        0isize
    }
}

impl Zero<f32> for f32 {
    fn zero() -> f32 {
        0f32
    }
}

impl Zero<f64> for f64 {
    fn zero() -> f64 {
        0f64
    }
}

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





//trait Field<T> : Zero<>+One+Add+Sub+Mul+Div {};
