//! Polygon
//!
//! Functions for working with polygons in 2D.
//!
//! ## Available Functions
//!
//! - [`signed_area_mul2`]

use crate::geometry::vector_2d::Vector2D;

/// Computes twice the signed area of a polygon.
///
/// ## Definition
///
/// For a polygon with vertices p0, p1, ..., pn (in order), the signed area is computed using the
/// shoelace formula:
///
/// `2A = sum { xi y(i+1) - x(i+1)yi }`
///
/// where indices are taken modulo n.
///
/// Sign convention is
/// - **positive**: Vertices are ordered *counter-clockwise*.
/// - **negative**: Vertices are ordered *clockwise*.
/// - **zero**: Degenerate polygon (all vertices are collinear)
///
/// The sign and value are independent of which vertex is chosen as the starting point.
///
/// ## Parameters
///
/// - `polygon`: A slice of vertices in order
///
/// ## Returns
///
/// Twice the signed area as an `i64` value.
///
/// ## Complexity
///
/// `O(n)`, where `n` is the number of vertices.
pub fn signed_area_mul2(polygon: &[Vector2D]) -> i64 {
    if polygon.len() < 3 {
        return 0;
    }

    let mut area = polygon.windows(2).map(|w| w[0].cross(&w[1])).sum::<i64>();
    area += polygon[polygon.len() - 1].cross(&polygon[0]);
    area
}
