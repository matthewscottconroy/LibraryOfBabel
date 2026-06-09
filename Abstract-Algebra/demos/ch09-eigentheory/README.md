# Chapter 9 — Eigentheory

Explorer for eigenvalues, eigenvectors, characteristic polynomials, power iteration, diagonalizability, and Markov chain steady states for 2×2 and 3×3 matrices.

## Usage

### Interactive mode
```
cargo run -p ch09-eigentheory
```

### Non-interactive (scriptable)
```
cargo run -p ch09-eigentheory -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch09-eigentheory -- --run demo --format svg > output.svg
cargo run -p ch09-eigentheory -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch09-eigentheory -- --run demo --format tex > output.tex
cargo run -p ch09-eigentheory -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch09-eigentheory -- --run demo --save state.toml
cargo run -p ch09-eigentheory -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `mat <n> <entries>` | Set the current n×n matrix (n = 2 or 3) |
| `show` | Display the current matrix |
| `char_poly` | Compute the characteristic polynomial det(A − λI) |
| `eigenvalues` | Find all real eigenvalues by solving the characteristic polynomial |
| `eigenvector <lambda>` | Compute and verify a unit eigenvector for the given λ |
| `power <n>` | Power iteration for n steps; converges to the dominant eigenvalue |
| `diag` | Check diagonalizability; report whether A = PDP⁻¹ exists over R |
| `markov <n> <entries>` | Run a stochastic matrix to its steady-state distribution (n = 2–6) |
| `demo` | Showcase: 2×2 full analysis, 3×3 diagonal matrix, Markov chain |
| `quit` | Exit |

## Mathematical Content

An eigenvalue λ of A satisfies det(A − λI) = 0; the characteristic polynomial encodes all eigenvalues as its roots. For 2×2 matrices this is a quadratic solvable by the quadratic formula; for 3×3 matrices the demo searches for rational and half-integer roots then factors the remaining quadratic. Eigenvectors are found by row-reducing (A − λI) and reading off the free-variable solution. A matrix is diagonalizable over R when it has n distinct real eigenvalues. Power iteration repeatedly multiplies a vector by A and normalizes, converging to the dominant eigenpair. The `markov` command iterates a stochastic matrix from a uniform initial distribution until the state vector converges, illustrating that the steady-state vector is an eigenvector with eigenvalue 1.

## Visualizations

- **SVG**: The matrix displayed as a grid; each eigenvalue drawn as a colored circle with its value and unit eigenvector; arrows from the matrix to each eigenvalue; a DIAGONALIZABLE / NOT diagonalizable status label.
- **DOT**: Matrix node A → characteristic polynomial node → one eigenvalue node per root, each with a child eigenvector node.
- **TikZ**: Matrix node A, characteristic polynomial node, and one node per eigenvalue arranged vertically; stealth arrows connecting each stage.
- **ASCII**: Matrix printed row by row; eigenvalues and unit eigenvectors listed; diagonalizability verdict.

## Default State

The default TOML state includes:
- `mat_n`: matrix dimension (`2`)
- `mat_data`: flat space-separated entries of the current matrix (`"4 1 2 3"`, representing `[[4,1],[2,3]]`)
