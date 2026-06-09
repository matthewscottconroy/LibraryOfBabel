# Chapter 28 — Tensor Products of Modules

The universal bilinear construction that turns bilinear maps into linear maps.

## Usage

### Interactive mode
```
cargo run -p ch28-tensor-products
```

### Non-interactive (scriptable)
```
cargo run -p ch28-tensor-products -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch28-tensor-products -- --run demo --format svg > output.svg
cargo run -p ch28-tensor-products -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch28-tensor-products -- --run demo --format tex > output.tex
cargo run -p ch28-tensor-products -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch28-tensor-products -- --run demo --save state.toml
cargo run -p ch28-tensor-products -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `tensor_zn <n> <m>` | ℤ/nℤ ⊗_ℤ ℤ/mℤ ≅ ℤ/gcd(n,m)ℤ |
| `base_change <a> <b>` | Illustrate base change ℚ ⊗_ℤ ℤ[√a] |
| `hom_tensor <n> <m> <k>` | Hom-Tensor adjunction: both sides |
| `annihilate` | Show ℤ/2ℤ ⊗ ℤ/3ℤ = 0 |
| `outer_product <v1> <v2>` | Outer product of two vectors (matrix) |
| `flat <n>` | Test if ℤ/nℤ is flat |
| `demo` | Run a showcase of key results |
| `help` | Show this help |
| `quit` | Exit |

## Mathematical Content

The tensor product A ⊗_R B is the universal target for bilinear maps out of A × B. For cyclic groups, ℤ/nℤ ⊗_ℤ ℤ/mℤ ≅ ℤ/gcd(n,m)ℤ: every elementary tensor is annihilated by both n and m, so by their gcd. The Hom-Tensor adjunction Hom(A⊗B, C) ≅ Hom(A, Hom(B,C)) is verified by counting homomorphisms between cyclic groups. Flatness measures whether tensoring preserves exact sequences; ℤ/nℤ is never flat for n > 1 because tensoring kills the injection ×n: ℤ → ℤ.

## Visualizations

- **SVG**: Commutative triangle for the universal property of ℤ/nℤ ⊗ ℤ/mℤ, showing the bilinear map φ, an arbitrary bilinear f, and the unique linear factorization.
- **DOT**: Directed graph of the same universal property triangle with labeled edges for the bilinear and unique linear maps.
- **TikZ**: TikZ commutative diagram with nodes for ℤ/nℤ × ℤ/mℤ, ℤ/gcd(n,m)ℤ, and A, with dashed arrow for the unique factorization.
- **ASCII**: Text-art commutative triangle of the tensor product universal property with computed gcd.

## Default State

- `n`: modulus of first factor, default `6`
- `m`: modulus of second factor, default `4`
