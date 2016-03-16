use numbers::binomial;
use numeric::Field;

pub fn catalan<T>(n: T) -> T 
    where T: Field + Clone + Copy
{
    let two = T::one() + T::one();
    binomial(two * n, n) / ( n + T::one())
}

