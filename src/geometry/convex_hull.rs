//! Convex Hull
//!
//! Algorithms for computing convex hull.
//!
//! ## Definition
//!
//! Let `S` be a set of points in R². The **convex hull** of `S`, denoted `conv(S)`, is the
//! smallest convex set containing `S`. Equivalently:
//!
//! ```text
//! conv(S) = { ∑ λᵢpᵢ | pᵢ ∈ S, λᵢ ≥ 0, ∑ λᵢ = 1 }
//! ```
//!
//! That is, the convex hull is the set of all convex combinations of points in `S`.
//!
//! For a finite set of points in the plane, the convex hull forms a convex polygon whose
//! vertices are a subset of the input points.

use crate::geometry::vector_2d::Vector2D;

/// Computes the convex hull using Andrew's monotone chain algorithm.
///
/// ## Parameters
///
/// - `points`: A slice of points in 2D space
///
/// ## Returns
///
/// A vector of points representing the convex hull in counter-clockwise order.
///
/// ## Complexity
///
/// `O(n log n)`
pub fn andrew(points: &[Vector2D]) -> Vec<Vector2D> {
    let mut pts = points.to_vec();

    pts.sort_unstable_by_key(|point| (point.x(), point.y()));
    pts.dedup();
    let n = pts.len();

    if n <= 2 {
        return pts;
    }

    let mut hull: Vec<Vector2D> = Vec::new();

    for pt in pts.iter().copied() {
        while hull.len() >= 2 {
            let a = hull[hull.len() - 2];
            let b = hull[hull.len() - 1];
            if b.sub(&a).cross(&pt.sub(&a)) <= 0 {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(pt);
    }

    let lower_len = hull.len();
    for &pt in pts.iter().rev().skip(1) {
        while hull.len() > lower_len {
            let a = hull[hull.len() - 2];
            let b = hull[hull.len() - 1];
            if b.sub(&a).cross(&pt.sub(&a)) <= 0 {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(pt);
    }

    hull.pop();
    hull
}
