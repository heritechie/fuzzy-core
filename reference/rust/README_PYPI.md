<p align="center">
  <img src="https://raw.githubusercontent.com/heritechie/fuzzy-core/main/assets/logo.svg" width="88" />
</p>

# fuzzy-core

A fast, deterministic fuzzy string similarity library.

Implemented in Rust with Python bindings via PyO3.

## Installation

```bash
pip install fuzzy-core
```

## Usage

```python
import fuzzy_core

score = fuzzy_core.similarity("Hello, World!", "hello world")
print(score) # ~1.0
```
