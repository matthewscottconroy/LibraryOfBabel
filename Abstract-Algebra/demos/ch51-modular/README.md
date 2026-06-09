# Chapter 51 — Modular Representation Theory

Representations of Lie algebras and algebraic groups over fields of prime characteristic p.

## Usage

### Interactive mode
```
cargo run -p ch51-modular
```

### Non-interactive (scriptable)
```
cargo run -p ch51-modular -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch51-modular -- --run demo --format svg > output.svg
cargo run -p ch51-modular -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch51-modular -- --run demo --format tex > output.tex
cargo run -p ch51-modular -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch51-modular -- --run demo --save state.toml
cargo run -p ch51-modular -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `char_p <p>` | Characteristic p: what changes from char 0? |
| `restricted <p>` | Restricted Lie algebras in char p, x^[p] operation |
| `steinberg <p> <n>` | Steinberg module dimension p^n for GL_n over 𝔽_p |
| `linkage <p> <lambda> <mu>` | Check if λ, μ are linked (same block) via Jantzen linkage |
| `tilting <p> <n>` | Tilting module T(n): self-dual indecomposable |
| `demo` | Run a showcase of all commands |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

In characteristic p, Maschke's theorem fails for groups of order divisible by p, and sl(2) acquires a p-map x ↦ x^[p] making it a restricted Lie algebra; the baby Verma modules Z(λ) of dimension p and the irreducibles L(0), ..., L(p-1) are enumerated. The Steinberg module St for GL_n over 𝔽_p has dimension p^(n(n-1)/2), is simultaneously irreducible and projective, and equals L(p-1) for sl(2). Jantzen's linkage principle identifies the block structure via the affine Weyl group dot action, and indecomposable tilting modules T(n) are computed using p-adic digit decomposition (Steinberg's tensor product theorem).

## Visualizations

- **SVG**: Row of coloured boxes for each irreducible L(0), ..., L(p-1) labelled with dimension, with the Steinberg module L(p-1) highlighted in gold; below, block pairs linked under the affine Weyl group are shown in separate boxes.
- **DOT**: Nodes for each L(i) (Steinberg as doubleoctagon), with dashed "linked" edges between modules in the same block.
- **TikZ**: Row of rectangle nodes for L(0) through L(p-1), with the Steinberg module drawn with a double border.
- **ASCII**: List of irreducible modules with dimensions and a "[Steinberg]" annotation, followed by the Steinberg dimension.

## Default State

- `prime`: characteristic p, default `5`
- `tilting_n`: highest weight for `tilting`, default `7`
