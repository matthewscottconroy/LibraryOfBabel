# Chapter 31 — Galois Theory

The fundamental theorem establishing an order-reversing bijection between subgroups of a Galois group and intermediate fields.

## Usage

### Interactive mode
```
cargo run -p ch31-galois-theory
```

### Non-interactive (scriptable)
```
cargo run -p ch31-galois-theory -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch31-galois-theory -- --run demo --format svg > output.svg
cargo run -p ch31-galois-theory -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch31-galois-theory -- --run demo --format tex > output.tex
cargo run -p ch31-galois-theory -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch31-galois-theory -- --run demo --save state.toml
cargo run -p ch31-galois-theory -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `galois_group <p_coeffs>` | Compute Galois group of polynomial (deg ≤ 4) |
| `discriminant <p_coeffs>` | Discriminant of quadratic or cubic |
| `correspondence <n>` | Galois correspondence for ℚ(ζ_n)/ℚ |
| `fixed_field <p_coeffs> <auto>` | Show fixed field for an automorphism |
| `sqrt2_sqrt3` | Full example: Gal(ℚ(√2,√3)/ℚ) |
| `cyclotomic_galois <n>` | Gal(ℚ(ζ_n)/ℚ) ≅ (ℤ/nℤ)* |
| `demo` | Run a showcase of key results |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

For a Galois extension K/F the Fundamental Theorem gives a bijection: subgroups H of Gal(K/F) correspond to intermediate fields K^H, with larger subgroups corresponding to smaller fields (order-reversing). The Galois group of a polynomial is computed from its discriminant: for cubics, Gal ≅ A₃ when √Δ ∈ ℚ and Gal ≅ S₃ otherwise. The cyclotomic field ℚ(ζ_n) has Galois group isomorphic to (ℤ/nℤ)*, with the automorphism σ_k sending ζ_n to ζ_n^k. The worked example Gal(ℚ(√2,√3)/ℚ) ≅ ℤ/2ℤ × ℤ/2ℤ illustrates all four subgroups and their three fixed fields ℚ(√2), ℚ(√3), ℚ(√6).

## Visualizations

- **SVG**: Side-by-side correspondence diagram for ℚ(ζ_n)/ℚ, with a subgroup lattice on the left (G, H, {e}) and the dual field lattice on the right (ℚ, K^H, ℚ(ζ_n)), connected by Fix/Gal labels.
- **DOT**: Directed graph with both the subgroup lattice and the field lattice, linked by dashed correspondence edges.
- **TikZ**: TikZ diagram of the same Galois correspondence with dashed bidirectional arrows labeled Fix.
- **ASCII**: Text-art two-column correspondence table showing the order-reversing pairing.

## Default State

- `n`: index of the cyclotomic field ℚ(ζ_n), default `5`
