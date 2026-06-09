# Chapter 27 — Structure Theorem for Modules over PIDs

Implements the structure theorem for finitely generated modules over PIDs via Smith normal form, invariant factors, primary decomposition, rational canonical form, and Jordan normal form.

## Usage

### Interactive mode
```
cargo run -p ch27-structure-theorem
```

### Non-interactive (scriptable)
```
cargo run -p ch27-structure-theorem -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch27-structure-theorem -- --run demo --format svg > output.svg
cargo run -p ch27-structure-theorem -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch27-structure-theorem -- --run demo --format tex > output.tex
cargo run -p ch27-structure-theorem -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch27-structure-theorem -- --run demo --save state.toml
cargo run -p ch27-structure-theorem -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `smith <r> <c> <entries>` | Smith normal form with step-by-step row/column operations; read off cokernel |
| `classify_module <d...>` | Write module from invariant factors as a direct sum; show primary decomposition |
| `rational_form <char_poly...>` | Rational canonical form from characteristic polynomial; companion matrix |
| `jordan_form <lambda> <mult> <exp>` | Jordan block for eigenvalue, algebraic multiplicity, and min-poly exponent |
| `compare <d...> / <e...>` | Isomorphism test for two f.g. abelian groups given by invariant factors |
| `tensor_zn <n> <m>` | Z/nZ ⊗_Z Z/mZ = Z/gcd(n,m)Z, derived from the free resolution |
| `demo` | Showcase: smith, classify_module(2,6), tensor(4,6), compare(4,3 vs 12) |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The structure theorem states that every finitely generated module M over a PID is isomorphic to Z^r ⊕ Z/d_1 ⊕ ... ⊕ Z/d_k with d_1 | d_2 | ... | d_k; the invariant factors d_i are the nonunit diagonal entries of the Smith normal form of any presentation matrix. Two finitely generated abelian groups are isomorphic if and only if they have identical invariant factors, making the Smith normal form a complete isomorphism invariant. For modules over k[x] the structure theorem yields rational canonical form (defined over any field) whose blocks are companion matrices of the invariant factors, and Jordan normal form (requires algebraically closed field) whose blocks are determined by eigenvalue, algebraic multiplicity, and minimal polynomial exponent. The tensor product formula Z/nZ ⊗ Z/mZ = Z/gcd(n,m)Z follows directly by applying ⊗ Z/mZ to the free resolution of Z/nZ.

## Visualizations

- **SVG**: Decomposition diagram with M at the top, cyclic summands as labeled boxes below connected by arrows, and primary-factor labels beneath each summand; the invariant factor divisibility chain shown at the bottom.
- **DOT**: Tree rooted at M with summand nodes and their primary component leaves, all labeled with Z/dZ notation.
- **TikZ**: The same two-level tree using `\mathbb{Z}/d\mathbb{Z}` and `\oplus` notation, with dashed arrows from summands to their primary factors.
- **ASCII**: Module decomposition with each summand's primary factorization and the divisibility chain.

## Default State

- `inv_factors`: invariant factors used for the decomposition display; initial value `[2, 6]`
