/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
/// that the 6th prime is 13. What is the 10 001st prime number?

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
    println!("{}", Prime::new().skip(10000).next().unwrap());
}
