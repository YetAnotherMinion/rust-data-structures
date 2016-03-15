use numeric::{Zero, One};
use core::ops::{Add, Sub, Mul, Div};


pub fn binomial<T>(n: T, k: T) -> T 
    where T: Add+Sub+Mul+Div+Zero<T>+One<T>
{
   T::one() 
}

//pub fn binomial(n: u64, k: u64) -> u64 {
//    assert!(n >= k);
//    1u64
//}

// start must be less than end

pub fn capital_pi<T>(start: T, end: T) -> T
    where T: Add<Output = T> + Mul<Output = T> + One<T> + PartialOrd + Copy
{
    if !(start < end) {
        panic!("start of range must be less thand end of range");
    }
    let mut cursor = start;
    let mut result = T::one();
    while !(cursor < end) {
        result = result * cursor;
        cursor = cursor + T::one();
    }
    result
}

#[test]
#[should_panic]
fn pi_invalid_range() {
    let x = 5;
    let y = capital_pi(x, x);
    assert_eq!(x ,y);
}
