//! Strongly Connected Components (SCC)
//!
//! Algorithms for computing strongly connected components in direced graphs.
//!
//! ## Definition
//!
//! A directed graph is called **strongly connected** if there is a path in each direction between
//! each pair of vertices of the graph.
//!
//! The binary relation of being strongly connected is an equivalence relation, and the induced
//! subgraphs of its equivalence classes are called **strongly connected components**.
//! Equivalently, a strongly connected components of a directed graph `G` is a subgraph that is
//! strongly connected, and is maximal with this property:
//! no set of additional edges or vertices from `G` can be included in the subgraph without
//! breaking its property of being strongly connected.

/// Computes strongly connected components using Kosaraju's algorithm.
///
/// ## Parameters
///
/// - `n`: The number of vertices in the graph (vertices are indexed `0..n`)
/// - `e`: A slice of directed edges `(u, v)` representing an edge from `u` to `v`.
///
/// ## Returns
///
/// A vector of strongly connected components `[component_0, component_1, ..., component_k]` where:
/// - Each `component_i` is a vector of vertex indices belonging to the i-th strongly connected component.
/// - Components are ordered in **topological order** of the condensation graph.
/// - If there is an edge from `component_i` to `component_j` in the condensation graph,
///   then `i < j`.
///
/// ## Complexity
///
/// `O(n + |e|)` where `|e|` is the number of edges.
pub fn kosaraju(n: usize, e: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n];
    let mut rev_graph = vec![vec![]; n];
    for &(u, v) in e {
        graph[u].push(v);
        rev_graph[v].push(u);
    }

    let mut order = Vec::with_capacity(n);
    let mut used = vec![false; n];

    for i in 0..n {
        if !used[i] {
            _kosaraju_dfs_finish(i, &graph, &mut used, &mut order);
        }
    }

    let mut scc = vec![];
    used.fill(false);

    for &i in order.iter().rev() {
        if !used[i] {
            let mut component = vec![];
            _kosaraju_dfs_collect(i, &rev_graph, &mut used, &mut component);
            scc.push(component);
        }
    }

    scc
}

fn _kosaraju_dfs_finish(i: usize, graph: &[Vec<usize>], used: &mut [bool], order: &mut Vec<usize>) {
    used[i] = true;
    for &j in &graph[i] {
        if !used[j] {
            _kosaraju_dfs_finish(j, graph, used, order);
        }
    }
    order.push(i);
}

fn _kosaraju_dfs_collect(
    i: usize,
    rev_graph: &[Vec<usize>],
    used: &mut [bool],
    component: &mut Vec<usize>,
) {
    used[i] = true;
    component.push(i);
    for &j in &rev_graph[i] {
        if !used[j] {
            _kosaraju_dfs_collect(j, rev_graph, used, component);
        }
    }
}
