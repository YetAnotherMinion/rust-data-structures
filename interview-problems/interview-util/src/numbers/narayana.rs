use numeric::{One, Field};
use numbers::{binomial, range_product};


pub fn faster_narayana<T>(n: T, k: T) -> T
    where T: Field + Copy + Clone + Eq
{
    if k > n {
        panic!("k cannot be greater than n");
    }
    if n == T::one() {
        return T::one();
    }
    if k == T::one() {
        return T::one();
    }
    if k == n {
        return T::one();
    }
    let result: T = (binomial(n, k) * binomial(n, k - T::one())) / n;
    result
}
pub fn narayana<T>(n: T, k: T) -> T
    where T: Field + Copy + Clone + Eq
{
    if k > n {
        panic!("k cannot be greater than n");
    }
    if n == T::one() {
        return T::one();
    }
    if k == T::one() {
        return T::one();
    }
    if k == n {
        return T::one();
    }

    let r = if (k - T::one()) > (n - k + T::one()) {
        k - T::one()
    } else {
        n - k + T::one()
    };
    
    let two = T::one() + T::one();
    let first = range_product(r + two, n) / range_product(T::one(), n-r + T::one());
    let second = range_product(r + T::one(), n) / range_product(T::one(), n-r);
    let result = first * second;
    result / n
}

#[cfg(test)]
mod tests {
    #[test]
    fn spot_check_narayana() {
        assert_eq!(1, super::narayana(1, 1));
        assert_eq!(1, super::narayana(2, 1));
        assert_eq!(1, super::narayana(3, 1));
        assert_eq!(3, super::narayana(3, 2));
        assert_eq!(1, super::narayana(3, 3));
        assert_eq!(1, super::narayana(4, 1));
        assert_eq!(6, super::narayana(4, 2));
        assert_eq!(6, super::narayana(4, 3));
    }
}
