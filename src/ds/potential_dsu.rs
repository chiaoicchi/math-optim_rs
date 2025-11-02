//! Potential Disjoint Set Union (Union-Find)
//!
//! This data structure efficiently maintains a collection of disjoint set and its potential.
//! It supports the following primary operations:
//!
//! - **find**: Determine which set an element belongs to and its potential from representative (root).
//! - **union**: Define potential between two set.
//! - **potential**: Calculate potential between two vertex.

use crate::algebra::group::Group;

/// A *Potential DSU*, also known as a *Potential Union-Find* data structure.
///
/// Each element initially belongs to its own singleton set.
/// Sets can be merged (`union`) and queried for membership and its potential (`find`, `potential`)
///
/// ## Type Parameters
///
/// - `T`: A type implementing the [`Group`] trait. Group must not be commutative.
#[derive(Debug, Clone)]
pub struct PotentialDSU<T: Group> {
    parent: Box<[i32]>,
    potential: Box<[T::G]>,
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

    /// Returns the representative (root) of the set contianing `x` and potential from `root` to `x`.
    ///
    /// ## Returns
    ///
    /// A tuple `(root, potential)`
    ///
    /// - `root`: The index of the representative element of the set contianing `x`.
    /// - `potential`: Potential from `root` to `x`.
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

    /// If `from` and `to` belong to the same set returns potential from `from` to `to`, otherwise
    /// `None`.
    ///
    /// ## Returns
    ///
    /// A optional value
    ///
    /// - `Some(potential)`: potential from `from` to `to`.
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

    /// Merges the set containing `from` and `to` as `potential[to] - potential[from] == p`.
    ///
    /// ## Returns
    ///
    /// - `true` if it is well-defined to `potential[to] - potential[from] = p`
    /// - `false` if already `potential[to] - potential[from] != p` and abort this operation.
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
