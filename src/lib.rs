
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
pub fn is_u64_prime(n: u64) -> bool
{
        // if n < 9,080,191, it is enough to test a = 31 and 73;
        // if n < 25,326,001, it is enough to test a = 2, 3, and 5;
        // if n < 3,215,031,751, it is enough to test a = 2, 3, 5, and 7;
        // if n < 4,759,123,141, it is enough to test a = 2, 7, and 61;
        // if n < 1,122,004,669,633, it is enough to test a = 2, 13, 23, and 1662803;
        // if n < 2,152,302,898,747, it is enough to test a = 2, 3, 5, 7, and 11;
        // if n < 3,474,749,660,383, it is enough to test a = 2, 3, 5, 7, 11, and 13;
        // if n < 341,550,071,728,321, it is enough to test a = 2, 3, 5, 7, 11, 13, and 17.
        // Using the work of Feitsma and Galway enumerating all base 2 pseudoprimes in 2010, this was
        // extended (see OEIS: A014233), with the first result later shown using different methods in
        // Jiang and Deng:[10]
        //
        // if n < 3,825,123,056,546,413,051, it is enough to test a = 2, 3, 5, 7, 11, 13, 17, 19, and
        // 23.
        // if n < 18,446,744,073,709,551,616 = 264, it is enough to test a = 2, 3, 5, 7, 11, 13, 17,
        // 19, 23, 29, 31, and 37.
    if n == 2 || n == 3 {
        true
    } else if n & 1 == 0 {
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

fn sprp_u64(n: u64, p: u8) -> bool {
    unimplemented!();
}

fn sprp_u128(n: u128, p: u8) -> bool {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
