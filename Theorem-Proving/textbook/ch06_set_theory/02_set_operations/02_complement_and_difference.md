# Complement and Difference

Subtraction in set theory — removing elements from a collection — takes two forms: relative difference and absolute complement.

## Set Difference

The *difference* (or *relative complement*) of sets A and B is the set of elements in A that are not in B:

> **A \ B** = {x | x ∈ A ∧ x ∉ B}

Also written A − B in some texts. Read aloud: "A minus B" or "A without B."

```
A = {1, 2, 3, 4, 5}
B = {3, 4, 5, 6, 7}

A \ B = {1, 2}     (in A but not B)
B \ A = {6, 7}     (in B but not A)
```

Note that `A \ B ≠ B \ A` in general — difference is *not* commutative.

**Symmetric difference**: The symmetric difference `A △ B = (A \ B) ∪ (B \ A)` consists of elements in exactly one of A and B. It measures how much the sets disagree.

```
A △ B = {1, 2, 6, 7}
```

The symmetric difference makes sets into an abelian group under `△`, with the empty set as identity and every set as its own inverse (`A △ A = ∅`). This structure underlies coding theory.

## Absolute Complement

When working within a fixed *universe* U — a "background set" containing all objects under discussion — the *complement* of A is:

> **Aᶜ** = U \ A = {x ∈ U | x ∉ A}

The universe U must be specified: "the complement of the set of even numbers" means different things in ℕ vs. ℤ vs. ℝ.

```
U = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
A = {2, 4, 6, 8, 10}     (even numbers)
Aᶜ = {1, 3, 5, 7, 9}    (odd numbers)
```

## Laws

Complements satisfy the *De Morgan laws*, discovered by Augustus De Morgan in the 1840s:

> **(A ∪ B)ᶜ = Aᶜ ∩ Bᶜ**
> **(A ∩ B)ᶜ = Aᶜ ∪ Bᶜ**

In English: the complement of a union is the intersection of complements, and vice versa. These are the set-theoretic counterparts of the logical laws `¬(P ∨ Q) ↔ ¬P ∧ ¬Q`.

Further laws:
- **(Aᶜ)ᶜ = A** (double complement = identity)
- **A ∪ Aᶜ = U** (law of excluded middle for sets)
- **A ∩ Aᶜ = ∅** (law of non-contradiction for sets)
- **A \ B = A ∩ Bᶜ** (difference as intersection with complement)

## The Algebra of Sets

Together with union and intersection, complement makes sets into a *Boolean algebra*. The axioms of Boolean algebra (named after George Boole, who formalized logic in the 1850s) are:

1. Commutativity: A ∪ B = B ∪ A, A ∩ B = B ∩ A
2. Associativity
3. Distributivity: A ∪ (B ∩ C) = (A ∪ B) ∩ (A ∪ C)
4. Identity: A ∪ ∅ = A, A ∩ U = A
5. Complement: A ∪ Aᶜ = U, A ∩ Aᶜ = ∅

Every Boolean algebra is isomorphic to a field of sets — a theorem of Stone (1936) that connects abstract algebra to topology. The same Boolean algebra axioms govern propositional logic (with ∨, ∧, ¬) and digital circuits (with OR, AND, NOT). Sets, logic, and hardware share a common algebra.

## In Python

```python
A = {1, 2, 3, 4, 5}
B = {3, 4, 5, 6, 7}
U = set(range(1, 11))

diff = A - B          # {1, 2}
sym_diff = A ^ B      # {1, 2, 6, 7}
complement_A = U - A  # {1, 3, 5, 7, 9}
```

Python's `set` type directly implements these operations, reflecting that these are fundamental operations on collections.
