//! Rust reference implementation for fuzzy-core.

pub mod normalize;
pub mod levenshtein;
pub mod score;
mod python;

/// Compute fuzzy similarity score between two strings.
///
/// This is the **public API** of fuzzy-core.
///
/// Behavior:
/// - input strings are normalized
/// - Levenshtein distance is computed
/// - score is normalized to range [0.0, 1.0]
pub fn similarity(a: &str, b: &str) -> f64 {
    let na = normalize::normalize(a);
    let nb = normalize::normalize(b);
    score::similarity(&na, &nb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn similarity_identical_strings() {
        let s = similarity("hello", "hello");
        assert_eq!(s, 1.0);
    }

    #[test]
    fn similarity_case_insensitive() {
        let s = similarity("Hello", "hello");
        assert!(s > 0.99);
    }

    #[test]
    fn similarity_with_punctuation() {
        let s = similarity("Hello, World!", "hello world");
        assert!(s > 0.9);
    }

    #[test]
    fn similarity_empty_strings() {
        let s = similarity("", "");
        assert_eq!(s, 1.0);
    }
}
