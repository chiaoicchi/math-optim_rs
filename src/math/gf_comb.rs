//! Combinatorics
//!
//! Efficient combinatorial calculations in modular arithmetic using precomputed factorials.
//!
//! This module provides fast computation of the following numbers.
//!
//! - [`Comb::factorial`]: Calculate `n!`
//! - [`Comb::inv_factorial`]: Calculate `(n!)^(-1)`
//! - [`Comb::binom`]: Calculate binomial coefficient `C(n, r)`
//! - [`Comb::perm`]: Calculate permutation `P(n, r)`
//! - [`Comb::multinom`]: Calculate multinomial coefficient
//! - [`Comb::homogeneous`]: Calculate homogeneous product (stars and bars)

use super::gf::GF;

/// A structure for efficient combinatorial calculations modulo a prime.
///
/// Precomputes factorials and their modular inverses up to `n - 1`.
///
/// # Type Parameters
///
/// - `MOD`: The prime modulus
///   (satisfy `MOD` is prime)
///
/// # Requirements
///
/// **`MOD` must be prime.** Non-prime moduli do not form a field.
#[derive(Clone)]
pub struct Comb<const MOD: u32> {
    /// Maximum value for which factorials are precomputed (exclusive upper bound)
    n: u32,

    /// Precomputed factorials: `fact[i] = i! mod MOD`
    fact: Vec<GF<MOD>>,

    /// Precomputed inverse factorials: `inv_fact[i] = (i!)^(-1) mod MOD`
    inv_fact: Vec<GF<MOD>>,
}

impl<const MOD: u32> Comb<MOD> {
    /// Creates a new `Comb` structure with precomputed factorials up to `n - 1`.
    ///
    /// ## Parameters
    ///
    /// - `n`: Precompute factorials for `0!` to `(n - 1)!` (must satisfy `n >= 1`)
    ///
    /// ## Complexity
    ///
    /// `O(n)`
    ///
    /// ## Panics
    ///
    /// Panics if `n == 0`
    pub fn build(n: u32) -> Self {
        debug_assert!(n > 0, "n must be at least 1 (got n = {})", n);
        let mut fact = Vec::with_capacity(n as usize);
        fact.push(GF::<MOD>::new(1));
        for i in 1..n {
            fact.push(fact.last().unwrap() * GF::<MOD>::new(i));
        }
        let mut inv_fact = Vec::with_capacity(n as usize);
        inv_fact.push(fact.last().unwrap().inv());
        for i in (1..n).rev() {
            inv_fact.push(inv_fact.last().unwrap() * GF::<MOD>::new(i));
        }
        inv_fact.reverse();
        Self { n, fact, inv_fact }
    }

    /// Returns the factorial `n!` modulo `MOD`.
    ///
    /// ## Definition
    ///
    /// For a non-negative integer `n`:
    ///
    /// `n! = 1 * 2 * 3 * ... * n`
    ///
    /// with the convention that `0! = 1`.
    ///
    /// ## Panics
    ///
    /// Panics if `n >= self.n` (value not precomputed).
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn factorial(&self, n: u32) -> GF<MOD> {
        debug_assert!(
            n < self.n,
            "factorial({}) is not precomputed (max: {})",
            n,
            self.n - 1
        );
        self.fact[n as usize]
    }

    /// Returns the inverse factorial `(n!)^(-1)` modulo `MOD`.
    ///
    /// ## Panics
    ///
    /// Panics if `n >= self.n` (value not precomputed)
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn inv_factorial(&self, n: u32) -> GF<MOD> {
        debug_assert!(
            n < self.n,
            "inv_factorial({}) is not precomputed (max: {})",
            n,
            self.n - 1
        );
        self.inv_fact[n as usize]
    }

    /// Returns the binomial coefficient `C(n, r) = n! / (r! * (n-r)!)` modulo `MOD`.
    ///
    /// ## Definition
    ///
    /// For non-negative integers `n` and `r` with `r <= n`:
    ///
    /// `C(n, r) = n! / (r! * (n-r)!) = n choose r`
    ///
    /// ## Panics
    ///
    /// - Panics if `n >= self.n`.
    /// - Panics if `r > n`.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn binom(&self, n: u32, r: u32) -> GF<MOD> {
        debug_assert!(
            n < self.n,
            "binom({}, {}) requires n < {} (got n = {})",
            n,
            r,
            self.n,
            n
        );
        debug_assert!(
            r <= n,
            "binom({}, {}) requires r <= n (got n = {}, r = {})",
            n,
            r,
            n,
            r
        );
        self.fact[n as usize] * self.inv_fact[r as usize] * self.inv_fact[(n - r) as usize]
    }

    /// Returns the permutation `P(n, r) = n! / (n-r)!` modulo `MOD`.
    ///
    /// ## Definition
    ///
    /// For non-negative integers `n` and `r` with `r <= n`:
    ///
    /// `P(n, r) = n! / (n-r)! = n * (n - 1) * ... * (n-r+1)`
    ///
    /// ## Panics
    ///
    /// - Panics if `n >= self.n`.
    /// - Panics if `r > n`.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn perm(&self, n: u32, r: u32) -> GF<MOD> {
        debug_assert!(
            n < self.n,
            "perm({}, {}) requires n < {} (got n = {})",
            n,
            r,
            self.n,
            n
        );
        debug_assert!(
            r <= n,
            "perm({}, {}) requires r <= n (got n = {}, r = {})",
            n,
            r,
            n,
            r
        );
        self.fact[n as usize] * self.inv_fact[(n - r) as usize]
    }

    /// Returns the multinomial coefficient `n! / (k1! * k2! * ... * km!)` modulo `MOD`.
    ///
    /// ## Definition
    ///
    /// For non-negative integers `n` and `k1, k2, ..., km` with `k1 + k2 + ... + km = n`:
    ///
    /// `M(n; k1, k2, ..., km) = n! / (k1! * k2! * ... * km!)`
    ///
    /// ## Panics
    ///
    /// - Panics if `n >= self.n`
    /// - Panics if sum of `ks` does not equal to `n`
    ///
    /// ## Complexity
    ///
    /// `O(m)` where `m` is the length of `ks`
    #[inline]
    pub fn multinom(&self, n: u32, ks: &[u32]) -> GF<MOD> {
        debug_assert!(
            n < self.n,
            "multinom({}, ...) requires n < {} (got n = {})",
            n,
            self.n,
            n
        );
        debug_assert_eq!(
            ks.iter().sum::<u32>(),
            n,
            "multinom({}, {:?}) requires sum(ks) == n (got sum = {})",
            n,
            ks,
            ks.iter().sum::<u32>()
        );
        let mut res = self.fact[n as usize];
        for &k in ks {
            res *= self.inv_fact[k as usize];
        }
        res
    }

    /// Returns the homogeneous `(n+r-1)! / (n! * (r-1)!)` modulo `MOD`.
    ///
    /// ## Definition
    ///
    /// For non-negative integers `n` and `r`:
    ///
    /// `H(n, r) = C(n + r - 1, r - 1) = C(n + r - 1, n) = (n+r-1)! / (n! * (r-1)!)`
    ///
    /// ## Panics
    ///
    /// - Panics if `n + r - 1 >= self.n`
    /// - Panics if `r == 0`
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn homogeneous(&self, n: u32, r: u32) -> GF<MOD> {
        debug_assert!(
            n + r - 1 < self.n,
            "homogeneous({}, {}) requires n + r - 1 < {} (got n = {}, r = {})",
            n,
            r,
            self.n,
            n,
            r
        );
        if n == 0 {
            return GF::<MOD>::new(1);
        }
        debug_assert_ne!(
            r, 0,
            "homogeneous({}, {}) requires r > 0 (got r = {})",
            n, r, r
        );

        self.binom(n + r - 1, r - 1)
    }
}
