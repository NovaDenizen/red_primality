//! red_primality provides zero-setup primality testing and factoring for all u64
//!
//! # Example
//!
//! ```
//! use red_primality::{ is_u64_prime, factor, Prime, euler_totient, mobius };
//!
//! fn main() {
//!     // Primality testing
//!     assert!(is_u64_prime(5));
//!     assert!(!is_u64_prime(6));
//!
//!     // Factoring a near-worst-case semiprime
//!     let facs: Vec<(Prime, u64)> = factor(18302912619494838287).iter().collect();
//!     let p1 = Prime::new(4278190337).unwrap();
//!     let p2 = Prime::new(4278190351).unwrap();
//!     assert_eq!(facs, vec![(p1, 1), (p2, 1)]);
//!
//!     // Euler's totient function
//!     assert_eq!(euler_totient(180), (1*2) * (2*3) * (4));
//!
//!     // mobius(x,y) is the Möbius function of the ratio x/y
//!     assert_eq!(mobius(90, 2), 0);  // 90/2 = 45, contiains 3^2
//!     assert_eq!(mobius(90, 3), -1);  // 90/3 = 30 = 2*3*5, so -1
//!     assert_eq!(mobius(90, 6), 1);  // 90/6 = 15 = 3*5, so +1
//! }
//! ```

#![deny(missing_docs)]

mod prime;
pub use prime::*;

mod iter;
pub use iter::*;

mod factor;
pub use factor::*;
