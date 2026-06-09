# Chapter 37 — Abelian Categories

An interactive demo of exactness, kernels, cokernels, and the fundamental diagram lemmas in abelian categories, realized concretely in ℤ/nℤ.

## Usage

### Interactive mode
```
cargo run -p ch37-abelian-categories
```

### Non-interactive (scriptable)
```
cargo run -p ch37-abelian-categories -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch37-abelian-categories -- --run demo --format svg > output.svg
cargo run -p ch37-abelian-categories -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch37-abelian-categories -- --run demo --format tex > output.tex
cargo run -p ch37-abelian-categories -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch37-abelian-categories -- --run demo --save state.toml
cargo run -p ch37-abelian-categories -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `exact <n1> <f1> <n2> <f2> <n3>` | Check exactness of 0→ℤ/n1→ℤ/n2→ℤ/n3→0 |
| `snake <A1> <A2> <A3> <B1> <B2> <B3>` | Snake lemma in ℤ/nℤ (connecting homomorphism) |
| `kernel_cokernel <n> <m> <a>` | Kernel and cokernel of f: ℤ/nℤ→ℤ/mℤ, x↦ax |
| `five_lemma <n>` | Demonstrate the five lemma with ℤ/nℤ |
| `short_exact <n> <k>` | Display 0→kℤ/nℤ→ℤ/nℤ→ℤ/kℤ→0 |
| `split <n> <k>` | Check if 0→ℤ/kℤ→ℤ/nkℤ→ℤ/nℤ→0 splits |
| `embedding` | Freyd-Mitchell embedding theorem |
| `demo` | Showcase of abelian category concepts |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

An abelian category is an additive category in which every morphism has a kernel and cokernel and every monomorphism is a kernel. This demo verifies the exactness condition im(f₁) = ker(f₂) in short exact sequences 0→A→B→C→0 over ℤ/nℤ. The snake lemma is illustrated by constructing the connecting homomorphism ∂: ker(f₃)→coker(f₁) from a commutative diagram with exact rows, and the five lemma is demonstrated via diagram chasing. The Freyd-Mitchell embedding theorem, which allows element-chasing arguments to apply to any small abelian category, is explained with concrete instances for group rings ℤ[ℤ/nℤ].

## Visualizations

- **SVG**: Horizontal sequence 0→A→B→C→0 with labeled maps f, g, h and exactness annotations im(f)=ker(g) and ker(h)=im(g).
- **DOT**: Directed graph of the short exact sequence nodes 0, A, B, C, 0 with edges labeled by the maps.
- **TikZ**: TikZ diagram of the short exact sequence suitable for LaTeX documents.
- **ASCII**: Text diagram of 0--f-->A--g-->B--h-->C-->0 with exactness conditions listed below.

## Default State

No persistent state; all computations are driven by command arguments.
