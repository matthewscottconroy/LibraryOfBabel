# Chapter 3 — Cardinality and the Axiom of Choice

Explorer for infinite cardinality: the Cantor diagonal argument, Hilbert's Hotel, the Cantor pairing function, and bijections between number sets.

## Usage

### Interactive mode
```
cargo run -p ch03-cardinality
```

### Non-interactive (scriptable)
```
cargo run -p ch03-cardinality -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch03-cardinality -- --run demo --format svg > output.svg
cargo run -p ch03-cardinality -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch03-cardinality -- --run demo --format tex > output.tex
cargo run -p ch03-cardinality -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch03-cardinality -- --run demo --save state.toml
cargo run -p ch03-cardinality -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `diagonal <n>` | Cantor diagonal argument with n binary sequences (n = 2–16) |
| `hotel <n>` | Hilbert's Hotel: accommodate n new guests in a full infinite hotel |
| `pairing <a> <b>` | Cantor pairing function π(a, b) → N with grid display |
| `unpairing <n>` | Inverse Cantor pairing π⁻¹(n) |
| `compare <n> <m>` | Bijection between {0..n-1} and Z/mZ (when n = m) |
| `naturals_to_z` | Explicit bijection N → Z |
| `sizes` | Survey of the aleph_0 / continuum cardinality hierarchy |
| `demo` | Showcase of cardinality concepts |
| `quit` | Exit |

## Mathematical Content

Cardinality measures the "size" of sets, including infinite ones. Two sets have the same cardinality when a bijection exists between them; this demo shows that |N| = |Z| = |Q| = ℵ₀ via explicit bijections, while Cantor's diagonal argument proves |R| > ℵ₀ by constructing a binary sequence that differs from every enumerated sequence. The Cantor pairing function π(k₁, k₂) = (k₁+k₂)(k₁+k₂+1)/2 + k₂ witnesses |N×N| = ℵ₀. Hilbert's Hotel illustrates that an infinite set can accommodate finitely many new elements by a shift bijection.

## Visualizations

- **SVG**: 6×6 grid of the Cantor pairing function values π(a, b); diagonal cells highlighted in amber to show the anti-diagonal traversal.
- **DOT**: Hierarchy graph with nodes for finite sets, ℵ₀, 2^ℵ₀ = |R|, and 2^|R| = |P(R)|, connected by strict-inequality edges labeled with the theorem that justifies each step.
- **TikZ**: Circular nodes for N, Z, Q, and R with bijection arrows; N, Z, Q annotated ℵ₀ and R annotated 2^ℵ₀; the N→Z and Z→Q arrows are solid blue, Q→R is dashed red.
- **ASCII**: Cardinality comparison arrow diagram followed by a 5×5 Cantor pairing table.

## Default State

The default TOML state includes:
- `diagonal_n`: number of sequences for the diagonal argument (`6`)
- `hotel_guests`: number of new guests for Hilbert's Hotel (`3`)
