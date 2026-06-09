# Chapter 1 — Logic and the Art of Proof

Interactive propositional logic explorer with a recursive-descent parser for formulas over variables p, q, r, s, t.

## Usage

### Interactive mode
```
cargo run -p ch01-logic
```

### Non-interactive (scriptable)
```
cargo run -p ch01-logic -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch01-logic -- --run demo --format svg > output.svg
cargo run -p ch01-logic -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch01-logic -- --run demo --format tex > output.tex
cargo run -p ch01-logic -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch01-logic -- --run demo --save state.toml
cargo run -p ch01-logic -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `truth <formula>` | Print full truth table for the formula |
| `equiv <f1> / <f2>` | Check logical equivalence (separate formulas with `/`) |
| `tautology <formula>` | Check if formula is true under every assignment |
| `satisfy <formula>` | Find a satisfying variable assignment |
| `contrapositive <f>` | Show the contrapositive of `P -> Q` |
| `demo` | Showcase of key tautologies and equivalences |
| `quit` | Exit the program |

Variables: `p`, `q`, `r`, `s`, `t`

Connectives: `!` (not), `&` (and), `|` (or), `->` (implies), `<->` (iff)

Precedence (lowest to highest): `<->`, `->`, `|`, `&`, `!`

## Mathematical Content

Propositional logic is the study of truth-valued formulas built from atomic variables and Boolean connectives. This demo parses and evaluates formulas over up to five variables, enumerating all 2^n truth assignments to build complete truth tables. Key concepts demonstrated include tautologies (law of excluded middle, hypothetical syllogism), contradictions, logical equivalence (De Morgan's laws), satisfiability, and the contrapositive of an implication.

## Visualizations

- **SVG**: Color-coded truth table rendered as a grid; result column highlighted green (T) or red (F); status label TAUTOLOGY / CONTRADICTION / CONTINGENT shown below.
- **DOT**: Each truth assignment displayed as a node colored by its result; the formula appears as a central ellipse node.
- **TikZ**: Truth table rendered inside a `tabular` environment as a standalone LaTeX document.
- **ASCII**: Plain-text truth table with variable columns and a `res` result column.

## Default State

The default TOML state includes:
- `last_formula`: the formula used when no argument is supplied to visualization commands (`p -> q`)
