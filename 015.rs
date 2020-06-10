// cargo-deps: num-traits = "0.2.11", num-bigint = "0.2.6"

/// Starting in the top left corner of a 2×2 grid, and only being able to move
/// to the right and down, there are exactly 6 routes to the bottom right corner.
/// How many such routes are there through a 20×20 grid?

// http://robertdickau.com/lattices.html
// http://robertdickau.com/manhattan.html

// The nth central binomial coefficient is given by: (2n)! / (n!)^2

extern crate num_traits;
extern crate num_bigint;

// https://github.com/rust-num/num-traits
use num_traits::*;
// https://github.com/rust-num/num-bigint
use num_bigint::*;

fn fac(n: usize) -> BigUint {
    (1..=n).map(BigUint::from).fold(BigUint::one(), std::ops::Mul::mul)
}

fn main() {
    const N: usize = 20;

    println!("{}", fac(2 * N) / fac(N).pow(BigUint::new(vec![2])));
}
