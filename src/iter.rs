
use super::is_u64_prime;
use super::Prime;

/// PrimeIter returns a sequence of primes in ascending order.
///
/// # Panics
///
/// This iterator will panic if it tries to generate a prime larger than `std::u64::MAX`.
///
/// To avoid panicking, use `Iterator::take_while()` or some other mechanism for limiting
/// consumption.
#[derive(Clone)]
pub struct PrimeIter {
    last_output: u64, 
    next_jump: u64,
}

impl PrimeIter {
    /// Returns an iterator that generates all u64 primes in ascending order starting at the first
    /// on or after the parameter `n`.
    ///
    /// # Example
    ///
    /// ```
    /// use red_primality::PrimeIter;
    ///
    /// fn main() {
    ///     let small_primes: Vec<u64> = PrimeIter::from(5).take_while(|n| n < &20).collect();
    ///     assert_eq!(small_primes, vec![5, 7, 11, 13, 17, 19]);
    /// }
    /// ```
    ///
    pub fn from(n: u64) -> Self {
        let last_output = if is_u64_prime(n) {
            // safe since n >= 2
            n - 1
        } else {
            n
        };
        let next_jump = if last_output < PrimeIter::PRIME_JUMPS.len() as u64 {
            1
        } else {
            PrimeIter::PRIME_JUMPS[(n % (PrimeIter::PRIME_JUMPS.len() as u64)) as usize]
        } as u64;
        PrimeIter { last_output, next_jump }
    }

    /// Returns an iterator that generates all u64 primes in ascending order.
    ///
    pub fn all() -> Self {
        Self::from(2)
    }
    // cargo test -- --nocapture dump_jumps
    // average jump len = 3.6952380952380954
    const PRIME_JUMPS: [u8; 210] = [1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3,
        2, 1, 6, 5, 4, 3, 2, 1, 2, 1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1,
        6, 5, 4, 3, 2, 1, 2, 1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 6, 5,
        4, 3, 2, 1, 8, 7, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 8, 7, 6, 5,
        4, 3, 2, 1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1,
        2, 1, 6, 5, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1, 2, 1,
        6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2];

}

/// Produces a sequence of certified primes.
///
/// Cost and behavior should be identical to PrimeIter, since it is a zero-cost wrapper around
/// PrimeIter.
///
/// CertIter will panic if it tries to produce a value larger than MAX_u64_PRIME, the same as
/// PrimeIter.
///
pub struct CertIter {
    pi: PrimeIter,
}

impl CertIter {
    /// Returns an CertIter that produces all u64 primes.
    pub fn all() -> Self {
        Self::from_pi(PrimeIter::all())
    }
    /// Returns a CertIter that produces all u64 primes at or above `n`.
    pub fn from(n: u64) -> Self {
        Self::from_pi(PrimeIter::from(n))
    }
    /// Turns a PrimeIter into a CertIter.
    pub fn from_pi(pi: PrimeIter) -> Self {
        CertIter { pi }
    }
}

impl From<PrimeIter> for CertIter {
    fn from(pi: PrimeIter) -> Self {
        CertIter::from_pi(pi)
    }
}


impl Iterator for CertIter {
    type Item = Prime;
    fn next(&mut self) -> Option<Self::Item> {
        // this is safe because the PrimeIter only outputs primes.
        self.pi.next().map(|n| unsafe { Prime::new_unsafe(n) })
    }
}

#[test]
fn dump_jumps() {
    use num_integer::Integer;
    let len = 210; // 2*3*5*7
    let mut v = Vec::new();
    for i in 0_u64..len {
        for j in 1..30 {
            // we're looking for the first ofset that results in a number that doesn't obviously
            // share any factors with len.
            if (i + j).gcd(&len) == 1 {
                v.push(j);
                break;
            }
        }
    }
    { // collect stats
        let mut tot_jump = 0;
        for i in 0..(v.len()/2) {
            tot_jump += v[i*2 + 1];
        }
        println!("average jump len = {}", tot_jump as f64 / (len as f64) * 2.0);
    }
    println!("const PRIME_JUMPS: [u8; {}] = {:?};", len, v);
}

#[test]
fn dump_end() {
    for p in (std::u64::MAX - 1000)..=std::u64::MAX {
        if is_u64_prime(p) {
            println!("{} (2^64 - {}) is prime", p, std::u64::MAX - p + 1);
        }
    }
    // results appear to match https://primes.utm.edu/lists/2small/0bit.html
}


#[test]
#[should_panic]
fn run_past_end() {
    let start = std::u64::MAX - 1000;
    let ps = PrimeIter::from(start);
    let mut got_biggest = false;
    // expect ps to panic when it tries to move past end
    for p in ps {
        println!("got {}", p);
        if got_biggest {
            println!("Should not have gotten a prime after {}", super::MAX_U64_PRIME);
            return;  // this causes test failure because of [should_pannic]
        }
        if p == super::MAX_U64_PRIME {
            got_biggest = true;
        }
    }
}
#[test]
fn check_includes_biggest() {
    let start = std::u64::MAX - 1000;
    let ps = PrimeIter::from(start);
    for p in ps {
        if p == super::MAX_U64_PRIME {
            return;
        }
    }
    panic!("Never got biggest u64 prime {}", super::MAX_U64_PRIME);
}

impl Iterator for PrimeIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // overflow panic will occur here, after last valid prime returned
            self.last_output += self.next_jump;
            self.next_jump = 
                if self.last_output < PrimeIter::PRIME_JUMPS.len() as u64 {
                    1
                } else {
                    PrimeIter::PRIME_JUMPS[(self.last_output % (PrimeIter::PRIME_JUMPS.len() as u64)) as usize] as u64
                };
            if is_u64_prime(self.last_output) {
                return Some(self.last_output);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMIT: u64 = 1_000_000;
    #[test]
    fn compare_iter() {
        use primal::Primes;
        let mut ps1 = Primes::all().map(|n| n as u64).take_while(|n| n < &LIMIT);
        let mut ps2 = PrimeIter::all().take_while(|n| n < &LIMIT);
        loop {
            let v1 = ps1.next();
            let v2 = ps2.next();
            assert_eq!(v1, v2, "Iterators were inconsistent");
            if v1.is_none() {
                break;
            }
        }
    }
}
