//! Standard implementations of monoids.
//!
//! This module provides commonly used monoid implementations.
//!
//! # Available monoids
//!
//! - [`AffineMonoid`]: Composition of affine transformations

use super::monoid::Monoid;

use std::marker::PhantomData;

/// Affine transformation composition monoid over type `T`.
/// Represents affine functions `f(x) = ax + b` as tuples `(a, b)`.
///
/// - **set**: `(i64, i64)` representing `f(x) = ax + b`.
/// - **operation**: Function composition `g ∘ f`.
/// - **identity**: `(1, 0)` representing the identity function `f(x) = x`.
///
/// # Composition Order
///
/// `op(&a, &b)` computes `b ∘ a`, which applies `a` first, then `b`.
///
/// If `a = (a1, b1)` represents `f(x) = a1x + b1` and
/// `b = (a2, b2)` represents `g(x) = a2x + b2`, then
/// `op(&a, &b)` returns `(g ∘ f)(x)` where:
///
/// ```text
/// (g ∘ f)(x) = g(f(x)) = a2(a1x + b1) + b2 = (a1a2)x + (a2b1 + b2)
/// ```
///
/// # Supported Types
///
/// - `i32`, `i64`, `i128`
/// - `u32`, `u64`, `u128`
pub struct AffineMonoid<T>(PhantomData<T>);
/// Implements [`Monoid`] for [`AffineMonoid<T>`] for multiple integer types.
macro_rules! impl_affine_monoid {
    ($($t:ty),* $(,)?) => {
        $(
            impl Monoid for AffineMonoid<$t> {
                type S = ($t, $t);

                #[inline]
                fn identity() -> Self::S {
                    (1, 0)
                }

                #[inline]
                fn op(a: &Self::S, b: &Self::S) -> Self::S {
                    (a.0 * b.0, a.1 * b.0 + b.1)
                }
            }
        )*
    };
}
impl_affine_monoid!(i32, i64, i128, u32, u64, u128);
