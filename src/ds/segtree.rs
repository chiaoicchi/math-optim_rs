//! Segment Tree
//!
//! A data structure for efficiently performing range queries and point updates on a sequence
//! with an associative binary operation (monoid).
//!
//! It supports the following operations:
//!
//! - **set**: Update a single element
//! - **range fold**: Compute the result of a monoid operation over a range

use std::ops::RangeBounds;

use crate::algebra::monoid::Monoid;

/// A *Segment Tree* that supports range queries and point updates.
///
/// The time complexity of operations depends on the cost of the monoid operations:
///
/// - `clone()`: Creating a copy of a monoid element.
/// - `op()`: Computing the binary operation.
///
/// If these operations take `O(1)` time, all segment tree operations are `O(log n)`.
///
/// # Type Parameters
///
/// - `T`: A type implementing the [`Monoid`] trait.
#[derive(Debug, Clone)]
pub struct SegTree<T: Monoid> {
    /// The number of the sequence which is managed by this.
    n: usize,

    /// The capacity of the underlying array (next power of two >= n)
    m: usize,

    /// Internal data array of size `2 * m`.
    ///
    /// ## Definition
    ///
    /// Let `a` denote the sequence managed by this structure.
    ///
    /// - If `m <= i < n + m` then `data[i] == a[i - m]`
    /// - If `n + m <= i` then `data[i] == T::identity()`
    /// - If `0 < i < m` then `data[i] = T::op(&data[2 * i], &data[2 * i + 1])`
    /// - If `i == 0` then `data[i]` is unimplemented
    data: Box<[T::S]>,
}

impl<T: Monoid> SegTree<T> {
    /// Creates a new Segment Tree with identity sequence with length `n`,
    /// `a_i == T::identity()` for all `0 <= i < n`.
    ///
    /// ## Parameters
    ///
    /// - `n`: Length of sequence.
    ///
    /// ## Complexity
    ///
    /// `O(n)`
    pub fn new(n: usize) -> Self {
        let m = n.next_power_of_two();
        Self {
            n,
            m,
            data: vec![T::identity(); 2 * m].into_boxed_slice(),
        }
    }

    /// Creates a new Segment Tree from a vector.
    ///
    /// ## Parameters
    ///
    /// - `a`: Reference of a sequence which is managed by this structure.
    ///
    /// ## Complexity
    ///
    /// `O(n)`
    pub fn from_vec(a: &[T::S]) -> Self {
        let n = a.len();
        let m = n.next_power_of_two();
        let mut data = vec![T::identity(); 2 * m];
        data[m..n + m].clone_from_slice(a);
        for i in (1..m).rev() {
            data[i] = T::op(&data[2 * i], &data[2 * i + 1]);
        }
        Self {
            n,
            m,
            data: data.into_boxed_slice(),
        }
    }

    /// Updates the element at index `i` to the value `x`.
    ///
    /// ## Parameters
    ///
    /// - `i`: Index to update (must satisfy `0 <= i < n`).
    /// - `x`: The new value you want to set to `a[i]`.
    ///
    /// ## Panics
    ///
    /// Panics if `n <= i`.
    ///
    /// ## Complexity
    ///
    /// `O(log n)`
    #[inline]
    pub fn set(&mut self, mut i: usize, x: T::S) {
        debug_assert!(
            i < self.n,
            "invalid index: {} must be smaller than {}",
            i,
            self.n
        );
        i += self.m;
        self.data[i] = x;
        i >>= 1;
        while i > 0 {
            self.data[i] = T::op(&self.data[2 * i], &self.data[2 * i + 1]);
            i >>= 1;
        }
    }

    /// Returns the reference of index `i`.
    ///
    /// ## Parameters
    ///
    /// - `i`: Index to get (must satisfy `0 <= i < n`).
    ///
    /// ## Panics
    ///
    /// Panics if `i >= n`.
    ///
    /// ## Complexity
    ///
    /// `O(1)`.
    #[inline]
    pub fn get(&self, i: usize) -> &T::S {
        debug_assert!(
            i < self.n,
            "invalid index: {} must be smaller than {}",
            i,
            self.n
        );
        &self.data[i + self.m]
    }

    /// Calculates the monoid operation over a range.
    ///
    /// ## Parameters
    ///
    /// - `range`: The range of intervals. This must be `RangeBounds<usize>`.
    ///
    /// ## Returns
    ///
    /// If sequence managed by this is `a` and `range` is `[l, r)`,
    /// returns `a[l] * a[l + 1] * ... * a[r - 1]`.
    ///
    /// ## Complexity
    ///
    /// `O(log n)`
    #[inline]
    pub fn range_fold(&self, range: impl RangeBounds<usize>) -> T::S {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let mut l = match range.start_bound() {
            Unbounded => 0,
            Included(x) => *x,
            Excluded(x) => x + 1,
        } + self.m;
        let mut r = match range.end_bound() {
            Unbounded => self.n,
            Included(x) => x + 1,
            Excluded(x) => *x,
        } + self.m;
        debug_assert!(
            l <= r,
            "invalid range: start {} must be smaller than or equal to end {}",
            l - self.m,
            r - self.m
        );
        debug_assert!(
            r <= self.n + self.m,
            "invalid range: range end {} must be smaller than length {}",
            r - self.m,
            self.n
        );
        let mut left = T::identity();
        let mut right = T::identity();
        while l < r {
            if l & 1 == 1 {
                left = T::op(&left, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = T::op(&self.data[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(&left, &right)
    }

    /// Return the monoid operation over a sequence.
    ///
    /// ## Returns
    ///
    /// If the sequence managed by this is `a`, returns `a[0] * a[1] * ... * a[n - 1]`.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn all_fold(&self) -> T::S {
        self.data[1].clone()
    }
}
