//! Angular Operations
//!
//! Functions for working with angles and polar coordinates in 2D geometry.
//!
//! # Available Functions
//!
//! - [`arg_cmp`]: Compares two points by their argument.

use std::cmp::Ordering;

/// Compares two points by their argument (angle from positive x-axis).
///
/// ## Returns
///
/// An [`Ordering`] value:
/// - [`Ordering::Less`]: The first point has a smaller argument (appears earlier in counter-clockwise order)
/// - [`Ordering::Equal`]: Both points have the same argument (are collinear with the origin, or both are the origin)
/// - [`Ordering::Greater`]: The first point has a larger argument (appears later in counter-clockwise order)
///
/// ## Complexity
///
/// `O(1)`
pub fn arg_cmp((x0, y0): &(i64, i64), (x1, y1): &(i64, i64)) -> Ordering {
    ((*y0, *x0) < (0, 0))
        .cmp(&((*y1, *x1) < (0, 0)))
        .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
}
