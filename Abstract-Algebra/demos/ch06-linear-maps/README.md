# Chapter 6 — Linear Maps

Explorer for linear transformations T: R^n → R^m represented by matrices, including kernel, image, rank-nullity, composition, and the dual map.

## Usage

### Interactive mode
```
cargo run -p ch06-linear-maps
```

### Non-interactive (scriptable)
```
cargo run -p ch06-linear-maps -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch06-linear-maps -- --run demo --format svg > output.svg
cargo run -p ch06-linear-maps -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch06-linear-maps -- --run demo --format tex > output.tex
cargo run -p ch06-linear-maps -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch06-linear-maps -- --run demo --save state.toml
cargo run -p ch06-linear-maps -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `map <rows> <cols> <entries>` | Define the current map T as an m×n matrix |
| `show` | Display T and its rank |
| `kernel` | Compute a basis for ker(T) = null space; report dim and injectivity |
| `image` | Compute a basis for im(T) = column space; report dim |
| `ranknullity` | Verify rank + nullity = dim(domain); report injectivity/surjectivity |
| `compose <rA> <cA> <A> / <rB> <cB> <B>` | Compute the composition A∘B |
| `dual` | Compute the transpose T^T (dual map) |
| `demo` | Showcase of linear map concepts |
| `quit` | Exit |

## Mathematical Content

A linear map T: R^n → R^m preserves addition and scalar multiplication; it is completely determined by its m×n matrix. The kernel ker(T) is the preimage of 0 and its dimension is the nullity; the image im(T) is the column space and its dimension is the rank. The rank-nullity theorem states rank(T) + nullity(T) = n for any linear map from R^n; T is injective iff nullity = 0 and surjective iff rank = m. The dual map T^T: R^m → R^n is represented by the transpose matrix and satisfies rank(T^T) = rank(T).

## Visualizations

- **SVG**: Domain R^n box on the left with basis-vector circles; codomain R^m box on the right; labeled arrow T between them; im(T) shaded inside the codomain with its dimension; ker(T) shaded inside the domain (if non-trivial); rank-nullity equation shown at the bottom.
- **DOT**: Four nodes — domain, codomain, im(T), ker(T) — with directed edges T, null-space, subset.
- **TikZ**: Domain and codomain circles connected by a stealth arrow; ker(T) and im(T) as subordinate nodes connected by dashed and hook arrows; rank-nullity label above the main arrow.
- **ASCII**: Box diagram of R^n → R^m with the map T labeled on the arrow; rank, nullity, and rank+nullity printed below; matrix entries listed row by row.

## Default State

The default TOML state includes:
- `map_rows`: number of rows of T (`[2]`)
- `map_cols`: number of columns of T (`[3]`)
- `current_map`: the 2×3 matrix `[[1, 0, 2], [0, 1, -1]]` representing T: R³ → R²
