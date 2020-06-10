// cargo-deps: num-traits = "0.2.11", num-bigint = "0.2.6"

/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
/// What is the sum of the digits of the number 2^1000?

extern crate num_traits;
extern crate num_bigint;

use num_traits::*;
use num_bigint::*;

fn main() {
    println!("{}", BigUint::new(vec![2])
                    .pow(BigUint::new(vec![1000]))
                    .to_string()
                    .chars()
                    .map(|c| c.to_digit(10))
                    .fold(0, |acc, x| acc + x.unwrap()));
}
