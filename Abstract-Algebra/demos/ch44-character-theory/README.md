# Chapter 44 — Character Theory

An interactive demo of character tables, orthogonality relations, inner products of characters, the class function basis, and Burnside's p^a·q^b solvability theorem, for cyclic, symmetric, dihedral, and Klein four-groups.

## Usage

### Interactive mode
```
cargo run -p ch44-character-theory
```

### Non-interactive (scriptable)
```
cargo run -p ch44-character-theory -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch44-character-theory -- --run demo --format svg > output.svg
cargo run -p ch44-character-theory -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch44-character-theory -- --run demo --format tex > output.tex
cargo run -p ch44-character-theory -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch44-character-theory -- --run demo --save state.toml
cargo run -p ch44-character-theory -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `table <type> <n>` | Print character table: Z, S2, S3, S4, D3, D4, V4 |
| `orthogonality <type> <n>` | Verify row and column orthogonality relations |
| `inner_product <chi1> / <chi2> <type> <n>` | Inner product ⟨χ₁, χ₂⟩ = (1/|G|)Σ|C|χ₁(C)χ₂(C)̄ |
| `detect_abelian <type> <n>` | Check if G is abelian from character degrees |
| `burnside_paqb <p> <a> <q> <b>` | Burnside p^a·q^b solvability theorem with proof sketch |
| `conjugacy_classes <type> <n>` | List conjugacy classes with sizes |
| `class_functions <type> <n>` | Verify irreducible characters form an orthonormal basis |
| `demo` | Showcase: character tables and orthogonality |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

The character χ_V of a representation V sends each group element to the trace of its action matrix; characters are class functions constant on conjugacy classes. The first and second orthogonality relations are verified numerically: row orthogonality ⟨χᵢ, χⱼ⟩=δᵢⱼ and column orthogonality Σᵢ χᵢ(C)χᵢ(D)̄=(|G|/|C|)δ_{CD}. Irreducible characters form an orthonormal basis for the space of class functions, so a character is irreducible iff its self-inner-product is 1. Burnside's theorem that any group of order p^a·q^b is solvable is proved using the character-theoretic lemma that if |χ(g)|=dim χ for a non-trivial irrep then g is central, combined with Sylow theory.

## Visualizations

- **SVG**: Character table of S₃ with class sizes in a header row, followed by χ₁, χ₂, χ₃, plus orthogonality verification ⟨χ₃,χ₃⟩=(4+0+2)/6=1 and a Burnside solvability note.
- **DOT**: Bipartite graph from G to irreducible characters χ₁, χ₂, χ₃ and from those to the orthogonality node ⟨χᵢ,χⱼ⟩=δᵢⱼ.
- **TikZ**: Three character nodes χ₁, χ₂, χ₃ each with an arrow to the orthogonality formula node.
- **ASCII**: Character table of S₃ in plain text with row orthogonality reminder and the dimension sum formula.

## Default State

No persistent state; all computations are driven by command arguments.
