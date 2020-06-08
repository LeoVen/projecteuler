/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        // #372
        let (x, y) = (b, a % b);
        a = x;
        b = y;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn main() {
    let mut min = 1;

    for i in 1..20 {
        min = lcm(min, i)
    }

    println!("{}", min);
}
