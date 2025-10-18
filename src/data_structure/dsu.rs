//! Disjoint Set Union (Union-Find)
//!
//! This data structure efficiently maintains a collection of disjoint set.
//! It supports two primary operations:
//!
//! - **find**: Determine which set an element belongs to.
//! - **union**: Merge two disjoint sets into one.

///  A *Disjoint Set Union* (DSU), also known as a *Union-Find* data structure.
///
///  Each element initially belongs to its own singleton set.
///  Sets can be merged (`union`) and queried for membership (`find` or `same`).
#[derive(Debug, Clone)]
pub struct DSU {
    parent: Box<[i32]>,
    num_sets: usize,
}

impl DSU {
    /// Creates a new DSU with `n` disjoint singleton sets.
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
            num_sets: n,
        }
    }

    /// Returns the representative (root) of the set containing `x`.
    ///
    /// ## Returns
    ///
    /// The index of the representative element of the set containing `x`.
    ///
    /// ## Complexity
    ///
    /// `O(α(n))`, where α is the inverse Ackermann function.
    #[inline(always)]
    pub fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] >= 0 {
            root = self.parent[root] as usize;
        }
        while self.parent[x] >= 0 {
            let parent = self.parent[x] as usize;
            self.parent[x] = root as i32;
            x = parent;
        }
        root
    }

    /// Returns `true` if `x` and `y` belong to the same set.
    ///
    /// ## Complexity
    ///
    /// `O(α(n))`, where α is the inverse Ackermann function.
    #[inline(always)]
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Merges the sets containing `x` and `y`.
    ///
    /// ## Returns
    ///
    /// - `true` if the sets were disjoint and are now merged.
    /// - `false` if `x` and `y` were already in the same set.
    ///
    /// ## Complexity
    ///
    /// `O(α(n))`, where α is the inverse Ackermann function.
    #[inline(always)]
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }

        if self.parent[x] > self.parent[y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent[x] += self.parent[y];
        self.parent[y] = x as i32;
        self.num_sets -= 1;
        true
    }

    /// Returns the size of the set containing `x`.
    ///
    /// ## Complexity
    ///
    /// `O(α(n))`, where α is the inverse Ackermann function.
    pub fn set_size(&mut self, x: usize) -> usize {
        let root = self.find(x) as usize;
        -self.parent[root] as usize
    }

    /// Returns the total number of disjoint sets.
    ///
    /// # Complexity
    ///
    /// `O(1)`
    pub fn num_sets(&self) -> usize {
        self.num_sets
    }

    /// Returns the total number of elements managed by this DSU.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// Returns `true` if the DSU contains no elements.
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}
