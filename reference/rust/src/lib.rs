//! Rust reference implementation for fuzzy-core.

pub mod normalize;
pub mod levenshtein;
pub mod score;

pub fn version() -> &'static str {
    "0.1.0"
}

#[cfg(test)]
mod tests {
    use super::levenshtein::levenshtein;
    use super::normalize::normalize;

    #[test]
    fn levenshtein_basic() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
    }

    #[test]
    fn levenshtein_empty() {
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", ""), 3);
    }

    #[test]
    fn levenshtein_normalized_input() {
        let a = normalize("Hello, World!");
        let b = normalize("hello world");
        assert_eq!(levenshtein(&a, &b), 0);
    }

    #[test]
    fn similarity_basic() {
        use super::score::similarity;

        let s = similarity("ab", "ac");
        assert!(s > 0.4 && s < 0.6);
    }

    #[test]
    fn similarity_identical() {
        use super::score::similarity;

        let s = similarity("test", "test");
        assert_eq!(s, 1.0);
    }

    #[test]
    fn similarity_empty() {
        use super::score::similarity;

        let s = similarity("", "");
        assert_eq!(s, 1.0);
    }
}
