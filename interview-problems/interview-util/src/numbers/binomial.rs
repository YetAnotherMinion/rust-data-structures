use numeric::{One, Field};
use core::ops::{Add, Mul};

pub fn binomial<T>(n: T, k: T) -> T 
    where T: Field + Copy
{
    if k > n {
        panic!("k must be <= n");
    }
    if k < T::one() || n < T::one() {
        panic!("cannot have zero or negative integer inputs");
    }

    if k == T::one() {
        return n;
    } else if k == n {
        return T::one();
    }

    let r: T = if k > (n - k) {
        k
    } else {
        n - k
    };
    
    let numerator = range_product(r + T::one(), n);
    let denominator = range_product(T::one(), n-r);
    numerator / denominator
}

// start must be less than end

pub fn range_product<T>(start: T, end: T) -> T
    where T: Add<Output = T> + Mul<Output = T> + One + PartialOrd + Copy + Eq
{
    if start == end {
        return start;
    }
    if start > end {
        panic!("start of range must be less than end of range");
    }
    let mut cursor = start;
    let mut result = T::one();
    while !(end < cursor) {
        result = result * cursor;
        cursor = cursor + T::one();
    }
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn invalid_range() {
        let x = 5;
        let y = super::range_product(x, x);
        assert_eq!(x ,y);
    }

    #[test]
    fn small_range() {
        assert_eq!(360, super::range_product(3, 6));
    }
}

