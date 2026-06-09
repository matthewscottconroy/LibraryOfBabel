# Chapter 22 — Divisibility and the Domain Hierarchy

Implements the Euclidean algorithm, Bézout's identity, Eisenstein's irreducibility criterion, and the hierarchy ED < PID < UFD < integral domain, illustrated with Z[i] and Z[sqrt(-5)].

## Usage

### Interactive mode
```
cargo run -p ch22-divisibility
```

### Non-interactive (scriptable)
```
cargo run -p ch22-divisibility -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch22-divisibility -- --run demo --format svg > output.svg
cargo run -p ch22-divisibility -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch22-divisibility -- --run demo --format tex > output.tex
cargo run -p ch22-divisibility -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch22-divisibility -- --run demo --save state.toml
cargo run -p ch22-divisibility -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `euclidean <a> <b>` | Euclidean algorithm with all division steps |
| `gcd_poly <f...> / <g...>` | Euclidean algorithm on polynomials |
| `bezout <a> <b>` | Bézout coefficients: as + bt = gcd(a,b), with back-substitution |
| `ufd_check <ring>` | Check UFD property for `gaussian` or `zsqrt5` |
| `norm_gaussian <a> <b>` | Norm of a+bi, unit and irreducibility check |
| `split_prime <p>` | How prime p splits/ramifies/stays inert in Z[i] |
| `eisenstein <coeffs> <p>` | Apply Eisenstein's criterion with prime p |
| `hierarchy` | Table: ED/PID/UFD/domain status for common rings |
| `demo` | Showcase: gcd(48,18), Bézout(35,15), prime 5 in Z[i], Z[sqrt(-5)] |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The Euclidean algorithm generalizes from Z to polynomial rings and Gaussian integers via a suitable norm function; Bézout's identity then expresses the gcd as an explicit linear combination. In Z[i] every prime p either ramifies (p=2), splits into conjugate Gaussian primes (p ≡ 1 mod 4), or remains inert (p ≡ 3 mod 4), determined by quadratic reciprocity. The domain hierarchy ED ⊂ PID ⊂ UFD ⊂ integral domain is illustrated concretely: Z, Z[i], and k[x] are Euclidean; Z[x] is a UFD but not a PID; Z[sqrt(-5)] is a domain but not a UFD, as shown by the two distinct factorizations of 6.

## Visualizations

- **SVG**: Hasse diagram of divisors of gcd(a, b), with prime divisors highlighted and cover edges drawn.
- **DOT**: The same divisor Hasse diagram in bottom-to-top layout, with prime nodes colored distinctly.
- **TikZ**: Divisor Hasse diagram with nodes positioned by prime-factor level.
- **ASCII**: Step-by-step Euclidean algorithm output followed by the domain hierarchy summary.

## Default State

- `last_a`: first argument for Euclidean algorithm; initial value `48`
- `last_b`: second argument; initial value `18`
