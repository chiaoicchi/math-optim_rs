//! Modular arithmetic utilities
//!
//! There are functions for modular exponentiation and inverse.
//!
//! # Available functions
//!
//! - [`pow_mod`]: Calculate exponentiation modular by some value.

use std::ops::{Div, Mul, Rem};

/// Returns `a^n` modulo `m`.
///
/// ## Parameters
///
/// - `a`: Powered value
/// - `n`: The exponent
/// - `m`: Modulo
///
/// ## Complexity
///
/// `O(log n)`
#[inline]
pub fn pow_mod<T: Copy + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + From<u8>>(
    mut a: T,
    mut n: u64,
    m: T,
) -> T {
    let mut res = T::from(1u8);
    a = a % m;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        n >>= 1;
    }
    res
}
