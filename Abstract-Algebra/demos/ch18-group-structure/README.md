# Chapter 18 — Structure of Groups

Explores how groups decompose into simple and abelian pieces via derived series, composition series, solvability, nilpotency, the Jordan-Hölder theorem, free groups, and the simplicity of the alternating groups.

## Usage

### Interactive mode
```
cargo run -p ch18-group-structure
```

### Non-interactive (scriptable)
```
cargo run -p ch18-group-structure -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch18-group-structure -- --run demo --format svg > output.svg
cargo run -p ch18-group-structure -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch18-group-structure -- --run demo --format tex > output.tex
cargo run -p ch18-group-structure -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch18-group-structure -- --run demo --save state.toml
cargo run -p ch18-group-structure -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `derived <Z\|D> <n>` | Commutator (derived) series G ≥ G' ≥ G'' ≥ … for ℤ/nℤ or D_n |
| `composition <Z\|D\|S> <n>` | Composition series and all composition factors for ℤ/nℤ, D_n, or S_n |
| `solvable <n>` | Derived series of S_n showing solvability for n ≤ 4 and non-solvability for n ≥ 5 |
| `simple <n>` | Simplicity proof outline for A_n: trivial for n=2,3; non-simple for n=4; simple for n≥5 |
| `free_word <n>` | Enumerate all reduced words of F_2 = ⟨a,b⟩ up to length n with growth formula |
| `jordan_holder` | Jordan-Hölder theorem for S_4: three composition series with identical factor multisets |
| `nilpotent <p> <a>` | Lower and upper central series of ℤ/p^aℤ proving nilpotency of class 1 |
| `demo` | Showcase: derived/composition series for the current group type and order |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

The derived series G^(0) = G, G^(i+1) = [G^(i), G^(i)] reaches {e} iff G is solvable; abelian groups are solvable of class 1, dihedral groups are solvable with D_n' = ⟨r²⟩, while S_n is not solvable for n ≥ 5 because A_n is a perfect, non-abelian simple group. The Jordan-Hölder theorem states that any two composition series of a group have the same length and the same multiset of composition factors (all of which are simple groups), making these factors invariants of G. Every p-group is nilpotent — its lower central series G_0 ≥ G_1 ≥ … terminates — because p-groups always have non-trivial centers by the class equation argument. The unsolvability of S_n for n ≥ 5 is the group-theoretic core of the Abel-Ruffini theorem on the insolubility of the general degree-5 polynomial by radicals.

## Visualizations

- **SVG**: Vertical chain of boxes for the derived series (G → G' → {e} for Z type; G → ⟨r²⟩ → {e} for D type), with arrows, factor labels, and a solvability annotation, followed by the list of composition factors.
- **DOT**: Linear chain of nodes representing the derived series, with "contains" edges connecting consecutive terms.
- **TikZ**: Vertical node chain connected by downward arrows, rendering the derived series as a Hasse diagram.
- **ASCII**: Step-by-step derived series with commutator annotations, solvability conclusion, and composition factor list.

## Default State

- `group_type`: `"Z"` — group type for the derived series (Z = cyclic, D = dihedral)
- `group_n`: `12` — group parameter n
