extern crate core;

pub mod numbers;
pub mod numeric;

use self::numbers::{binomial};

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        //let x = binomial(3, 4);
        println!("hello from lib {}", 37);
    }

    #[test]
    #[should_panic]
    fn bad_binomial() {
        let x = super::numbers::binomial(3, 4);
    }

    #[test]
    fn small_binomial_u64() {
        let x = super::numbers::binomial(3u64, 2u64);
        assert_eq!(3u64, x);

        let y = super::numbers::binomial(3u64, 3u64);
        assert_eq!(1, y);
        
        let z = super::numbers::binomial(3u64, 1u64);
        assert_eq!(3, z);

        let k = super::numbers::binomial(20u64, 10u64);
        assert_eq!(184756, k);
    }
}
