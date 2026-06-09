# Chapter 20 — Rings and Ring Homomorphisms

Explores the structure of commutative rings through units, zero divisors, nilpotents, and concrete examples including Gaussian integers and Z[sqrt(-5)].

## Usage

### Interactive mode
```
cargo run -p ch20-rings
```

### Non-interactive (scriptable)
```
cargo run -p ch20-rings -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch20-rings -- --run demo --format svg > output.svg
cargo run -p ch20-rings -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch20-rings -- --run demo --format tex > output.tex
cargo run -p ch20-rings -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch20-rings -- --run demo --save state.toml
cargo run -p ch20-rings -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `zmod <n>` | Explore Z/nZ: addition/multiplication tables, zero divisors, units, nilpotents |
| `gaussian <a> <b> <c> <d>` | Multiply (a+bi)(c+di) and compute norms in Z[i] |
| `zsqrt5 <a> <b> <c> <d>` | Multiply in Z[sqrt(-5)] and show norm |
| `factor6` | Demonstrate 6 = 2·3 = (1+sqrt(-5))(1-sqrt(-5)): failure of UFD |
| `units <n>` | All units of Z/nZ (elements with gcd 1) |
| `zero_div <n>` | All zero divisors in Z/nZ |
| `nilp <n>` | All nilpotent elements in Z/nZ |
| `wedderburn` | Wedderburn's theorem: Z/pZ is a field iff p is prime |
| `demo` | Showcase ring examples for n=6 and n=8 |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

A commutative ring's elements partition into units (invertible), zero divisors (annihilate a nonzero element), and nilpotents (some power is zero); these properties are computed for Z/nZ by examining gcd conditions and modular arithmetic. The Gaussian integers Z[i] form a Euclidean domain whose norm N(a+bi) = a²+b² is multiplicative, making irreducibility detectable by primality of the norm. By contrast, Z[sqrt(-5)] is not a UFD: the element 6 factors into two genuinely distinct products of irreducibles, illustrated concretely by the `factor6` command.

## Visualizations

- **SVG**: Multiplication table for Z/nZ (capped at n=10), with the unit group listed below the table.
- **DOT**: A diagram of Z/nZ as a ring node with edges to subsets labeled Units, Zero Divisors, and Nilpotents, including a "contained in" edge from nilpotents to zero divisors.
- **TikZ**: The same three-subset diagram using `\mathbb{Z}/n\mathbb{Z}` notation.
- **ASCII**: Ring structure summary (units, zero divisors, field/non-field type) with an inline multiplication table for n ≤ 6.

## Default State

- `last_n`: the most recently explored modulus; initial value `6`
