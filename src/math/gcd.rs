//! GCD (Greatest Common Divisor) and LCM (Least Common Multiple)
//!
//! There are functions about GCD and LCM.
//!
//! # Available functions
//!
//! - [`gcd`]: Calculate GCD using Euclidean algorithm.
//! - [`lcm`]: Calculate LCM using Euclidean algorithm.
//! - [`binary_gcd`]: Calculate GCD using Stein's algorithm. (u64-only)
//! - [`binary_lcm`]: Calculate LCM using Stein's algorithm. (u64-only)

use std::ops::{Div, Mul, Rem};

/// Returns the greatest common divisor of `a` and `b`.
/// Uses the Euclidean algorithm.
///
/// ## Definition
///
/// For integers `a` and `b`, `gcd(a, b)` is the largest integer `g` such that:
/// - `g` divides `a` (i.e., there exists an integer `k` with `a = kg`)
/// - `g` divides `b` (i.e., there exists an integer `k` with `b = kg`)
///
/// Formally: `gcd(a, b) = max{d : d|a and d|b}`
///
/// ## Special Cases
///
/// - `gcd(a, 0) = a`
/// - `gcd(0, b) = b`
/// - `gcd(0, 0) = 0` (by convention)
///
/// ## Type Parameters
///
/// - `T`: `Mul, Div, Rem, Default` must be implemented like integer.
///
/// ## Complexity
///
/// `O(log (min(a, b)))`
pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + PartialEq + Default + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
    while b != T::default() {
        (a, b) = (b, a % b);
    }
    a
}

/// Returns the least common multiple of `a` and `b`.
/// Uses the equality of `gcd(a, b) * lcm(a, b) == a * b` and use `gcd` to calculate `gcd`.
///
/// ## Definition
///
/// For integers `a` and `b`, `lcm(a, b)` is the smallest positive integer `l` such that:
/// - `a` divides `l` (i.e., there exists an integer `k` with `l = ka`)
/// - `b` divides `l` (i.e., there exists an integer `k` with `l = kb`)
///
/// Formally: `lcm(a, b) = min{m > 0 : a|m and b|m}`
///
/// ## Special Cases
///
/// - `lcm(a, 0) = 0`
/// - `lcm(0, b) = 0`
/// - `lcm(0, 0) = 0` (by convention)
///
/// ## Type Parameters
///
/// - `T`: `Mul, Div, Rem, Default` must be implemented like integer.
///
/// ## Complexity
///
/// `O(log (min(a, b)))`
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + PartialEq + Default + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
    if a == T::default() || b == T::default() {
        return T::default();
    }
    a / gcd(a, b) * b
}

/// Returns the greatest common divisor of `a` and `b`.
/// Uses the Stein's algorithm which avoids division operations.
///
/// ## Definition
///
/// For integers `a` and `b`, `gcd(a, b)` is the largest integer `g` such that:
/// - `g` divides `a` (i.e., there exists an integer `k` with `a = kg`)
/// - `g` divides `b` (i.e., there exists an integer `k` with `b = kg`)
///
/// Formally: `gcd(a, b) = max{d : d|a and d|b}`
///
/// ## Special Cases
///
/// - `gcd(a, 0) = a`
/// - `gcd(0, b) = b`
/// - `gcd(0, 0) = 0` (by convention)
///
/// ## Complexity
///
/// `O(log (min(a, b)))`
pub fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 || b == 0 {
        return a + b;
    }
    let x = a.trailing_zeros();
    let y = b.trailing_zeros();
    a >>= x;
    b >>= y;
    while a != b {
        let x = (a ^ b).trailing_zeros();
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        a = (a - b) >> x;
    }
    a << x.min(y)
}

/// Returns the least common multiple of `a` and `b`.
/// Uses the equality of `gcd(a, b) * lcm(a, b) == a * b` and use `binary_gcd` to calculate `gcd`.
///
/// ## Definition
///
/// For integers `a` and `b`, `lcm(a, b)` is the smallest positive integer `l` such that:
/// - `a` divides `l` (i.e., there exists an integer `k` with `l = ka`)
/// - `b` divides `l` (i.e., there exists an integer `k` with `l = kb`)
///
/// Formally: `lcm(a, b) = min{m > 0 : a|m and b|m}`
///
/// ## Special Cases
///
/// - `lcm(a, 0) = 0`
/// - `lcm(0, b) = 0`
/// - `lcm(0, 0) = 0` (by convention)
///
/// ## Complexity
///
/// `O(log (min(a, b)))`
pub fn binary_lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / binary_gcd(a, b) * b
}
