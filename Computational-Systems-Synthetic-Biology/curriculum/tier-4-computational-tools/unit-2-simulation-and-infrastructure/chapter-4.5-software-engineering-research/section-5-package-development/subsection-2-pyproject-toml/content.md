# pyproject.toml: Modern Python Package Configuration

Before 2016, publishing a Python package to PyPI required maintaining at least five separate files: a `setup.py` script that executed arbitrary code to describe your package, a `setup.cfg` that held static metadata, a `requirements.txt` for dependencies, a `.flake8` file for linting configuration, and a `pytest.ini` for test settings. These files were not coordinated — a version number change in `setup.py` did not automatically propagate to `setup.cfg`. There was no standard way to declare build dependencies before the build ran, which meant installing packages with compiled extensions could fail in circular ways. Testing environments could accidentally import the wrong version of the code they were supposed to be testing.

**pyproject.toml** is the PEP 517/518 standard configuration file for Python packages, replacing the fragmented ecosystem of `setup.py`, `setup.cfg`, and `requirements.txt`. It defines the **build system** (how to build a wheel), **project metadata** (name, version, authors, dependencies), and configuration for development tools (pytest, ruff, mypy). A single `pyproject.toml` is the authoritative source for all of a package's metadata and tooling configuration.

## PEP 517/518 Background

Before `pyproject.toml`, Python packages used `setup.py` — an executable Python script that configured the build. This had several problems:
- Running `setup.py` to determine metadata required importing the package and running arbitrary code
- Build dependencies (e.g., Cython, numpy) had no standard way to be specified before the build ran
- Tool configurations were scattered across `setup.cfg`, `.flake8`, `pytest.ini`, `mypy.ini`

**PEP 518** (2016) introduced `pyproject.toml` to declare build dependencies before the build runs. **PEP 517** defined the build system interface. Modern build backends — **Hatchling**, **Flit**, **PDM**, **setuptools** — all conform to PEP 517 and are configured via `pyproject.toml`.

## Complete pyproject.toml Example

```toml
# pyproject.toml — complete configuration for a systems biology package

# ── Build system ────────────────────────────────────────────────────────────
[build-system]
requires      = ["hatchling>=1.21"]
build-backend = "hatchling.build"

# ── Project metadata (PEP 621) ──────────────────────────────────────────────
[project]
name     = "bioanalysis"
version  = "0.3.1"
description = "Computational tools for systems and synthetic biology"
readme   = "README.md"
license  = { file = "LICENSE" }
requires-python = ">=3.10"

authors = [
    { name = "Research Lab", email = "research@example.edu" }
]
maintainers = [
    { name = "Research Lab", email = "research@example.edu" }
]

# PyPI classifiers: used for search and filtering
classifiers = [
    "Development Status :: 3 - Alpha",
    "Intended Audience :: Science/Research",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Topic :: Scientific/Engineering :: Bio-Informatics",
]

keywords = ["bioinformatics", "systems biology", "RNA-seq", "network analysis"]

# Core runtime dependencies (pinned loosely to allow version flexibility)
dependencies = [
    "numpy>=1.24",
    "scipy>=1.10",
    "pandas>=2.0",
    "matplotlib>=3.7",
    "networkx>=3.1",
    "biopython>=1.81",
    "h5py>=3.8",
    "anndata>=0.10",
    "scanpy>=1.9",
]

# ── Optional dependency groups ──────────────────────────────────────────────
[project.optional-dependencies]
# Install with: pip install "bioanalysis[ml]"
ml = [
    "scikit-learn>=1.3",
    "torch>=2.0",
    "torch-geometric>=2.4",
]

# Install with: pip install "bioanalysis[simulation]"
simulation = [
    "roadrunner>=2.5",
    "tellurium>=2.2",
    "pysb>=1.15",
]

# Install with: pip install "bioanalysis[dev]"
dev = [
    "pytest>=7.0",
    "pytest-cov>=4.0",
    "pytest-xdist>=3.0",      # parallel test execution
    "hypothesis>=6.0",         # property-based testing
    "ruff>=0.3",               # linting + formatting
    "mypy>=1.8",               # static type checking
    "sphinx>=7.0",             # documentation
    "sphinx-autodoc-typehints",
    "myst-parser",             # Markdown in Sphinx
    "pre-commit>=3.0",
]

# All optional dependencies
all = [
    "bioanalysis[ml]",
    "bioanalysis[simulation]",
    "bioanalysis[dev]",
]

# ── Console scripts (command-line entry points) ─────────────────────────────
[project.scripts]
bioanalysis-qc      = "bioanalysis.cli.qc:main"
bioanalysis-network = "bioanalysis.cli.network:main"

# ── Project URLs ────────────────────────────────────────────────────────────
[project.urls]
Homepage      = "https://github.com/researchlab/bioanalysis"
Documentation = "https://bioanalysis.readthedocs.io"
Repository    = "https://github.com/researchlab/bioanalysis"
"Bug Tracker" = "https://github.com/researchlab/bioanalysis/issues"
Changelog     = "https://github.com/researchlab/bioanalysis/blob/main/CHANGELOG.md"

# ── Hatchling build configuration ──────────────────────────────────────────
[tool.hatch.build.targets.wheel]
packages = ["src/bioanalysis"]   # tells hatchling where to find the package

[tool.hatch.version]
path = "src/bioanalysis/__init__.py"   # read version from __version__ variable

# ── pytest configuration ────────────────────────────────────────────────────
[tool.pytest.ini_options]
testpaths   = ["tests"]
addopts     = [
    "--strict-markers",    # fail if unknown marks are used
    "--tb=short",
    "--cov=bioanalysis",
    "--cov-report=term-missing",
    "--cov-report=xml:coverage.xml",
]
markers = [
    "slow: tests that take more than 30 seconds",
    "integration: tests requiring external tools or data",
    "gpu: tests requiring a GPU",
]

# ── Coverage configuration ──────────────────────────────────────────────────
[tool.coverage.run]
source   = ["src/bioanalysis"]
omit     = ["*/tests/*", "*/__main__.py"]

[tool.coverage.report]
fail_under = 80   # CI fails if coverage drops below 80%
show_missing = true

# ── Ruff linting and formatting ─────────────────────────────────────────────
[tool.ruff]
src            = ["src"]
target-version = "py310"
line-length    = 100

[tool.ruff.lint]
select = [
    "E",   # pycodestyle errors
    "W",   # pycodestyle warnings
    "F",   # pyflakes
    "I",   # isort
    "N",   # pep8-naming
    "UP",  # pyupgrade
    "B",   # flake8-bugbear
    "SIM", # flake8-simplify
]
ignore = [
    "E501",  # line too long (handled by formatter)
    "N803",  # argument name should be lowercase (X is conventional for matrices)
]

[tool.ruff.format]
quote-style         = "double"
indent-style        = "space"
skip-magic-trailing-comma = false

# ── Mypy static type checking ───────────────────────────────────────────────
[tool.mypy]
python_version = "3.10"
strict         = false
warn_return_any     = true
warn_unused_imports = true
ignore_missing_imports = true   # many scientific packages lack stubs

[[tool.mypy.overrides]]
module = ["bioanalysis.models.*"]
ignore_errors = true   # relax for complex ML code
```

## Building and Publishing

```bash
# Install build tools
pip install build twine

# Build wheel and source distribution
python -m build
# Creates:
#   dist/bioanalysis-0.3.1-py3-none-any.whl
#   dist/bioanalysis-0.3.1.tar.gz

# Check the distribution before publishing
twine check dist/*

# Upload to TestPyPI first (testing)
twine upload --repository testpypi dist/*

# Upload to PyPI (production)
twine upload dist/*

# Install from PyPI
pip install bioanalysis
pip install "bioanalysis[ml]"
pip install "bioanalysis[dev]"
```

## Automated Releases with GitHub Actions

```yaml
# .github/workflows/publish.yml
name: Publish to PyPI

on:
  push:
    tags:
      - "v*"   # triggered by version tags like v0.3.1

jobs:
  build-and-publish:
    runs-on: ubuntu-latest
    environment:
      name: pypi
      url: https://pypi.org/p/bioanalysis
    permissions:
      id-token: write   # needed for trusted publishing

    steps:
      - uses: actions/checkout@v4

      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: "3.11"

      - name: Build package
        run: |
          pip install build
          python -m build

      - name: Publish to PyPI
        uses: pypa/gh-action-pypi-publish@release/v1
        # Trusted publishing: no API token needed
```

## Version Management

```bash
# Using hatch for version bumping
pip install hatch

hatch version patch    # 0.3.1 → 0.3.2
hatch version minor    # 0.3.1 → 0.4.0
hatch version major    # 0.3.1 → 1.0.0

# hatch updates __version__ in src/bioanalysis/__init__.py
# then tag the commit:
git add src/bioanalysis/__init__.py
git commit -m "Bump version to $(hatch version)"
git tag -a "v$(hatch version)" -m "Release v$(hatch version)"
git push --follow-tags
```

## Why This Matters

`pyproject.toml` consolidates all the metadata and tooling configuration that previously required maintaining 5+ separate files, reducing configuration complexity and the chance of inconsistencies. The optional dependency groups (`[dev]`, `[ml]`, `[simulation]`) mean users install only what they need, while developers get linting, testing, and documentation tools with a single command. Entry points (console scripts) make package functions accessible from the command line without users knowing any Python — a critical usability feature for tools that will be used inside Snakemake rules or SLURM batch scripts. For research code, publishing to PyPI (even as a pre-release) transforms the distribution story from "clone this repo and add it to your PYTHONPATH" to "pip install mypackage" — dramatically lowering the barrier to reuse and collaboration.
