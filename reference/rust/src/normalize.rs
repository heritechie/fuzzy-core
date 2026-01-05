/// Normalize input string according to fuzzy-core spec.
///
/// Rules:
/// - lowercase
/// - remove punctuation
/// - collapse whitespace
pub fn normalize(input: &str) -> String {
    let mut out = String::new();
    let mut prev_was_space = false;

    for ch in input.chars() {
        let ch = ch.to_ascii_lowercase();

        if ch.is_alphanumeric() {
            out.push(ch);
            prev_was_space = false;
        } else if ch.is_whitespace() {
            if !prev_was_space {
                out.push(' ');
                prev_was_space = true;
            }
        }
        // punctuation is ignored
    }

    out.trim().to_string()
}
