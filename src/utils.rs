//! Utilities
//!
//! There are many useful functions and traits.
//!
//! # Available traits
//!
//! - [`ChMinMax`]: Be able to use `chmin, chmax` methods on `PartialOrd` values.

/// A trait that provides `chmin` and `chmax` methods for updating values.
///
/// These methods are commonly used in competitive programming for updating minimum or maximum
/// values in-place.
///
/// # Available methods
///
/// - [`chmin`](Self::chmin): Compares value and updates to the smaller value
/// - [`chmax`](Self::chmax): Compares value and updates to the larger value
pub trait ChMinMax {
    fn chmin(&mut self, x: Self) -> bool;
    fn chmax(&mut self, x: Self) -> bool;
}

impl<T: PartialOrd> ChMinMax for T {
    /// Updates `self` to the minimum of `self` and `x`.
    /// Returns `true` if `self` was updated (i.e., `self > x`), `false` otherwise.
    fn chmin(&mut self, x: Self) -> bool {
        *self > x && {
            *self = x;
            true
        }
    }

    /// Updates `self` to the maximum of `self` and `x`.
    /// Returns `true` if `self` was updated (i.e., `self < x`), `false` otherwise.
    fn chmax(&mut self, x: Self) -> bool {
        *self < x && {
            *self = x;
            true
        }
    }
}
