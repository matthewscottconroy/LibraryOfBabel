# Chapter 33 — Categories and Functors

The language of modern mathematics: objects, morphisms, composition, and structure-preserving maps between categories.

## Usage

### Interactive mode
```
cargo run -p ch33-categories
```

### Non-interactive (scriptable)
```
cargo run -p ch33-categories -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch33-categories -- --run demo --format svg > output.svg
cargo run -p ch33-categories -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch33-categories -- --run demo --format tex > output.tex
cargo run -p ch33-categories -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch33-categories -- --run demo --save state.toml
cargo run -p ch33-categories -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `category <name>` | Describe a category (Set/Grp/Vect/Top/Pos) |
| `functor <name>` | Describe a functor |
| `compose_functor` | Show functor composition |
| `commute <f1> <f2> <g1> <g2>` | Check commutativity of a square of ℤ/nℤ maps |
| `isomorphism <n> <m>` | Automorphisms of ℤ/nℤ (n=m required) |
| `opposite` | Explain opposite category |
| `examples` | Table of categories and their morphisms |
| `demo` | Run a showcase of key results |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

A category consists of objects, morphisms between them, an associative composition law, and identity morphisms. Functors are structure-preserving maps between categories, sending objects to objects and morphisms to morphisms while respecting composition and identities. The forgetful functor U: Grp → Set is faithful but not full, while the free group functor F: Set → Grp is its left adjoint. Commutativity of a square of homomorphisms between cyclic groups is verified elementwise; automorphisms of ℤ/nℤ are exactly the maps x ↦ ax with gcd(a,n) = 1, so Aut(ℤ/nℤ) ≅ (ℤ/nℤ)*.

## Visualizations

- **SVG**: Commutative square diagram with objects A, B, C, D and morphisms f, g, h, k, with a boxed label confirming h∘f = k∘g.
- **DOT**: Directed graph of the same four-object commutative square.
- **TikZ**: TikZ commutative square with labeled arrows between circular nodes.
- **ASCII**: Text-art commutative square with the commutativity condition written below.

## Default State

- `cat`: name of the current category, default `"grp"`
