
// TODO: Remove this
#![allow(dead_code)]

use super::*;

use std::collections::BTreeMap;


#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct PrimeFactorization {
    facs: BTreeMap<u64, u64>,
}

impl PrimeFactorization {
    fn new() -> Self {
        PrimeFactorization { facs: BTreeMap::new() }
    }
    fn add(&mut self, prime: u64, power: u64) {
        assert!(is_u64_prime(prime), "PrimeFactorization tried to insert composite {}", prime);
        *self.facs.entry(prime).or_insert(0) += power;
    }
    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (u64, u64)>
    {
        self.facs.iter().map(|(x,y)| (*x, *y))
    }
}

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
    fn add(&mut self, _n: u64) {
        unimplemented!();
    }

}


pub fn factor(_n: u64) -> PrimeFactorization
{
    unimplemented!();
}

