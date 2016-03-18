#![feature(test)]
extern crate test;
extern crate core;
// the crate under test
extern crate yam_interview_util;

#[cfg(test)]
mod tests {
    use yam_interview_util::numbers::{catalan, range_product};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_small_catalan(b: &mut Bencher) {
        b.iter(|| {
            let n: u64 = black_box(20u64);
            catalan(n);
        });
    }

    #[bench]
    fn bench_big_int_catalan(b: &mut Bencher) {
        b.iter(|| {
            let n: u64 = black_box(59u64);
            catalan(n);
        });
    }
}

