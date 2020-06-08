/// The prime factors of 13195 are 5, 7, 13 and 29.
/// What is the largest prime factor of the number 600851475143 ?

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
    let mut num: usize = 600851475143;
    let mut fac: usize = 0;

    for i in Prime::new() {
        if num % i == 0 {
            num /= i;
            fac = i;
        }

        if num == 1 {
            break;
        }
    }

    println!("{}", fac);
}
