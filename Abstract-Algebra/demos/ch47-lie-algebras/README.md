# Chapter 47 — Lie Algebras

The linearisation of a Lie group at the identity, studied through the bracket, adjoint representation, and the Killing form.

## Usage

### Interactive mode
```
cargo run -p ch47-lie-algebras
```

### Non-interactive (scriptable)
```
cargo run -p ch47-lie-algebras -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch47-lie-algebras -- --run demo --format svg > output.svg
cargo run -p ch47-lie-algebras -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch47-lie-algebras -- --run demo --format tex > output.tex
cargo run -p ch47-lie-algebras -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch47-lie-algebras -- --run demo --save state.toml
cargo run -p ch47-lie-algebras -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `bracket <a b c d> / <e f g h>` | Lie bracket [X,Y]=XY-YX for 2×2 matrices |
| `sl2` | Generators e,f,h of sl(2) and bracket relations |
| `jacobi <a b c d> / <e f g h> / <i j k l>` | Verify Jacobi identity |
| `adjoint <a> <b> <c>` | Adjoint rep of sl(2): ad(ae+bf+ch) as 3×3 matrix |
| `killing <a1 b1 c1> / <a2 b2 c2>` | Killing form B(X,Y) = Tr(ad(X)∘ad(Y)) |
| `cartan_criterion` | Killing form non-degeneracy → sl(2) semisimple |
| `demo` | Run a showcase of all commands |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The demo works concretely with sl(2,ℝ) and its standard generators e, f, h, verifying the structure constants [h,e]=2e, [h,f]=-2f, [e,f]=h numerically. The adjoint representation ad: sl(2) → gl(3) is computed explicitly and verified to be a Lie algebra homomorphism. The Killing form B(X,Y) = Tr(ad(X)∘ad(Y)) is computed and its matrix shown in the {e,f,h} basis; non-degeneracy (det(B) = -128 ≠ 0) establishes semisimplicity by Cartan's criterion, and simplicity is proved by showing every non-zero ideal equals sl(2).

## Visualizations

- **SVG**: Triangle diagram with nodes e, f, h connected by labeled arrows showing each bracket relation ([h,e]=2e, [h,f]=-2f, [e,f]=h), plus a caption for the Killing form.
- **DOT**: Directed graph on three nodes (e, f, h) with edges labeled by the bracket relations.
- **TikZ**: Triangle of nodes e, f, h with arrows labeled by the three bracket relations, including a bend on [e,f]=h.
- **ASCII**: Text layout placing e, f, h with bracket relation labels and the Killing form values B(e,f)=4, B(h,h)=8.

## Default State

- `algebra`: name of the active algebra, default `"sl2"`
