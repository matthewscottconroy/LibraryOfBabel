# Chapter 23 — Polynomial Rings

Implements arithmetic in Z[x] and Q[x], including division, GCD, irreducibility testing, root finding over finite fields, field extensions by root adjunction, and cyclotomic polynomials.

## Usage

### Interactive mode
```
cargo run -p ch23-polynomials
```

### Non-interactive (scriptable)
```
cargo run -p ch23-polynomials -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch23-polynomials -- --run demo --format svg > output.svg
cargo run -p ch23-polynomials -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch23-polynomials -- --run demo --format tex > output.tex
cargo run -p ch23-polynomials -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch23-polynomials -- --run demo --save state.toml
cargo run -p ch23-polynomials -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `add <f...> / <g...>` | Add two polynomials (coefficients space-separated, constant term first) |
| `mul <f...> / <g...>` | Multiply two polynomials |
| `div <f...> / <g...>` | Divide polynomials; show quotient and remainder |
| `gcd <f...> / <g...>` | Polynomial GCD via the Euclidean algorithm |
| `irreducible <f...> <p>` | Test irreducibility of f modulo prime p |
| `roots_mod <f...> <p>` | Find all roots of f in Z/pZ |
| `adjoin <f...>` | Describe Q[x]/(f): basis, degree, reduction rule, discriminant |
| `factor_zx <f...>` | Factor f over Z via the rational root theorem |
| `cyclotomic <n>` | Print the n-th cyclotomic polynomial Phi_n(x) |
| `demo` | Showcase: multiply x²-1 by itself, Phi_5, factor x²+x-6 |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

Polynomial rings k[x] over a field k are Euclidean domains, so the Euclidean algorithm and GCD extend directly from Z. A polynomial f of degree ≤ 3 is irreducible over Z/pZ iff it has no roots; for higher degrees the code uses a simplified reducibility check modulo p. Cyclotomic polynomials Phi_n(x) are computed recursively from x^n - 1 = product_{d|n} Phi_d(x), and for prime n are irreducible over Q by Eisenstein's criterion applied at p = n. Adjoining a root a of f gives a Q-basis {1, a, ..., a^(deg-1)} with the reduction rule a^deg expressed in terms of lower powers.

## Visualizations

- **SVG**: Integer roots of the last polynomial plotted as red dots on a number line in [-8, 8], with the factored form and Phi_5 displayed below.
- **DOT**: A node for the polynomial with edges to its integer roots (highlighted in red) or an "irreducible" node if none; cyclotomic polynomials Phi_3, Phi_4, Phi_5 shown as a separate cluster.
- **TikZ**: The polynomial at center with arrows to root nodes, or to an "irreducible" rectangle if no roots found.
- **ASCII**: Integer roots in [-10, 10] followed by a simple number-line diagram marking root positions, plus Phi_5 and Phi_6.

## Default State

- `last_poly`: coefficients of the most recently adjoined polynomial; initial value `[-1, 0, 1]` (i.e., x²-1)
