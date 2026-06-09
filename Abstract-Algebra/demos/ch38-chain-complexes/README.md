# Chapter 38 — Chain Complexes & Homology

An interactive demo of chain complexes, the boundary condition ∂∘∂=0, homology groups, and Euler characteristic, with built-in triangulations of S¹, S², and T².

## Usage

### Interactive mode
```
cargo run -p ch38-chain-complexes
```

### Non-interactive (scriptable)
```
cargo run -p ch38-chain-complexes -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch38-chain-complexes -- --run demo --format svg > output.svg
cargo run -p ch38-chain-complexes -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch38-chain-complexes -- --run demo --format tex > output.tex
cargo run -p ch38-chain-complexes -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch38-chain-complexes -- --run demo --save state.toml
cargo run -p ch38-chain-complexes -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `complex <d1_rows> <d1_cols> <entries...> ; <d2_rows> <d2_cols> <entries...>` | Define chain complex by boundary matrices |
| `homology <same format>` | Compute homology Hₙ = ker(∂ₙ)/im(∂ₙ₊₁) via Smith normal form |
| `circle` | Built-in: homology of S¹ (3 vertices, 3 edges) |
| `sphere` | Built-in: homology of S² (tetrahedron surface) |
| `torus` | Built-in: homology of T² |
| `euler <same format>` | Compute Euler characteristic χ = Σ(−1)ⁱ rank(Hᵢ) |
| `chain_homotopy` | Example of chain homotopy inducing the same map on homology |
| `demo` | Showcase: circle, sphere, torus homology |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

A chain complex is a sequence of abelian groups Cₙ with boundary maps ∂ₙ: Cₙ→Cₙ₋₁ satisfying ∂ₙ∘∂ₙ₊₁=0. Homology groups Hₙ = ker(∂ₙ)/im(∂ₙ₊₁) measure how far the complex is from being exact, and are computed here via Smith normal form of the integer boundary matrices. Chain homotopies between chain maps induce the same map on homology, which is the algebraic foundation of homotopy invariance. The demo includes the standard triangulations of S¹ (H₀=ℤ, H₁=ℤ), S² (H₀=ℤ, H₁=0, H₂=ℤ), and T² (H₀=ℤ, H₁=ℤ², H₂=ℤ).

## Visualizations

- **SVG**: Horizontal chain complex diagram …→Cₙ₊₁→Cₙ→Cₙ₋₁→… with the relation ∂∘∂=0 and the homology groups of S¹, S², and T² listed.
- **DOT**: Directed graph of chain complex nodes …, Cₙ₊₁, Cₙ, Cₙ₋₁, … with edges labeled ∂ₙ₊₁ and ∂ₙ.
- **TikZ**: TikZ diagram of three chain complex nodes with labeled boundary arrows.
- **ASCII**: Text chain complex with the key property d∘d=0 and homology groups of the three built-in examples.

## Default State

No persistent state; all computations are driven by command arguments.
