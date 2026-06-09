# Chapter 36 — Limits and Colimits

Universal constructions that unify products, equalizers, pullbacks, inverse limits, and their duals under a single framework.

## Usage

### Interactive mode
```
cargo run -p ch36-limits-colimits
```

### Non-interactive (scriptable)
```
cargo run -p ch36-limits-colimits -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch36-limits-colimits -- --run demo --format svg > output.svg
cargo run -p ch36-limits-colimits -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch36-limits-colimits -- --run demo --format tex > output.tex
cargo run -p ch36-limits-colimits -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch36-limits-colimits -- --run demo --save state.toml
cargo run -p ch36-limits-colimits -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `product <n> <m>` | ℤ/nℤ × ℤ/mℤ with universal property |
| `coproduct <n> <m>` | Coproduct in Ab and Grp |
| `equalizer <n> <a> <b>` | Equalizer of x↦ax and x↦bx on ℤ/nℤ |
| `pullback <n> <m> <k>` | Fiber product ℤ/nℤ ×_{ℤ/kℤ} ℤ/mℤ |
| `inverse_limit <p>` | ℤ_p = lim← ℤ/pⁿ for prime p |
| `pushout <n> <m> <k>` | Pushout (amalgamated sum) in Ab |
| `padic <p> <depth>` | p-adic integers: elements of lim← ℤ/pᵏ |
| `demo` | Run a showcase of key results |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

A limit of a diagram is a universal cone over it; a colimit is a universal cocone. Products and equalizers are limits; coproducts and coequalizers are colimits. In Ab, the product ℤ/nℤ × ℤ/mℤ is also the coproduct (direct sum), and is cyclic of order nm when gcd(n,m) = 1 by the Chinese Remainder Theorem. The equalizer of x ↦ ax and x ↦ bx on ℤ/nℤ is the subgroup {x : (a−b)x ≡ 0 (mod n)}, computed explicitly. The p-adic integers ℤ_p = lim← ℤ/pⁿ are constructed as compatible sequences; the demo shows ordinary integers, negative integers (with their eventually-all-(p−1) digit expansions), and p-adic arithmetic with carries.

## Visualizations

- **SVG**: Limit cone diagram with the pullback object at the apex, ℤ/nℤ and ℤ/mℤ as base objects, and ℤ/gcd(n,m)ℤ at the bottom of the span, with projection arrows π₁ and π₂ from the apex.
- **DOT**: Directed graph of the span diagram and limit with labeled projection and structure arrows.
- **TikZ**: TikZ diagram of the limit of a span: A, B mapping to K = ℤ/gcd(n,m)ℤ, with the Limit node above projecting to A and B.
- **ASCII**: Text-art cone diagram with the limit at the top and the base span below, plus colimit (pushout) annotation.

## Default State

- `n`: modulus of the first object, default `6`
- `m`: modulus of the second object, default `4`
- `p`: prime for p-adic constructions, default `2`
