# Chapter 49 — Root Systems

The combinatorial skeleton of semisimple Lie algebras: roots, reflections, Weyl groups, and Dynkin diagrams.

## Usage

### Interactive mode
```
cargo run -p ch49-root-systems
```

### Non-interactive (scriptable)
```
cargo run -p ch49-root-systems -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch49-root-systems -- --run demo --format svg > output.svg
cargo run -p ch49-root-systems -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch49-root-systems -- --run demo --format tex > output.tex
cargo run -p ch49-root-systems -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch49-root-systems -- --run demo --save state.toml
cargo run -p ch49-root-systems -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `roots <type>` | All roots of A1, A2, B2, G2 |
| `simple <type>` | Simple roots for A1, A2, B2 |
| `cartan_matrix <type>` | Cartan matrix ⟨αᵢ,αⱼ∨⟩ for A1, A2, B2 |
| `weyl_group <type>` | Weyl group generators and elements for A2 |
| `reflection <a b> / <c d>` | Apply reflection s_α(β) = β − ⟨β,α∨⟩α |
| `dynkin <type>` | Dynkin diagram description for A1, A2, B2, G2 |
| `demo` | Run a showcase of all commands |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

Root systems A1, A2, B2, and G2 are stored as exact 2D float vectors; root lengths and Cartan pairings ⟨β,α∨⟩ = 2⟨β,α⟩/⟨α,α⟩ are computed and displayed, distinguishing simply-laced (ADE) from non-simply-laced types. Simple roots provide a basis expressing all positive roots as non-negative integer combinations. The Weyl group W(A2) ≅ S₃ is enumerated with all six elements and their actions on simple roots, with braid relations noted. Dynkin diagrams for A1 through G2 are described with edge multiplicities and arrow directions encoding root length ratios.

## Visualizations

- **SVG**: Vector diagram of the chosen root system drawn from the origin, with positive roots in blue and negative roots in red, on a labelled coordinate plane.
- **DOT**: Graph with an origin node connected to each root as a labelled circle node.
- **TikZ**: Simple roots placed at scaled positions with an arrow between them.
- **ASCII**: Text Dynkin diagram (O---O for A2, O==>O for B2, O===>O for G2) with simple root labels and a note on root lengths.

## Default State

- `root_system`: active root system type, default `"A2"`
