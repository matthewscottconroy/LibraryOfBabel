# Chapter 24 — Commutative Algebra

Explores Noetherian rings, localization, the Hilbert Nullstellensatz, Krull dimension, primary decomposition, and integral elements.

## Usage

### Interactive mode
```
cargo run -p ch24-commutative-algebra
```

### Non-interactive (scriptable)
```
cargo run -p ch24-commutative-algebra -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch24-commutative-algebra -- --run demo --format svg > output.svg
cargo run -p ch24-commutative-algebra -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch24-commutative-algebra -- --run demo --format tex > output.tex
cargo run -p ch24-commutative-algebra -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch24-commutative-algebra -- --run demo --save state.toml
cargo run -p ch24-commutative-algebra -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `noetherian <n>` | Ascending chain condition on ideals of Z/nZ; maximal chain |
| `localize <p>` | Describe Z_(p): inverted elements, maximal ideal, residue field, DVR |
| `nullstellensatz <f...> <p>` | Zero set V(f) and I(V(f)) over F_p |
| `krull_z` | Krull dimension of Z: prime chains, dim = 1 |
| `krull_poly <n>` | Krull dimension of k[x_1,...,x_n] = n |
| `primary_decomp <n>` | Primary decomposition of (0) in Z/nZ |
| `integral <a> <coeffs>` | Check if a satisfies a given monic polynomial with integer coefficients |
| `demo` | Showcase: noetherian(12), localize(3), Krull dim(Z), primary_decomp(12) |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

A Noetherian ring satisfies the ascending chain condition on ideals; Z/nZ is both Noetherian and Artinian, with its ideal chains corresponding to chains of divisors of n. Localization at a prime p inverts all elements not divisible by p, producing the local ring Z_(p) with unique maximal ideal (p) and residue field F_p; Z_(p) is a discrete valuation ring with uniformizer p. The Krull dimension of a ring is the supremum of lengths of prime ideal chains: dim Z = 1, while dim k[x_1,...,x_n] = n for any field k. Primary decomposition expresses an ideal as an intersection of primary ideals, generalizing prime factorization to the ideal-theoretic setting.

## Visualizations

- **SVG**: Prime spectrum Spec(Z/nZ) showing the ring at top, prime ideals (p) as nodes with their residue fields F_p, and the zero ideal at bottom, connected by lines indicating the prime chain.
- **DOT**: The same spectrum as a top-to-bottom directed graph, with maximal ideal edges labeled and dashed edges from primes to (0).
- **TikZ**: Spec(Z/nZ) with the ring at top, prime ideal nodes in the middle tier, and (0) at bottom, connected by arrows.
- **ASCII**: Tabular list of prime ideals and their residue fields, Krull dimension of Z/nZ, Z, and k[x_1,...,x_n].

## Default State

- `last_n`: ring modulus for Noetherian/primary decomposition display; initial value `12`
- `last_p`: prime for localization display; initial value `3`
