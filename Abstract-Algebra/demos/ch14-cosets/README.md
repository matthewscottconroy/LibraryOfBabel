# Chapter 14 — Cosets, Normal Subgroups, and Quotient Groups

Explores how subgroups partition a group into cosets, Lagrange's theorem, normality, quotient group construction, and the circle group ℝ/ℤ.

## Usage

### Interactive mode
```
cargo run -p ch14-cosets
```

### Non-interactive (scriptable)
```
cargo run -p ch14-cosets -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch14-cosets -- --run demo --format svg > output.svg
cargo run -p ch14-cosets -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch14-cosets -- --run demo --format tex > output.tex
cargo run -p ch14-cosets -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch14-cosets -- --run demo --save state.toml
cargo run -p ch14-cosets -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `cosets <n> <gens...>` | Left cosets of ⟨generators⟩ in ℤ/nℤ with partition verification |
| `lagrange <n> <k>` | Verify Lagrange's theorem for the subgroup ⟨k⟩ in ℤ/nℤ |
| `normal <n> <subgroup_gens...>` | Check normality of a subgroup in D_n by testing gHg⁻¹ ⊆ H |
| `quotient <n> <k>` | Construct quotient group ℤ/nℤ / ⟨k⟩ with its Cayley table |
| `index <n> <subgroup_size>` | Compute the index [G:H] = |G|/|H| |
| `left_right <n> <gens...>` | Compare left and right cosets of ⟨gens⟩ in ℤ/nℤ to test normality |
| `rz` | ℝ/ℤ ≅ circle group S¹: fractional parts mapped to e^(2πix) |
| `demo` | Showcase: coset partition of ℤ/12ℤ by ⟨4⟩ |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

The left cosets of a subgroup H in G partition G into disjoint sets of equal size; Lagrange's theorem states |G| = |H| · [G:H]. A subgroup N is normal iff every left coset equals the corresponding right coset (equivalently, gNg⁻¹ = N for all g), making the set of cosets into a quotient group G/N under the operation (aN)(bN) = (ab)N. The circle group ℝ/ℤ ≅ S¹ arises as the quotient of ℝ by ℤ, and its characters χₙ(x) = e^(2πinx) are the basis of classical Fourier analysis.

## Visualizations

- **SVG**: Partition diagram with G = ℤ/nℤ at the top and each coset displayed as a labeled box below, with arrows from G to each coset and the Lagrange equation in the caption.
- **DOT**: Graph with G and H as nodes, each coset as a separate node, all connected to G by dashed edges, and the subgroup order labeled on the G→H edge.
- **TikZ**: Hasse-style diagram with G at the top, coset nodes in a row below connected by arrows, and H at the bottom with |G|, |H|, and [G:H] annotated.
- **ASCII**: Tabular listing of H = ⟨k⟩, the index [G:H], and each coset rep+H with its element set, followed by the Lagrange statement.

## Default State

- `group_n`: `12` — group order (ℤ/12ℤ)
- `subgroup_gen`: `4` — generator of the default subgroup ⟨4⟩
