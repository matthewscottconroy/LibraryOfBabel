# Chapter 40 — Ext and Tor

An interactive demo of the derived functors Ext and Tor over ℤ, computing extension groups, torsion products, group cohomology, and long exact sequences from short exact sequences of ℤ/nℤ modules.

## Usage

### Interactive mode
```
cargo run -p ch40-ext-tor
```

### Non-interactive (scriptable)
```
cargo run -p ch40-ext-tor -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch40-ext-tor -- --run demo --format svg > output.svg
cargo run -p ch40-ext-tor -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch40-ext-tor -- --run demo --format tex > output.tex
cargo run -p ch40-ext-tor -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch40-ext-tor -- --run demo --save state.toml
cargo run -p ch40-ext-tor -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `ext <n> <m>` | Compute Ext¹_ℤ(ℤ/nℤ, ℤ/mℤ) ≅ ℤ/gcd(n,m)ℤ |
| `tor <n> <m>` | Compute Tor₁^ℤ(ℤ/nℤ, ℤ/mℤ) ≅ ℤ/gcd(n,m)ℤ |
| `ext_n <n> <m> <k>` | Extᵏ_ℤ(ℤ/nℤ, ℤ/mℤ) for higher k (vanishes for k≥2) |
| `extensions <n> <m>` | List all extension classes 0→ℤ/nℤ→E→ℤ/mℤ→0 |
| `group_cohomology <n>` | H^k(ℤ/nℤ, ℤ) for k=0,1,2,3,4 (periodic of period 2) |
| `long_exact <n> <m> <k>` | Long exact Ext sequence from 0→ℤ/nℤ→ℤ/mnℤ→ℤ/mℤ→0 |
| `flat_test <n>` | Tor test for flatness of ℤ/nℤ (always non-flat for n≥2) |
| `demo` | Showcase of Ext/Tor computations |
| `help` | Show help |
| `quit` | Exit |

## Mathematical Content

Ext¹_ℤ(ℤ/nℤ, ℤ/mℤ) is computed by applying Hom(−, ℤ/mℤ) to the free resolution 0→ℤ→(×n)→ℤ→0 and taking cohomology, yielding ℤ/gcd(n,m)ℤ; it classifies extensions of ℤ/nℤ by ℤ/mℤ. Tor₁^ℤ(ℤ/nℤ, ℤ/mℤ) is computed by tensoring the same resolution with ℤ/mℤ, giving the same answer ℤ/gcd(n,m)ℤ — a coincidence special to ℤ. Group cohomology H^k(ℤ/nℤ, ℤ) is periodic of period 2: ℤ at k=0, 0 at odd k, and ℤ/nℤ at even k≥2, arising from the periodic resolution of ℤ over ℤ[ℤ/nℤ]. A short exact sequence of modules gives a long exact sequence in Ext, and ℤ/nℤ is never flat over ℤ since tensoring destroys injectivity.

## Visualizations

- **SVG**: Long exact Ext/Tor sequence displayed in three rows: the short exact sequence, the Hom sequence with connecting homomorphism ∂, and the Ext¹ sequence; formulas Ext¹=Tor₁=ℤ/gcd(n,m)ℤ shown at bottom.
- **DOT**: Graph relating Ext⁰, Ext¹, Tor₀, Tor₁ nodes with "derived" edges.
- **TikZ**: Diagram showing Hom, Ext¹, Ext²=0 in one row and Tor₀, Tor₁ in another, with connecting ∂ arrows.
- **ASCII**: Horizontal long exact sequence 0→Hom(C,k)→Hom(B,k)→Hom(A,k)→Ext¹(C,k)→… with the gcd formulas.

## Default State

No persistent state; all computations are driven by command arguments.
