# Parts and Wholes

Mereology (from Greek *meros*, part) is the formal study of the part-whole relation. It was founded by Stanisław Leśniewski in 1916 as part of his foundational project, and independently developed by Henry Leonard and Nelson Goodman in their "Calculus of Individuals" (1940).

## The Central Relation: Parthood

The primitive of mereology is the relation **P(x, y)**: "x is a part of y."

Axioms of *ground mereology* (M):

**M1. Reflexivity**: P(x, x) — everything is a part of itself.
**M2. Antisymmetry**: P(x, y) ∧ P(y, x) → x = y — if x and y are parts of each other, they are identical.
**M3. Transitivity**: P(x, y) ∧ P(y, z) → P(x, z) — the part of a part is a part.

So parthood is a *partial order*. Many standard orderings are mereological: part-of, subset-of (in extensional set theory), subregion-of.

## Derived Notions

From P, we define:
- **Proper part**: PP(x, y) ↔ P(x, y) ∧ x ≠ y
- **Overlap**: O(x, y) ↔ ∃z [P(z, x) ∧ P(z, y)] — x and y share a common part
- **Disjointness**: D(x, y) ↔ ¬O(x, y) — x and y share no part
- **Binary sum** (fusion): x + y = the thing that is the smallest object containing both x and y as parts

## Supplementation

A key question: if x is a proper part of y, must y have *other* parts besides x?

**Weak Supplementation (WSP)**: PP(x, y) → ∃z [PP(z, y) ∧ D(z, x)]
"If x is a proper part of y, then y has another part disjoint from x."

Without WSP, pathological objects are possible: x could be a proper part of y, but x is the *only* part of y — which seems incoherent (if x is the only part of y, how can x be less than y?).

**Strong Supplementation (SSP)**: ¬P(y, x) → ∃z [P(z, y) ∧ D(z, x)]
"If y is not a part of x, then y has some part disjoint from x."

SSP implies WSP. Ground mereology + SSP is called *Extensional Mereology* (EM).

## Mereological Extensionality

In EM, objects with the same proper parts are identical:
```
(∃z PP(z, x)) → [x = y ↔ ∀z (PP(z, x) ↔ PP(z, y))]
```

This is analogous to the set-theoretic axiom of extensionality. It means an object is fully determined by its parts — there are no "bare particulars" beyond their mereological structure.

This is controversial for physical objects: does a statue have the same parts as the lump of clay it's made from? If so, are they identical? Many philosophers think not — leading to *non-extensional* mereologies.
