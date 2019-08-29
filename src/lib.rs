
#![deny(missing_docs)]

//! red_primality provides a quick accurate primality test for all u64.

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
/// https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Testing_against_small_sets_of_bases
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

}
