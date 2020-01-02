
// TODO: Remove this
#![allow(dead_code)]

use super::*;

use std::collections::BTreeMap;


#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
/// Represents a collection of powers of prime factors.
pub struct PrimeFactorization {
    facs: BTreeMap<Prime, u64>,
}

impl PrimeFactorization {
    /// Creates a new PrimeFactoriazation
    pub fn new() -> Self {
        PrimeFactorization { facs: BTreeMap::new() }
    }
    /// Add a power of a prime to this factorization.
    pub fn add(&mut self, prime: Prime, power: u64) {
        *self.facs.entry(prime).or_insert(0) += power;
    }

    /// Add all the factors in the other PrimeFactorization into this one.
    pub fn add_pf(&mut self, pf: &Self, fac: u64) {
        for (n, np) in pf.iter() {
            self.add(n, np*fac);
        }
    }
    /// Create an iterator over the contained factors and powers.
    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (Prime, u64)>
    {
        self.facs.iter().map(|(x,y)| (*x, *y))
    }
    /// Multiply out the contained factors and powers, yielding the product they represent.
    pub fn product(&self) -> u64 {
        let mut res = 1;
        for (p, pow) in self.iter() {
            for _ in 0..pow {
                res *= p.get();
            }
        }
        res
    }
}

/// An incomplete factorization of a number.
#[derive(Debug)]
struct IncFac {
    /// composite factors, still need work
    comps: BTreeMap<u64, u64>,
    /// prime factors
    primes: PrimeFactorization,
}

impl IncFac {
    fn new() -> Self {
        IncFac { comps: BTreeMap::new(), primes: PrimeFactorization::new() }
    }
    fn add(&mut self, n: u64, np: u64) {
        match Prime::new(n) {
            Some(p) => self.primes.add(p, np),
            None => *self.comps.entry(n).or_insert(0) += np,
        }
    }
    fn add_pf(&mut self, pf: &PrimeFactorization) {
        self.primes.add_pf(pf, 1);
    }
    fn done(&self) -> bool {
        self.comps.len() == 0
    }
    fn take(self) -> PrimeFactorization {
        assert!(self.done(), "Tried to use incomplete PrimeFactorization");
        self.primes
    }
    fn take_composite(&mut self) -> Option<(u64, u64)> {
        let res = self.comps.iter().next().map( |(n, np)| (*n, *np));
        res.map(|(n, _)| self.comps.remove(&n));
        res
    }
}

/// TODO: this will overflor for big trial primes.  This shouldn't happen, but fix it.
fn trial_div(mut n: u64, limit: u64) -> (u64, PrimeFactorization)
{
    let mut ci = CertIter::all();
    let mut res = PrimeFactorization::new();
    assert!(n > 0, "trial_div trying to factor 0");
    loop {
        if n == 1 {
            break;
        }
        let p = ci.next().unwrap();
        let pp = p.get();
        if pp > limit {
            break;
        }
        if pp * pp > n {
            res.add(Prime::new(n).unwrap(), 1);
            n = 1;
            break;
        }
        while n % pp == 0 {
            res.add(p, 1);
            n /= pp;
        }
    }
    (n, res)
}

/// Pollard's rho algorithm, using the polynomial x^2 + r and initial value 2, using u128
/// intermediate values.
fn rho_u128(fac: &mut IncFac, n64: u64, np: u64, r: u64)
{
    use num::Integer;
    let r = r as u128;
    let mut a = 2_u128;
    let mut b = 2_u128;
    let n = n64 as u128;
    loop {
        a = (a*a + r) % n;
        a = (a*a + r) % n;
        b = (b*b + r) % n;
        let g = n.gcd(&(a + n - b));
        if g == n {
            // failed.
            fac.add(n64, np);
            return;
        } else if g > 1 {
            assert!(n % g == 0, "rho_u128, a={}, b={}, n={}, g={}, n%g={}",
                    a, b, n, g, n%g);
            let f = g as u64;
            fac.add(f, np);
            fac.add(n64/f, np);
            return;
        }
    }
}

/// Pollard's rho algorithm, using the polynomial x^2 + r and initial value 2, using u128
/// intermediate values.
fn rho_u64(fac: &mut IncFac, n64: u64, np: u64, r: u64)
{
    use num::Integer;
    let n = n64;
    let mut a = 2;
    let mut b = 2;
    loop {
        a = (a*a + r) % n;
        a = (a*a + r) % n;
        b = (b*b + r) % n;
        let g = n.gcd(&(a + n - b));
        if g == n {
            // failed.
            fac.add(n64, np);
            return;
        } else if g > 1 {
            assert!(n % g == 0, "rho_u128, a={}, b={}, n={}, g={}, n%g={}",
                    a, b, n, g, n%g);
            let f = g as u64;
            fac.add(f, np);
            fac.add(n64/f, np);
            return;
        }
    }
}
fn rho_step(fac: &mut IncFac, r: u64) {
    let (n64, np) = fac.take_composite().unwrap();
    let n = n64 as u128;
    if n*n + (r as u128) < (std::u64::MAX as u128) {
        rho_u64(fac, n64, np, r);
    } else {
        rho_u128(fac, n64, np, r);
    }
}

fn factor_rho(n: u64) -> PrimeFactorization {
    let mut fac = IncFac::new();
    fac.add(n, 1);
    let mut r = 1;
    while !fac.done() {
        if r > 1 {
            println!("r={}, fac={:?}", r, fac);
        }
        rho_step(&mut fac, r);
        r += 1;
    }
    fac.take()
}

/// Determines the prime factors of a given u64.
///
/// This function uses a few iterations of trial division, then switches to Pollard's rho
/// algorithm.  The algorithm is not deterministic, but On my laptop it averages less than 100ms
/// for products of two factors slightly smaller than 2^32, which is the expected worst case
/// scenario.
///
/// # Panics
///
/// This function will panic if it attempts to factor 0.
pub fn factor(n: u64) -> PrimeFactorization
{
    let limit = 100;
    let (n_left, pf) = trial_div(n, limit);
    if n_left == 1 {
        pf
    } else {
        let mut pf2 = factor_rho(n_left);
        pf2.add_pf(&pf, 1);
        pf2
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_factor(n: u64, noisy: bool) -> PrimeFactorization {
        let pf = factor(n);
        if noisy {
            println!("factor({}): {:?}", n, pf);
        }
        assert_eq!(pf.product(), n, "test_ffactor({}) didn't work", n);
        pf
    }

    #[test]
    fn factor_smalls() {
        let limit = 100_000;
        for i in 1..limit {
            if i % 1000 == 0 {
                println!("{}", i);
            }
            test_factor(i, false);
        }
    }

    #[test]
    #[should_panic]
    fn test_factor_0() {
        test_factor(0, false);
    }

    #[test]
    fn factor_bigs() {
        let radius = 100;
        for n in std::u64::MAX - radius..=std::u64::MAX {
            test_factor(n, false);
        }
    }

    /// returns a bunch of big primes just uner 2^32.
    fn medium_primes(count: usize) -> impl Iterator<Item=Prime>
    {
        CertIter::from(0xff00_0000).take(count)
    }
    #[test]
    fn factor_semiprimes() {
        let primes: Vec<Prime> = medium_primes(15).collect();
        for i in 0..primes.len() - 1 {
            for j in i+1..primes.len() {
                let p1 = primes[i];
                let p2 = primes[j];
                let mut pfguess = PrimeFactorization::new();
                pfguess.add(p1, 1);
                pfguess.add(p2, 1);
                let pf = test_factor(p1.get() * p2.get(), true);
                assert_eq!(pfguess, pf, "factor_semiprimes, p1={}, p2={}", p1, p2);
            }
        }
    }

}
