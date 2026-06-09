# Testing with pytest

Here is a class of error that appears in computational biology with unsettling frequency. You have a function that normalizes RNA-seq count data. It works correctly for all the inputs you've tried. You publish a paper using it. Then, six months later, you discover that when a sample has zero reads mapped — an edge case you never tested — the function silently produces NaN values rather than raising an error. Those NaN values propagated through your downstream analysis. The differential expression results were wrong. Not obviously wrong, not wrong in a way that triggered an error message. Just wrong.

**pytest** is the standard testing framework for Python scientific computing. Tests verify that code does what it claims to do, detect regressions when code changes, document expected behavior, and provide confidence for refactoring. For research code, testing is not optional: numerical bugs in analysis pipelines can silently corrupt results without producing errors, and a test suite is the only systematic guard against them.

## The Testing Philosophy

A **unit test** verifies one function in isolation. An **integration test** verifies that multiple components work together. For scientific code, tests should cover:
- **Correctness**: outputs match known analytical solutions or published reference values
- **Edge cases**: empty inputs, single-element arrays, NaN values, exact zeros
- **Error handling**: that invalid inputs raise the correct exceptions
- **Numerical stability**: that results are consistent within floating-point tolerance

pytest discovers tests automatically: any file named `test_*.py` or `*_test.py`, with functions named `test_*`, is collected and executed.

## Basic Test Structure

```python
# tests/test_preprocessing.py

import numpy as np
import pandas as pd
import pytest
from bioanalysis import normalize_counts, filter_low_expression, log_transform


# ── Basic correctness tests ────────────────────────────────────────────────

def test_cpm_normalization_sums_to_million():
    """CPM-normalized columns should sum to 1e6."""
    counts = np.array([
        [100, 200, 50],
        [400, 600, 950],
        [500, 200, 0],
    ], dtype=float)
    cpm = normalize_counts(counts, method="cpm")
    column_sums = cpm.sum(axis=0)
    np.testing.assert_allclose(column_sums, 1e6, rtol=1e-10)


def test_cpm_preserves_relative_proportions():
    """CPM normalization should not change relative abundance within a sample."""
    counts = np.array([[1.0], [2.0], [3.0]])
    cpm = normalize_counts(counts, method="cpm")
    ratios_before = counts[:, 0] / counts[:, 0].sum()
    ratios_after  = cpm[:, 0] / cpm[:, 0].sum()
    np.testing.assert_allclose(ratios_before, ratios_after, rtol=1e-10)


def test_cpm_preserves_dataframe_index():
    """DataFrames should retain gene names and sample names through normalization."""
    counts_df = pd.DataFrame(
        [[100, 200], [50, 100]],
        index=["GENE_A", "GENE_B"],
        columns=["sample1", "sample2"]
    )
    cpm_df = normalize_counts(counts_df, method="cpm")
    assert list(cpm_df.index)   == ["GENE_A", "GENE_B"]
    assert list(cpm_df.columns) == ["sample1", "sample2"]
    assert isinstance(cpm_df, pd.DataFrame)


def test_log_transform_pseudocount():
    """log2(0 + 1) should equal 0 with default pseudocount=1."""
    counts = np.array([[0.0, 0.0], [0.0, 0.0]])
    result = log_transform(counts, base=2, pseudocount=1.0)
    np.testing.assert_array_equal(result, 0.0)


def test_log_transform_known_value():
    """log2(3 + 1) = log2(4) = 2."""
    counts = np.array([[3.0]])
    result = log_transform(counts, base=2, pseudocount=1.0)
    np.testing.assert_allclose(result, [[2.0]], rtol=1e-10)
```

Notice the structure of each test: a docstring that states the invariant being tested, a minimal concrete example that exercises exactly that invariant, and an assertion that checks the result with appropriate tolerance. These tests are also documentation — they make the expected behavior of `normalize_counts` explicit in a form that can be automatically verified.

## Fixtures with conftest.py

**Fixtures** are reusable test dependencies — setup code that creates data, opens connections, or initializes objects that multiple tests share. Define them in `conftest.py`:

```python
# tests/conftest.py

import numpy as np
import pandas as pd
import pytest


@pytest.fixture
def small_count_matrix():
    """Small (5 genes × 4 samples) count matrix for quick tests."""
    rng = np.random.default_rng(seed=42)
    counts = rng.negative_binomial(n=10, p=0.1, size=(5, 4)).astype(float)
    return counts


@pytest.fixture
def count_dataframe():
    """Count matrix as a labeled DataFrame."""
    rng = np.random.default_rng(seed=0)
    data = rng.negative_binomial(n=20, p=0.2, size=(10, 6)).astype(float)
    return pd.DataFrame(
        data,
        index=[f"GENE_{i:03d}" for i in range(10)],
        columns=[f"sample_{j}" for j in range(6)]
    )


@pytest.fixture
def synthetic_network():
    """Small 20-node scale-free network for network analysis tests."""
    import networkx as nx
    G = nx.barabasi_albert_graph(20, 2, seed=123)
    # Add weights
    rng = np.random.default_rng(42)
    for u, v in G.edges():
        G[u][v]["weight"] = rng.uniform(0.5, 1.0)
    return G


@pytest.fixture(scope="session")
def large_count_matrix():
    """Large count matrix for performance tests (created once per session)."""
    rng = np.random.default_rng(999)
    return rng.negative_binomial(n=10, p=0.15, size=(20000, 100)).astype(float)
```

```python
# tests/test_preprocessing.py (continued, using fixtures)

def test_filter_low_expression_removes_zeros(small_count_matrix):
    """Genes that are zero everywhere should be removed."""
    # Zero out first gene
    small_count_matrix[0, :] = 0.0
    filtered, mask = filter_low_expression(small_count_matrix, min_cpm=1.0, min_samples=2)
    assert filtered.shape[0] == small_count_matrix.shape[0] - 1
    assert mask[0] == False


def test_filter_preserves_high_expression_genes(count_dataframe):
    """Genes above threshold in all samples should always be retained."""
    # Force one gene to be very highly expressed
    count_dataframe.iloc[0, :] = 10000
    filtered, mask = filter_low_expression(count_dataframe, min_cpm=1.0, min_samples=2)
    assert mask[0] == True  # high-expression gene always kept
```

## Parametrize Decorator

`@pytest.mark.parametrize` runs one test function with multiple input/expected pairs, avoiding code duplication:

```python
# tests/test_preprocessing.py — parametrized tests

@pytest.mark.parametrize("base,pseudocount,input_val,expected", [
    (2,    1.0,  1.0,  1.0),   # log2(1 + 1) = log2(2) = 1
    (2,    1.0,  3.0,  2.0),   # log2(3 + 1) = log2(4) = 2
    (2,    1.0,  7.0,  3.0),   # log2(7 + 1) = log2(8) = 3
    (10,   0.0, 10.0,  1.0),   # log10(10) = 1
    (np.e, 0.0, np.e,  1.0),   # ln(e) = 1
])
def test_log_transform_values(base, pseudocount, input_val, expected):
    """Verify log transform against known analytical values."""
    counts = np.array([[input_val]])
    result = log_transform(counts, base=base, pseudocount=pseudocount)
    np.testing.assert_allclose(result[0, 0], expected, rtol=1e-10)


@pytest.mark.parametrize("method", ["cpm"])
@pytest.mark.parametrize("n_genes,n_samples", [(10, 5), (100, 20), (5, 2)])
def test_normalization_output_shape(method, n_genes, n_samples):
    """Normalization should preserve shape."""
    counts = np.ones((n_genes, n_samples)) * 10
    result = normalize_counts(counts, method=method)
    assert result.shape == (n_genes, n_samples)
```

## Testing Error Conditions

```python
def test_normalize_unknown_method_raises():
    """Invalid normalization method should raise ValueError."""
    counts = np.ones((5, 3))
    with pytest.raises(ValueError, match="Unknown normalization method"):
        normalize_counts(counts, method="invalid_method")


def test_normalize_negative_counts_warns():
    """Negative counts should trigger a warning or error."""
    counts = np.array([[-1.0, 1.0], [1.0, 1.0]])
    with pytest.raises(ValueError):
        normalize_counts(counts, method="cpm")
```

## Numerical Tolerance Assertions

Scientific code involves floating-point arithmetic. Use appropriate tolerances:

```python
def test_pca_variance_sum(count_dataframe):
    """Principal components should collectively explain 100% of variance."""
    from bioanalysis.models import run_pca
    pcs, variance_explained = run_pca(count_dataframe, n_components=None)
    np.testing.assert_allclose(variance_explained.sum(), 1.0, atol=1e-6)
    # atol for absolute tolerance; rtol for relative tolerance


def test_correlation_symmetry(count_dataframe):
    """Gene-gene correlation matrix should be symmetric."""
    from bioanalysis.models import correlation_matrix
    C = correlation_matrix(count_dataframe)
    np.testing.assert_allclose(C, C.T, atol=1e-12)


def test_correlation_diagonal_ones(count_dataframe):
    """Correlation of a gene with itself should be exactly 1."""
    from bioanalysis.models import correlation_matrix
    C = correlation_matrix(count_dataframe)
    np.testing.assert_allclose(np.diag(C), 1.0, atol=1e-10)
```

## Running Tests

```bash
# Run all tests
pytest

# Run with verbose output
pytest -v

# Run only fast tests (excluding slow/integration)
pytest -m "not slow and not integration"

# Run with coverage report
pytest --cov=bioanalysis --cov-report=html

# Run tests in parallel (pytest-xdist)
pytest -n 4   # use 4 workers

# Run a specific test file or function
pytest tests/test_preprocessing.py::test_cpm_normalization_sums_to_million

# Show slowest tests
pytest --durations=10

# Stop after first failure (useful during development)
pytest -x
```

## Continuous Integration with GitHub Actions

```yaml
# .github/workflows/ci.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: ["3.10", "3.11", "3.12"]

    steps:
      - uses: actions/checkout@v4

      - name: Set up Python ${{ matrix.python-version }}
        uses: actions/setup-python@v5
        with:
          python-version: ${{ matrix.python-version }}

      - name: Install package and dev dependencies
        run: pip install -e ".[dev]"

      - name: Lint with ruff
        run: ruff check src/ tests/

      - name: Run tests with coverage
        run: pytest --cov=bioanalysis --cov-report=xml

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v4
        with:
          file: coverage.xml
```

## Why This Matters

Testing scientific code is both more important and more neglected than in software engineering. More important because scientific results based on buggy code can be published, cited, and acted upon before the error is discovered. More neglected because there is no customer to report a broken feature, no production system to crash, and no obvious incentive in the short term. The counter-argument is straightforward: a test suite pays for itself the first time it catches a regression — a subtle change in one function that breaks a downstream analysis. In computational biology, this happens constantly: updating a dependency, changing a normalization order, or fixing an edge case in one function can silently break another. Property-based testing with `hypothesis` (not shown here but available) can find such edge cases automatically by generating random inputs that violate invariants. A package with 80% test coverage is not paranoid bureaucracy; it is evidence that the code actually does what it claims.
