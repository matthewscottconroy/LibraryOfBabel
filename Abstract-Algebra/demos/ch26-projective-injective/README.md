# Chapter 26 — Free, Projective, and Injective Modules

Constructs free resolutions of Z/nZ, computes projective dimension, tests flatness and injectivity, applies Baer's criterion, and analyzes splitting of short exact sequences.

## Usage

### Interactive mode
```
cargo run -p ch26-projective-injective
```

### Non-interactive (scriptable)
```
cargo run -p ch26-projective-injective -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch26-projective-injective -- --run demo --format svg > output.svg
cargo run -p ch26-projective-injective -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch26-projective-injective -- --run demo --format tex > output.tex
cargo run -p ch26-projective-injective -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch26-projective-injective -- --run demo --save state.toml
cargo run -p ch26-projective-injective -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `free_res <n>` | Free resolution 0 → Z →[×n]→ Z →[π]→ Z/nZ → 0; Ext and Tor formulas |
| `projective_test <n> <k>` | Is Z/kZ projective over Z/nZ? (splitting criterion via gcd) |
| `injective_test <n>` | Is Z/nZ injective as a Z-module? |
| `flat_test <n>` | Is Z/nZ flat? (iff n is square-free) |
| `baer_criterion <n>` | Baer's criterion: attempt to extend maps from ideals into Z/nZ |
| `splitting <n> <k>` | Does 0 → Z/kZ → Z/nkZ → Z/nZ → 0 split? |
| `pd <n>` | Projective dimension of Z/nZ over Z, with Ext and Tor formulas |
| `demo` | Showcase: free_res(6), flat_test(6), splitting(2,3), injective_test(6) |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The minimal free resolution 0 → Z →×n→ Z → Z/nZ → 0 shows pd_Z(Z/nZ) = 1 for all n ≥ 2; from it one reads off Ext^1_Z(Z/nZ, M) = M/nM and Tor^1_Z(Z/nZ, M) = M[n] = {x : nx = 0}. No Z/nZ is injective as a Z-module (injective Z-modules are divisible, but Z/nZ has exponent n); Baer's criterion confirms this by finding ideals whose maps into Z/nZ cannot be extended. Z/nZ is flat if and only if n is square-free, equivalently when Z/nZ is semisimple. The short exact sequence 0 → Z/kZ → Z/nkZ → Z/nZ → 0 splits if and only if gcd(n,k) = 1, in which case Z/nkZ ≅ Z/nZ ⊕ Z/kZ by CRT.

## Visualizations

- **SVG**: Diagram of the free resolution 0 → Z →[×n]→ Z →[π]→ Z/nZ → 0 drawn as a horizontal sequence with labeled arrows, annotated with pd, Ext formula, and flatness status.
- **DOT**: Left-to-right graph of the same resolution with a "properties" note node listing pd, flatness, and injectivity.
- **TikZ**: The resolution as a row of math nodes connected by labeled arrows, using `\mathbb{Z}` and `\times n` notation.
- **ASCII**: Textual resolution sequence followed by pd, Ext, Tor formulas, flatness test, and the global dimension of Z.

## Default State

- `res_n`: the modulus used for the free resolution display; initial value `6`
