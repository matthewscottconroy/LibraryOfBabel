# Chapter 7 — Matrices and Matrix Algebra

Named-matrix calculator with step-by-step row reduction, RREF, rank, matrix inverse via the adjugate, and linear system solving.

## Usage

### Interactive mode
```
cargo run -p ch07-matrices
```

### Non-interactive (scriptable)
```
cargo run -p ch07-matrices -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch07-matrices -- --run demo --format svg > output.svg
cargo run -p ch07-matrices -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch07-matrices -- --run demo --format tex > output.tex
cargo run -p ch07-matrices -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch07-matrices -- --run demo --save state.toml
cargo run -p ch07-matrices -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `set <name> <r> <c> <entries>` | Store a named integer matrix |
| `show` | List all stored matrices with dimensions |
| `mul <A> <B>` | Multiply A by B |
| `add <A> <B>` | Add A and B |
| `reduce <A>` | Row reduce A with a printed step-by-step trace |
| `rref <A>` | Compute RREF of A and report rank |
| `rank <A>` | Compute rank and nullity; verify rank + nullity = cols |
| `inverse <A>` | Compute A⁻¹ via the adjugate (requires det ≠ 0) |
| `solve <A> <b>` | Solve Ax = b via augmented matrix row reduction |
| `demo` | Showcase: solving a 3×3 system and inverting a 2×2 matrix |
| `quit` | Exit |

## Mathematical Content

Matrix algebra is the computational backbone of linear algebra. This demo stores matrices by name and performs the standard operations: addition, multiplication (with dimension checking), row reduction via Gaussian elimination (printing each pivot step), and RREF. The inverse command constructs A⁻¹ = (1/det) adj(A) using cofactor expansion, and verifies A · adj(A) = det(A) · I. The solver augments [A | b], row-reduces, and reads off the unique solution, or identifies inconsistency and underdetermined systems.

## Visualizations

- **SVG**: Each stored matrix rendered as a labeled grid; rank annotated below each matrix; product formula reminder at the bottom.
- **DOT**: Each stored matrix as a node labeled with name, dimensions, rank, and determinant (for square matrices); if A and B are both stored and compatible, a product node AB is shown.
- **TikZ**: Each stored matrix (up to 2) shown as a labeled node with its entries in a small table; a multiplication arrow connects the first to the second.
- **ASCII**: Each stored matrix printed row by row with its rank.

## Default State

The default TOML state includes:
- `mat_A`: 2×2 integer matrix `[[1, 2], [3, 4]]`
- `mat_B`: 2×2 integer matrix `[[5, 6], [7, 8]]`
- `mat_names`: list of defined matrix names (`["A", "B"]`)
