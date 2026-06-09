# Chapter 29 — Field Extensions

Growing fields by adjoining roots of polynomials and measuring the resulting degree.

## Usage

### Interactive mode
```
cargo run -p ch29-field-extensions
```

### Non-interactive (scriptable)
```
cargo run -p ch29-field-extensions -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch29-field-extensions -- --run demo --format svg > output.svg
cargo run -p ch29-field-extensions -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch29-field-extensions -- --run demo --format tex > output.tex
cargo run -p ch29-field-extensions -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch29-field-extensions -- --run demo --save state.toml
cargo run -p ch29-field-extensions -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `minimal_poly <coeffs>` | Degree of extension from minimal polynomial |
| `degree <d1 d2 ...>` | Tower law: total degree from list of degrees |
| `algebraic <p_coeffs> <v>` | Check if v is a root of f (algebraic over ℚ) |
| `adjoin <p_coeffs>` | Describe ℚ[x]/(f): basis and arithmetic |
| `splitting <p_coeffs>` | Factor a cubic/quartic over its splitting field |
| `tower_example` | Tower ℚ ⊂ ℚ(√2) ⊂ ℚ(√2,√3) |
| `transcendental` | Contrast π, e (transcendental) vs √2 (algebraic) |
| `demo` | Run a showcase of key results |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

A field extension K/F is algebraic if every element of K satisfies a polynomial over F; the degree [K:F] equals the dimension of K as an F-vector space. The minimal polynomial of α is the monic irreducible polynomial of smallest degree satisfied by α, and [F(α):F] equals its degree. The Tower Law states [L:F] = [L:K][K:F] for a tower F ⊆ K ⊆ L. The splitting field of a polynomial is the smallest field over which it factors completely.

## Visualizations

- **SVG**: Vertical chain diagram of the tower ℚ ⊂ ℚ(√2) ⊂ ℚ(√2,√3), with degree labels [K:F]=2 and [L:K]=2 on each step and basis information at each level.
- **DOT**: Directed graph of the same tower with edge labels for the intermediate degrees.
- **TikZ**: TikZ node chain for ℚ, ℚ(√2), ℚ(√2,√3) with annotated arrows.
- **ASCII**: Text-art tower diagram with degree annotations and the tower law product.

## Default State

- `poly`: coefficients of the working polynomial (constant term first), default `"-2 0 1"` (i.e. x²−2)
