# Chapter 43 — Complete Reducibility

An interactive demo of Maschke's theorem, the Artin-Wedderburn decomposition of group algebras, central idempotents, and the failure of complete reducibility in characteristic p.

## Usage

### Interactive mode
```
cargo run -p ch43-complete-reducibility
```

### Non-interactive (scriptable)
```
cargo run -p ch43-complete-reducibility -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch43-complete-reducibility -- --run demo --format svg > output.svg
cargo run -p ch43-complete-reducibility -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch43-complete-reducibility -- --run demo --format tex > output.tex
cargo run -p ch43-complete-reducibility -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch43-complete-reducibility -- --run demo --save state.toml
cargo run -p ch43-complete-reducibility -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `maschke <n> <char_p>` | Maschke's theorem: char_p∤n iff k[G] is semisimple |
| `group_algebra <n>` | ℂ[ℤ/nℤ] ≅ ℂⁿ decomposition via DFT |
| `artin_wedderburn <type> <n>` | Artin-Wedderburn decomposition for Z, S3, S4 |
| `dimension_formula <type> <n>` | Verify Σ(dim Vᵢ)² = |G| for Z, S, D groups |
| `regular_decomp <n>` | Regular rep of ℤ/nℤ decomposed into irreps |
| `idempotent <n>` | Central idempotents e_k of ℂ[ℤ/nℤ] (Fourier basis) |
| `modular_failure <p>` | Maschke fails for ℤ/pℤ over 𝔽_p: indecomposable Jordan block |
| `demo` | Showcase of complete reducibility |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

Maschke's theorem states that the group algebra k[G] is semisimple — every representation completely decomposes into irreducibles — if and only if char(k) does not divide |G|; the proof constructs a G-equivariant projection by averaging P=(1/|G|)Σ_g g·P₀·g⁻¹. The Artin-Wedderburn theorem then identifies the semisimple algebra ℂ[G] ≅ ⊕ᵢ M_{nᵢ}(ℂ) where nᵢ=dim Vᵢ, and the dimension formula Σ(dim Vᵢ)²=|G| is verified for ℤ/nℤ, S₃, S₄, and dihedral groups. When char(k)=p divides |G|, the group algebra 𝔽_p[ℤ/pℤ] ≅ 𝔽_p[x]/(x−1)^p is a local ring with indecomposable but reducible modules (Jordan blocks), witnessed by the non-split extension 0→V₁→V₂→V₁→0. The central idempotents e_k=(1/n)Σ_j ω^{−jk}g^j are the Fourier basis elements that implement the Wedderburn isomorphism.

## Visualizations

- **SVG**: Character table of S₃ with entries for χ₁, χ₂, χ₃ across conjugacy classes e, (12), (123), plus the Artin-Wedderburn formula ℂ[S₃]≅M₁(ℂ)⊕M₁(ℂ)⊕M₂(ℂ) and the Maschke condition.
- **DOT**: Graph from k[G] to irreps V₁, V₂, Vₙ and from those to the Artin-Wedderburn summands M_{dᵢ}(ℂ).
- **TikZ**: Linear diagram G→k[G]→⊕ᵢM_{nᵢ}(k) with labeled arrows.
- **ASCII**: Summary of Maschke's theorem condition, the semisimplicity result, and the modular failure case.

## Default State

No persistent state; all computations are driven by command arguments.
