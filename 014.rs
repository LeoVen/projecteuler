/// The following iterative sequence is defined for the set of positive integers:
///     n → n/2 (n is even)
///     n → 3n + 1 (n is odd)
/// Using the rule above and starting with 13, we generate the following sequence:
///     13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
/// Which starting number, under one million, produces the longest chain?
/// NOTE: Once the chain starts the terms are allowed to go above one million.

struct Collatz(usize, bool);

impl Collatz {
    fn new(n: usize) -> Self {
        Self(n, false)
    }
}

impl Iterator for Collatz {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 {
            return None;
        }

        let out = self.0;

        self.0 = match self.0 {
            n if n <= 1 => {
                self.1 = true;
                self.0
            },
            n if n % 2 == 0 => self.0 / 2,
            n => self.0 * 3 + 1,
        };

        Some(out)
    }
}

fn main() {
    let mut max = 0;
    let mut nth = 0;

    for i in 0..1000000 {
        let count = Collatz::new(i).count();
        if count > max {
            nth = i;
            max = count;
        }
    }

    println!("{}", nth);
}
