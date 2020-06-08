/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
/// Find the sum of all the primes below two million.

struct Prime(usize);

impl Prime {
    pub fn new() -> Self {
        Self(1)
    }

    pub fn is_prime(n: usize) -> bool {
        if n < 2 {
            return false;
        } else if n <= 3 {
            return true;
        }

        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        for i in (5..((n as f64).powf(0.5) + 1.0) as usize).step_by(6) {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
        }

        true
    }
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;

        for i in self.0.. {
            if Self::is_prime(i) {
                self.0 = i;
                return Some(i);
            }
        }

        unreachable!();
    }
}

fn main() {
    println!("{}", Prime::new().take_while(|x| x < &2000000).sum::<usize>());
}
