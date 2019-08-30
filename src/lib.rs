

//! red_primality provides a quick accurate primality test for all u64.
//!
//! # Example
//!
//! ```
//! use red_primality::is_u64_prime;
//!
//! fn main() {
//!     assert!(is_u64_prime(5));
//!     assert!(!is_u64_prime(6));
//! }
//! ```

#![deny(missing_docs)]

/// Determines if the given parameter is prime.
///
/// The Miller-Rabin primality test can be said to have two results:  "Not Prime" or "Probably
/// Prime".  So in general, this test cannot deterministically guarantee primality.  It is
/// possible (though progressively less likely the more tests are applied) that an exceptionally
/// unlikely composite could be pronounced as prime by any sequence of tests.
///
/// However, it has been proven that certain small combinations of Miller-Rabin tests do not
/// have any exceptions under certainn lower bounds.  This function uses these vetted combinations
/// of tests to efficiently and determinstically determine primality for all integers inn the `u64`
/// range.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Testing_against_small_sets_of_bases) for more details. 
///
pub fn is_u64_prime(n: u64) -> bool
{
    if n == 2 || n == 3 {
        true
    } else if n & 1 == 0 || n < 5 {
        false
    } else if n < 2_047 {
        // if n < 2,047, it is enough to test a = 2;
        sprp_u64(n, 2)
    } else if n <  1_373_653 {
        // if n < 1,373,653, it is enough to test a = 2 and 3;
        sprp_u64(n, 2) && sprp_u64(n, 3)
    } else if n < 4_759_123_141 {
        // if n < 4,759,123,141, it is enough to test a = 2, 7, and 61;
        if n <= std::u32::MAX as u64 {
            sprp_u64(n, 2) && sprp_u64(n, 7) && sprp_u64(n, 61)
        } else {
            let n = n as u128;
            sprp_u128(n, 2) && sprp_u128(n, 7) && sprp_u128(n, 61)
        }
    } else {
        let n = n as u128;
        const P_LIST: [u8; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        for p in P_LIST.iter() {
            if !sprp_u128(n, *p) {
                return false;
            }
        }
        true
    } 
}


/// This is the largest prime integer that fits in a `u64`.
///
/// Equivalent to 2^64 - 59.
///
/// See [the prime pages](https://primes.utm.edu/lists/2small/0bit.html) for verification.
pub const MAX_U64_PRIME: u64 = 18_446_744_073_709_551_557;

fn sprp_u64(n: u64, a: u8) -> bool {
    let a = a as u64;
    let d = n - 1;
    let r = d.trailing_zeros();
    let d = d >> r;
    assert_eq!((1 << r) * d + 1, n);
    let mut x = pow_mod_u64(a, d, n);
    if x == 1 || x + 1 == n {
        return true;
    }
    for _ in 1..r {
        x = (x*x) % n;
        if x + 1 == n {
            return true;
        }
    }
    false
}

// assumes both x*x and m*m < std::u64::MAX
fn pow_mod_u64(mut x: u64, mut p: u64, m: u64) -> u64 {
    let mut res = 1;
    loop {
        // loop invariant: res * x^p congruent to original x^p
        if p & 1 == 1 {
            res = (res * x) % m;
            p -= 1;
        }
        if p > 0 {
            x = (x * x) % m;
            p /= 2;
        } else {
            break;
        }
    }
    res
}
// assumes both x*x and m*m < std::u128::MAX
fn pow_mod_u128(mut x: u128, mut p: u128, m: u128) -> u128 {
    let mut res = 1;
    loop {
        // loop invariant: res * x^p congruent to original x^p
        if p & 1 == 1 {
            res = (res * x) % m;
            p -= 1;
        }
        if p > 0 {
            x = (x * x) % m;
            p /= 2;
        } else {
            break;
        }
    }
    res
}

fn sprp_u128(n: u128, a: u8) -> bool {
    let a = a as u128;
    let d = n - 1;
    let r = d.trailing_zeros();
    let d = d >> r;
    assert_eq!((1 << r) * d + 1, n);
    let mut x = pow_mod_u128(a, d, n);
    if x == 1 || x + 1 == n {
        return true;
    }
    for _ in 1..r {
        x = (x*x) % n;
        if x + 1 == n {
            return true;
        }
    }
    false
}

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
    /// afer the parameter `n`.
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
            println!("Should not have gotten a prime after {}", MAX_U64_PRIME);
            return;  // this causes test failure because of [should_pannic]
        }
        if p == MAX_U64_PRIME {
            got_biggest = true;
        }
    }
}
#[test]
fn check_includes_biggest() {
    let start = std::u64::MAX - 1000;
    let ps = PrimeIter::from(start);
    for p in ps {
        if p == MAX_U64_PRIME {
            return;
        }
    }
    panic!("Never got biggest u64 prime {}", MAX_U64_PRIME);
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
    use primal::Sieve;

    fn test_prime_consistency(sieve: &Sieve, n: u64) {
        assert_eq!(is_u64_prime(n), sieve.is_prime(n as usize), "Primality test inconsistent for n={}", n);
    }
    const LIMIT: u64 = 1_000_000;
    #[test]
    fn small_numbers() {
        let sieve = Sieve::new(LIMIT as usize);
        for i in 0..LIMIT {
            test_prime_consistency(&sieve, i);
        }
    }
    fn excessive_sprp_test(n: u64) -> bool {
        assert!(n > LIMIT);
        let n = n as u128;
        for i in 0..100 {
            let k = 3 + i*2;
            if !sprp_u128(n, k as u8) {
                return false;
            }
        }
        true
    }
    fn test_prime_excessive(n: u64) {
        if n < LIMIT {
            return;  // donn't bother testinng small ones.
        }
        assert_eq!(excessive_sprp_test(n), is_u64_prime(n), "excessive test failed f= rn={}", n);
    }
    #[test]
    fn big_numbers() {
        use std::num::Wrapping;
        let inc = Wrapping(1_234_567_123_456_892);  // an arbitrarily chosen big even number
        let mut x = Wrapping(1);
        let count = 10_000;
        for _ in 0..count {
            x += inc;
            test_prime_excessive(x.0);
        }
    }

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
