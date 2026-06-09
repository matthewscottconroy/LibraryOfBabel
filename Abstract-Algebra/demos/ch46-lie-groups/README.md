# Chapter 46 — Lie Groups

Smooth manifolds that are also groups, studied through matrix groups, the exponential map, and covering spaces.

## Usage

### Interactive mode
```
cargo run -p ch46-lie-groups
```

### Non-interactive (scriptable)
```
cargo run -p ch46-lie-groups -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch46-lie-groups -- --run demo --format svg > output.svg
cargo run -p ch46-lie-groups -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch46-lie-groups -- --run demo --format tex > output.tex
cargo run -p ch46-lie-groups -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch46-lie-groups -- --run demo --save state.toml
cargo run -p ch46-lie-groups -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `so2 <theta_deg>` | Rotation matrix in SO(2), verify det=1 and Rᵀ=R⁻¹ |
| `su2 <a_re> <a_im> <b_re> <b_im>` | SU(2) matrix from (a,b), verify unitarity |
| `gl2 <a> <b> <c> <d>` | GL(2,ℝ) element: determinant, Lie algebra log |
| `exp_mat <a> <b> <c> <d>` | Matrix exponential of 2×2 via power series |
| `one_param <a> <b> <c> <d> <t>` | One-parameter subgroup exp(tX) for tangent X |
| `cover` | SU(2) → SO(3) double cover explained |
| `demo` | Run a showcase of all commands |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The demo implements the classical matrix Lie groups SO(2), SU(2), and GL(2,ℝ), verifying their defining properties (orthogonality, unitarity, invertibility). The matrix exponential is computed via a 20-term power series, demonstrating Jacobi's formula det(exp(X)) = exp(tr(X)) and the correspondence between Lie algebra elements and one-parameter subgroups γ(t) = exp(tX). The double cover π: SU(2) → SO(3) is explained via the conjugation action on traceless Hermitian matrices, with the kernel {±I} ≅ ℤ/2ℤ illustrating why a 360° rotation sends a spinor to its negative.

## Visualizations

- **SVG**: Manifold diagram showing SO(2) ≅ S¹ and SU(2) ≅ S³ as circles, connected by a 2:1 cover arrow, with a label for the exponential map from Lie algebra to group.
- **DOT**: Directed graph of the Lie group hierarchy GL(2,ℝ) → SL(2,ℝ) → SO(2), and the separate SU(2) → SO(3) double cover edge.
- **TikZ**: Linear chain of nodes GL(2,ℝ) → SL(2,ℝ) → SO(2) with edge labels det=1 and orthogonal.
- **ASCII**: Text hierarchy showing GL(2,R) ⊃ SL(2,R) ⊃ SO(2) and SU(2) ⊃ SO(3), with the exponential map noted at the bottom.

## Default State

- `group`: active group name, default `"SO2"`
- `theta`: rotation angle in degrees, default `45.0`
