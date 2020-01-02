
// TODO: Remove this
#![allow(dead_code)]

use super::*;

use std::collections::BTreeMap;


#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct PrimeFactorization {
    facs: BTreeMap<Prime, u64>,
}

impl PrimeFactorization {
    fn new() -> Self {
        PrimeFactorization { facs: BTreeMap::new() }
    }
    fn add(&mut self, prime: Prime, power: u64) {
        *self.facs.entry(prime).or_insert(0) += power;
    }
    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (Prime, u64)>
    {
        self.facs.iter().map(|(x,y)| (*x, *y))
    }
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

/*
/// An incomplete factorization of a number.
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
    fn add(&mut self, n: u64) {
        match Prime::new(n) {
            Some(p) => self.primes.add(p, 1),
            None => *self.comps.entry(n).or_insert(0) += 1,
        }
    }
    fn done(&self) -> bool {
        self.comps.len() == 0
    }
    fn take(self) -> PrimeFactorization {
        assert!(self.done(), "Tried to use incomplete PrimeFactorization");
        self.primes
    }
}
*/

/// TODO: this will overflor for big u64.  fix it.
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

pub fn factor(n: u64) -> PrimeFactorization
{
    let limit = 1_000_000;
    let (n_left, pf) = trial_div(n, limit);
    assert!(n_left == 1, "trial_div with limit={} didn't work for {}", limit, n);
    pf
}

fn test_factor(n: u64) {
    let pf = factor(n);
    assert_eq!(pf.product(), n, "test_ffactor({}) didn't work", n);
}

#[test]
fn factor_smalls() {
    let limit = 50_000;
    for i in 1..limit {
        if i % 1000 == 0 {
            println!("{}", i);
        }
        test_factor(i);
    }
}

