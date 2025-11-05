//! Character Position Queries
//!
//! Efficient precomputation and query structures for character positions and frequencies in
//! strings.
//!
//! # Available functions
//!
//! - [`next_pos`]: Computes the next occurrence position.
//! - [`prev_pos`]: Computes the previous occurrence position.
//! - [`cumulative_count`]: Computes the cumulative count.

/// Computes the next occurrence position for each character from each index.
///
/// For each position `i` in the string and each character `c` in the range `[l, r)`, this function
/// computes the nearest index `j >= i` where `s[j] == c`.
///
/// ## Parameters
///
/// - `s`: The input string as a byte slice.
/// - `l`: The smallest character value to consider (inclusive)
/// - `r`: The largest character value to consider (exclusive)
///
/// ## Returns
///
/// A vector `next` where `next[i][c - l]` is the smallest index `j >= i` such that `s[j] == c`, or
/// `n` (the length of `s`) if no such index exists.
///
/// ## Complexity
///
/// `O(n * (r - l))`, where `n = s.len()`.
pub fn next_pos(s: &[u8], l: u8, r: u8) -> Vec<Vec<usize>> {
    let n = s.len();
    let mut res = Vec::with_capacity(n);
    let mut next = vec![n; (r - l) as usize];
    for (i, &c) in s.iter().enumerate().rev() {
        next[(c - l) as usize] = i;
        res.push(next.clone());
    }
    res.reverse();
    res
}

/// Computes the previous occurrence position for each character from each index.
///
/// For each position `i` in the string and each character `c` in the range `[l, r)`, this function
/// computes the largest index `j <= i` where `s[j] == c`.
///
/// ## Parameters
///
/// - `s`: The input string as a byte slice.
/// - `l`: The smallest character value to consider (inclusive)
/// - `r`: The largest character value to consider (exclusive)
///
/// ## Returns
///
/// A vector `prev` where `prev[i][c - l]` is the largest index `j <= i` such that `s[j] == c`, or
/// `n` (the length of `s`) if no such index exists.
///
/// ## Complexity
///
/// `O(n * (r - l))`, where `n = s.len()`.
pub fn prev_pos(s: &[u8], l: u8, r: u8) -> Vec<Vec<usize>> {
    let n = s.len();
    let mut res = Vec::with_capacity(n);
    let mut prev = vec![n; (r - l) as usize];
    for (i, &c) in s.iter().enumerate() {
        prev[(c - l) as usize] = i;
        res.push(prev.clone());
    }
    res
}

/// Computes the comulative count of each character up to each position.
///
/// For each position `i` in the string and each character `c` in the range `[l, r)`, this function
/// computes how many times `c` appears in the prefix `s[0..=i]`.
///
/// ## Parameters
///
/// - `s`: The input string as a byte slice.
/// - `l`: The smallest character value to consider (inclusive)
/// - `r`: The largest character value to consider (exclusive)
///
/// ## Returns
///
/// A vector `count` where `count[i][c - l]` is the number of occurrences of character in the
/// substring `s[0..=i]`.
///
/// ## Complexity
///
/// `O(n * (r - l))`, where `n = s.len()`.
pub fn cumulative_count(s: &[u8], l: u8, r: u8) -> Vec<Vec<usize>> {
    let n = s.len();
    let mut res = Vec::with_capacity(n);
    let mut count = vec![0; (r - l) as usize];
    for &c in s {
        count[(c - l) as usize] += 1;
        res.push(count.clone());
    }
    res
}
