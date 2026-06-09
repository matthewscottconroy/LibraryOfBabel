# Chapter 52 — Geometric Representation Theory

Flag varieties, Schubert calculus, perverse sheaves, and the BGG resolution as geometric approaches to representation theory.

## Usage

### Interactive mode
```
cargo run -p ch52-geometric
```

### Non-interactive (scriptable)
```
cargo run -p ch52-geometric -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch52-geometric -- --run demo --format svg > output.svg
cargo run -p ch52-geometric -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch52-geometric -- --run demo --format tex > output.tex
cargo run -p ch52-geometric -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch52-geometric -- --run demo --save state.toml
cargo run -p ch52-geometric -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `flag_variety <n>` | Flag variety GL_n/B: dimension, Bruhat cell decomposition |
| `schubert <perm>` | Schubert cell C_w for permutation w ∈ Sₙ, dim=inv(w) |
| `springer` | Springer correspondence: nilpotent orbits ↔ Weyl group reps |
| `perverse` | Perverse sheaves on flag variety: IC sheaves |
| `bgg` | BGG resolution: 0→M(w₀·λ)→...→M(λ)→L(λ)→0 |
| `demo` | Run a showcase of all commands |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The complete flag variety FL(n) = GL_n/B is decomposed into Bruhat cells C_w ≅ ℂ^{l(w)} for each w ∈ Sₙ, with the Poincaré polynomial computed from cell dimensions. The Schubert variety X_w = closure(C_w) has dimension l(w) = #inversions(w), and the Bruhat order is described. The Springer correspondence between nilpotent orbits in gl_n (indexed by partitions of n) and irreducible representations of Sₙ is tabulated for n=2,3,4. The BGG resolution 0 → M(w₀·λ) → ... → M(λ) → L(λ) → 0 is given explicitly for sl(2) and sl(3), connecting to the Weyl character formula via the Euler characteristic in the Grothendieck group.

## Visualizations

- **SVG**: Layered diagram of Schubert cells for FL(n) grouped by codimension l(w), with each cell as a labelled circle node and the length l shown on the left.
- **DOT**: Bruhat order graph for Sₙ (up to n=3), with cover relations shown as directed edges between permutation nodes.
- **TikZ**: Grid layout of Schubert cell nodes by dimension layer, with inner-sep style for compact display.
- **ASCII**: Cells grouped by codimension l=0, 1, 2, ..., with the dimension and total cell count at the top.

## Default State

- `flag_n`: n used by `flag_variety` when no argument given, default `3`
