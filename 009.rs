/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
///     a^2 + b^2 = c^2
/// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.

fn main() {
    const SUM: usize = 1000;

    for a in 1..SUM {
        for b in 1..SUM {
            let c = SUM - a - b;
            if a < b && b < c && (a * a + b * b == c * c) {
                // Should be printed only once
                println!("{}", a * b * c);
            }
        }
    }
}
