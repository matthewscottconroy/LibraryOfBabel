# Chapter 15 — Homomorphisms and the Isomorphism Theorems

Explores structure-preserving maps between groups: kernels, images, the first isomorphism theorem, the sign homomorphism, semidirect products, and automorphism groups.

## Usage

### Interactive mode
```
cargo run -p ch15-homomorphisms
```

### Non-interactive (scriptable)
```
cargo run -p ch15-homomorphisms -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch15-homomorphisms -- --run demo --format svg > output.svg
cargo run -p ch15-homomorphisms -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch15-homomorphisms -- --run demo --format tex > output.tex
cargo run -p ch15-homomorphisms -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch15-homomorphisms -- --run demo --save state.toml
cargo run -p ch15-homomorphisms -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `homomorphism <n> <m> <a>` | Map φ(x) = ax: ℤ/nℤ → ℤ/mℤ with full table and homomorphism verification |
| `kernel <n> <m> <a>` | Kernel of φ(x) = ax with element-by-element check |
| `image <n> <m> <a>` | Image of φ, identified as ⟨gcd(a,m)⟩ in ℤ/mℤ |
| `first_iso <n> <a>` | First isomorphism theorem: explicit isomorphism ℤ/nℤ / ker ≅ im(φ) |
| `sign <n>` | Sign homomorphism Sₙ → {±1} with inversion counts; kernel = Aₙ |
| `semidirect <n>` | D_n as ℤ/nℤ ⋊ ℤ/2ℤ: normal subgroup, complement, and action |
| `automorphisms <n>` | Aut(ℤ/nℤ) ≅ (ℤ/nℤ)* listing all automorphisms φ_a(x) = ax |
| `demo` | Showcase: φ(x) = 2x from ℤ/6ℤ to ℤ/3ℤ with kernel, image, and quotient |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

A group homomorphism φ: G → H satisfies φ(xy) = φ(x)φ(y); the kernel ker(φ) is a normal subgroup of G and the image im(φ) is a subgroup of H. The first isomorphism theorem states G/ker(φ) ≅ im(φ), giving an explicit bijection between cosets and image elements. For φ(x) = ax on ℤ/nℤ, the image is ⟨gcd(a,m)⟩ and the kernel has order n/|im(φ)|. The automorphism group Aut(ℤ/nℤ) ≅ (ℤ/nℤ)* via the correspondence a ↦ φ_a, and the dihedral group D_n is the semidirect product ℤ/nℤ ⋊ ℤ/2ℤ where the nontrivial element of ℤ/2ℤ acts by inversion.

## Visualizations

- **SVG**: Function diagram with domain ℤ/nℤ on the left and codomain ℤ/mℤ on the right; kernel elements highlighted in red, image elements in green, arrows showing each mapping φ(x), and the first isomorphism theorem labeled below.
- **DOT**: Graph with G, H, ker(φ), im(φ), and G/ker as nodes, connected by labeled edges for the map, kernel, image, and the isomorphism dashed arrow.
- **TikZ**: Ellipse nodes for G, H, ker(φ), im(φ), and G/ker with arrows for φ, the kernel and image inclusions, and a dashed isomorphism arrow G/ker → im(φ).
- **ASCII**: Table of x and φ(x) with kernel membership markers, followed by ker and im sets and the first isomorphism theorem order identity.

## Default State

- `n`: `6` — domain group order (ℤ/6ℤ)
- `m`: `3` — codomain group order (ℤ/3ℤ)
- `a`: `2` — multiplication factor (φ(x) = 2x)
