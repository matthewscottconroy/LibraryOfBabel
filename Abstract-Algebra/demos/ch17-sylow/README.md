# Chapter 17 — The Sylow Theorems

Applies the three Sylow theorems to analyze prime-power subgroups, constrain the number of Sylow subgroups, test for simplicity, and classify groups of small order.

## Usage

### Interactive mode
```
cargo run -p ch17-sylow
```

### Non-interactive (scriptable)
```
cargo run -p ch17-sylow -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch17-sylow -- --run demo --format svg > output.svg
cargo run -p ch17-sylow -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch17-sylow -- --run demo --format tex > output.tex
cargo run -p ch17-sylow -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch17-sylow -- --run demo --save state.toml
cargo run -p ch17-sylow -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `sylow <n>` | List all Sylow p-subgroups of ℤ/nℤ for each prime p dividing n |
| `count_sylow <order> <p>` | All possible values of n_p satisfying Sylow III: n_p \| m and n_p ≡ 1 mod p |
| `simple_check <n>` | Use Sylow constraints to determine if a group of order n must be non-simple |
| `classify <n>` | Classify groups of order n ≤ 30 using Sylow analysis and known results |
| `p_group <p> <a>` | Properties of groups of order p^a: non-trivial center, nilpotency, lower central series |
| `sylow_conj <n> <p>` | Verify that all Sylow p-subgroups of ℤ/nℤ are conjugate (trivially, since abelian) |
| `demo` | Showcase: Sylow subgroup analysis for the current group order |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

Sylow I guarantees that for each prime power p^a exactly dividing |G|, a subgroup of order p^a exists; Sylow II states that all such Sylow p-subgroups are conjugate; Sylow III constrains the count n_p to satisfy n_p | (|G|/p^a) and n_p ≡ 1 (mod p). If n_p = 1 is forced, the unique Sylow subgroup is normal, which is a powerful tool for ruling out simplicity of groups of composite order. Every p-group has a non-trivial center (proved via the class equation), is nilpotent, and can be analyzed via its lower central series.

## Visualizations

- **SVG**: G = ℤ/nℤ drawn at center with one colored box per prime factor showing Syl_p(G), its order, and generator, connected by arrows from G, followed by the three Sylow theorem statements.
- **DOT**: Lattice with G at top, one Syl_p(G) ellipse node per prime factor in the middle, and the trivial subgroup {e} at the bottom, with containment edges.
- **TikZ**: Vertical Hasse diagram with G at top, Sylow subgroup ellipses spread horizontally below, and {e} at the bottom, connected by arrows.
- **ASCII**: Factorization of n, one line per prime with Syl_p order, generator, and possible n_p values, followed by the three Sylow theorem statements.

## Default State

- `group_n`: `12` — group order for Sylow analysis
- `prime_p`: `2` — prime for Sylow subgroup queries
