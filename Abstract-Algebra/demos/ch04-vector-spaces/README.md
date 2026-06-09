# Chapter 4 — Fields and Vector Spaces

Explorer for vector addition, scalar multiplication, span, linear independence, subspaces, and finite fields F_p.

## Usage

### Interactive mode
```
cargo run -p ch04-vector-spaces
```

### Non-interactive (scriptable)
```
cargo run -p ch04-vector-spaces -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch04-vector-spaces -- --run demo --format svg > output.svg
cargo run -p ch04-vector-spaces -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch04-vector-spaces -- --run demo --format tex > output.tex
cargo run -p ch04-vector-spaces -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch04-vector-spaces -- --run demo --save state.toml
cargo run -p ch04-vector-spaces -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `add <v1> ; <v2>` | Vector addition; use `;` to separate vectors |
| `scale <s> <v>` | Scalar multiplication of a vector |
| `span <vecs>` | Compute the rank of the span of the given vectors |
| `indep <vecs>` | Test linear independence (rank = number of vectors?) |
| `subspace <vecs>` | Confirm the span is a subspace and report its dimension |
| `field <p>` | Switch the working field to F_p = Z/pZ (p must be prime) |
| `zero_div <n>` | Find zero divisors in Z/nZ; confirm whether Z/nZ is a field |
| `demo` | Showcase of vector space concepts |
| `quit` | Exit |

Vectors are space-separated numbers. Separate multiple vectors with `;`.

## Mathematical Content

A vector space over a field F is a set equipped with addition and scalar multiplication satisfying eight axioms; the key example is R^n. Linear independence is tested via row reduction: a set of vectors is independent when their matrix has full row rank. The demo illustrates the standard basis {e₁, e₂, e₃} of R³, a dependent pair {(1,2,3), (2,4,6)}, and the distinction between fields and non-fields: Z/nZ is a field exactly when n is prime, and zero divisors witness the failure of the field property when n is composite.

## Visualizations

- **SVG**: Coordinate axes for R² with the standard basis vectors e₁ and e₂ drawn in blue and green, a general vector v = (3,2) in magenta with dashed decomposition lines, and a panel showing the standard basis for R³.
- **DOT**: Hierarchy graph showing a subspace W inside a vector space V, with `span{v1,...,vk}` spanning W and `{0}` as the trivial subspace.
- **TikZ**: Origin O with stealth arrows to e₁, e₂, and v = 2e₁ + 1.5e₂; dashed gray lines show the coordinate decomposition.
- **ASCII**: Standard basis for R³, a linear independence rank check for {e₁, e₂, e₃}, a rank check for the dependent pair {(1,2,3),(2,4,6)}, and a reminder of the subspace axioms.

## Default State

The default TOML state includes:
- `field_p`: current prime field modulus (`0` = real numbers)
