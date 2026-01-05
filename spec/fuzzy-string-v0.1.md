# fuzzy-core specification v0.1

## Normalization

- Lowercase
- Trim whitespace
- Collapse multiple spaces
- Remove punctuation characters

## Similarity Metric

### Normalized Levenshtein

score = 1 - (distance / max(len(a), len(b)))

## Output

- Score range: [0.0, 1.0]
- Deterministic
