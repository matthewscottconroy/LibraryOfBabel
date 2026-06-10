# Finite Sets and Counting

Before Cantor extended cardinality to the infinite, there was counting — the fundamental operation of matching a set to the natural numbers.

## Cardinality of Finite Sets

A set A is *finite* if there exists a bijection between A and the set {0, 1, ..., n-1} for some natural number n. We write |A| = n and call n the *cardinality* of A.

This definition requires that we can line up the elements of A and label them 0 through n-1. The key property:

> **Pigeonhole Principle**: If |A| = |B| = n and f : A → B is injective, then f is bijective (surjective too).

In other words: no injective function from an n-element set to itself can "miss" any element. This seems obvious but requires proof — and the proof uses induction.

## The Pigeonhole Principle

**Statement**: If n items are placed in m containers and n > m, then some container has more than one item.

**Formally**: If f : A → B with |A| > |B|, then f is not injective.

**Proof**: By induction on |B|. If |B| = 0, then B = ∅ and f : A → ∅ is impossible for non-empty A. If |B| = m+1: pick b ∈ B and consider f⁻¹(b) and f restricted to A \ f⁻¹(b) → B \ {b}. Either |f⁻¹(b)| > 1 (done) or we apply induction. ∎

**Applications**:
- Among 13 people, two share a birth month (13 > 12 months)
- Among 367 people, two share a birthday (367 > 366 days)
- In any sequence of n²+1 distinct numbers, there is a monotone subsequence of length n+1 (Erdős-Szekeres theorem)
- Hash collisions are unavoidable when hashing more keys than buckets

## Counting Laws

For finite sets A and B:

| Formula | Condition | Meaning |
|---------|-----------|---------|
| \|A ∪ B\| = \|A\| + \|B\| | A ∩ B = ∅ | Disjoint union |
| \|A ∪ B\| = \|A\| + \|B\| − \|A ∩ B\| | General | Inclusion-exclusion |
| \|A × B\| = \|A\| · \|B\| | — | Product rule |
| \|B^A\| = \|B\|^{\|A\|} | — | Function count |
| \|𝒫(A)\| = 2^{\|A\|} | — | Power set |

**Inclusion-exclusion** generalizes:
```
|A ∪ B ∪ C| = |A| + |B| + |C| - |A∩B| - |A∩C| - |B∩C| + |A∩B∩C|
```

The pattern: add singletons, subtract pairs, add triples, ... This is the inclusion-exclusion principle, central to combinatorics.

## Proof of Finiteness

To prove a set is finite, exhibit a bijection to {0,...,n-1}. In practice, we use:

- **Subset**: Any subset of a finite set is finite.
- **Image**: f(A) is finite if A is finite (surjections preserve finiteness).
- **Union**: A ∪ B is finite if A and B are finite.
- **Product**: A × B is finite if A and B are finite.

## Infinite Sets: The First Glimpse

A set is *infinite* if it is not finite — no bijection to any {0,...,n-1} exists.

How do we know ℕ is infinite? Suppose |ℕ| = n. Then there's a bijection f : ℕ → {0,...,n-1}. But ℕ contains {0,...,n-1,n} — an (n+1)-element subset — and no injective function from an (n+1)-element set to an n-element set exists. Contradiction.

More directly: the function f(n) = n+1 is injective but not surjective (0 has no preimage), showing ℕ has a proper subset (ℕ \ {0}) in bijection with itself. This self-similarity — being equipotent to a proper subset — is Dedekind's definition of *infinite*.

Finite sets obey the pigeonhole principle strictly. Infinite sets systematically violate it: ℕ injects into ℕ \ {0} bijectively. This is the entry point to Cantor's paradise.
