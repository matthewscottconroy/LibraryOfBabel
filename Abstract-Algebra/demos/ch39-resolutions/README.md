# Chapter 39 — Resolutions

An interactive demo of free and projective resolutions, syzygy modules, projective dimension, Koszul complexes, and Hilbert's syzygy theorem, worked out over ℤ and polynomial rings.

## Usage

### Interactive mode
```
cargo run -p ch39-resolutions
```

### Non-interactive (scriptable)
```
cargo run -p ch39-resolutions -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch39-resolutions -- --run demo --format svg > output.svg
cargo run -p ch39-resolutions -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch39-resolutions -- --run demo --format tex > output.tex
cargo run -p ch39-resolutions -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch39-resolutions -- --run demo --save state.toml
cargo run -p ch39-resolutions -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `free_res <n>` | Free resolution 0→ℤ→(×n)→ℤ→ℤ/nℤ→0 over ℤ |
| `projective_res <n>` | Projective resolution of ℤ/nℤ (equals free over ℤ, a PID) |
| `syzygy <n> <k>` | Syzygy module: kernel of ℤᵏ→ℤ/nℤ with k generators |
| `pd <n>` | Projective dimension of ℤ/nℤ as a ℤ-module |
| `koszul <n>` | Koszul complex for (x) in ℤ[x]/(xⁿ) with nilpotent shift matrix |
| `hilbert_syzygy` | Hilbert's syzygy theorem illustrated for k[x], k[x,y], k[x,y,z] |
| `minimal_res <n>` | Minimal free resolution of ℤ/nℤ over ℤ[x]/(xⁿ) (periodic) |
| `demo` | Showcase of resolution concepts |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

A free resolution of a module M is an exact sequence …→P₁→P₀→M→0 where each Pᵢ is free. Over ℤ (a PID), every module has a resolution of length at most 1, so pd(ℤ/nℤ)=1, witnessed by 0→ℤ→(×n)→ℤ→ℤ/nℤ→0. The Koszul complex K(x) for the element x in ℤ[x]/(xⁿ) gives H₀=ℤ and H₁=ℤ/nℤ, and is the fundamental building block for resolutions over hypersurface rings. Hilbert's syzygy theorem states that over k[x₁,…,xₙ] every finitely generated module has a free resolution of length ≤ n, establishing global dimension n for polynomial rings.

## Visualizations

- **SVG**: Free resolution diagram 0→P₁=ℤ→(×n)→P₀=ℤ→ℤ/nℤ→0 with node labels indicating freeness and torsion, plus a note on pd=1.
- **DOT**: Directed graph of the resolution nodes 0, P₁, P₀, M, 0 with edges labeled ∂₁ and ε.
- **TikZ**: TikZ diagram of the four-node free resolution with labeled maps ∂₁ and ε.
- **ASCII**: Text resolution 0-->P_1--d_1-->P_0--eps-->M-->0 with pd formula.

## Default State

No persistent state; all computations are driven by command arguments.
