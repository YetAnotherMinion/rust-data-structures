use numbers::{binomial, range_product};
use numeric::Field;

pub fn catalan<T>(n: T) -> T 
    where T: Field + Clone + Copy
{
    let two = T::one() + T::one();
    binomial(two * n, n) / ( n + T::one())
}

pub fn faster_catalan<T>(n: T) -> T
    where T: Field + Clone + Copy
{
    let two = T::one() + T::one();
    range_product(n + two, two * n) / range_product(T::one(), n)   
}

#[cfg(test)]
mod tests {
    #[test]
    fn spot_check_catalan() {
        assert_eq!(1, super::catalan(1));
        assert_eq!(2, super::catalan(2));
        assert_eq!(5, super::catalan(3));
        assert_eq!(14, super::catalan(4));
        assert_eq!(42, super::catalan(5));
    }

}

