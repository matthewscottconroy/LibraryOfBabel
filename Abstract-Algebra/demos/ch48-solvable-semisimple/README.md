# Chapter 48 — Solvable & Semisimple Lie Algebras

The structure theory of Lie algebras via derived series, Engel's theorem, Lie's theorem, and the Levi decomposition.

## Usage

### Interactive mode
```
cargo run -p ch48-solvable-semisimple
```

### Non-interactive (scriptable)
```
cargo run -p ch48-solvable-semisimple -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch48-solvable-semisimple -- --run demo --format svg > output.svg
cargo run -p ch48-solvable-semisimple -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch48-solvable-semisimple -- --run demo --format tex > output.tex
cargo run -p ch48-solvable-semisimple -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch48-solvable-semisimple -- --run demo --save state.toml
cargo run -p ch48-solvable-semisimple -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `derived <n>` | Derived series of gl(2): g⁽⁰⁾⊃g⁽¹⁾⊃... up to n steps |
| `lower_central <n>` | Lower central series of gl(2): g₀⊃g₁=[g,g₀]⊃... |
| `is_solvable <type>` | Is gl2, sl2, b (upper-tri), n (strict upper-tri) solvable? |
| `levi` | Levi decomposition: g = rad ⋊ semisimple |
| `engel` | Engel's theorem: all ad(x) nilpotent ⟹ g nilpotent |
| `lie_theorem` | Lie's theorem: solvable ⟹ common eigenvector over ℂ |
| `demo` | Run a showcase of all commands |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

Derived and lower central series are computed for gl(2) over integer structure constants using Gaussian elimination to track subspace dimensions. The solvability of the four algebras gl2, sl2, b, and n is analysed: the Borel subalgebra b (upper triangular) is solvable with b ⊃ n ⊃ 0, while sl(2) is perfect ([sl(2),sl(2)] = sl(2)) and not solvable. The Levi decomposition g = rad(g) ⋊ s is illustrated for gl(2) = ℝ·I ⊕ sl(2) and for b = n ⋊ h. Engel's theorem (all ad(x) nilpotent ⟺ g nilpotent) and Lie's theorem (solvable algebra over ℂ has common eigenvector) are explained with explicit examples.

## Visualizations

- **SVG**: Box diagram splitting g into the solvable radical rad(g) and Levi factor s, with a semidirect product arrow between them, and the derived series gl(2) ⊃ sl(2) ⊃ sl(2) noted at the bottom.
- **DOT**: Directed graph with nodes g, rad(g), s, and 0, with edges labelled "ideal", "complement", and "derived series".
- **TikZ**: Three-node diagram: g at the top, rad(g) and s below, with labelled arrows.
- **ASCII**: Two side-by-side boxes for rad(g) and the Levi factor s, with the derived and nilpotent series written out below.

## Default State

- `algebra`: type used by `is_solvable` when no argument given, default `"b"`
- `series_steps`: number of steps computed by `derived` and `lower_central`, default `3`
