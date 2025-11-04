//! Prime number utilities
//!
//! Functions for prime number operation.
//!
//! # Available functions
//!
//! - [`is_prime`]: Returns `true` if a number is prime number.
//! - [`divisors`]: Calulate all divisors of a given number.
//! - [`factorize`]: Factorize a number into its prime factors.
//! - [`primitive_root`]: Find a primitive root modulo a prime number.

use super::gcd::binary_gcd;

/// Returns `true` if `n` is prime number, otherwise `false`.
/// Uses the deterministic Miller-Rabin primality test.
/// This test is **guaranteed to be correct** for all `u64` values.
///
/// ## Definition
///
/// A prime number is a natural number greater than 1 that has no positive divisors
/// other than 1 and itself. Formally, `p` is prime if:
///
/// - `p > 1`
/// - For all integers `a`, `b` with `p = a * b`, either `a = 1` or `b = 1`
///
/// ## Complexity
///
/// `O(log n)`
pub fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {
        return false;
    } else if n == 2 {
        return true;
    } else if n & 1 == 0 {
        return false;
    }

    let r = (n - 1).trailing_zeros();
    let d = (n - 1) >> r;

    let miller_rabin = |a: u64| -> bool {
        let n = n as u128;
        let mut base = a as u128;
        let mut pow = 1;
        let mut d = d;
        while d > 0 {
            if d & 1 == 1 {
                pow *= base;
                pow %= n;
            }
            base *= base;
            base %= n;
            d >>= 1;
        }
        if pow == 1 || pow == n - 1 {
            return true;
        }
        for _ in 1..r {
            pow *= pow;
            pow %= n;
            if pow == n - 1 {
                return true;
            }
        }
        false
    };

    if n < 4_759_123_141 {
        miller_rabin(2) && miller_rabin(7) && miller_rabin(61)
    } else {
        miller_rabin(2)
            && miller_rabin(325)
            && miller_rabin(9_375)
            && miller_rabin(28_178)
            && miller_rabin(450_775)
            && miller_rabin(9_780_504)
            && miller_rabin(1_795_265_022)
    }
}

/// Returns all divisors of `n` in ascending order.
///
/// ## Definition
///
/// `divisors(n) = {d > 0 : d|n}`
///
/// ## Special Cases
///
/// - `divisors(0)` returns an empty vector `[]`
/// - `divisors(1)` returns `[1]`
///
/// ## Complexity
///
/// Time: `O(√n)`
/// Space: `O(d(n))` where `d(n)` is the number of divisors of `n`.
pub fn divisors(n: u64) -> Vec<u64> {
    let mut prefix = vec![];
    let mut suffix = vec![];
    for i in (1..).take_while(|i| i * i <= n) {
        if n % i == 0 {
            prefix.push(i);
            if i * i != n {
                suffix.push(n / i);
            }
        }
    }
    prefix.extend(suffix.iter().rev());
    prefix
}

/// Factorize `n` into its prime factors.
/// Uses Pollard's rho algorithm for finding larger prime factors.
///
/// ## Returns
///
/// A vector of prime factors in ascending order.
///
/// The returned vector satisfies:
///
/// - All elements are prime numbers
/// - The product of all elements equals `n`
/// - Elements are sorted in ascending order
///
/// ## Special Cases
///
/// - `factorize(1)` returns an empty vector `[]`
///
/// ## Panics
///
/// Panics if `n == 0`
///
/// ## Complexity
///
/// `O(n^(1/4) log n)`
pub fn factorize(mut n: u64) -> Vec<u64> {
    debug_assert_ne!(n, 0, "`n` must not be zero");
    let two = n.trailing_zeros();
    let mut res = vec![2; two as usize];
    n >>= two;
    while n % 3 == 0 {
        res.push(3);
        n /= 3;
    }
    if n == 1 {
        return res;
    }

    let mut i = res.len();
    res.push(n);
    while i < res.len() {
        let n = res[i];
        if is_prime(n) {
            i += 1;
            continue;
        }
        'LOOP: for t in 1.. {
            let mut x = t as u64;
            let mut y = ((x as u128 * x as u128 + t) % n as u128) as u64;
            loop {
                let g = binary_gcd(y + n - x, n);
                if g == 0 || g == n {
                    break;
                }
                if g != 1 {
                    res[i] /= g;
                    res.push(g);
                    break 'LOOP;
                }
                x = ((x as u128 * x as u128 + t) % n as u128) as u64;
                y = ((y as u128 * y as u128 + t) % n as u128) as u64;
                y = ((y as u128 * y as u128 + t) % n as u128) as u64;
            }
        }
    }
    res.sort_unstable();
    res
}

/// Returns the smallest primitive root modulo prime `p`.
///
/// ## Definition
///
/// For a prime `p`, an integer `g` is a **primitive root modulo p** if the powers
/// `g^0, g^1, ..., g^(p-2)` modulo `p` produce all non-zero residues modulo `p`.
///
/// Equivalently, `g` is a primitive root modulo `p` if:
/// - The multiplicative order of `g` modulo `p` is `p - 1`
/// - For all `1 <= k < p - 1`, `g^k ≢ 1 (mod p)`
/// - `g^(p-1) ≡ 1 (mod p)` (by Fermat's Little Theorem)
///
/// ## Parameters
///
/// - `p`: A prime number (must be prime: not verified in release mode)
///
/// ## Returns
///
/// The smallest positive integer that is a primitive root modulo `p`.
///
/// ## Complexity
///
/// `O(n^(1/4) log^2 n)`
pub fn primitive_root(p: u64) -> u64 {
    debug_assert!(is_prime(p), "`p` must be prime, but {} is not prime", p);

    if p == 2 {
        return 1;
    }

    let mut factor = factorize(p - 1);
    factor.dedup();
    for g in 2..p {
        let mut is_primitive = true;
        for f in &factor {
            let mut base = g;
            let mut exp = (p - 1) / f;
            let mut res = 1;
            let modulo = p as u128;
            while exp > 0 {
                if exp & 1 == 1 {
                    res = res * base as u128 % modulo;
                }
                base = (base as u128 * base as u128 % modulo) as u64;
                exp >>= 1;
            }
            if res == 1 {
                is_primitive = false;
                break;
            }
        }
        if is_primitive {
            return g;
        }
    }
    unreachable!();
}
