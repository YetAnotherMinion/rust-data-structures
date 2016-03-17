use numeric::{One, Field};
use numbers::binomial;


pub fn narayana<T>(n: T, k: T) -> T
    where T: Field + Copy + Clone
{
    let result: T = (binomial(n, k) * binomial(n, k - T::one())) / n;
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn spot_check_narayana() {
        assert_eq!(1, super::narayana(1, 1));
    }
}
