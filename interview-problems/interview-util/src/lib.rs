extern crate core;

pub mod numbers;

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
}
