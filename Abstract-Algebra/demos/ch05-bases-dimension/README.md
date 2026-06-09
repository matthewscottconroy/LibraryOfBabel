# Chapter 5 — Bases, Dimension, and Coordinates

Explorer for testing bases, computing coordinates in non-standard bases, extending partial sets to full bases, and constructing change-of-basis matrices.

## Usage

### Interactive mode
```
cargo run -p ch05-bases-dimension
```

### Non-interactive (scriptable)
```
cargo run -p ch05-bases-dimension -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch05-bases-dimension -- --run demo --format svg > output.svg
cargo run -p ch05-bases-dimension -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch05-bases-dimension -- --run demo --format tex > output.tex
cargo run -p ch05-bases-dimension -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch05-bases-dimension -- --run demo --save state.toml
cargo run -p ch05-bases-dimension -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `basis <vecs>` | Test whether the given vectors form a basis for R^n |
| `coords <v> / <basis>` | Express v in the given basis coordinates (solve Bx = v) |
| `dim <vecs>` | Dimension of the span of the given vectors |
| `extend <vecs>` | Extend a linearly independent set to a basis of R^n by adding standard basis vectors |
| `change <B1> / <B2>` | Compute the change-of-basis matrix P that converts B1 coordinates to B2 coordinates |
| `standard <n>` | Display the standard basis for R^n (n ≤ 8) |
| `demo` | Showcase of basis and dimension concepts |
| `quit` | Exit |

Separate multiple vectors with `;`; separate the vector from the basis with `/`.

## Mathematical Content

A basis for a vector space is a linearly independent spanning set; every basis of R^n contains exactly n vectors, so n is the dimension. This demo uses row reduction (RREF) to check independence and spanning simultaneously. The `coords` command solves the linear system Bx = v to express a vector in non-standard coordinates; `change` constructs the n×n matrix whose columns are the B1 basis vectors expressed in B2 coordinates. The `extend` command greedily adds standard basis vectors until the rank reaches n.

## Visualizations

- **SVG**: The n×n identity matrix displayed as a grid (n = ambient_dim, capped at 4); each column labeled e₁…eₙ in blue; a shaded rectangle below displaying `span{e1,...,en} = R^n`.
- **DOT**: Each standard basis vector eᵢ as a node with its coordinates; all nodes pointing to a central `R^n (dim=n)` node labeled `basis vector`.
- **TikZ**: Origin O with stealth arrows to e₁, e₂, … (up to 4) and a final `R^n` node; dimension annotation above the midpoint.
- **ASCII**: Standard basis vectors for R^n listed vertically; `dim(R^n) = n` and the characterization `Basis = independent + spanning`.

## Default State

The default TOML state includes:
- `ambient_dim`: the ambient dimension R^n used for visualizations (`3`)
