/// Compute Levenshtein distance between two strings.
///
/// This implementation is:
/// - deterministic
/// - allocation-bounded
/// - safe (no unsafe code)
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let len_a = a_chars.len();
    let len_b = b_chars.len();

    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }

    let mut prev: Vec<usize> = (0..=len_b).collect();
    let mut curr = vec![0; len_b + 1];

    for i in 0..len_a {
        curr[0] = i + 1;

        for j in 0..len_b {
            let cost = if a_chars[i] == b_chars[j] { 0 } else { 1 };

            curr[j + 1] = std::cmp::min(
                std::cmp::min(
                    curr[j] + 1,       // insertion
                    prev[j + 1] + 1,   // deletion
                ),
                prev[j] + cost,       // substitution
            );
        }

        std::mem::swap(&mut prev, &mut curr);
    }

    prev[len_b]
}

#[cfg(test)]
mod tests {
    use super::*;

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
        use crate::normalize::normalize; // akses normalize()

        let a = normalize("Hello, World!");
        let b = normalize("hello world");
        assert_eq!(levenshtein(&a, &b), 0);
    }
}