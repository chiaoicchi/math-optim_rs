//! Potential Disjoint Set Union (Union-Find)
//!
//! A data structure that efficiently maintains a collection of disjoint sets with potential
//! functions.
//!
//! # Definition
//!
//! A potential DSU manages `n` elements (indexed `0..n`) partitioned into disjoint sets.
//! Each element `x` has a **potential value** `p[x]` relative to the representative (root) of its
//! set.
//!
//! For elements `x` and `y` in the same set with root `r`:
//! - `p[x]`: potential from `r` to `x`
//! - `p[y]`: potential from `r` to `y`
//! - The potential difference from `x` to `y` is: `p[x]^(-1) * p[y]`
//!
//! The potentials from a group structure `(G, *, e)`, allowing both commutative and
//! non-commutative operations.

use crate::algebra::group::Group;

/// A *Potential DSU*, also known as a *Potential Union-Find* data structure.
///
/// Manages a collection of disjoint sets where each element has a potential value relative to the
/// root of its set. The potential values from a group structure.
///
/// ## Type Parameters
///
/// - `T`: A type implementing the [`Group`] trait. Group must not be commutative.
#[derive(Debug, Clone)]
pub struct PotentialDSU<T: Group> {
    /// Parent array: negative values indicate root with set size (stored as `-size`),
    /// non-negative values indicate parent index.
    parent: Box<[i32]>,

    /// Potential from parent to this node using the group operation.
    /// For root nodes, this value is not used.
    potential: Box<[T::G]>,

    /// Number of disjoint sets.
    num_sets: usize,
}

impl<T: Group> PotentialDSU<T>
where
    T::G: PartialEq,
{
    /// Creates a new potential DSU with `n` disjoint singleton sets.
    ///
    /// ## Parameters
    ///
    /// - `n`: Number of elements (indexed `0..n`).
    ///
    /// ## Complexity
    ///
    /// `O(n)`
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![-1; n].into_boxed_slice(),
            potential: vec![T::identity(); n].into_boxed_slice(),
            num_sets: n,
        }
    }

    /// Finds the representative (root) of the set contianing `x` and computes the potential from the root to `x`.
    ///
    /// ## Parameters
    ///
    /// - `x`: Index of the element (must satisfy `0 <= x < n`).
    ///
    /// ## Returns
    ///
    /// A tuple `(root, potential)` where:
    ///
    /// - `root`: The index of the representative element of the set contianing `x`.
    /// - `potential`: The potential value from root to `x`.
    ///
    /// ## Definition
    ///
    /// If the path from `x` to root is `x -> p1 -> p2 -> ... -> pm -> root`, then:
    ///
    /// `potential = potential[root] * potential[pm] * ... * potential[p1] -> potential[x]`,
    ///
    /// where `*` denotes the group operation [`Group::op`].
    ///
    /// ## Complexity
    ///
    /// `O(log n)`
    #[inline]
    pub fn find(&self, mut x: usize) -> (usize, T::G) {
        let mut potential = self.potential[x].clone();
        while self.parent[x] >= 0 {
            x = self.parent[x] as usize;
            potential = T::op(&self.potential[x], &potential);
        }
        (x, potential)
    }

    /// Returns the potential difference from `from` to `to` if they are in the same set.
    ///
    /// ## Returns
    ///
    /// A optional value
    ///
    /// - `Some(p)`: potential from `from` to `to`, if they are in the same set, where `p`
    ///   satisfies: `potential(to) = potential(from) * p`, equivalently
    ///   `p = potential(from)^(-1) * potential(to)`.
    ///
    /// - `None`: `from` and `to` do not belong to the same set.
    ///
    /// ## Complexity
    ///
    /// `O(log n)`
    #[inline]
    pub fn potential(&self, from: usize, to: usize) -> Option<T::G> {
        let (from, potential_from) = self.find(from);
        let (to, potential_to) = self.find(to);
        if from == to {
            Some(T::op(&T::inv(&potential_from), &potential_to))
        } else {
            None
        }
    }

    /// Merges the set containing `from` and `to` with the constraint that the potential difference
    /// from `from` to `to` equals `p`.
    ///
    /// ## Parameters
    ///
    /// - `from`: Index of the source element.
    /// - `to`: Index of the target element.
    /// - `p`: The required potential difference from `from` to `to`.
    ///
    /// ## Returns
    ///
    /// - `true`: If the constraint is consistent and the merge succeeded.
    /// - `false`: If `from` and `to` are alredy in the same set but the constraint  is
    /// inconsistent with the existing potential difference. No merge is performed.
    ///
    /// ## Complexity
    ///
    /// `O(log n)`
    #[inline]
    pub fn union(&mut self, from: usize, to: usize, p: &T::G) -> bool {
        let (mut from, potential_from) = self.find(from);
        let (mut to, potential_to) = self.find(to);

        if from == to {
            T::op(&potential_from, p) == potential_to
        } else {
            let mut p = T::div(&T::op(&potential_from, p), &potential_to);
            if self.parent[from] > self.parent[to] {
                std::mem::swap(&mut from, &mut to);
                p = T::inv(&p);
            }
            self.parent[from] += self.parent[to];
            self.parent[to] = from as i32;
            self.potential[to] = p;
            self.num_sets -= 1;
            true
        }
    }

    /// Returns the size of the set containing `x`.
    ///
    /// ## Complexity
    ///
    /// `O(log n)`
    pub fn set_size(&self, x: usize) -> usize {
        -self.parent[self.find(x).0] as usize
    }

    /// Returns the total number of disjoint sets.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    pub fn num_sets(&self) -> usize {
        self.num_sets
    }

    /// Returns the total number of elements managed by this potential DSU.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// Returns `true` if the potential DSU contains no elements.
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}
