//! red_primality provides zero-setup primality testing and factoring for all u64
//!
//! # Example
//!
//! ```
//! use red_primality::{ is_u64_prime, factor, Prime };
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
//! }
//! ```

#![deny(missing_docs)]

mod prime;
pub use prime::*;

mod iter;
pub use iter::*;

mod factor;
pub use factor::*;
