#![feature(test)]

extern crate core;
extern crate test;

pub mod numbers;
pub mod numeric;

use self::numbers::{binomial};

#[cfg(test)]
mod tests {
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

    use test::{black_box, Bencher};
    #[bench]
    #[should_panic] // overflow
    fn bench_small_catalan(b: &mut Bencher) {
    
        b.iter(|| {
            let n: u64 = black_box(30u64);
            super::numbers::catalan(n)
        });
    }
    #[bench]
    #[should_panic] // overflow
    fn bench_large_catalan(b: &mut Bencher) {
    
        b.iter(|| {
            let n: u64 = black_box(59u64);
            super::numbers::catalan(n)
        });
    }
    #[bench]
    fn bench_small_narayana(b: &mut Bencher) {
        b.iter(|| {
            let n: u64 = black_box(19u64);
            let k: u64 = black_box(10u64);
            super::numbers::narayana(n, k);
        });
    }
    #[bench]
    fn bench_small_faster_narayana(b: &mut Bencher) {
        b.iter(|| {
            let n: u64 = black_box(19u64);
            let k: u64 = black_box(10u64);
            super::numbers::faster_narayana(n, k);
        });
    }

}
