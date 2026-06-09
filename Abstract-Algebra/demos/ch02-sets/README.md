# Chapter 2 — Sets, Relations, and Functions

Interactive explorer for finite sets of integers, set operations, binary relations, powersets, and bijections.

## Usage

### Interactive mode
```
cargo run -p ch02-sets
```

### Non-interactive (scriptable)
```
cargo run -p ch02-sets -- --run <cmd> [args] [--format text|svg|dot|tex|ascii]
```

### Visualize
```
cargo run -p ch02-sets -- --run demo --format svg > output.svg
cargo run -p ch02-sets -- --run demo --format dot | dot -Tpdf > output.pdf
cargo run -p ch02-sets -- --run demo --format tex > output.tex
cargo run -p ch02-sets -- --run demo --format ascii
```

### Persist state
```
cargo run -p ch02-sets -- --run demo --save state.toml
cargo run -p ch02-sets -- --load state.toml --run demo
```

## Commands

| Command | Description |
|---------|-------------|
| `set <name> <elems>` | Define a named set, e.g. `set A 1 2 3` |
| `show` | List all defined sets with cardinalities |
| `union <A> <B>` | Compute A ∪ B |
| `inter <A> <B>` | Compute A ∩ B |
| `diff <A> <B>` | Compute A \ B and symmetric difference |
| `power <A>` | Enumerate the powerset of A (2^|A| subsets) |
| `cart <A> <B>` | Cartesian product A × B |
| `relation <A>` | Analyze empty, full, identity, and ≤ relations on A for reflexivity, symmetry, transitivity, antisymmetry |
| `bijection <A> <B>` | Display the natural bijection when \|A\| = \|B\| |
| `demo` | Showcase of set operations |
| `quit` | Exit |

Elements are space-separated integers.

## Mathematical Content

Set theory provides the language for all of mathematics. This demo implements the core operations on finite integer sets: union, intersection, difference, symmetric difference, powerset (limited to sets of size ≤ 16), and Cartesian product. The `relation` command checks four standard properties — reflexivity, symmetry, transitivity, and antisymmetry — on canonical relations over a set, identifying equivalence relations and partial orders. The `bijection` command illustrates that two finite sets have the same cardinality if and only if there is a bijection between them.

## Visualizations

- **SVG**: Venn diagram with two overlapping ellipses; elements of A only, A ∩ B, and B only are placed in their respective regions; A ∪ B and A ∩ B are labeled below.
- **DOT**: Graph with nodes for A, B, their union, and their intersection; directed edges labeled `union` and `inter`.
- **TikZ**: Four TikZ nodes (A, B, A ∪ B, A ∩ B) connected by stealth arrows.
- **ASCII**: Text listing of A, B, A ∪ B, A ∩ B, A \ B, B \ A, followed by a simple three-column ASCII Venn table.

## Default State

The default TOML state includes:
- `set_A`: integer set `[1, 2, 3]`
- `set_B`: integer set `[2, 3, 4]`
