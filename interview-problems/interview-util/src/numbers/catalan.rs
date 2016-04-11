use numbers::{binomial, range_product};
use numeric::Field;

pub fn faster_catalan<T>(n: T) -> T 
    where T: Field + Clone + Copy
{
    let two = T::one() + T::one();
    binomial(two * n, n) / ( n + T::one())
}

pub fn catalan<T>(n: T) -> T
    where T: Field + Clone + Copy + Eq
{
    if n == T::one() {
        return T::one();
    }
    let two = T::one() + T::one();
    if n == two {
        return two;
    }
    range_product(n + two, two * n) / range_product(T::one(), n)   
}

