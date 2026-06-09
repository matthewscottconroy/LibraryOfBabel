# Chapter 12 — Multilinear Algebra and Tensors

Explores tensor products, the exterior algebra of wedge products, the Hodge dual, the determinant as a top-degree form, and metric tensors.

## Usage

### Interactive mode
```
cargo run -p ch12-multilinear
```

### Non-interactive (scriptable)
```
cargo run -p ch12-multilinear -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch12-multilinear -- --run demo --format svg > output.svg
cargo run -p ch12-multilinear -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch12-multilinear -- --run demo --format tex > output.tex
cargo run -p ch12-multilinear -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch12-multilinear -- --run demo --save state.toml
cargo run -p ch12-multilinear -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `wedge <v1...> / <v2...>` | Wedge product v1∧v2 in ℝ³: all three basis components, area, and antisymmetry check |
| `cross <v1...> / <v2...>` | Cross product and its identification as the Hodge dual of the wedge product |
| `det_as_top <n> <entries>` | Determinant as the coefficient of e1∧...∧en in the expansion of column wedges |
| `tensor <v1...> / <v2...>` | Outer product v1⊗v2 displayed as a rank-1 matrix |
| `sym <v1...> / <v2...>` | Symmetric product v1⊙v2 = v1⊗v2 + v2⊗v1 |
| `alt <v1...> / <v2...>` | Alternating part v1∧v2 = v1⊗v2 − v2⊗v1 with decomposition identity |
| `metric <g_entries> / <v...>` | Vector length under a 3×3 metric tensor g (Riemannian or pseudo-Riemannian) |
| `demo` | Showcase: e1∧e2, e1×e2, and the rank-1 tensor e1⊗e2 |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

The wedge product v1∧v2 is a bilinear, antisymmetric map whose norm measures the area of the parallelogram spanned by v1 and v2; in ℝ³ the three components correspond to the e1∧e2, e1∧e3, and e2∧e3 basis 2-forms. Every tensor decomposes as the sum of its symmetric part (in Sym²(V)) and its alternating part (in Λ²(V)), via v1⊗v2 = ½(v1⊙v2) + ½(v1∧v2). The Hodge star ⋆ is an isomorphism Λᵏ(ℝⁿ) → Λⁿ⁻ᵏ(ℝⁿ), and in ℝ³ it identifies the cross product with ⋆(v1∧v2). A metric tensor g is a symmetric positive-definite (0,2)-tensor that defines lengths via ‖v‖²_g = vᵀgv.

## Visualizations

- **SVG**: Grid showing the outer product v1⊗v2 as a 3×3 matrix of products, with the Sym+Alt decomposition formula and the parallelogram area interpretation.
- **DOT**: Concept graph connecting tensor product V⊗W to its symmetric and alternating parts, the cross product (Hodge dual), the determinant (top form), and the metric tensor.
- **TikZ**: Arrow diagram from V⊗W to Sym²(V) and Λ²(V), with Λ²(V) further mapped to the determinant top form, and the decomposition identity labeling the root node.
- **ASCII**: Layout of the rank-1 tensor grid, the Sym/Alt decomposition, wedge component formulas, and the identification of the cross product as the Hodge dual.

## Default State

- `v1`: `[1, 0, 0]` — first default vector (e1)
- `v2`: `[0, 1, 0]` — second default vector (e2)
