# Chapter 35 — Adjoint Functors

The most ubiquitous structure in mathematics: a pair of functors L ⊣ R with a natural bijection Hom(L(A), B) ≅ Hom(A, R(B)).

## Usage

### Interactive mode
```
cargo run -p ch35-adjoints
```

### Non-interactive (scriptable)
```
cargo run -p ch35-adjoints -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch35-adjoints -- --run demo --format svg > output.svg
cargo run -p ch35-adjoints -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch35-adjoints -- --run demo --format tex > output.tex
cargo run -p ch35-adjoints -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch35-adjoints -- --run demo --save state.toml
cargo run -p ch35-adjoints -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `free_forget <n> <m>` | Free-forgetful adjunction for S={1..n} and G=ℤ/mℤ |
| `tensor_hom <n> <m> <k>` | Tensor-Hom adjunction: Hom(ℤ/nℤ⊗ℤ/mℤ,ℤ/kℤ) = Hom(ℤ/nℤ,Hom(ℤ/mℤ,ℤ/kℤ)) |
| `unit_counit <n> <m>` | Unit and counit of the free-forgetful adjunction |
| `preserve_limits` | Right adjoints preserve limits: U(G×H) = U(G)×U(H) |
| `galois_adjunction <n>` | Galois correspondence as an adjunction |
| `examples` | Classic adjunctions |
| `demo` | Run a showcase of key results |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

An adjunction F ⊣ G between functors F: C → D and G: D → C is a natural bijection Hom_D(F(A), B) ≅ Hom_C(A, G(B)), equivalently given by a unit η: Id_C → G∘F and counit ε: F∘G → Id_D satisfying the triangle identities. The free abelian group functor F: Set → Ab is left adjoint to the forgetful functor U: Ab → Set; this is verified by counting: |Hom_Ab(ℤⁿ, ℤ/mℤ)| = mⁿ = |Hom_Set({1..n}, ℤ/mℤ)|. The Tensor-Hom adjunction Hom(A⊗B,C) ≅ Hom(A,Hom(B,C)) is the categorical version of currying. Right adjoints preserve limits (in particular products), and left adjoints preserve colimits; the Galois correspondence between subgroups and intermediate fields is itself an adjunction between posets.

## Visualizations

- **SVG**: Two-category diagram with C on the left and D on the right, with blue L arrow going right and red R arrow going left, and the adjunction isomorphism Hom(L(A),B) ≅ Hom(A,R(B)) in a box at the bottom.
- **DOT**: Directed graph with C and D as box nodes, L and R as labeled edges in opposite directions, and an annotation node for the adjunction iso.
- **TikZ**: TikZ diagram of the two categories with bent arrows for L and R and a label L ⊣ R below.
- **ASCII**: Text-art two-box diagram with left/right arrows and the adjunction formula, unit, counit, and triangle identities written out.

## Default State

- `n`: size of the generating set S, default `3`
- `m`: modulus of the target group ℤ/mℤ, default `6`
