extern crate yam_interview_util;

use std::env;
use yam_interview_util::numbers::catalan;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        return;
    }
    let n: u64 = args[1].parse().ok().expect("Wanted a number");
    let k: u64 = args[2].parse().ok().expect("Wanted a number");
    println!("Number of Dyack Paths is {}", catalan(n));
}
