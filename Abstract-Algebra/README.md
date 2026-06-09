# The Structure of Algebra

A textbook covering logic through the Langlands program, paired with a Rust workspace of 54 interactive chapter demos. Each demo runs as a REPL, a scriptable CLI, and a native GUI.

The compiled book lives at `book/abstract-algebra.pdf` (1,900 pages).

## Quick Start

```bash
# Build all chapter demos
cargo build --workspace

# Interactive REPL for any chapter
cargo run -p ch13-groups

# Single command, pipe-friendly
cargo run -p ch13-groups -- --run cayley Z 6

# Produce an SVG diagram
cargo run -p ch31-galois-theory -- --run galois --format svg > galois.svg

# GUI — browse all 54 chapters
cargo run -p gui
```

## Repository Layout

```
book/                  Markdown source + build scripts for the PDF
demos/
  common/              Shared library: math, rendering, CLI, REPL
  visualizer/          Standalone render CLI (SVG/DOT/TikZ/ASCII/PNG/JPEG)
  gui/                 egui native GUI (chapter browser + output viewer)
  ch01-logic/          Chapter 1 demo
  ch02-sets/           Chapter 2 demo
  ...
  ch54-langlands/      Chapter 54 demo
```

## Three Ways to Run a Demo

### Interactive REPL

```bash
cargo run -p ch13-groups
# ch13> help
# ch13> cayley Z 6
# ch13> quit
```

Tab-completion hints are available. Type `help` to list commands for that chapter.

### Scriptable CLI

```bash
cargo run -p CRATE -- --run CMD [ARGS…] [--format FMT] [--output FILE] [--load FILE] [--save FILE]
```

| Flag | Description |
|------|-------------|
| `--run CMD` | Run one command and exit |
| `--format` | `text` (default), `svg`, `dot`, `tex`, `ascii` |
| `--output FILE` | Write to file instead of stdout |
| `--load FILE` | Restore state from a TOML file before running |
| `--save FILE` | Persist state to a TOML file after running |

### GUI

```bash
cargo run -p gui
# Override the workspace root if running the binary directly:
ABSTRACT_ALGEBRA_ROOT=/path/to/repo cargo run -p gui
```

The GUI has a chapter sidebar, command input, format selector, and three output tabs: plain text, SVG, and a TOML state editor.

## Output Formats

| Format | Notes |
|--------|-------|
| `text`  | Human-readable terminal output |
| `svg`   | Open in any browser or Inkscape |
| `dot`   | Pipe to `dot -Tpdf` or `dot -Tsvg` |
| `tex`   | Standalone TikZ document; compile with `pdflatex` |
| `ascii` | Terminal-friendly diagram |

```bash
# SVG → PDF
cargo run -p ch49-root-systems -- --run roots --format svg \
  | inkscape --pipe --export-pdf=roots.pdf

# DOT → PDF
cargo run -p ch31-galois-theory -- --run galois --format dot \
  | dot -Tpdf -o galois.pdf

# TikZ → PDF
cargo run -p ch07-matrices -- --run demo --format tex > matrix.tex \
  && pdflatex matrix.tex
```

## State Persistence

Each chapter stores working data in a `StateMap` (a TOML-backed key/value store). Save, hand-edit, and reload state between sessions:

```bash
cargo run -p ch07-matrices -- --run demo --save my_matrix.toml
# edit my_matrix.toml
cargo run -p ch07-matrices -- --load my_matrix.toml --run det
```

## Standalone Visualizer

The `visualizer` binary wraps every chapter demo and adds raster output via `resvg`:

```bash
visualizer list
visualizer --chapter 13 --cmd cayley --format svg --output cayley.svg
visualizer --chapter 13 --cmd cayley --format png --output cayley.png
visualizer --chapter 31 --cmd galois --format dot | dot -Tpdf > galois.pdf
```

Set `ABSTRACT_ALGEBRA_ROOT` when running outside the workspace.

## Chapter Index

| Part | # | Crate | Topic |
|------|---|-------|-------|
| **I — Language of Mathematics** | 1 | `ch01-logic` | Logic and Proofs |
| | 2 | `ch02-sets` | Sets and Functions |
| | 3 | `ch03-cardinality` | Cardinality |
| **II — Linear Algebra** | 4 | `ch04-vector-spaces` | Vector Spaces |
| | 5 | `ch05-bases-dimension` | Bases and Dimension |
| | 6 | `ch06-linear-maps` | Linear Maps |
| | 7 | `ch07-matrices` | Matrices |
| | 8 | `ch08-determinants` | Determinants |
| | 9 | `ch09-eigentheory` | Eigentheory |
| | 10 | `ch10-canonical-forms` | Canonical Forms |
| | 11 | `ch11-inner-products` | Inner Products |
| | 12 | `ch12-multilinear` | Multilinear Algebra |
| **III — Group Theory** | 13 | `ch13-groups` | Groups |
| | 14 | `ch14-cosets` | Cosets and Lagrange |
| | 15 | `ch15-homomorphisms` | Homomorphisms |
| | 16 | `ch16-group-actions` | Group Actions |
| | 17 | `ch17-sylow` | Sylow Theorems |
| | 18 | `ch18-group-structure` | Group Structure |
| | 19 | `ch19-abelian-groups` | Abelian Groups |
| **IV — Ring Theory** | 20 | `ch20-rings` | Rings |
| | 21 | `ch21-ideals` | Ideals |
| | 22 | `ch22-divisibility` | Divisibility |
| | 23 | `ch23-polynomials` | Polynomials |
| | 24 | `ch24-commutative-algebra` | Commutative Algebra |
| **V — Module Theory** | 25 | `ch25-modules` | Modules |
| | 26 | `ch26-projective-injective` | Projective and Injective |
| | 27 | `ch27-structure-theorem` | Structure Theorem |
| | 28 | `ch28-tensor-products` | Tensor Products |
| **VI — Field and Galois Theory** | 29 | `ch29-field-extensions` | Field Extensions |
| | 30 | `ch30-normal-separable` | Normal and Separable |
| | 31 | `ch31-galois-theory` | Galois Theory |
| | 32 | `ch32-galois-applications` | Galois Applications |
| **VII — Category Theory** | 33 | `ch33-categories` | Categories |
| | 34 | `ch34-yoneda` | Yoneda Lemma |
| | 35 | `ch35-adjoints` | Adjoints |
| | 36 | `ch36-limits-colimits` | Limits and Colimits |
| **VIII — Homological Algebra** | 37 | `ch37-abelian-categories` | Abelian Categories |
| | 38 | `ch38-chain-complexes` | Chain Complexes |
| | 39 | `ch39-resolutions` | Resolutions |
| | 40 | `ch40-ext-tor` | Ext and Tor |
| | 41 | `ch41-spectral-sequences` | Spectral Sequences |
| **IX — Finite Group Representations** | 42 | `ch42-representations` | Representations |
| | 43 | `ch43-complete-reducibility` | Complete Reducibility |
| | 44 | `ch44-character-theory` | Character Theory |
| | 45 | `ch45-induced-representations` | Induced Representations |
| **X — Lie Theory** | 46 | `ch46-lie-groups` | Lie Groups |
| | 47 | `ch47-lie-algebras` | Lie Algebras |
| | 48 | `ch48-solvable-semisimple` | Solvable and Semisimple |
| | 49 | `ch49-root-systems` | Root Systems |
| | 50 | `ch50-highest-weight` | Highest Weight Theory |
| **XI — Advanced Representations** | 51 | `ch51-modular` | Modular Representation |
| | 52 | `ch52-geometric` | Geometric Representation |
| | 53 | `ch53-quantum-groups` | Quantum Groups |
| | 54 | `ch54-langlands` | The Langlands Program |

## Common Library

All shared infrastructure lives in `demos/common/src/lib.rs`:

- **Display** — `print_banner`, `print_section`, `print_ok`, `print_err`, `print_info`, colored output helpers
- **REPL** — `repl(prompt, handler)`: readline loop with quit handling
- **CLI** — `AppArgs::parse()`, `AppMode`, `OutputFormat`, `app.emit()`
- **Number theory** — `gcd`, `lcm`, `ext_gcd`, `is_prime`, `factor`, `euler_totient`, `mod_pow`, `mult_order`, `divisors`, `legendre`, `jacobi`, `mobius`, `quadratic_residues`
- **Matrices** — `Mat` (integer), `FMat` (float): row reduction, Smith normal form, rank, LU decomp, determinant
- **Polynomials** — `Poly` over ℤ: `div_rem`, `gcd_poly`, `is_irreducible_mod`, `factor_mod`, `reduce_mod`
- **Groups** — `zn_add_table`, `zn_mul_table`, Cayley printing, permutation utilities (`perm_compose`, `perm_inverse`, `perm_order`, `perm_sign`, `perm_cycle_notation`, `symmetric_group`, `group_from_generators`)
- **Diagrams** — `SvgCanvas`, `DotGraph`, `TikzDoc`, `AsciiCanvas`, `svg_divisor_hasse`, `svg_hasse`, `svg_cayley_table`, `svg_comm_diagram`
- **State** — `StateMap` (TOML-backed): `save_state`, `load_state`, typed accessors

## Requirements

- Rust 1.75+
- [Graphviz](https://graphviz.org/) — for `--format dot` rendering
- A LaTeX distribution with TikZ — for `--format tex` compilation
- Inkscape or `rsvg-convert` — for SVG-to-PDF conversion
- X11 or Wayland — for the GUI on Linux

## Building the Book

```bash
cd book
python3 build-book.py   # requires pandoc and xelatex
```

Output: `book/abstract-algebra.pdf`
