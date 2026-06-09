# Chapter 10 — Canonical Forms

Explores how matrices decompose into Jordan normal form and rational canonical form via minimal and characteristic polynomials.

## Usage

### Interactive mode
```
cargo run -p ch10-canonical-forms
```

### Non-interactive (scriptable)
```
cargo run -p ch10-canonical-forms -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch10-canonical-forms -- --run demo --format svg > output.svg
cargo run -p ch10-canonical-forms -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch10-canonical-forms -- --run demo --format tex > output.tex
cargo run -p ch10-canonical-forms -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch10-canonical-forms -- --run demo --save state.toml
cargo run -p ch10-canonical-forms -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `mat <n> <entries>` | Set current n×n integer matrix |
| `minimal` | Compute minimal polynomial (step-by-step search through degrees 1–3) |
| `jordan2` | Jordan normal form for current 2×2 matrix |
| `nilpotent <n>` | n×n nilpotent Jordan block and its successive powers |
| `companion <coeffs...>` | Companion matrix for a monic polynomial given by its coefficients |
| `rational` | Rational canonical form for current 2×2 matrix |
| `demo` | Showcase: nilpotent block N_3, companion of x²−3x+2, rational form |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

The minimal polynomial is the monic polynomial of least degree that annihilates a matrix; by Cayley-Hamilton it divides the characteristic polynomial. The Jordan normal form groups eigenvectors and generalized eigenvectors into blocks, each of the form λI + N for a nilpotent N. The rational canonical form uses companion matrices of invariant factors and works over any field, unlike Jordan form which requires eigenvalues in the base field.

## Visualizations

- **SVG**: Side-by-side display of the current matrix A, its companion (RCF) block, and a schematic Jordan block, labeled with the characteristic polynomial.
- **DOT**: Directed graph showing the relationships among A, its characteristic polynomial, minimal polynomial, rational canonical form, and Jordan form.
- **TikZ**: Arrow diagram connecting matrix A to its RCF and Jordan form, annotated with the characteristic polynomial.
- **ASCII**: Text layout showing the matrix, characteristic polynomial, companion (RCF) block, Jordan block schematic, and the Cayley-Hamilton divisibility relation.

## Default State

- `mat_n`: `2` — dimension of the current matrix
- `mat_data`: `[2, 1, 0, 2]` — entries of the 2×2 Jordan block with eigenvalue 2
