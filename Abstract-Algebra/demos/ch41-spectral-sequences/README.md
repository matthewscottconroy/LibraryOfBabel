# Chapter 41 — Spectral Sequences

An interactive demo of spectral sequences, with the Serre spectral sequence for the Hopf fibration S¹→S³→S² and the Lyndon-Hochschild-Serre sequence for group extensions as worked examples.

## Usage

### Interactive mode
```
cargo run -p ch41-spectral-sequences
```

### Non-interactive (scriptable)
```
cargo run -p ch41-spectral-sequences -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch41-spectral-sequences -- --run demo --format svg > output.svg
cargo run -p ch41-spectral-sequences -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch41-spectral-sequences -- --run demo --format tex > output.tex
cargo run -p ch41-spectral-sequences -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch41-spectral-sequences -- --run demo --save state.toml
cargo run -p ch41-spectral-sequences -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `e2_page <rows> <cols> <entries>` | Define and display an E₂ page as a rows×cols bigraded grid |
| `differential <sp> <sq> <tp> <tq> <n>` | Apply differential dᵣ: Eᵣ^{p,q}→Eᵣ^{p-r,q+r-1} multiplied by n |
| `serre_hopf` | Serre spectral sequence for the Hopf fibration S¹→S³→S² |
| `lhs_example <n>` | LHS spectral sequence for ℤ/nℤ ◁ ℤ/n²ℤ ↠ ℤ/nℤ |
| `collapse <r>` | Show E_∞ page when the spectral sequence collapses at page r |
| `filtration <n>` | Filtered complex for ℤ/nℤ and its associated spectral sequence |
| `convergence` | Explain convergence: E_∞ as the associated graded of H_* |
| `demo` | Showcase: Hopf fibration and LHS examples |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

A spectral sequence is a sequence of bigraded pages (Eᵣ, dᵣ) where dᵣ has bidegree (−r, r−1) and Eᵣ₊₁ = H(Eᵣ, dᵣ); under suitable boundedness conditions it converges to the associated graded of some target homology. The Serre spectral sequence has E₂^{p,q} = H_p(B; H_q(F)) converging to H_{p+q}(E), and the demo works through the Hopf fibration where d₂: E₂^{2,0}→E₂^{0,1} must be an isomorphism to recover H_*(S³). The LHS spectral sequence E₂^{p,q} = H_p(Q, H_q(N, ℤ)) converges to H_{p+q}(G, ℤ) for a group extension 1→N→G→Q→1. Convergence is explained carefully: E_∞ gives only the associated graded of H_*, and extension problems may remain.

## Visualizations

- **SVG**: E₂ page grid for the Hopf fibration with entries ℤ at (0,0), (0,1), (2,0), (2,1), a red d₂ arrow from (2,0) to (0,1), and a note that E₃=E_∞ recovers H_*(S³).
- **DOT**: Sequence of nodes E₂→E₃→E_∞→H_*(Total) with edges labeled by differentials and convergence.
- **TikZ**: Grid nodes for the E₂ page at positions (0,0), (2,0), (0,2), (2,2) with a d₂ arrow.
- **ASCII**: Table of the E₂ page for the Hopf fibration with a note on d₂ and collapse to E_∞.

## Default State

No persistent state; all computations are driven by command arguments.
