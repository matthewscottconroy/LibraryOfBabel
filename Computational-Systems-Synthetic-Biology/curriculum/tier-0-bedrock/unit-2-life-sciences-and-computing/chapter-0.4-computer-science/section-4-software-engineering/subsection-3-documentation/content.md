# Documentation

There is a test you can apply to any piece of code: give it to a competent colleague who was not involved in writing it, and ask them to use it correctly for a real task. If they cannot — if they have to ask you, or they use it incorrectly, or they give up — the code is undocumented, regardless of whether it has comments. Documentation is a communication act. Code without documentation is a black box that only its author understands, temporarily. Good documentation answers three questions: what does this code do, how do I use it, and why was it written this way. For computational biology tools that will be used to make scientific conclusions, documentation is as important as the algorithm itself — an undocumented tool cannot be evaluated, reproduced, or correctly applied.

## Docstrings: Inline Documentation

**Docstrings** are the string literals at the start of a module, class, or function. Two standard styles are used in scientific Python:

### NumPy/SciPy Style (Preferred for Scientific Code)

```python
def align_sequences(
    seq1: str,
    seq2: str,
    match: int = 2,
    mismatch: int = -1,
    gap_open: int = -5,
    gap_extend: int = -1,
) -> tuple[str, str, float]:
    """
    Perform local sequence alignment using Smith-Waterman algorithm.

    Uses affine gap penalties (gap_open + k * gap_extend for a gap of
    length k). Implements the Smith-Waterman algorithm with SIMD
    acceleration via parasail.

    Parameters
    ----------
    seq1 : str
        First sequence (query). Must contain only A, C, G, T or amino
        acid single-letter codes. Case-insensitive.
    seq2 : str
        Second sequence (database/reference). Same requirements as seq1.
    match : int, optional
        Score for a matching position. Default 2.
    mismatch : int, optional
        Penalty for a mismatch. Default -1.
    gap_open : int, optional
        Penalty for opening a gap. Default -5.
    gap_extend : int, optional
        Penalty for extending a gap by one position. Default -1.

    Returns
    -------
    aligned_seq1 : str
        Aligned version of seq1, with '-' for gaps.
    aligned_seq2 : str
        Aligned version of seq2, with '-' for gaps.
    score : float
        Alignment score.

    Raises
    ------
    ValueError
        If either sequence contains invalid characters.

    Examples
    --------
    >>> a1, a2, score = align_sequences("ACGT", "ACGT")
    >>> score
    8.0
    >>> a1
    'ACGT'

    >>> a1, a2, score = align_sequences("ACGT", "AGT")
    >>> print(a1, a2, sep='\\n')
    ACG-T
    A-GGT

    Notes
    -----
    For nucleotide sequences longer than 10 kb, consider using BWA-MEM
    via subprocess for significantly better performance.

    See Also
    --------
    global_align : For global (Needleman-Wunsch) alignment.
    """
    ...
```

### Google Style

Google style uses different section headers and is more compact:

```python
def gc_content(seq: str) -> float:
    """Return GC fraction of a DNA/RNA sequence.

    Args:
        seq: DNA or RNA string. Accepts uppercase or lowercase.
            N characters are excluded from both numerator and denominator.

    Returns:
        Fraction of G and C characters in the non-N portion of seq.
        Returns 0.0 for empty input or input containing only N.

    Raises:
        ValueError: If seq contains characters other than A,C,G,T,N,U (and lowercase).

    Examples:
        >>> gc_content("ACGT")
        0.5
        >>> gc_content("")
        0.0
    """
    ...
```

## Sphinx: Generating HTML Documentation

**Sphinx** generates documentation websites from docstrings. The autodoc extension extracts docstrings automatically:

```bash
# Install and set up Sphinx
pip install sphinx sphinx-autodoc-typehints furo

# Initialize (creates docs/ directory with conf.py and index.rst)
sphinx-quickstart docs/

# Build HTML documentation
cd docs && make html
```

`conf.py` configuration:
```python
extensions = [
    "sphinx.ext.autodoc",          # extract from docstrings
    "sphinx.ext.napoleon",         # parse NumPy/Google style
    "sphinx.ext.viewcode",         # link to source
    "sphinx.ext.autosummary",      # summary tables
    "sphinx_autodoc_typehints",    # type hints in docs
]
html_theme = "furo"  # clean, modern theme
```

`index.rst`:
```rst
.. automodule:: mypackage.sequence
   :members:
   :undoc-members:

.. automodule:: mypackage.alignment
   :members:
```

## README Files: First Point of Contact

The README is the first thing a new user reads. A good README for a bioinformatics tool includes:

1. **What it does**: one sentence, non-jargon
2. **Installation**: exact commands that work from scratch, including dependencies
3. **Quick start**: a minimal working example with realistic data
4. **Usage**: full command-line interface description; all options
5. **Input/output formats**: format specification with examples
6. **Citation**: how to cite if used in a publication

Example README structure:

```markdown
# kmer-toolkit

Count and compare k-mer spectra in DNA sequences.

## Installation

```bash
pip install kmer-toolkit
```

Requires Python ≥3.9 and numpy ≥1.21.

## Quick Start

```python
from kmer_toolkit import KmerCounter, jaccard_distance

counter1 = KmerCounter(k=21)
counter2 = KmerCounter(k=21)

counter1.update_from_fasta("genome1.fa")
counter2.update_from_fasta("genome2.fa")

dist = jaccard_distance(counter1, counter2)
print(f"Jaccard distance: {dist:.4f}")
```

## Citation

If you use kmer-toolkit in published work, please cite:
> Smith J et al. (2024) kmer-toolkit: efficient k-mer analysis for genomics. *Bioinformatics* 40:123-456.
```

## Jupyter Notebooks as Interactive Documentation

Jupyter notebooks serve a dual role: exploratory analysis tool and interactive documentation. Best practices:
- **Clear narrative**: text cells explain what you are doing and why; code cells implement it
- **Linear execution**: notebooks should run top-to-bottom without error; test by restarting the kernel and running all cells
- **Version control**: `nbstripout` removes cell outputs before committing (outputs make diffs unreadable)
- **Separate from library**: notebooks call library functions rather than containing core algorithmic logic

The last point deserves emphasis. A Jupyter notebook is not a good place for a function you will need more than once or that another script will call. Write such functions in `.py` files in your library package, import them into the notebook, and keep the notebook as a story about what you did and what you found — not as a source of algorithms.

```bash
# Strip outputs before git commit
pip install nbstripout
nbstripout --install  # installs as a git filter automatically
```

## Why This Matters for Computational Biology

Undocumented bioinformatics tools are nearly unusable by anyone but their creator. The reproducibility crisis in computational biology is partly a documentation crisis: methods sections in papers describe what was done but not with what exact parameters, what version, or what choices were made. Well-documented code with docstrings, README, and example notebooks is the difference between a tool that gets used and cited, and code that sits on GitHub unused. Many high-impact tools (GATK, DESeq2, Salmon, Snakemake) have extensive documentation that is explicitly a competitive advantage — users choose them over alternatives partly because they can understand how to use them correctly. Writing documentation is also a debugging practice: if you cannot explain what a function does clearly, you often do not fully understand it yourself.
