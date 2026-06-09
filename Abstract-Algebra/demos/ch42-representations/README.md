# Chapter 42 — Group Representations

An interactive demo of irreducible representations and character tables for cyclic, dihedral, and symmetric groups, including Schur's lemma, the regular representation, and tensor products of irreps.

## Usage

### Interactive mode
```
cargo run -p ch42-representations
```

### Non-interactive (scriptable)
```
cargo run -p ch42-representations -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch42-representations -- --run demo --format svg > output.svg
cargo run -p ch42-representations -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch42-representations -- --run demo --format tex > output.tex
cargo run -p ch42-representations -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch42-representations -- --run demo --save state.toml
cargo run -p ch42-representations -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `irreps <type> <n>` | List irreps: Z=ℤ/nℤ, D=dihedral Dₙ, S=symmetric Sₙ (n=2,3,4) |
| `character <type> <n>` | Full character table with roots-of-unity entries |
| `schur <n>` | Schur's lemma for ℤ/nℤ: End_G(V)≅ℂ for each irrep |
| `decompose <type> <n> <vals>` | Decompose a character into irreducibles via inner product |
| `regular <n>` | Regular representation of ℤ/nℤ: each irrep appears once |
| `direct_sum <vals> / <vals>` | Direct sum of two characters (pointwise addition) |
| `tensor_rep <N> <k1> <k2>` | Tensor product ρ_k1 ⊗ ρ_k2 of irreps of ℤ/NℤZ |
| `demo` | Showcase of representation theory |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

A representation of G is a group homomorphism ρ: G→GL(V); for abelian G every complex irrep is one-dimensional and given by a character ρ_k(j)=e^{2πijk/n}. Schur's lemma states that any G-equivariant map between irreducible representations is either zero or an isomorphism, and that End_G(V)≅ℂ when k is algebraically closed. The regular representation ℂ[G] decomposes as the direct sum of all irreps, each appearing with multiplicity equal to its dimension (Peter-Weyl). For ℤ/nℤ the tensor product of irreps satisfies ρ_k⊗ρ_l≅ρ_{k+l mod n}, making the set of irreps a group isomorphic to ℤ/nℤ itself.

## Visualizations

- **SVG**: The four group elements of ℤ/4ℤ arranged on a circle, with the value ρ_k(1) for each of the four irreps listed in the lower left.
- **DOT**: Bipartite graph from G=ℤ/nℤ to irreps ρ₀, ρ₁, ρₙ₋₁ and from each irrep to GL₁(ℂ).
- **TikZ**: Diagram with G branching to representations V and W, both mapping into GL(V).
- **ASCII**: Character table for ℤ/4ℤ listing all four 1-dimensional irreps with values at each group element.

## Default State

No persistent state; all computations are driven by command arguments.
