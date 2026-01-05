//! Rust reference implementation for fuzzy-core.

pub mod normalize;

pub fn version() -> &'static str {
    "0.1.0"
}

#[cfg(test)]
mod tests {
    use super::normalize::normalize;

    #[test]
    fn normalize_basic() {
        let s = normalize(" Hello,   World! ");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn normalize_punctuation() {
        let s = normalize("foo-bar_baz");
        assert_eq!(s, "foobarbaz");
    }
}
