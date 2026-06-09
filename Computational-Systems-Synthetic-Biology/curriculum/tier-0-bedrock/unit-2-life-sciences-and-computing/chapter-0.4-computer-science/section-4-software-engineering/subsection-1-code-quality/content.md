# Code Quality

In 2016, a paper in *Genome Biology* reported that roughly 20% of genomics papers with supplementary gene lists had errors introduced by Excel — gene names like SEPT2 (Septin 2) and MARCH1 (Membrane Associated Ring-CH-Type Finger 1) had been automatically converted to dates: September 2, March 1. The genes were simply renamed. This is an extreme example of a general problem: scientific code that cannot be reviewed, cannot be tested, and cannot be read by anyone except its author tends to contain errors that propagate into published results.

Bioinformatics has a reproducibility problem. A 2018 survey found that 70% of researchers could not reproduce another scientist's results, and computational studies are particularly affected — undocumented code, hard-coded paths, inconsistent analysis choices, and copy-paste programming compound to make published analyses impossible to validate or reuse. Code quality is not an aesthetic preference; it is a scientific requirement. Code that is unreadable cannot be reviewed for correctness. Code that is unreusable will be reimplemented incorrectly by the next person. Code that is poorly structured is difficult to extend without introducing bugs.

## PEP 8: The Python Style Standard

**PEP 8** (Python Enhancement Proposal 8) is the Python community's style guide. Tools enforce it automatically.

Key rules:
- **Indentation**: 4 spaces per level (never tabs)
- **Line length**: ≤79 characters for code, ≤72 for docstrings/comments
- **Naming conventions**:
  - `snake_case` for functions and variables: `compute_gc_content`, `kmer_size`
  - `PascalCase` for classes: `SequenceAligner`, `GeneModel`
  - `UPPER_SNAKE_CASE` for module-level constants: `DEFAULT_K = 31`
  - Leading underscore for private names: `_validate_input`
- **Blank lines**: 2 between top-level definitions; 1 between methods within a class
- **Imports**: one import per line; stdlib → third-party → local (blank lines between groups)
- **Whitespace**: spaces around operators and after commas; no spaces inside brackets

Automated enforcement:
```bash
# Check style violations
flake8 my_analysis.py

# Auto-format code (Black is opinionated but widely adopted)
black --line-length 88 my_analysis.py

# Sort imports automatically
isort my_analysis.py

# Type checking
mypy my_analysis.py
```

## Functions: Single Responsibility and Testability

Good functions do one thing, have a descriptive name that says what that one thing is, and are small enough to fit in a terminal window. The **single responsibility principle**: a function should have one reason to change.

The test of whether a function follows this principle is simple: can you write a sentence describing what it does without using "and"? If the honest description is "it reads the file, cleans the data, runs the statistics, and writes the output", you have four functions masquerading as one.

**Bad:**
```python
def process_data(filename):
    # reads file, cleans data, computes stats, writes output — 80 lines
    ...
```

**Good:**
```python
def parse_bed_file(path: str) -> list[dict]:
    """Parse a BED3/BED6/BED12 file into a list of interval dicts."""
    intervals = []
    with open(path) as f:
        for line in f:
            if line.startswith("#") or not line.strip():
                continue
            fields = line.rstrip().split("\t")
            intervals.append({
                "chrom": fields[0],
                "start": int(fields[1]),
                "end":   int(fields[2]),
                "name":  fields[3] if len(fields) > 3 else ".",
            })
    return intervals

def compute_interval_lengths(intervals: list[dict]) -> list[int]:
    """Return lengths of intervals."""
    return [iv["end"] - iv["start"] for iv in intervals]

def write_length_histogram(lengths: list[int], output_path: str) -> None:
    """Write a TSV of length → count to output_path."""
    counts = Counter(lengths)
    with open(output_path, "w") as f:
        for length, count in sorted(counts.items()):
            f.write(f"{length}\t{count}\n")
```

Each function is testable in isolation, nameable, and reusable.

## DRY: Don't Repeat Yourself

Every instance of duplicated code is a maintenance liability — when the logic needs to change, you must change it in every location (and inevitably miss one).

**Bad:**
```python
# GC content repeated three times with slight variations
gc_coding = (cds.count("G") + cds.count("C")) / len(cds)
gc_utr5 = (utr5.count("G") + utr5.count("C")) / len(utr5)
gc_utr3 = (utr3.count("G") + utr3.count("C")) / len(utr3)
```

**Good:**
```python
def gc_content(seq: str) -> float:
    """Return GC fraction of a DNA/RNA string. Returns 0.0 for empty input."""
    if not seq:
        return 0.0
    return (seq.count("G") + seq.count("C")) / len(seq)

gc_coding = gc_content(cds)
gc_utr5   = gc_content(utr5)
gc_utr3   = gc_content(utr3)
```

The DRY principle also applies to analysis parameters. Hard-coded thresholds (`if padj < 0.05`) scattered through a script are a maintenance nightmare — they need to be changed in every location when a reviewer asks you to try a different cutoff. Parameters belong in one place, ideally a configuration file.

## Type Hints and Descriptive Naming

Type hints (Python 3.5+) serve as inline documentation and enable static analysis:

```python
from typing import Optional
import numpy as np
import pandas as pd

def differential_expression(
    counts: pd.DataFrame,          # genes × samples integer count matrix
    metadata: pd.DataFrame,        # samples × conditions metadata
    treatment_col: str,            # column name in metadata for condition
    reference: str,                # reference level for comparison
    alpha: float = 0.05,           # FDR threshold
    min_count: int = 10,           # minimum total count to include gene
) -> pd.DataFrame:                 # returns DESeq2-style results table
    ...
```

**Naming conventions that communicate intent**:
- `n_reads` not `x` (n_ prefix for counts)
- `is_significant` not `sig` (is_ prefix for booleans)
- `gene_id_to_name` not `d` (descriptive for dictionaries)
- `compute_jaccard` not `process` (verb describing action)
- `KmerCounter` not `KC` (full words for classes)

## Code Organization: Modules and Packages

Organize code into logical modules. A bioinformatics project:

```
myproject/
├── myproject/               # installable package
│   ├── __init__.py
│   ├── io.py                # file I/O functions (parse_fasta, write_bed)
│   ├── alignment.py         # alignment algorithms
│   ├── statistics.py        # statistical tests
│   └── visualization.py     # plotting functions
├── scripts/
│   └── run_analysis.py      # thin wrapper calling library functions
├── tests/
│   ├── test_io.py
│   └── test_alignment.py
├── pyproject.toml           # package metadata
└── README.md
```

Scripts should be thin wrappers that parse arguments, call library functions, and handle errors. The library should be importable and testable independently of the command-line interface.

## Why This Matters for Computational Biology

Scientific code is read far more often than it is written — by collaborators, reviewers, your future self, and other labs trying to reproduce your results. A codebase where functions have clear names, are documented, and do one thing is verifiable; a monolithic 500-line script is not. The bioinformatics literature has documented numerous retracted papers and major corrections resulting from code bugs that would have been caught by code review of readable, well-structured code. Automated linting and type checking (flake8, mypy) catch entire classes of bugs (undefined variable, wrong type passed to function) before runtime. Adopting these practices from the beginning of a project — not retrofitting them at submission — is the only realistic approach.
