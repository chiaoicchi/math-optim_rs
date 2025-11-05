//! Vector in 2D
//!
//! This module provides a 2D vector type with common geometric operations.

/// A 2D Vector with integer coordinates.
///
/// This structure represents a vector in 2D space with `i64` coordinates, commonly used in
/// computational geometry problems.
///
/// # Definition
///
/// A vector in the 2-dimensional Euclidean space ℝ².
/// `Vector2D((x, y))` represents the vector from the origin to the point `(x, y)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Vector2D(pub (i64, i64));

impl Vector2D {
    /// Creates a new vector from x and y coordinates.
    ///
    /// ## Parameters
    ///
    /// - `x`: The x-coordinate
    /// - `y`: The y-coordinate
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    pub fn new(x: i64, y: i64) -> Self {
        Self((x, y))
    }

    /// Returns the x-coordinate of the vector.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn x(&self) -> i64 {
        self.0.0
    }

    /// Returns the y-coordinate of the vector.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn y(&self) -> i64 {
        self.0.1
    }

    /// Adds two vectors.
    ///
    /// ## Parameters
    ///
    /// - `other`: The vector to add
    ///
    /// ## Returns
    ///
    /// A new vector representing the sum `self + other`.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn add(&self, other: &Self) -> Self {
        Self((self.x() + other.x(), self.y() + other.y()))
    }

    /// Subtracts another vector from this vector.
    ///
    /// ## Parameters
    ///
    /// - `other`: The vector to subtract
    ///
    /// ## Returns
    ///
    /// A new vector representing the difference `self - other`.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn sub(&self, other: &Self) -> Self {
        Self((self.x() - other.x(), self.y() - other.y()))
    }

    /// Multiplies the vector by a scalar.
    ///
    /// ## Parameters
    ///
    /// - `a`: The scalar value
    ///
    /// ## Returns
    ///
    /// A new vector representing `a * self`.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn scalar(&self, a: i64) -> Self {
        Self((a * self.x(), a * self.y()))
    }

    /// Computes the dot product with another vector.
    ///
    /// ## Definition
    ///
    /// The dot product of vector `a = (xa, ya)` and `b = (xb, yb)` is defined as:
    ///
    /// `xa * xb + ya * yb`
    ///
    /// ## Returns
    ///
    /// The dot product as an `i64` value.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn dot(&self, other: &Self) -> i64 {
        self.x() * other.x() + self.y() * other.y()
    }

    /// Computes the cross product with another vector.
    ///
    /// ## Definition
    ///
    /// In 2D, the cross product of vector `a = (xa, ya)` and `b = (xb, yb)` returns a scalar representing
    /// the z-component of the 3D cross product:
    ///
    /// `xa * yb - xb * ya`
    ///
    /// ## Returns
    ///
    /// The cross product as an `i64` value. Positive if `other` is counter-clockwise from `self`,
    /// negative if clockwise, and zero if collinear.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn cross(&self, other: &Self) -> i64 {
        self.x() * other.y() - self.y() * other.x()
    }

    /// Computes the squared norm (magnitude squared) of the vector.
    ///
    /// ## Returns
    ///
    /// `x^2 + y^2`.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn norm_pow2(&self) -> i64 {
        self.x() * self.x() + self.y() * self.y()
    }
}

impl From<(i64, i64)> for Vector2D {
    fn from(t: (i64, i64)) -> Self {
        Vector2D(t)
    }
}
impl From<Vector2D> for (i64, i64) {
    fn from(v: Vector2D) -> Self {
        v.0
    }
}
