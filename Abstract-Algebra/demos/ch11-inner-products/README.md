# Chapter 11 — Inner Product Spaces

Explores the geometry of vector spaces through inner products, orthogonalization, QR decomposition, least-squares approximation, and the spectral theorem.

## Usage

### Interactive mode
```
cargo run -p ch11-inner-products
```

### Non-interactive (scriptable)
```
cargo run -p ch11-inner-products -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch11-inner-products -- --run demo --format svg > output.svg
cargo run -p ch11-inner-products -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch11-inner-products -- --run demo --format tex > output.tex
cargo run -p ch11-inner-products -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch11-inner-products -- --run demo --save state.toml
cargo run -p ch11-inner-products -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `inner <v1...> / <v2...>` | Inner product, lengths, angle, and Cauchy-Schwarz verification |
| `gram_schmidt <v1> \| <v2> \| ...` | Gram-Schmidt orthogonalization (vectors separated by `\|`) |
| `project <v...> / <onto...>` | Orthogonal projection of v onto the span of onto_v |
| `qr <n> <entries>` | QR decomposition of an n×n matrix via Gram-Schmidt |
| `least_squares <A...> / <b...>` | Solve Ax≈b via normal equations AᵀAx = Aᵀb |
| `spectral <n> <entries>` | Largest eigenvalue of a symmetric matrix via power iteration |
| `demo` | Showcase: orthogonal standard basis vectors and a Gram-Schmidt example |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

An inner product on a vector space encodes both length (‖v‖ = √⟨v,v⟩) and angle (cos θ = ⟨v,w⟩/(‖v‖‖w‖)), and satisfies the Cauchy-Schwarz inequality |⟨v,w⟩| ≤ ‖v‖‖w‖. The Gram-Schmidt process converts any linearly independent set into an orthonormal basis by iteratively subtracting projections, and factors a matrix as A = QR with Q orthogonal and R upper triangular. The spectral theorem states that every real symmetric matrix is orthogonally diagonalizable: A = QΛQᵀ with real eigenvalues.

## Visualizations

- **SVG**: 2D vector diagram showing v1, v2, the normalized e1, the projection of v2 onto e1, the orthogonal complement u2, a right-angle mark, and the Cauchy-Schwarz inequality.
- **DOT**: Graph of concepts — inner product space, orthonormal basis (via Gram-Schmidt), projection onto subspace, QR decomposition, and least-squares solution.
- **TikZ**: Vector diagram with arrows for v1, v2, e1, u2, and the orthogonality relation ⟨e1,e2⟩ = 0.
- **ASCII**: Step-by-step Gram-Schmidt walkthrough for [1,1,0] and [1,0,1], showing e1, the projection coefficient, u2, e2, and the QR factorization statement.

## Default State

- `v1`: `[1, 0, 0]` — first default vector
- `v2`: `[0, 1, 0]` — second default vector (orthogonal to v1)
