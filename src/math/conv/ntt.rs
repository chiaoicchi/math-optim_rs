//! Number Theoretic Transform (NTT)
//!
//! Functions for NTT and convolution.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::math::gf::GF;
use crate::math::modint::pow_mod;
use crate::math::prime::primitive_root;

/// Maximum number of levels supported for NTT.
const MAX_NTT_LEVELS: usize = 32;

/// Type alias for precomputed NTT roots: (forward_roots, inverse_roots).
type NttRoots = ([u32; MAX_NTT_LEVELS], [u32; MAX_NTT_LEVELS]);

/// Type alias for the NTT roots cache.
type NttRootsCache = Mutex<HashMap<u32, NttRoots>>;

/// Global cache for NTT roots, keyed by modulus.
///
/// This cache stores precomputed roots of unity for each modulus to avoid redundant computation.
/// The cache is lazily initialized on first access and protected by a Mutex for thread-safe access.
static NTT_ROOTS_CACHE: LazyLock<NttRootsCache> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Returns precomputed NTT roots for the given modulus.
///
/// Roots are computed once per modulus and cached for subsequent calls.
/// This function is thread-safe.
///
/// ## Returns
///
/// A tuple `(roots, inv_roots)` where:
/// - `root[i]`: contains the forward root for NTT at level `i`.
/// - `inv_root[i]`: contains the inverse root for INTT at level `i`.
fn get_ntt_roots(modulo: u32) -> ([u32; MAX_NTT_LEVELS], [u32; MAX_NTT_LEVELS]) {
    let mut cache = NTT_ROOTS_CACHE.lock().unwrap();
    if let Some(&roots) = cache.get(&modulo) {
        roots
    } else {
        let roots = compute_ntt_roots(modulo);
        cache.insert(modulo, roots);
        roots
    }
}

/// Compute NTT roots for the given modulus.
///
/// ## Requirements
///
/// - `modulo` must be a prime of the form `c * 2^k + 1` where `k >= 2`.
///
/// ## Panics
///
/// - Panics if `k >= MAX_NTT_LEVELS`
///
/// ## Returns
///
/// Let `g` be the smallest primitive root and `omega := g^c`,
/// A tuple `(roots, inv_roots)` where:
/// - `roots[i]`: contains `omega^(2^i) * zeta_i mod modulo`
/// - `inv_roots[i]`: contains `omega^(-2^i) * zeta_i^(-1) mod modulo`
///
/// where `zeta_i` are adjustment factors for bit-reversal ordering.
fn compute_ntt_roots(modulo: u32) -> ([u32; MAX_NTT_LEVELS], [u32; MAX_NTT_LEVELS]) {
    let k = (modulo - 1).trailing_zeros() as usize;
    assert!(
        k < MAX_NTT_LEVELS,
        "`modulo - 1` must have fewer than {} trailing zeros, got 2^{} | {}",
        MAX_NTT_LEVELS,
        k,
        modulo - 1
    );

    let g = primitive_root(modulo as u64);
    let mut omega = pow_mod(g, ((modulo - 1) >> k) as u64, modulo as u64);
    let mut inv_omega = pow_mod(omega, (modulo - 2) as u64, modulo as u64);

    let mut omega_pow = [0u32; MAX_NTT_LEVELS];
    let mut inv_omega_pow = [0u32; MAX_NTT_LEVELS];

    for i in (0..k - 1).rev() {
        omega_pow[i] = omega as u32;
        inv_omega_pow[i] = inv_omega as u32;
        omega = (omega * omega) % modulo as u64;
        inv_omega = (inv_omega * inv_omega) % modulo as u64;
    }

    let mut roots = [0u32; MAX_NTT_LEVELS];
    let mut inv_roots = [0u32; MAX_NTT_LEVELS];

    let mut zeta = 1u64;
    let mut inv_zeta = 1u64;

    for i in 0..k - 1 {
        roots[i] = (omega_pow[i] as u64 * zeta % modulo as u64) as u32;
        inv_roots[i] = (inv_omega_pow[i] as u64 * inv_zeta % modulo as u64) as u32;
        zeta = zeta * inv_omega_pow[i] as u64 % modulo as u64;
        inv_zeta = inv_zeta * omega_pow[i] as u64 % modulo as u64;
    }

    (roots, inv_roots)
}

/// Performs Number Theoretic Transform (in-place).
///
/// ## Definition
///
/// For input sequence `a = (a0, a1, ..., a(n-1))`, computes output `f = (f0, f1, ..., f(n-1))`
/// where:
///
/// `fk = sum_j {aj * omega^(jk)} mod MOD`
///
/// where `omega` is a primitive n-th root of unity modulo MOD.
///
/// ## Requirements
///
/// - `data.len()` must be a power of two.
/// - `MOD` must be a prime of the form `c * 2^k + 1` where `k >= log2(data.len())`.
///
/// ## Complexity
///
/// `O(n log n)` where `n = data.len()`
pub fn ntt<const MOD: u32>(data: &mut [GF<MOD>]) {
    let n = data.len();
    let k = n.trailing_zeros() as usize;
    let (roots, _) = get_ntt_roots(MOD);
    for t in (0..k).rev() {
        let t = 1 << t;
        let mut coef = GF::new(1);
        for (i, data) in data.chunks_exact_mut(2 * t).enumerate() {
            let (x, y) = data.split_at_mut(t);
            for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                (*x, *y) = (*x + *y * coef, *x - *y * coef);
            }
            coef *= GF::new(roots[(!i).trailing_zeros() as usize]);
        }
    }
}

/// Performs Inverse Number Theoretic Transform (in-place).
///
/// ## Definition
///
/// For input sequence `f = (f0, f1, ..., f(n-1))` (output of NTT),
/// computes output `a = (a0, a1, ..., a(n-1))` where:
///
/// `ak = n^(-1) * sum_j {fj * omega^(-jk)} mod MOD`
///
/// where `omega` is a primitive n-th root of unity modulo MOD.
///
/// This operation satisfies: `INTT(NTT(a)) = a` for all sequence `a`.
///
/// ## Requirements
///
/// - `data.len()` must be a power of two.
/// - `MOD` must be a prime of the form `c * 2^k + 1` where `k >= log2(data.len())`.
///
/// ## Complexity
///
/// `O(n log n)` where `n = data.len()`
pub fn intt<const MOD: u32>(data: &mut [GF<MOD>]) {
    let n = data.len();
    let k = n.trailing_zeros() as usize;
    let (_, inv_roots) = get_ntt_roots(MOD);
    for t in 0..k {
        let t = 1 << t;
        let mut coef = GF::new(1);
        for (i, data) in data.chunks_exact_mut(2 * t).enumerate() {
            let (x, y) = data.split_at_mut(t);
            for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                (*x, *y) = (*x + *y, (*x - *y) * coef);
            }
            coef *= GF::new(inv_roots[(!i).trailing_zeros() as usize]);
        }
    }
    let coef = GF::new(2).inv().pow(k as u32);
    for data in data.iter_mut() {
        *data *= coef;
    }
}

/// Computes the convolution of two sequences using NTT.
///
/// ## Definition
///
/// For sequences `a = (a0, a1, ..., a(n-1))` and `b = (b0, b1, ..., b(n-1))`,
/// computes the convolution `c = (c0, c1, ..., c(n-1))` where:
///
/// `ck = sum i ai * b(k-i) mod MOD`
///
/// with the convolution that `ai = 0` for `i >= n` and `b_j = 0` for `j >= m`.
///
/// ## Requirements
///
/// - `MOD` must be a prime of the form `c * 2^k + 1` where `k >= log2(lhs.len() + rhs.len() - 1)`.
///
/// ## Complexity
///
/// `O(n log n)` where `n` is the smallest power of two >= `lhs.len() + rhs.len() - 1`.
pub fn ntt_conv<const MOD: u32>(lhs: &[GF<MOD>], rhs: &[GF<MOD>]) -> Vec<GF<MOD>> {
    if lhs.is_empty() || rhs.is_empty() {
        return vec![];
    }
    let size = (lhs.len() + rhs.len() - 1).next_power_of_two();
    let mut a = vec![GF::new(0); size];
    let mut b = vec![GF::new(0); size];
    a[..lhs.len()].copy_from_slice(lhs);
    b[..rhs.len()].copy_from_slice(rhs);

    ntt(&mut a);
    ntt(&mut b);

    let mut c = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| a * b)
        .collect::<Vec<_>>();
    intt(&mut c);

    c.truncate(lhs.len() + rhs.len() - 1);
    c
}
