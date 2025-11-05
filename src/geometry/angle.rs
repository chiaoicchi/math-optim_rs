//! Angular Operations
//!
//! Functions for working with angles and polar coordinates in 2D geometry.
//!
//! # Available Functions
//!
//! - [`arg_cmp`]: Compares two points by their argument.

use std::cmp::Ordering;

use crate::geometry::vector_2d::Vector2D;

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
pub fn arg_cmp(a: &Vector2D, b: &Vector2D) -> Ordering {
    ((a.y(), a.x()) < (0, 0))
        .cmp(&((b.y(), b.x()) < (0, 0)))
        .then_with(|| (b.x() * a.y()).cmp(&(a.x() * b.y())))
}
