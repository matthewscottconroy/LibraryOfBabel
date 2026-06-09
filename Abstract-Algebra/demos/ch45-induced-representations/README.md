# Chapter 45 — Induced Representations

An interactive demo of induced and restricted representations, Frobenius reciprocity (Ind ⊣ Res adjunction), Mackey's formula, and Frobenius groups, worked out for subgroups of cyclic groups and the affine group Aff(ℤ/nℤ).

## Usage

### Interactive mode
```
cargo run -p ch45-induced-representations
```

### Non-interactive (scriptable)
```
cargo run -p ch45-induced-representations -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch45-induced-representations -- --run demo --format svg > output.svg
cargo run -p ch45-induced-representations -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch45-induced-representations -- --run demo --format tex > output.tex
cargo run -p ch45-induced-representations -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch45-induced-representations -- --run demo --save state.toml
cargo run -p ch45-induced-representations -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `induce <G_size> <H_size> <chi_H>` | Compute induced character Ind_H^G(χ) and decompose into G-irreps |
| `restrict <G_chi> / <H_indices>` | Restrict a G-character to elements of H |
| `frobenius_reciprocity <Gs> <Hs> <chiH> <psiG>` | Verify ⟨Ind χ, ψ⟩_G = ⟨χ, Res ψ⟩_H numerically |
| `induce_trivial <n> <k>` | Induce the trivial character of kℤ/nℤ to ℤ/nℤ |
| `mackey <G> <H> <K>` | Mackey's formula for Res_K^G ∘ Ind_H^G (abelian simplification) |
| `frobenius_group <n>` | Frobenius group Aff(ℤ/nℤ) for prime n: kernel, complement, irreps |
| `demo` | Showcase of induced representation examples |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

The induced representation Ind_H^G(V) is constructed from the coset decomposition G/H: Ind_H^G(χ)(g)=(1/|H|)Σ_{x: x⁻¹gx∈H} χ(x⁻¹gx), which for abelian G reduces to [G:H]·χ(g) on H and 0 off H. Frobenius reciprocity is the adjunction Hom_G(Ind_H^G V, W)≅Hom_H(V, Res_H^G W), verified numerically as ⟨Ind χ, ψ⟩_G=⟨χ, Res ψ⟩_H. Mackey's formula describes Res_K^G∘Ind_H^G as a sum over double cosets K\G/H; for abelian groups conjugation is trivial so the formula simplifies. The affine group Aff(ℤ/nℤ) for prime n is a Frobenius group with kernel K≅ℤ/nℤ and complement H≅ℤ/(n−1)ℤ, having (n−1) one-dimensional irreps inflated from H and one irrep of dimension n−1 induced from a non-trivial character of K.

## Visualizations

- **SVG**: The six elements of ℤ/6ℤ arranged on a circle with the subgroup H={0,2,4} highlighted; legend shows the induced character values and the decomposition Ind_H^G(1)=ρ₀⊕ρ₂⊕ρ₄ via Frobenius reciprocity.
- **DOT**: Two-headed diagram between Rep(H) and Rep(G) with Ind_H^G and Res_H^G arrows, adjunction label, and Frobenius reciprocity formula.
- **TikZ**: Bidirectional diagram Rep(H)⇄Rep(G) labeled Ind and Res, with coset nodes below.
- **ASCII**: Coset decomposition G/H={H, 1+H} for G=ℤ/6ℤ, H={0,2,4}, with induced character values and the Frobenius reciprocity equation.

## Default State

No persistent state; all computations are driven by command arguments.
