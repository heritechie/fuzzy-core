use crate::levenshtein::levenshtein;

/// Compute normalized similarity score using Levenshtein distance.
///
/// Score range: [0.0, 1.0]
pub fn similarity(a: &str, b: &str) -> f64 {
    let dist = levenshtein(a, b);
    let max_len = a.chars().count().max(b.chars().count());

    if max_len == 0 {
        return 1.0;
    }

    let score = 1.0 - (dist as f64 / max_len as f64);
    score.max(0.0)
}
