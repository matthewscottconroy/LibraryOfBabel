# Chapter 16 — Group Actions

Explores how groups act on sets: orbits, stabilizers, the orbit-stabilizer theorem, Burnside's lemma for counting colorings up to symmetry, the class equation, and Cayley's embedding theorem.

## Usage

### Interactive mode
```
cargo run -p ch16-group-actions
```

### Non-interactive (scriptable)
```
cargo run -p ch16-group-actions -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch16-group-actions -- --run demo --format svg > output.svg
cargo run -p ch16-group-actions -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch16-group-actions -- --run demo --format tex > output.tex
cargo run -p ch16-group-actions -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch16-group-actions -- --run demo --save state.toml
cargo run -p ch16-group-actions -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `orbit <n> <g> <x>` | Orbit of x under ⟨g⟩ acting on ℤ/nℤ by addition |
| `stabilizer <n> <g> <x>` | Stabilizer of x under the g-action, with orbit-stabilizer verification |
| `orbit_stab <n> <g>` | Verify |Orb(x)| · |Stab(x)| = |⟨g⟩| for every x in ℤ/nℤ |
| `burnside <n> <k>` | Count distinct k-colorings of an n-cycle up to rotation using Burnside's lemma |
| `class_eq <Z\|D> <n>` | Class equation for ℤ/nℤ (each singleton) or D_n (rotation and reflection classes) |
| `cayley_action <n>` | Cayley's theorem: embed ℤ/nℤ into S_n as left-multiplication permutations |
| `necklace <n> <k>` | Count distinct necklaces with n beads and k colors under the full dihedral action |
| `demo` | Showcase: orbits of ℤ/6ℤ under +2 and Burnside 2-colorings |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

A group action of G on a set X assigns to each g a bijection of X such that (gh)·x = g·(h·x); the orbit Orb(x) and stabilizer Stab(x) satisfy |Orb(x)| · |Stab(x)| = |G| (orbit-stabilizer theorem). Burnside's lemma counts orbits as (1/|G|) Σ_g |Fix(g)|, reducing combinatorial enumeration to fixed-point counts; for rotation of an n-cycle, Fix(rotation by d) = k^gcd(d,n). The class equation |G| = |Z(G)| + Σ [G:C_G(g)] expresses |G| as a sum over conjugacy classes, and Cayley's theorem embeds any group G into S_{|G|} via the left-multiplication action.

## Visualizations

- **SVG**: Elements of ℤ/nℤ arranged in a circle, with arrows showing the generator's action (+g), nodes colored by orbit membership, and a legend identifying each orbit.
- **DOT**: Directed graph with nodes 0..n−1 and edges i → (i+g) mod n labeled "+g", showing the cyclic orbit structure.
- **TikZ**: Row of nodes 0..n−1 with bent arrows connecting each node to its image under the action.
- **ASCII**: Orbit list with arrow-chain notation (0 → g → 2g → …) and a Burnside 2-coloring count.

## Default State

- `group_n`: `6` — set size and group order (ℤ/6ℤ)
- `gen`: `2` — generator of the acting subgroup
