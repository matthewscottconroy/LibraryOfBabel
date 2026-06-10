# Partial Orders

Not everything can be compared. Some things are related but not ranked; some are ranked only in some pairs. Partial orders capture this graduated notion of comparison.

## Definition

A *partial order* on a set A is a relation ≤ that is:
1. **Reflexive**: a ≤ a for all a ∈ A
2. **Antisymmetric**: if a ≤ b and b ≤ a, then a = b
3. **Transitive**: if a ≤ b and b ≤ c, then a ≤ c

A set with a partial order is a *partially ordered set* or *poset*, written (A, ≤).

The word "partial" reflects that some pairs may be *incomparable* — neither a ≤ b nor b ≤ a.

## Examples

**Total order (linear order)**: ≤ on ℝ. Every pair of real numbers is comparable. Also called a *chain*.

**Subset order**: (𝒫(A), ⊆). Sets are ordered by inclusion. {1,2} and {2,3} are incomparable (neither is a subset of the other).

**Divisibility**: (ℕ, |) where a | b means "a divides b." The number 6 is above 1, 2, 3, 6; the numbers 4 and 6 are incomparable.

**Prefix order**: On strings, s ≤ t if s is a prefix of t. "cat" ≤ "catch" but "cat" and "car" are incomparable.

**Logical implication**: On propositions, P ≤ Q if P → Q. This makes propositions (up to logical equivalence) into a Boolean algebra.

## Special Elements

In a poset (A, ≤):

- **Maximum** (greatest element): m such that a ≤ m for all a ∈ A. Unique if it exists.
- **Minimum** (least element): m such that m ≤ a for all a ∈ A.
- **Maximal element**: m such that no b > m exists (but m need not be ≥ everything).
- **Minimal element**: m such that no b < m exists.

A maximum is always maximal, but not vice versa. In the subset order on {∅, {1}, {2}}: both {1} and {2} are maximal (nothing is above them), but there is no maximum.

**Upper bound** of S ⊆ A: an element u with s ≤ u for all s ∈ S.
**Least upper bound** (lub, join, supremum): the smallest upper bound. Written ⊔S or sup S.
**Greatest lower bound** (glb, meet, infimum): the largest lower bound. Written ⊓S or inf S.

## Lattices

A poset where every pair of elements has both a join and a meet is a *lattice*:

> (A, ≤) is a lattice if ∀a, b. a ⊔ b exists and a ⊓ b exists.

The subset lattice (𝒫(X), ⊆) is a *complete lattice* — every subset (not just pairs) has a join (= union) and meet (= intersection).

Lattices appear throughout mathematics: topology (open sets form a lattice), logic (propositions form a Heyting algebra), and computer science (program analysis uses lattices via Tarski's fixed-point theorem).

**Tarski's fixed-point theorem**: Every monotone function on a complete lattice has a least fixed point. This theorem underlies the semantics of recursive programs, Datalog evaluation, and static analysis.

## Total Orders and Well-Orders

A **total order** (linear order) is a partial order where every pair is comparable:

> ∀a, b. a ≤ b ∨ b ≤ a

A **well-order** is a total order where every non-empty subset has a least element. The natural numbers ℕ with ≤ are well-ordered. The integers ℤ are not (the negative integers have no least element).

Well-orderings are crucial for transfinite induction: if every non-empty subset has a least element, we can do induction along the ordering.

The *Well-Ordering Theorem* — every set can be well-ordered — is equivalent to the Axiom of Choice. This equivalence, proved by Zermelo in 1904, was one of the most startling results in early set theory.

## Hasse Diagrams

Posets are visualized with Hasse diagrams: draw elements as nodes, with edges going upward from a to b when a < b and there's no c with a < c < b (a *cover*).

```
        {1,2,3}
       / |    \
  {1,2} {1,3} {2,3}
    |  X  |  X  |
   {1}  {2}  {3}
        |
        ∅
```

The power set of {1,2,3} as a Hasse diagram — the quintessential example of a non-trivial lattice.
