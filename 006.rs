/// The sum of the squares of the first ten natural numbers is,
///     1^2 + 2^2 + ... + 10^2 = 385
/// The square of the sum of the first ten natural numbers is,
///     (1 + 2 + ... + 10)^2 = 552 = 3025
/// Hence the difference between the sum of the squares of the first ten natural
/// numbers and the square of the sum is 3025 − 385 = 2640.
/// Find the difference between the sum of the squares of the first one hundred
/// natural numbers and the square of the sum.

fn main() {
    let mut sqr_sum = 0;
    let mut sum_sqr = 0;

    for i in 1..=100 {
        sqr_sum += i * i;
        sum_sqr += i;
    }

    sum_sqr *= sum_sqr;

    println!("{}", sum_sqr - sqr_sum);
}
