# Mereological Proof: The Fusion of All Parts

## Theorem

In Classical Mereology (GEM), if x and y overlap (share a common part), then there exists a product — the greatest common part of x and y.

## Setup

Given: P (parthood, a partial order), O (overlap: O(x,y) ↔ ∃z. P(z,x) ∧ P(z,y)).

**Axioms** (GEM):
1. P is a partial order (reflexive, antisymmetric, transitive)
2. Strong Supplementation (SSP): ¬P(y,x) → ∃z(P(z,y) ∧ ¬O(z,x))
3. Unrestricted Fusion: for non-empty φ, ∃y Fu(y, φ) where Fu(y,φ) iff ∀x(φ(x)→P(x,y)) ∧ ∀z(∀x(φ(x)→P(x,z))→P(y,z))

## Proof

**Claim**: If O(x, y), then there exists a product x ⊓ y (the greatest common part).

Define φ(z) ≡ P(z, x) ∧ P(z, y) — the condition for being a common part of x and y.

Since O(x, y), ∃z. P(z,x) ∧ P(z,y), so φ is non-empty.

By Unrestricted Fusion, there exists f with Fu(f, φ):
- ∀z(φ(z) → P(z, f)): f is an upper bound of all common parts.
- ∀w(∀z(φ(z) → P(z, w)) → P(f, w)): f is the *least* such upper bound.

We claim f = x ⊓ y.

**f is a common part of x and y**: We need P(f, x).

Suppose ¬P(f, x). By SSP, ∃k(P(k, f) ∧ ¬O(k, x)).
Since P(k, f) and f is a fusion of common parts, every part of f overlaps some common part of x and y (by the fusion property).
But a part of a common part of x is a part of x, so it overlaps x. Contradiction with ¬O(k, x).
Therefore P(f, x). Similarly P(f, y). So f is a common part of x and y.

**f is the greatest**: If P(g, x) and P(g, y), then φ(g) holds, so P(g, f) by the fusion property. ∎

## Significance

This theorem shows GEM has *binary meets*. Combined with unrestricted fusion (arbitrary joins), GEM is a complete join-semilattice. With strong supplementation implying atoms or atomlessness, GEM yields rich algebraic structure — a complete Boolean algebra (minus bottom) on the non-empty parts.
