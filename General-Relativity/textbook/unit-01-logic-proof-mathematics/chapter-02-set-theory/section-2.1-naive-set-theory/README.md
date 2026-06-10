# Section 2.1: Naive Set Theory

---

## Section Introduction

A **set** is a collection of objects, called its **elements** or **members**. This definition is deliberately informal — it is the "naive" approach, relying on our pre-mathematical intuition about what a "collection" is, without specifying precisely what counts as a legitimate collection.

The word "naive" does not mean simple or wrong — naive set theory contains genuine and deep mathematics. It means only that we have not axiomatized the system; we rely on informal intuition rather than explicit rules about what sets exist.

For the majority of mathematics — analysis, linear algebra, differential geometry, general relativity — naive set theory provides everything we need. The axiomatic subtleties (Section 2.2) become important only in extreme situations (very large sets, foundational questions, the axiom of choice).

---

## 2.1.1 Sets and Membership

We write **a ∈ A** to mean "a is an element of A" (read "a belongs to A" or "a is in A"), and **a ∉ A** for its negation.

**Specifying a set:**

1. **Roster notation**: List all elements between braces. {1, 2, 3, 4, 5} is the set of the first five positive integers. {red, green, blue} is the set of primary additive colors.

2. **Set-builder notation**: Specify elements by a property. {x ∈ ℝ : x² < 2} is the set of real numbers whose square is less than 2 (the interval (-√2, √2)). The colon ":" is read "such that."

3. **Description**: "The set of all prime numbers," "the set of all 4×4 symmetric matrices with real entries."

**Important sets:**
- ∅ (the **empty set**): the unique set with no elements. ∅ = { }.
- ℕ = {0, 1, 2, 3, ...}: the natural numbers. (Some authors start with 1; we include 0 following the ISO standard and the convention used in Peano's axioms.)
- ℤ = {..., -2, -1, 0, 1, 2, ...}: the integers.
- ℚ = {p/q : p, q ∈ ℤ, q ≠ 0}: the rational numbers.
- ℝ: the real numbers (constructed in Section 3.3).
- ℂ = {a + bi : a, b ∈ ℝ, i² = -1}: the complex numbers.

**Equality of sets**: Two sets A and B are **equal** (A = B) iff they have exactly the same elements:

$$A = B \iff (\forall x, x \in A \iff x \in B)$$

Note: sets are *unordered* (the set {1, 2, 3} is the same as {3, 1, 2}) and *without repetition* (the set {1, 1, 2} is the same as {1, 2}, since a set either contains an element or it does not — there is no "multiplicity").

---

## 2.1.2 Subsets

**A is a subset of B** (written A ⊆ B) if every element of A is also an element of B:

$$A \subseteq B \iff \forall x, (x \in A \to x \in B)$$

**A is a proper subset of B** (written A ⊊ B) if A ⊆ B and A ≠ B — there is at least one element in B not in A.

**Key facts:**
- ∅ ⊆ A for every set A. (The empty set is a subset of every set. Proof: the statement ∀x (x ∈ ∅ → x ∈ A) is vacuously true, since x ∈ ∅ is always false.)
- A ⊆ A for every set A. (Every set is a subset of itself.)
- A = B iff A ⊆ B and B ⊆ A. (This is how equality of sets is typically proved: show mutual containment.)

---

## 2.1.3 Set Operations

Let A and B be subsets of some universal set U (the set of all objects under consideration in a given context).

**Union**: A ∪ B = {x : x ∈ A ∨ x ∈ B} — elements in A, or B, or both.

**Intersection**: A ∩ B = {x : x ∈ A ∧ x ∈ B} — elements in both A and B.

**Complement**: Aᶜ = A' = U \ A = {x ∈ U : x ∉ A} — elements of U not in A.

**Set difference**: A \ B (read "A minus B") = {x : x ∈ A ∧ x ∉ B} — elements in A but not in B.

**Symmetric difference**: A △ B = (A \ B) ∪ (B \ A) — elements in exactly one of A, B.

**Power set**: 𝒫(A) (or 2^A) = {S : S ⊆ A} — the set of all subsets of A. If A has n elements, 𝒫(A) has 2ⁿ elements.

**Cartesian product**: A × B = {(a, b) : a ∈ A, b ∈ B} — the set of all ordered pairs.

**Properties** (all verifiable from definitions):
- Commutativity: A ∪ B = B ∪ A and A ∩ B = B ∩ A.
- Associativity: (A ∪ B) ∪ C = A ∪ (B ∪ C) and similarly for ∩.
- Distributivity: A ∪ (B ∩ C) = (A ∪ B) ∩ (A ∪ C) and A ∩ (B ∪ C) = (A ∩ B) ∪ (A ∩ C).
- De Morgan's laws: (A ∪ B)ᶜ = Aᶜ ∩ Bᶜ and (A ∩ B)ᶜ = Aᶜ ∪ Bᶜ.
- Identity: A ∪ ∅ = A and A ∩ U = A.
- Absorption: A ∪ (A ∩ B) = A and A ∩ (A ∪ B) = A.

These properties mirror the Boolean algebra of propositional logic exactly — because set algebra *is* Boolean algebra, with sets corresponding to propositions and set operations to logical connectives.

---

## 2.1.4 Russell's Paradox

Consider the naive definition: a set can be formed from any property. This gives us the **comprehension schema**: for any property P(x), {x : P(x)} is a set.

Bertrand Russell discovered in 1901 (letter to Frege; published in Russell 1903) that this leads to contradiction. Let:

$$R = \{x : x \notin x\}$$

— the set of all sets that do not contain themselves. Does R contain itself?

- If R ∈ R, then by the defining property of R, R ∉ R. Contradiction.
- If R ∉ R, then R satisfies the defining property, so R ∈ R. Contradiction.

In either case, we get a contradiction. The comprehension schema is inconsistent: we cannot freely form sets from arbitrary properties.

This paradox destroyed Frege's logical program and forced mathematicians to be more careful about what constitutes a legitimate set. The resolution requires axiomatic set theory (Section 2.2), where the comprehension schema is restricted.

**Connection to logic**: Russell's paradox is essentially the same as the Liar's Paradox ("this statement is false"), with "contains itself" playing the role of "is true." Both exploit self-reference in a way that breaks the classical true/false dichotomy.

---

## References for Section 2.1

- Cantor, G. (1895). "Beiträge zur Begründung der transfiniten Mengenlehre I." *Mathematische Annalen*, 46, 481–512. [Cantor's own presentation of set theory; English translation by P. Jourdain, *Contributions to the Founding of the Theory of Transfinite Numbers*, Dover, 1955.]
- Halmos, P.R. (1960). *Naive Set Theory*. Van Nostrand. Reprinted Springer, 1974. [The best introduction to set theory for mathematicians — elegant, precise, and readable.]
- Russell, B. (1902). "Letter to Frege." In van Heijenoort (1967), pp. 124–125. [The original statement of Russell's paradox.]
- Russell, B. (1903). *The Principles of Mathematics*. Cambridge University Press. [Contains the first published statement of Russell's paradox in §101.]
- Zermelo, E. (1908). "Untersuchungen über die Grundlagen der Mengenlehre I." *Mathematische Annalen*, 65, 261–281. [The axiomatic response to Russell's paradox.]
