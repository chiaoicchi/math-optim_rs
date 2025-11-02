//! Group
//!
//! This trait represents **group**.
//!
//! Group is a set equipped with associative binary operation and an identity element and inverse
//! operation.
//!
//! # Definition
//!
//! A set `G` equipped with a binary operation `G x G -> G`, which we will denote `*`, is a group
//! if it satisfies the following axioms.
//!
//! - **associativity**: for all `a, b, c` in `G`, the equation `(a * b) * c = a * (b * c)` holds.
//! - **identity**: there exists an element `e` in `G`, such that for all `a` in `G`,
//!   `a * e = e * a = a`.
//! - **inverse**: for every `a` in `G`, there exists an element `a^(-1)` in `G` such that
//!   `a * a^(-1) = a^(-1) * a = e`.

/// A trait representating a *group*.
///
/// A group consists of:
/// - A set `G` (the associated type [`Self::G`])
/// - An associative binary operation ([`Self::op`])
/// - An identity element ([`Self::identity`])
/// - Inverse elements ([`Self::inv`])
pub trait Group {
    /// The underlying set of the group.
    type G: Clone;

    /// Returns the identity element of the group.
    ///
    /// The identity element `e` satisfies `op(&a, &e) = op(&e, &a) = a` for all `a` in `G`.
    fn identity() -> Self::G;

    /// Returns the result of binary operation on `a` and `b`.
    ///
    /// This operation must be associative:
    /// `op(&op(&a, &b), &c) = op(&a, &op(&b, &c))` for all `a, b, c` in `G`.
    fn op(a: &Self::G, b: &Self::G) -> Self::G;

    /// Returns the inverse element of `a`.
    ///
    /// The inverse satisfies:
    /// `op(&a, &inv(&a)) = op(&inv(&a), &a) = Self::identity()` for all `a` in `G`.
    fn inv(a: &Self::G) -> Self::G;

    /// Returns the result of `op(&a, &inv(&b))`.
    ///
    /// This is a convenience method for computing `a * b^(-1)`.
    ///
    /// ## Default Implementation
    ///
    /// The default Implementation computes `op(&a, &inv(&b))`.
    #[inline]
    fn div(a: &Self::G, b: &Self::G) -> Self::G {
        Self::op(a, &Self::inv(b))
    }
}
