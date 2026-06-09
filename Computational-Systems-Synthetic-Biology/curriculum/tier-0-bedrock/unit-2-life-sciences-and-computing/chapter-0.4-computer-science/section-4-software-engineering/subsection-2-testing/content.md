# Testing

The Retraction Watch database is not pleasant reading. Among the retractions for computational biology are cases of wrong coordinate systems, sign errors in log-likelihoods, off-by-one errors in reading frame calculations, normalization bugs that reversed the direction of all fold changes. What is striking about these cases is not that they happened — complex code has bugs — but that they published. They passed peer review, the analytical pipeline ran to completion, figures were made, conclusions were drawn, and only later did someone notice that the numbers were wrong. Systematic testing is the only reliable way to catch these errors before they propagate into the scientific record.

Bioinformatics code processes biological data that is often too large to manually inspect, passes through many processing steps, and produces numerical outputs that are difficult to verify by eye. Bugs in bioinformatics code can be subtle — wrong coordinate system (0-based vs. 1-based), off-by-one errors in sliding window calculations, silent failures in error handling — and often go undetected until they affect a key result. Systematic testing is the only way to develop confidence that code is correct. It is also the infrastructure that makes refactoring safe: change the implementation, run the tests, and know immediately if you broke something.

## pytest: The Standard Testing Framework

**pytest** discovers and runs tests with minimal boilerplate. Any function named `test_*` in a file named `test_*.py` is automatically discovered.

```bash
# Install pytest
pip install pytest pytest-cov

# Run all tests
pytest tests/

# Run with coverage report
pytest tests/ --cov=mypackage --cov-report=term-missing

# Run a specific test
pytest tests/test_alignment.py::test_smith_waterman_exact_match

# Run tests matching a keyword
pytest -k "kmer"
```

## Unit Testing: Testing Functions in Isolation

**Unit tests** test a single function with known inputs and expected outputs. They should be:
- **Fast**: complete in milliseconds; if a test needs to load a 1 GB file, it is not a unit test
- **Isolated**: no side effects; no network calls; no shared state
- **Deterministic**: same result every run
- **Complete**: cover normal cases, edge cases, and expected error conditions

```python
# tests/test_sequence.py
import pytest
from mypackage.sequence import gc_content, count_kmers, reverse_complement

class TestGcContent:
    def test_all_gc(self):
        assert gc_content("GCGC") == 1.0

    def test_no_gc(self):
        assert gc_content("ATAT") == 0.0

    def test_mixed(self):
        assert gc_content("ACGT") == 0.5

    def test_empty_string(self):
        assert gc_content("") == 0.0

    def test_lowercase(self):
        # Define expected behavior for lowercase input
        assert gc_content("gcgc") == pytest.approx(1.0)

    def test_invalid_characters(self):
        # Should raise ValueError or handle gracefully — test whichever is specified
        with pytest.raises(ValueError):
            gc_content("ACGN!!")

class TestKmerCounting:
    def test_known_sequence(self):
        seq = "ACGACG"
        counts = count_kmers(seq, k=3)
        assert counts["ACG"] == 2
        assert counts["CGA"] == 1
        assert counts["GAC"] == 1

    def test_k_larger_than_sequence(self):
        assert count_kmers("AC", k=5) == {}

    def test_single_kmer(self):
        assert count_kmers("AAAA", k=4) == {"AAAA": 1}
```

**pytest fixtures** provide shared setup code (reuse without code duplication):

```python
@pytest.fixture
def sample_alignment():
    """Return a pre-built alignment for use in multiple tests."""
    return {
        "seq1": "ACGT-CGT",
        "seq2": "ACG-ACGT",
        "score": 12
    }

def test_alignment_score(sample_alignment):
    assert sample_alignment["score"] == 12

def test_alignment_length(sample_alignment):
    assert len(sample_alignment["seq1"]) == len(sample_alignment["seq2"])
```

## Regression Tests: Preventing Re-Introduction of Known Bugs

Once a bug is fixed, write a test that captures the exact case that failed. This is a **regression test** — it ensures the bug cannot silently return. The discipline here is important: when you find a bug, the first thing you do is write a test that fails because of the bug. Then you fix the bug. Then you verify that the test passes. The test now lives in your test suite permanently, protecting against recurrence.

```python
# Bug: GC content calculation was dividing by length including N characters
# Reported 2024-03-15; fixed by filtering N before division

def test_gc_content_regression_n_characters():
    """Regression test: GC content should not count N in denominator."""
    # Sequence "ACGN" should give GC = (1+1)/(3) = 0.667, not (1+1)/(4) = 0.5
    result = gc_content("ACGN")
    assert result == pytest.approx(2/3)
```

## Property-Based Testing: Hypothesis

**Property-based testing** (with the `hypothesis` library) generates hundreds of random inputs and checks that specified properties hold. This finds edge cases that hand-written tests miss.

```python
from hypothesis import given, settings
from hypothesis import strategies as st
from mypackage.sequence import reverse_complement

# Property: reverse complement is an involution (applying it twice gives the original)
@given(seq=st.text(alphabet="ACGT", min_size=1, max_size=1000))
def test_reverse_complement_involution(seq: str):
    assert reverse_complement(reverse_complement(seq)) == seq

# Property: GC content is symmetric (same for seq and reverse complement)
@given(seq=st.text(alphabet="ACGT", min_size=1, max_size=1000))
def test_gc_content_symmetric(seq: str):
    assert gc_content(seq) == pytest.approx(gc_content(reverse_complement(seq)))

# Property: total k-mer count equals len(seq) - k + 1
@given(
    seq=st.text(alphabet="ACGT", min_size=1, max_size=100),
    k=st.integers(min_value=1, max_value=10)
)
def test_kmer_count_total(seq: str, k: int):
    if k <= len(seq):
        total = sum(count_kmers(seq, k).values())
        assert total == len(seq) - k + 1
```

Hypothesis will automatically find the minimal failing example (shrinking) when a property is violated. This is particularly valuable for sequence algorithms: rather than writing tests for sequences of length 1, 2, 10, and 100, hypothesis generates hundreds of sequences automatically and tells you exactly which minimal sequence broke your invariant.

## Numerical Testing: pytest.approx and Tolerance

For floating-point computations (alignment scores, log-likelihoods, ODE solutions), exact equality is wrong:

```python
import numpy as np

def test_monod_growth_rate():
    mu_max, Ks, S = 1.0, 0.1, 0.5
    expected = 1.0 * 0.5 / (0.1 + 0.5)  # = 0.8333...
    result = monod_growth_rate(mu_max, Ks, S)
    assert result == pytest.approx(expected, rel=1e-6)  # 1 ppm relative tolerance

def test_ode_steady_state():
    """ODE solution should reach steady state within 0.1% of analytical value."""
    t, y = solve_lac_operon(t_end=200)
    y_ss_analytical = 2.88  # molecules per cell (analytical solution)
    assert y[-1] == pytest.approx(y_ss_analytical, rel=0.001)
```

## Test Coverage

**Coverage** measures what fraction of code lines are executed during tests. 100% coverage does not mean all bugs are caught — you can cover every line without covering meaningful edge cases — but low coverage means known untested code:

```bash
pytest --cov=mypackage --cov-report=html tests/
# Open htmlcov/index.html to see which lines are not covered
```

A practical target: >80% coverage for library code; critical algorithmic functions deserve 100% coverage.

## Why This Matters for Computational Biology

The Retraction Watch database lists dozens of papers retracted due to software bugs in bioinformatics analyses. Several high-profile cases involved alignment coordinate errors, sign errors in log-likelihoods, and off-by-one errors in reading frame calculations — all of which would have been caught by systematic testing. The scipy and NumPy projects have extensive test suites because scientific code must be correct to the precision users depend on. When you build a tool that others will use to make biological conclusions, testing is how you take responsibility for those conclusions. Test-driven development (writing tests before implementation) is particularly effective for algorithmic code: the specification of expected input-output behavior drives both correctness and clarity of design.
