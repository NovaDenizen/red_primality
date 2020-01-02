
use super::*;

use std::collections::BTreeMap;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
struct PrimeFactorization {
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
}

/*
enum FacResult {
    Incomplete(Vec<(u64, u64)>),
}*/


pub fn factor(n: u64) -> PrimeFactorization
{
    unimplemented!();
}
