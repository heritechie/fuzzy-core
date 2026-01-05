//! Rust reference implementation for fuzzy-core.

pub mod normalize;
pub mod levenshtein;

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
}
