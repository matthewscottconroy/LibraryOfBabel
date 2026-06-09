# Chapter 19 — Finitely Generated Abelian Groups

Classifies all abelian groups of a given order using the structure theorem, Smith normal form, and primary decomposition.

## Usage

### Interactive mode
```
cargo run -p ch19-abelian-groups
```

### Non-interactive (scriptable)
```
cargo run -p ch19-abelian-groups -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch19-abelian-groups -- --run demo --format svg > output.svg
cargo run -p ch19-abelian-groups -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch19-abelian-groups -- --run demo --format tex > output.tex
cargo run -p ch19-abelian-groups -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch19-abelian-groups -- --run demo --save state.toml
cargo run -p ch19-abelian-groups -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `classify <n>` | All abelian groups of order n |
| `smith <r> <c> <entries>` | Smith normal form of an integer matrix |
| `invariant_factors <d...>` | Write group from invariant factors |
| `primary <d...>` | Convert invariant factors to primary decomposition |
| `is_cyclic <n>` | Check if Z/nZ is cyclic (and show CRT decomposition) |
| `homology <entries>` | H_1 of a simplicial 1-complex via boundary matrix |
| `all_to_30` | List all abelian groups of orders 1–30 |
| `demo` | Showcase: classify orders 1, 4, 6, 8, 12, 16, 24, 36 |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

Every finitely generated abelian group is isomorphic to a direct sum Z^r + Z/d_1 + ... + Z/d_k where d_1 | d_2 | ... | d_k; the classification theorem identifies all such groups of a given finite order by partitioning the exponents of its prime factors. The Smith normal form of an integer matrix computes these invariant factors via row and column operations over Z. Primary decomposition further refines each cyclic summand Z/d_i into prime-power components Z/p^e, connecting the invariant factor form to the primary form.

## Visualizations

- **SVG**: Rounded-rectangle tiles listing each isomorphism class for the last-classified order, with the prime factorization shown below.
- **DOT**: A star graph rooted at a node labeled |G|=n with one leaf per isomorphism class.
- **TikZ**: The same star layout using `\mathbb{Z}` notation and `\oplus` for direct sums.
- **ASCII**: A numbered list of all isomorphism classes for the last-classified order.

## Default State

- `last_n`: the most recently classified order; initial value `12`
