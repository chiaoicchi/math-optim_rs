//! Monoid Action
//!
//! This trait represents a **monoid action**.
//!
//! # Definition
//!
//! A monoid action consists of:
//! - A data monoid `(S, *, e_S)`
//! - An operator monoid `(F, *, e_F)`
//! - An action `F x S -> S` that satisfies:
//!   - **Homomorphism**: `f(a * b) = f(a) * f(b)` for all `f` in `F` and `a, b` in `S`.
//!   - **Composition compatibility**: `(f * g)(x) = f(g(x))` for all `f, g` in `F`.
//!   - **Identity**: `e(x) = x` for all `x` in `S`.

/// A trait representing a *monoid action*.
///
/// A monoid action consists of:
/// - A value set `S` (the associated type [`Self::S`])
/// - An associative binary operation in value ([`Self::op_s`])
/// - An identity element in value ([`Self::identity_s`])
/// - An operator set `F` (the associated type [`Self::F`])
/// - An associative binary operation in operator ([`Self::op_f`])
/// - An identity element in operator ([`Self::identity_f`])
pub trait MonoidAction {
    /// The underlying value type.
    type S;

    /// The underlying operator type.
    type F: Clone;

    /// Returns the identity element of the value set `S`.
    ///
    /// The identity element `e` satisfies `op_s(&a, &e) = op(&e, &a) = a` for all `a` in `S`.
    fn identity_s() -> Self::S;

    /// Returns the identity element of the operator set `F`.
    ///
    /// The identity element `e` satisfies `op_f(&a, &e) = op(&e, &a) = a` for all `a` in `S`.
    fn identity_f() -> Self::F;

    /// Returns the result of binary operation on `a` and `b` in `S`.
    ///
    /// This operation must be associative:
    /// `op_s(&op_s(&a, &b), &c) = op_s(&a, &op_s(&b, &c))` for all `a, b, c` in `S`.
    fn op_s(a: &Self::S, b: &Self::S) -> Self::S;

    /// Returns the result of binary operation on `a` and `b` in `F`.
    ///
    /// This operation must be associative:
    /// `op_f(&op_f(&a, &b), &c) = op_f(&a, &op_f(&b, &c))` for all `a, b, c` in `F`.
    fn op_f(a: &Self::F, b: &Self::F) -> Self::F;

    /// Applies an operator to data in-place.
    ///
    /// This operation must satisfy the homomorphism property.
    fn apply(x: &mut Self::S, f: &Self::F);
}
