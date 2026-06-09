# Chapter 8 — Determinants

Explorer for determinant computation via cofactor expansion, geometric interpretation as parallelogram area, Cramer's rule, Vandermonde determinants, and key multiplicative properties.

## Usage

### Interactive mode
```
cargo run -p ch08-determinants
```

### Non-interactive (scriptable)
```
cargo run -p ch08-determinants -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch08-determinants -- --run demo --format svg > output.svg
cargo run -p ch08-determinants -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch08-determinants -- --run demo --format tex > output.tex
cargo run -p ch08-determinants -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch08-determinants -- --run demo --save state.toml
cargo run -p ch08-determinants -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `det <rows> <cols> <entries>` | Compute det with cofactor expansion steps (any square matrix) |
| `area <v1x> <v1y> <v2x> <v2y>` | Area of the parallelogram spanned by two 2D vectors; orientation sign |
| `vandermonde <x1> <x2> ...` | Vandermonde matrix and its determinant ∏(xᵢ − xⱼ) (2–5 points) |
| `cramer <A entries> / <b>` | Solve a 2×2 or 3×3 system using Cramer's rule |
| `properties` | Verify det(AB) = det(A)det(B) and det(Aᵀ) = det(A) on concrete matrices |
| `demo` | Showcase of all determinant features |
| `quit` | Exit |

## Mathematical Content

The determinant of a square matrix is the signed volume of the parallelepiped spanned by its column (or row) vectors; it is zero exactly when the matrix is singular. This demo computes determinants by cofactor expansion along the first row, printing each signed minor contribution. The Vandermonde determinant det(V(x₁,…,xₙ)) = ∏ᵢ>ⱼ(xᵢ − xⱼ) is verified symbolically. Cramer's rule expresses each variable of a non-singular linear system as a ratio of determinants. The `properties` command confirms the multiplicative property det(AB) = det(A)det(B) and that transposition preserves the determinant.

## Visualizations

- **SVG**: The stored matrix displayed as a grid; determinant value labeled in green (non-zero) or red (zero); for 3×3 matrices, cofactor expansion terms listed as colored rows; for 2×2 matrices, the parallelogram drawn with column vectors and area labeled.
- **DOT**: Root node showing det value; one child node per cofactor term along row 1, each labeled with sign × entry × minor-det = contribution.
- **TikZ**: The stored matrix rendered as a `pmatrix` inside a `\det{...}` expression; a node below stating whether the matrix is invertible.
- **ASCII**: Matrix printed row by row; determinant and invertibility status; cofactor expansion terms listed for matrices up to 3×3.

## Default State

The default TOML state includes:
- `last_mat`: 3×3 integer matrix `[[1,2,3],[4,5,6],[7,8,9]]`
