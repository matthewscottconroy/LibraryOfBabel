# Chapter 34 — Natural Transformations and the Yoneda Lemma

Every functor is completely determined by where it sends morphisms, and natural transformations out of a representable functor are in bijection with the functor's value at the representing object.

## Usage

### Interactive mode
```
cargo run -p ch34-yoneda
```

### Non-interactive (scriptable)
```
cargo run -p ch34-yoneda -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch34-yoneda -- --run demo --format svg > output.svg
cargo run -p ch34-yoneda -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch34-yoneda -- --run demo --format tex > output.tex
cargo run -p ch34-yoneda -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch34-yoneda -- --run demo --save state.toml
cargo run -p ch34-yoneda -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `natural <n>` | Natural transformation: det: GL_n → (-)* is natural |
| `yoneda <n>` | Yoneda: Nat(Hom(ℤ/nℤ,-), Hom(ℤ/nℤ,-)) = ℤ/nℤ |
| `double_dual` | V ≅ V** (natural) vs V ≅ V* (requires basis) |
| `representable <n>` | Hom(ℤ/nℤ,-) is representable by ℤ/nℤ |
| `functor_category <n> <m>` | Count Nat(Hom(ℤ/nℤ,-), Hom(ℤ/mℤ,-)) = gcd(n,m) |
| `demo` | Run a showcase of key results |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The Yoneda Lemma states that for any functor F: C → Set and object A, natural transformations Nat(Hom(A,−), F) are in bijection with F(A), via the map η ↦ η_A(id_A). Applied to Ab with A = ℤ/nℤ and F = Hom(ℤ/mℤ,−), the count Nat(Hom(ℤ/nℤ,−), Hom(ℤ/mℤ,−)) = |Hom(ℤ/mℤ, ℤ/nℤ)| = gcd(m,n) is computed explicitly. The Yoneda embedding A ↦ Hom(A,−) is fully faithful, meaning morphisms in Ab correspond bijectively to natural transformations between the represented functors. Naturality is contrasted concretely: the double-dual embedding V → V** is natural (defined without a basis), while the isomorphism V ≅ V* requires a basis choice and is not natural.

## Visualizations

- **SVG**: Yoneda embedding diagram with Ab on the left and Fun(Ab, Set) on the right, showing an object A and morphism f in Ab mapped to representable functors and a natural transformation, linked by labeled Yoneda arrows.
- **DOT**: Directed graph of the Yoneda embedding with dashed arrows marking the functor Y from objects to their represented functors.
- **TikZ**: TikZ naturality square for the Yoneda embedding with dashed Y arrows and a natural transformation η.
- **ASCII**: Text-art naturality square for Y(ℤ/nℤ) = Hom(ℤ/nℤ,−) with the Yoneda Lemma statement.

## Default State

- `n`: modulus of the representing object ℤ/nℤ, default `6`
- `m`: modulus of the second object for functor-category counts, default `4`
