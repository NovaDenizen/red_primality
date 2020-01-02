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

mod prime;

pub use prime::is_u64_prime;
pub use prime::MAX_U64_PRIME;

mod iter;
pub use iter::PrimeIter;

