# Tarski's World and Quantifiers

Tarski's World is a software microworld in which first-order logic meets geometric intuition. Understanding quantifiers through it builds the visual and semantic intuitions that abstract syntax alone cannot provide.

## The Setting

A Tarski's World scenario consists of a grid containing geometric objects (blocks, tetrahedra, dodecahedra) of various sizes (small, medium, large) and positions. The language includes:

- **Predicates**: `Cube(x)`, `Tet(x)`, `Dodec(x)`, `Small(x)`, `Medium(x)`, `Large(x)`, `Leftof(x,y)`, `Rightof(x,y)`, `Frontof(x,y)`, `Backof(x,y)`, `Between(x,y,z)`, `SameSize(x,y)`, `SameShape(x,y)`, `SameRow(x,y)`, `SameCol(x,y)`, `Adjoins(x,y)`
- **Individual constants**: names for specific objects (a, b, c, ...)
- **Variables**: x, y, z, ...
- **Quantifiers**: ∀ (for all) and ∃ (there exists)

## Evaluating Quantified Sentences

Consider a world with objects: a (small cube at position (1,1)), b (large tet at (3,2)), c (medium cube at (2,1)).

**∀x Cube(x)**: Is every object a cube? Check each: a is a cube ✓, b is not a cube ✗. **False.**

**∃x Cube(x)**: Is some object a cube? a is a cube ✓. **True.**

**∀x (Cube(x) → Small(x))**: For every object, if it's a cube, is it small? Only cubes are a and c. Is a small? Yes. Is c small? No (medium). **False.**

**∃x (Cube(x) ∧ Large(x))**: Is some object both a cube and large? Neither a nor c is large. **False.**

**∀x ∃y Leftof(x,y)**: For every object, is there something to its right? b is at (3,2) — nothing is to its right. **False.**

## The Domain Makes Everything

Quantifiers range over the *domain* — the objects in the world. Change the world, change the truth value.

In a world with only one object a (a cube):
- `∀x Cube(x)` is **True** — vacuously checked on a alone
- `∀x ∃y x ≠ y` is **False** — a has no other object to be different from

This teaches a crucial lesson: quantifiers are *domain-relative*. When mathematicians write `∀x (x > 0 → ∃y y² = x)`, they must specify *which* domain: this is true in ℝ but false in ℚ (¼ works, but 3 does not).

## Nested Quantifiers: Order Matters

**∀x ∃y Leftof(x,y)** vs **∃y ∀x Leftof(x,y)**:

The first says: for each object, *find something to its right* (the something can depend on x).
The second says: there is one fixed object to the right of *every* object.

In a world with a at (1,1), b at (2,1), c at (3,1):
- `∀x ∃y Leftof(x,y)`: a finds b to its right ✓, b finds c ✓, but c finds... nothing. **False.**
- `∃y ∀x Leftof(x,y)`: We need one object to the right of all. No such object exists. **False.**

In a world with a at (1,1) and b at (2,1) only:
- `∀x ∃y Leftof(x,y)`: a finds b ✓, but b finds nothing ✗. **False.**
- `∃y ∀x Leftof(x,y)`: Does b serve? b is right of a ✓, but is b to the right of itself? No. **False.**

The quantifier ordering `∀x ∃y` is weaker than `∃y ∀x` — finding a *witness depending on x* is easier than finding a *single universal witness*. In mathematics: "for every ε there exists δ" (continuity) is weaker than "there exists δ for all ε" (uniform continuity).

## Building Logical Intuition

Tarski's World provides an environment where:

1. **Truth is verifiable**: You can check a formula against a concrete world, building semantic intuition before abstract proof.

2. **Counterexamples are constructible**: To refute `∀x ∃y φ(x,y)`, build a world where some x has no suitable y.

3. **Logical equivalences become visible**: You can see *why* `¬∀x P(x) ↔ ∃x ¬P(x)` — the negation of "all blocks are cubes" is "some block is not a cube."

4. **The gap between syntax and semantics**: The formula `∀x (P(x) ∨ ¬P(x))` is a tautology — true in every world — not because of any constraint on the world, but because of the logical structure alone.

The microworld methodology — making logic tangible before making it abstract — is the pedagogical philosophy underlying modern logic courses. Tarski himself, who developed the semantic theory of truth, believed that understanding a sentence is understanding its truth conditions in models.
