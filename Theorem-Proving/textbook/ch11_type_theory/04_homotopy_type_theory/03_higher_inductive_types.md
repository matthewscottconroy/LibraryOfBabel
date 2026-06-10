# Higher Inductive Types

Ordinary inductive types are defined by their *constructors* — ways of building elements. Higher Inductive Types (HITs) add a new dimension: *path constructors* that specify equalities between elements.

## Motivation: Quotient Types

In mathematics, we routinely form quotient constructions: ℤ = ℕ × ℕ / ~ where (a,b) ~ (c,d) iff a+d = b+c. We want a type representing integers, where (3,1) and (4,2) and (5,3) are all *equal* (they all represent +2).

In ordinary type theory, implementing quotients requires either:
1. Picking canonical representatives (tedious, requires choice)
2. Setoid hell (carry equivalence relations everywhere)
3. Quotient types as a primitive (ad hoc, not general)

HITs solve this cleanly: add constructors that *assert equalities*.

## The Circle as a HIT

```
Inductive S¹ : Type :=
  | base : S¹
  | loop : base = base    ← path constructor
```

The circle has one point (`base`) and one non-trivial loop. Functions out of `S¹` must send `base` to some point and `loop` to a loop at that point.

```
-- A function f : S¹ → X is determined by:
f base : X
f loop : f base = f base    (a loop in X)
```

Computing with `S¹` reveals topology: `π₁(S¹) = ℤ`, provable in type theory.

## The Integers as a HIT

```
Inductive ℤ : Type :=
  | zero : ℤ
  | succ : ℤ → ℤ
  | pred : ℤ → ℤ
  | succ_pred : ∀ n, succ (pred n) = n    ← path constructor
  | pred_succ : ∀ n, pred (succ n) = n    ← path constructor
```

This defines ℤ as a type where `succ` and `pred` are inverse operations. No quotient needed — the equalities are built into the type.

## Truncation

One of the most important HITs is *propositional truncation* `‖A‖` (or `∥A∥`):

```
Inductive ‖A‖ : Prop :=
  | |·| : A → ‖A‖
  | squash : ∀ (x y : ‖A‖), x = y    ← all proofs are equal
```

`‖A‖` is the proposition "A is inhabited" — it collapses all information about *which* element of `A` we have, retaining only that *some* element exists.

This lets us define:
- **Existence without witness**: `∃x. P(x)` = `‖Σ(x:A). P(x)‖`
- **Disjunction**: `P ∨ Q` = `‖P + Q‖`

The truncation HIT mediates between the constructive world (where witnesses matter) and the classical world (where existence is a bare fact).

## Set Truncation and Quotients

*Set truncation* `‖A‖₀` makes a type into a set by forcing all paths between points to be equal:

```
Inductive ‖A‖₀ :=
  | |·|₀ : A → ‖A‖₀
  | trunc : ∀ (x y : ‖A‖₀) (p q : x = y), p = q
```

*Quotient types* `A/R` are built as a HIT:

```
Inductive A/R :=
  | [·] : A → A/R
  | quot : ∀ (a b : A), R a b → [a] = [b]
  | trunc : (set truncation)
```

Every element of `A` maps to `A/R`, and elements that are `R`-related become equal.

## Suspension and Spheres

The *suspension* `ΣA` of a type `A`:
```
Inductive ΣA :=
  | N : ΣA          (north pole)
  | S : ΣA          (south pole)
  | merid : A → N = S    (a meridian for each element of A)
```

Iterating suspension builds spheres: `S¹ = Σ(Bool)`, `S² = Σ(S¹)`, and so on. The entire homotopy theory of spheres unfolds from simple HIT definitions.

## Computational Behavior

HITs require *computation rules* for path constructors, parallel to the β-rules for ordinary inductives. For the circle:

- `f base` reduces to the specified point
- `ap f loop` reduces to the specified loop

This is where HoTT is still evolving: *cubical type theory* (implemented in Cubical Agda and the Cubical library in Lean) gives a rigorous computational interpretation of HITs, allowing actual execution of programs defined over HITs.

## The Promise

HITs demonstrate that type theory is not limited to discrete, set-like structures. It can natively express continuous, topological, algebraic structures — circles, spheres, quotients, truncations — all as types with computational content. The boundary between algebra and topology, between proof and program, dissolves further.
