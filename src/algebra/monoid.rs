//! Monoid
//!
//! This trait represents **monoid**.
//!
//! Monoid is a set equipped with associative binary operation and an identity element.
//!
//! # Definition
//!
//! A set `S` equipped with a binary operation `S x S -> S`, which we will denote `*`, is a monoid.
//! It satisfies the following two axioms.
//!
//! - **associativity**: for all `a, b, c` in `S`, the equation `(a * b) * c == a * (b * c)` holds.
//! - **identity element**: There exists an element `e` in S such that for every element `a` in
//! `S`, the equalities `a * e == e * a == a` hold.

/// A trait representating a *monoid*.
///
/// A monoid consists of:
/// - A set `S` (the associated type [`Self::S`])
/// - An associative binary operation ([`Self::op`])
/// - An identity element ([`Self::identity`])
pub trait Monoid {
    /// The underlying set of the monoid.
    type S: Clone;

    /// Returns the identity element of the monoid.
    ///
    /// The identity element `e` satisfies `op(&a, &e) == op(&e, &a) == a` for all `a` in `S`.
    fn identity() -> Self::S;

    /// Returns the result of binary operation on `a` and `b`.
    ///
    /// This operation must be associative:
    /// `op(&op(&a, &b), &c) == op(&a, &op(&b, &c))` for all `a, b, c` in `S`.
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}
