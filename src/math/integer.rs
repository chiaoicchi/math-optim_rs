//! Integer utilities
//!
//! Functions for integer operations.
//!
//! # Available functions
//!
//! - [`isqrt`]: Calculate the integer square root of a number.

/// Returns the integer square root of `n`.
/// Uses Newton's method for integer arithmetic.
///
/// ## Definition
///
/// For a non-negative integer `n`, the integer square root is the largest integer `k`
/// such that `k^2 <= n`. Formally:
///
/// `isqrt(n) = floor(√n) = max{k : k^2 <= n}`
///
/// ## Complexity
///
/// `O(log n)`
pub fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut x = 1 << ((63 - n.leading_zeros()) / 2 + 1);
    loop {
        let y = (x + n / x) / 2;
        if y >= x {
            return x;
        }
        x = y
    }
}
