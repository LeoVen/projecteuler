/// A palindromic number reads the same both ways. The largest palindrome made
/// from the product of two 2-digit numbers is 9009 = 91 × 99.
/// Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let mut max = 0;

    for i in 100..=999 {
        for j in 100..=999 {
            let prod = i * j;

            if is_palindrome(&prod.to_string()) {
                max = std::cmp::max(max, prod);
            }
        }
    }

    println!("{}", max);
}
