# Chapter 50 — Highest Weight Theory

Classifying finite-dimensional representations of semisimple Lie algebras by their highest weights.

## Usage

### Interactive mode
```
cargo run -p ch50-highest-weight
```

### Non-interactive (scriptable)
```
cargo run -p ch50-highest-weight -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch50-highest-weight -- --run demo --format svg > output.svg
cargo run -p ch50-highest-weight -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch50-highest-weight -- --run demo --format tex > output.tex
cargo run -p ch50-highest-weight -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch50-highest-weight -- --run demo --save state.toml
cargo run -p ch50-highest-weight -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `weights <n>` | Weight spaces of V(n) for sl(2): weights n, n−2, ..., −n |
| `verma <lambda>` | Verma module M(λ): infinite-dim, weights λ, λ−2, λ−4, ... |
| `dim_formula <lambda>` | Weyl dimension formula for sl(2): dim V(λ) = λ+1 |
| `tensor_product <n> <m>` | Clebsch-Gordan: V(n)⊗V(m) = ⊕ V(k) |
| `character <n>` | Character of V(n): ch = Σ e^μ over weights |
| `dominant <a> <b>` | Check if (a,b) is a dominant integral weight for A2 |
| `demo` | Run a showcase of all commands |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The irreducible sl(2)-module V(n) is constructed with explicit basis vectors v_{n}, v_{n-2}, ..., v_{-n} and the action of e, f, h on each; the Clebsch-Gordan decomposition V(n)⊗V(m) = ⊕ V(k) is computed with a dimension check. Verma modules M(λ) are described with the singular vector at weight -(λ+2) giving the maximal proper submodule, and the irreducible quotient L(λ) = V(λ). The Weyl character formula and Weyl dimension formula are derived for sl(2) and tabulated for the A2 case (dim V(a,b) = (a+1)(b+1)(a+b+2)/2). Dominant integral weights for A2 are checked and their weight lattices partially enumerated.

## Visualizations

- **SVG**: Weight lattice diagram for V(n) showing each weight space as a labeled blue circle on a horizontal axis, with red f-arrows connecting adjacent weights.
- **DOT**: Linear chain of weight-space nodes connected by f-arrows.
- **TikZ**: Row of circle nodes labelled v_{+k} at equally spaced x-positions, connected left-to-right.
- **ASCII**: Weights listed in a chain separated by "← f →" arrows, with dimension and action formulas below.

## Default State

- `weight`: highest weight n used by `weights`, `verma`, `dim_formula`, `character`, default `3`
- `dominant_a`: a-coordinate for `dominant`, default `2`
- `dominant_b`: b-coordinate for `dominant`, default `1`
