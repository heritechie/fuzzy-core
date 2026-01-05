import fuzzy_core


def test_import():
    assert hasattr(fuzzy_core, "similarity")


def test_similarity_identical():
    s = fuzzy_core.similarity("hello", "hello")
    assert s == 1.0


def test_similarity_case_insensitive():
    s = fuzzy_core.similarity("Hello", "hello")
    assert s > 0.99


def test_similarity_with_punctuation():
    s = fuzzy_core.similarity("Hello, World!", "hello world")
    assert s > 0.9


def test_similarity_simple_difference():
    s = fuzzy_core.similarity("ab", "ac")
    assert 0.4 < s < 0.6


def test_similarity_empty_strings():
    s = fuzzy_core.similarity("", "")
    assert s == 1.0
