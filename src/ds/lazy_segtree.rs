//! Lazy Segment Tree
//!
//! A data structure for efficiently performing range queries and range updates on a sequence.
//!
//! It supports the following operations:
//!
//! - **range_apply**: Apply an operator to all elements in a range.
//! - **range_fold**: Compute the result of a monoid operation over a range.
//!
//! The key feature is that updates are performed *lazily*...

use crate::algebra::monoid_action::MonoidAction;

/// A *Lazy Segment Tree* that supports range queries and range updates.
///
/// The time complexity of operations depends on the cost of the monoid action operations:
/// If these operations take `O(1)` time, run in `O(log n)` time.
///
/// # Type Parameters
///
/// - `T`: A type implementing the [`MonoidAction`] trait.
#[derive(Debug, Clone)]
pub struct LazySegTree<T: MonoidAction> {
    /// The number of the sequence which is managed by this.
    n: usize,

    /// The capacity of the underlying array (next power of two >= n)
    m: usize,

    /// Internal data array of size `2 * m`.
    /// Indices `[m, m + n)` store the actual data.
    /// Indices `[1, m)` store aggregated results.
    /// Index `0` is unused.
    data: Box<[T::S]>,

    /// Internal lazy propagation array of size `2 * m`.
    /// - Stores pending operators that need to be propagated to children.
    /// - `func[k]` is the operator pending for the subtree rooted at node `k`.
    func: Box<[T::F]>,
}

impl<T: MonoidAction> LazySegTree<T> {
    /// Creates a new Lazy Segment Tree with identity sequence with length `n`,
    /// `a_i == T::identity_s()` for all `0 <= i < n`.
    ///
    /// ## Parameters
    ///
    /// - `n`: Length of sequence.
    ///
    /// ## complexity
    ///
    /// `O(n)`
    pub fn new(n: usize) -> Self {
        let m = n.next_power_of_two();
        Self {
            n,
            m,
            data: std::iter::repeat_with(T::identity_s).take(2 * m).collect(),
            func: vec![T::identity_f(); 2 * m].into_boxed_slice(),
        }
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
    pub fn range_fold(&mut self, range: impl std::ops::RangeBounds<usize>) -> T::S {
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

        for k in (1..=self.m.trailing_zeros()).rev() {
            if (l >> k) << k != l {
                self.push(l >> k);
            }
            if (r >> k) << k != r {
                self.push((r - 1) >> k);
            }
        }

        let mut left = T::identity_s();
        let mut right = T::identity_s();
        while l < r {
            if l & 1 == 1 {
                left = T::op_s(&left, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = T::op_s(&self.data[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op_s(&left, &right)
    }

    /// Applies the operator to all elements in a range.
    ///
    /// ## Parameters
    ///
    /// - `range`: The range of intervals. This must be `RangeBounds<usize>`.
    /// - `f`: An operator.
    ///
    /// ## Complexity
    ///
    /// `O(log n)`
    #[inline]
    pub fn range_apply(&mut self, range: impl std::ops::RangeBounds<usize>, f: &T::F) {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(x) => *x,
            Excluded(x) => x + 1,
        } + self.m;
        let r = match range.end_bound() {
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

        for k in (1..=self.m.trailing_zeros()).rev() {
            if (l >> k) << k != l {
                self.push(l >> k);
            }
            if (r >> k) << k != r {
                self.push((r - 1) >> k);
            }
        }

        {
            let (mut l, mut r) = (l, r);
            while l < r {
                if l & 1 == 1 {
                    T::apply(&mut self.data[l], f);
                    self.func[l] = T::op_f(&self.func[l], f);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    T::apply(&mut self.data[r], f);
                    self.func[r] = T::op_f(&self.func[r], f);
                }
                l >>= 1;
                r >>= 1;
            }
        }

        for k in 1..=self.m.trailing_zeros() {
            if (l >> k) << k != l {
                self.data[l >> k] = T::op_s(&self.data[2 * (l >> k)], &self.data[2 * (l >> k) + 1]);
            }
            if (r >> k) << k != r {
                self.data[(r - 1) >> k] = T::op_s(
                    &self.data[2 * ((r - 1) >> k)],
                    &self.data[2 * ((r - 1) >> k) + 1],
                );
            }
        }
    }

    #[inline]
    fn push(&mut self, k: usize) {
        let f = std::mem::replace(&mut self.func[k], T::identity_f());
        T::apply(&mut self.data[2 * k], &f);
        T::apply(&mut self.data[2 * k + 1], &f);
        self.func[2 * k] = T::op_f(&self.func[2 * k], &f);
        self.func[2 * k + 1] = T::op_f(&self.func[2 * k + 1], &f);
    }
}

impl<T: MonoidAction> LazySegTree<T>
where
    T::S: Clone,
{
    /// Creates a new Lazy Segment Tree from a vector.
    ///
    /// ## Parameters
    ///
    /// - `a`: Reference of a sequence which is managed by this structure.
    ///
    /// ## Complexity
    ///
    /// `O(n)`
    pub fn from_slice(a: &[T::S]) -> Self {
        let n = a.len();
        let m = n.next_power_of_two();
        let mut data = vec![T::identity_s(); 2 * m];
        data[m..m + n].clone_from_slice(a);
        for i in (1..m).rev() {
            data[i] = T::op_s(&data[2 * i], &data[2 * i + 1]);
        }
        Self {
            n,
            m,
            data: data.into_boxed_slice(),
            func: vec![T::identity_f(); 2 * m].into_boxed_slice(),
        }
    }
}
