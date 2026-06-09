# Python Package Structure

Every computational biology lab has a graveyard of scripts. There's `normalize_v2_final.py` and `normalize_v2_final_REAL.py`. There's a function called `my_normalize` duplicated in seven different analysis scripts with subtle variations between them, none of which are documented. When a new lab member arrives, they spend their first week asking what each script does and which version to use. When a bug is found in the normalization logic, it needs to be fixed in seven places — and there's no guarantee someone finds all seven. The difference between a research project that produces a script graveyard and one that produces usable software is package structure.

A Python **package** is a directory of modules that can be installed, imported, and distributed. Writing reusable analysis code as a package — rather than a collection of scripts — enables importing functions across scripts, running automated tests, sharing with collaborators via `pip install`, and versioning the code independently of data.

## The `src` Layout

The modern recommended layout for Python packages uses a `src/` directory to separate installable package code from project-level files:

```
my_bioanalysis/
├── src/
│   └── bioanalysis/           # the actual Python package
│       ├── __init__.py        # package init; defines public API
│       ├── io.py              # file I/O functions
│       ├── preprocessing.py   # data normalization/QC
│       ├── models.py          # statistical/ML models
│       ├── visualization.py   # plotting utilities
│       └── utils.py           # shared helper functions
│
├── tests/                     # test suite
│   ├── conftest.py            # pytest fixtures
│   ├── test_io.py
│   ├── test_preprocessing.py
│   └── test_models.py
│
├── notebooks/                 # exploratory Jupyter notebooks
│   └── 01_exploratory_analysis.ipynb
│
├── data/                      # small reference data (not large datasets)
│   └── gene_sets/
│       └── hallmark_genesets.gmt
│
├── docs/                      # documentation source
│   └── api.rst
│
├── pyproject.toml             # build system + project metadata (PEP 517/518)
├── README.md
├── LICENSE
└── .github/
    └── workflows/
        └── ci.yml             # GitHub Actions CI/CD
```

The `src/` layout ensures that tests always run against the installed package, not a local directory that happens to be on `sys.path` — preventing a class of subtle bugs where tests pass locally but fail after installation.

## The `__init__.py` File

The `__init__.py` defines what users see when they import the package:

```python
# src/bioanalysis/__init__.py

"""
bioanalysis: A toolkit for computational systems biology analysis.

Provides:
  - RNA-seq normalization and QC
  - Network construction and analysis
  - Statistical models for gene expression
"""

__version__ = "0.3.1"
__author__ = "Research Lab"

# Expose the most-used functions at the top level
# so users can write `from bioanalysis import normalize` 
# instead of `from bioanalysis.preprocessing import normalize`

from bioanalysis.preprocessing import (
    normalize_counts,
    filter_low_expression,
    log_transform
)

from bioanalysis.io import (
    load_count_matrix,
    load_gene_annotation,
    save_results
)

# Submodule imports that users access as bioanalysis.models.X
from bioanalysis import models
from bioanalysis import visualization

# Define __all__ for explicit public API documentation
__all__ = [
    "normalize_counts",
    "filter_low_expression",
    "log_transform",
    "load_count_matrix",
    "load_gene_annotation",
    "save_results",
    "models",
    "visualization",
    "__version__",
]
```

## Module Design: `preprocessing.py`

```python
# src/bioanalysis/preprocessing.py

"""
RNA-seq count normalization and preprocessing utilities.

All functions accept numpy arrays or pandas DataFrames with genes as rows,
samples as columns. Functions return the same type as input.
"""

from __future__ import annotations

import logging
from typing import Union

import numpy as np
import pandas as pd

logger = logging.getLogger(__name__)

# Type alias for annotated signatures
CountMatrix = Union[np.ndarray, pd.DataFrame]


def normalize_counts(
    counts: CountMatrix,
    method: str = "cpm",
    pseudocount: float = 0.0,
) -> CountMatrix:
    """
    Normalize raw count matrix.

    Parameters
    ----------
    counts : array-like, shape (n_genes, n_samples)
        Raw integer counts. Genes as rows, samples as columns.
    method : {'cpm', 'tpm', 'tmm'}
        Normalization method.
        - 'cpm': counts per million
        - 'tpm': transcripts per million (requires 'lengths' argument via **kwargs)
    pseudocount : float, optional
        Value added before log transformation (default 0). Use 1 for log2(CPM+1).

    Returns
    -------
    CountMatrix
        Normalized counts, same type as input.

    Examples
    --------
    >>> import numpy as np
    >>> counts = np.array([[100, 200], [50, 100], [0, 0]])
    >>> cpm = normalize_counts(counts, method='cpm')
    >>> cpm.sum(axis=0)  # should be [1e6, 1e6]
    array([1000000., 1000000.])
    """
    is_dataframe = isinstance(counts, pd.DataFrame)
    X = counts.values if is_dataframe else np.array(counts, dtype=float)

    if method == "cpm":
        library_sizes = X.sum(axis=0)
        if np.any(library_sizes == 0):
            logger.warning("Found samples with zero library size; these will produce NaN")
        normalized = (X + pseudocount) / library_sizes[np.newaxis, :] * 1e6

    elif method == "tpm":
        raise NotImplementedError("TPM requires gene lengths; use normalize_tpm() instead")

    else:
        raise ValueError(f"Unknown normalization method: {method!r}. Choose 'cpm' or 'tpm'.")

    if is_dataframe:
        return pd.DataFrame(normalized, index=counts.index, columns=counts.columns)
    return normalized


def filter_low_expression(
    counts: CountMatrix,
    min_cpm: float = 1.0,
    min_samples: int = 2,
) -> tuple[CountMatrix, np.ndarray]:
    """
    Remove lowly expressed genes.

    Parameters
    ----------
    counts : CountMatrix
        Raw counts (genes × samples).
    min_cpm : float
        Minimum CPM threshold.
    min_samples : int
        Gene must exceed min_cpm in at least this many samples.

    Returns
    -------
    filtered_counts : CountMatrix
        Counts with low-expression genes removed.
    keep_mask : np.ndarray of bool
        Boolean mask of kept genes, shape (n_genes,).
    """
    cpm_matrix = normalize_counts(counts, method="cpm")
    X_cpm = cpm_matrix.values if isinstance(cpm_matrix, pd.DataFrame) else cpm_matrix

    keep_mask = (X_cpm >= min_cpm).sum(axis=1) >= min_samples

    n_removed = (~keep_mask).sum()
    logger.info(f"filter_low_expression: removed {n_removed} genes, kept {keep_mask.sum()}")

    if isinstance(counts, pd.DataFrame):
        return counts.loc[keep_mask], keep_mask.values
    return counts[keep_mask], keep_mask


def log_transform(
    counts: CountMatrix,
    base: float = 2.0,
    pseudocount: float = 1.0,
) -> CountMatrix:
    """
    Apply log transformation: log_base(counts + pseudocount).

    Parameters
    ----------
    base : float
        Log base (2 for log2, natural base for ln).
    pseudocount : float
        Value added before taking log to avoid log(0).
    """
    is_dataframe = isinstance(counts, pd.DataFrame)
    X = counts.values if is_dataframe else np.array(counts, dtype=float)

    result = np.log(X + pseudocount) / np.log(base)

    if is_dataframe:
        return pd.DataFrame(result, index=counts.index, columns=counts.columns)
    return result
```

## Internal Imports and Utilities

```python
# src/bioanalysis/utils.py

"""
Shared utilities. Import these from other modules, not directly by users.
"""

import functools
import time
import logging

logger = logging.getLogger(__name__)


def timer(func):
    """Decorator that logs function execution time."""
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        start = time.perf_counter()
        result = func(*args, **kwargs)
        elapsed = time.perf_counter() - start
        logger.debug(f"{func.__qualname__} took {elapsed:.3f}s")
        return result
    return wrapper


def check_array(X, n_dims=2, allow_nan=False, name="input"):
    """Validate array shape and content."""
    import numpy as np
    X = np.asarray(X)
    if X.ndim != n_dims:
        raise ValueError(f"{name} must be {n_dims}D, got shape {X.shape}")
    if not allow_nan and np.any(np.isnan(X)):
        raise ValueError(f"{name} contains NaN values")
    if np.any(X < 0):
        raise ValueError(f"{name} contains negative values (expected non-negative counts)")
    return X
```

## Installation in Development Mode

With `pyproject.toml` defined (see next subsection):

```bash
# Install in editable mode (changes to src/ are immediately reflected)
pip install -e ".[dev]"

# Verify installation
python -c "import bioanalysis; print(bioanalysis.__version__)"

# The package is now importable from anywhere:
python -c "from bioanalysis import normalize_counts"
```

## Why This Matters

Package structure is not bureaucracy — it is the difference between analysis code that can be tested, shared, and built upon versus a collection of scripts that only work in the exact directory where they were written. The `src` layout prevents the common error of accidentally importing from the local directory instead of the installed package. Well-designed `__init__.py` files create a stable public API: collaborators can use `from bioanalysis import normalize_counts` without knowing the internal module structure, and you can refactor internals without breaking their code. Type annotations and docstrings with `Parameters/Returns/Examples` sections enable automatic documentation generation with Sphinx and serve as a first line of documentation for collaborators. The habits formed writing structured Python packages — modular functions, clear interfaces, explicit dependencies — are the same habits that make research code trustworthy.
