//! Tree Diameter
//!
//! Algorithms for computing the diameter of a tree.
//!
//! ## Definition
//!
//! The **diameter** of a tree is the length of the longest path between any two vertices in the
//! tree. More formally, for a tree `T = (V, E)`, the diameter is defined as:
//!
//! `diameter(T) = max { d(u, v) : u, v in V}`
//!
//! where `d(u, v)` is the distance (number of edges) between vertices `u` and `v`.

/// Compute the diameter of an unweighted tree.
///
/// ## Parameters
///
/// - `n`: The number of vertices in the tree (vertices are indexed `0..n`)
/// - `e`: A slice of undirected edges `(u, v)` representing an edge between `u` and `v`.
///
/// ## Panics
///
/// - Panics if `n != e.len() + 1`.
///
/// ## Complexity
///
/// `O(n)`
pub fn diameter(n: usize, e: &[(usize, usize)]) -> usize {
    assert_eq!(n, e.len() + 1);
    let mut graph = vec![vec![]; n];
    for &(u, v) in e {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dist = vec![!0usize; n];
    dist[0] = 0;
    let mut stack = vec![0];
    while let Some(i) = stack.pop() {
        for &j in &graph[i] {
            if dist[j] == !0 {
                dist[j] = dist[i] + 1;
                stack.push(j);
            }
        }
    }
    let (mut p, mut mx) = (0, 0);
    for (i, &d) in dist.iter().enumerate() {
        if d > mx {
            p = i;
            mx = d;
        }
    }

    dist.fill(!0);
    stack.push(p);
    dist[p] = 0;
    while let Some(i) = stack.pop() {
        for &j in &graph[i] {
            if dist[j] == !0 {
                dist[j] = dist[i] + 1;
                stack.push(j);
            }
        }
    }

    let mx = dist.iter().filter(|d| **d != !0).max().unwrap();
    let q = dist.iter().position(|d| d == mx).unwrap();
    dist[q]
}

/// Compute the diameter of a weighted tree.
///
/// ## Parameters
///
/// - `n`: The number of vertices in the tree (vertices are indexed `0..n`)
/// - `e`: A slice of undirected edges and its weight `(u, v, w)` representing an edge between `u`
///   and `v` with `w` weight.
///
/// ## Panics
///
/// - Panics if `n != e.len() + 1`.
///
/// ## Complexity
///
/// `O(n)`
pub fn weighted_diameter(n: usize, e: &[(usize, usize, u64)]) -> u64 {
    assert_eq!(n, e.len() + 1);
    let mut graph = vec![vec![]; n];
    for &(u, v, d) in e {
        graph[u].push((v, d));
        graph[v].push((u, d));
    }

    let mut dist = vec![!0; n];
    dist[0] = 0;
    let mut stack = vec![0];
    while let Some(i) = stack.pop() {
        for &(j, d) in &graph[i] {
            if dist[j] == !0 {
                dist[j] = dist[i] + d;
                stack.push(j);
            }
        }
    }
    let (mut p, mut mx) = (0, 0);
    for (i, &d) in dist.iter().enumerate() {
        if d > mx {
            p = i;
            mx = d;
        }
    }

    dist.fill(!0);
    stack.push(p);
    dist[p] = 0;
    while let Some(i) = stack.pop() {
        for &(j, d) in &graph[i] {
            if dist[j] == !0 {
                dist[j] = dist[i] + d;
                stack.push(j);
            }
        }
    }

    let mx = dist.iter().filter(|d| **d != !0).max().unwrap();
    let q = dist.iter().position(|d| d == mx).unwrap();
    dist[q]
}
