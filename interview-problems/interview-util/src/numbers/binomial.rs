use super::numeric::{Zero, One};

use core::ops::{Add, Sub, Mul, Div};


pub fn binomial<T>(n: T, k: T) -> T 
    where T: Add+Sub+Mul+Div+Eq+Zero<T>+One<T>
{
     
}

//pub fn binomial(n: u64, k: u64) -> u64 {
//    assert!(n >= k);
//    1u64
//}

// start must be less than end
pub fn capital_pi<T>(start: T, end: T) -> T
    where T: Add+Sub+Mul+Div+Eq+Zero<T>+One<T>
{
    for x in start..end {
        let y: i32 = 1;
    }
    start
}

pub fn hello() -> String {
    "inside numbers".to_string()
}
