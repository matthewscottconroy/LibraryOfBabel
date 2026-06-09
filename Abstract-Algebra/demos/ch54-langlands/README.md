# Chapter 54 — The Langlands Program

The web of conjectures and theorems unifying Galois representations, automorphic forms, and L-functions across number theory, geometry, and representation theory.

## Usage

### Interactive mode
```
cargo run -p ch54-langlands
```

### Non-interactive (scriptable)
```
cargo run -p ch54-langlands -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch54-langlands -- --run demo --format svg > output.svg
cargo run -p ch54-langlands -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch54-langlands -- --run demo --format tex > output.tex
cargo run -p ch54-langlands -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch54-langlands -- --run demo --save state.toml
cargo run -p ch54-langlands -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `local_langlands <p> <n>` | Local Langlands for GL_n(ℚ_p): correspondence |
| `weil_group <p>` | Weil group W_{ℚ_p}: Frobenius, inertia, structure |
| `l_function <a> <b>` | L-function of elliptic curve y²=x³+ax+b |
| `modularity <a> <b>` | Modularity theorem for elliptic curve y²=x³+ax+b |
| `galois_rep <p> <n>` | n-dim Galois representation mod p: Frobenius data |
| `geometric_langlands` | Geometric Langlands: D-modules and local systems |
| `demo` | Run a showcase of all commands |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The local Langlands correspondence for GL_n(ℚ_p) is stated as a bijection between irreducible smooth representations and n-dimensional Weil-Deligne representations, with n=1 (local class field theory) and n=2 (principal series, special, supercuspidal) described in detail. For elliptic curves y²=x³+ax+b, the program counts points over 𝔽_p for primes up to 47, computes a_p = p+1−#E(𝔽_p), assembles the local L-factors, and explains how the modularity theorem (Wiles-Taylor-Wiles) equates L(E,s) with an automorphic L-function, with the Birch–Swinnerton-Dyer conjecture noted. The geometric Langlands programme is described as an equivalence D^b(D-mod(Bun_G)) ≃ D^b(QCoh(LocSys_Ĝ(X))), with the Langlands dual group Ĝ tabulated and the Fargues-Scholze geometrisation mentioned.

## Visualizations

- **SVG**: Two-panel diagram with the arithmetic side (Galois reps, L-functions, elliptic curves, Weil group) on the left and the automorphic side (automorphic forms, Hecke eigenvalues, modular forms, geometric Langlands) on the right, connected by double-headed arrows.
- **DOT**: Graph with nodes for Galois reps, automorphic forms, L-functions, local LLC, and geometric Langlands, with directed edges showing their relationships.
- **TikZ**: Three-node diagram: Galois reps and automorphic forms at the top connected by a "Langlands" arrow, both pointing down to L-functions.
- **ASCII**: Two-column table comparing the arithmetic and automorphic sides, with the functoriality equation L(s,ρ)=L(s,π) and the geometric Langlands line.

## Default State

- `ec_a`: a-coefficient of the elliptic curve, default `-1`
- `ec_b`: b-coefficient of the elliptic curve, default `0`
- `prime`: prime p for local computations, default `5`
