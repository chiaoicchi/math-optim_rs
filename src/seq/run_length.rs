//! Run Length
//!
//! Algorithm for compressing and decompressing sequences using run-length encoding.
//!
//! # Definition
//!
//! **Run length encoding (RLE)** is lossless compression method that represents a sequence by
//! encoding consecutive identical elements as pairs `(value, count)`.
//!
//! For a sequence `a = [a0, a1, ..., an]`, its run-length encoding is a sequence
//! `r = [(v0, c0), (v1, c1), ..., (vm, cm)]` where:
//! - Each `vi` is a value from the original sequence
//! - Each `ci > 0` is the count of consecutive occurrences
//! - `vi != v(i+1)` for all `0 <= i < m` (consecutive runs have different values)
//! - The original sequence can be recovered by repeating each `vi` exactly `ci` times.
//!
//! This compression is bijection.

/// Encodes a sequence using run-length encoding.
///
/// ## Definition
///
/// Compresses a sequence by replacing consecutive runs of identical elements with pairs `(value,
/// count)`.
///
/// ## Parameters
///
/// - `a`: A slice of elements to encode.
///
/// ## Returns
///
/// A vector of pairs `(value, count)` where:
/// - `value` is an element from the input sequence
/// - `count` is the number of consecutive occurrences
/// - Consecutive pairs have different values
///
/// ## Complexity
///
/// `O(n)`, where `n` is the length of the input sequence.
pub fn run_length_encoding<T: Copy + PartialEq>(a: &[T]) -> Vec<(T, usize)> {
    let mut a = a.iter().copied().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}

/// Decodes a run-length encoded sequence.
///
/// For any sequence `s`, the following identity holds:
///
/// `run_length_decofing(&run_length_encoding(&s)) = s`
///
/// ## Parameters
///
/// - `a`: A slice of pairs `(value, count)` representing a run-length encoding.
///
/// ## Returns
///
/// The decoded sequence where each `value` is repeated `count` times.
///
/// ## Complexity
///
/// `O(n)`, where `n` is the total length of the decoded sequence.
pub fn run_length_decofing<T: Copy + PartialEq>(a: &[(T, usize)]) -> Vec<T> {
    a.iter()
        .map(|a| std::iter::repeat(a.0).take(a.1))
        .flatten()
        .collect()
}
