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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn similarity_basic() {
        let s = similarity("ab", "ac");
        assert!(s > 0.4 && s < 0.6);
    }

    #[test]
    fn similarity_identical() {
        let s = similarity("test", "test");
        assert_eq!(s, 1.0);
    }

    #[test]
    fn similarity_empty() {
        let s = similarity("", "");
        assert_eq!(s, 1.0);
    }
}
