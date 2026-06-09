# Chapter 53 — Quantum Groups

q-deformations of universal enveloping algebras, crystal bases, the Yang-Baxter equation, and connections to knot theory.

## Usage

### Interactive mode
```
cargo run -p ch53-quantum-groups
```

### Non-interactive (scriptable)
```
cargo run -p ch53-quantum-groups -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch53-quantum-groups -- --run demo --format svg > output.svg
cargo run -p ch53-quantum-groups -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch53-quantum-groups -- --run demo --format tex > output.tex
cargo run -p ch53-quantum-groups -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch53-quantum-groups -- --run demo --save state.toml
cargo run -p ch53-quantum-groups -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `uq_sl2` | Generators E, F, K of U_q(sl(2)) and relations |
| `q_number <n> <qnum> <qden>` | Quantum integer [n]_q = (q^n−q^{−n})/(q−q^{−1}) |
| `quantum_dim <n> <qnum> <qden>` | Quantum dimension [n+1]_q |
| `crystal <n>` | Crystal basis of V(n) for sl(2) |
| `root_of_unity <l>` | U_q at l-th root of unity: what changes |
| `yang_baxter` | Yang-Baxter equation and braid group connection |
| `demo` | Run a showcase of all commands |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The quantum group U_q(sl(2)) is presented with generators E, F, K, K⁻¹ and its Hopf algebra structure (comultiplication, counit, antipode); as q → 1 the algebra degenerates to U(sl(2)). Quantum integers [n]_q = (q^n − q^{−n})/(q − q^{−1}) and Gaussian binomials are computed numerically. At a primitive l-th root of unity, E^l, F^l, K^l become central, representation theory changes dramatically, and the modular data (quantum dimensions sin(kπ/l)/sin(π/l)) is tabulated; the link to characteristic-p representation theory via Lusztig's divided-power form is explained. The R-matrix for V(1)⊗V(1) is displayed, the Yang-Baxter equation R₁₂R₁₃R₂₃ = R₂₃R₁₃R₁₂ is verified at q=3/2, and the connection to braid group representations and the Jones polynomial is given.

## Visualizations

- **SVG**: Side-by-side boxes comparing sl(2) (q→1 limit) and U_q(sl(2)) with their defining relations, an arrow for the q-deformation, and labels for the crystal basis and R-matrix features.
- **DOT**: Crystal graph for V(n): nodes b_0, ..., b_n labelled with weights, connected by f̃ arrows.
- **TikZ**: Linear chain of crystal basis nodes at x = 0, 2, 4, ... connected by f̃ arrows.
- **ASCII**: Crystal graph chain written as b_0 -f~-> b_1 -f~-> ... with notes on q-deformation and Yang-Baxter.

## Default State

- `q_num`: numerator of q, default `3.0`
- `q_den`: denominator of q, default `2.0`
- `crystal_n`: highest weight for `crystal`, default `3`
