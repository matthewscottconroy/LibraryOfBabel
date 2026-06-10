# The ZFC Axioms

## The Architecture of Restriction

Zermelo's 1908 paper begins with a problem: Cantor's set theory is indispensable but contradictory. The solution is not to abandon sets but to discipline them. Instead of permitting every property to define a set, we specify exactly which construction principles are valid — and nothing more.

The result is a list of axioms. Each one sanctions a specific type of set-existence claim. Together, they are strong enough to build all of ordinary mathematics, and (we believe) weak enough to avoid contradiction.

We have ten axioms: Extensionality, Empty Set, Pairing, Union, Power Set, Separation, Infinity, Replacement, Foundation, and Choice. We take them one by one.

## Axiom 1: Extensionality

**Axiom of Extensionality.** Two sets are equal if and only if they have the same elements:

∀A. ∀B. (A = B ↔ ∀x. (x ∈ A ↔ x ∈ B))

This axiom defines what "set equality" means. A set is completely characterized by its members. Two sets that contain exactly the same elements are the same set, regardless of how they were defined, constructed, or named.

**Philosophical note.** Extensionality distinguishes sets from *intensions* (properties or descriptions). The property "x is even and less than 5" and the property "x ∈ {2, 4}" define the same extension — the set {2, 4} — and so they define the same set. In HoTT, by contrast, types can be *extensionally* equal (have the same elements) without being *intensionally* equal (defined the same way), and the Univalence Axiom says: type equivalence (the right intensional notion) implies type equality.

## Axiom 2: Empty Set

**Axiom of Empty Set.** There exists a set with no elements:

∃∅. ∀x. x ∉ ∅

By extensionality, there is exactly one such set, called the *empty set* and denoted ∅. This is the set with no members.

**Motivation.** We need a starting point. Without the empty set, we cannot even begin the cumulative hierarchy V₀ = ∅, V₁ = 𝒫(∅) = {∅}, V₂ = {∅, {∅}}, ....

In type theory, the corresponding object is the *empty type* 0 (⊥) — the type with no inhabitants. The elimination principle for 0 says: from an element of 0, you can derive anything (ex falso quodlibet). This is the type-theoretic form of the classical logical principle.

## Axiom 3: Pairing

**Axiom of Pairing.** For any two sets a, b, there exists a set {a, b} containing exactly a and b:

∀a. ∀b. ∃P. ∀x. (x ∈ P ↔ (x = a ∨ x = b))

From this, we can define singletons: {a} = {a, a}.

**Motivation.** We need to form ordered pairs, which are the building blocks of relations and functions. The Kuratowski encoding defines (a, b) = {{a}, {a, b}}. One can verify that (a, b) = (c, d) iff a = c and b = d — this is the right property for an ordered pair.

## Axiom 4: Union

**Axiom of Union.** For any set A, there exists a set ⋃A containing exactly the elements of elements of A:

∀A. ∃U. ∀x. (x ∈ U ↔ ∃B. (B ∈ A ∧ x ∈ B))

The set U is denoted ⋃A or ⋃_{B∈A} B.

**Applications.** The binary union A ∪ B = ⋃{A, B} is a special case. We can also form the union of any collection of sets: if F = {A₁, A₂, A₃, ...} is a family, then ⋃F = A₁ ∪ A₂ ∪ A₃ ∪ ....

## Axiom 5: Power Set

**Axiom of Power Set.** For any set A, there exists the set 𝒫(A) of all subsets of A:

∀A. ∃P. ∀B. (B ∈ P ↔ B ⊆ A)

**Motivation.** Power sets are the primary source of new, larger sets. Without the Power Set axiom, we could form countable unions but could not jump to uncountable sets. With it, we can form ℝ as 𝒫(ℕ) (or as a subset thereof), and the entire cumulative hierarchy becomes possible.

**Size.** For a finite set A with n elements, |𝒫(A)| = 2ⁿ. For infinite sets, |𝒫(A)| > |A| (Cantor's theorem). The Power Set axiom is what allows us to "go up" in the hierarchy of cardinalities.

## Axiom 6: Separation (Subset Selection)

**Axiom Schema of Separation (Aussonderung).** For any set A and any formula φ(x) (with parameters), there exists a set containing exactly those elements of A satisfying φ:

∀A. ∃S. ∀x. (x ∈ S ↔ (x ∈ A ∧ φ(x)))

This is a *schema* — one axiom for each formula φ. We write S = {x ∈ A | φ(x)}.

**Key difference from naive comprehension.** We must start from an *existing* set A. We can carve out subsets, but we cannot form sets from scratch using arbitrary properties. This blocks Russell's paradox: to form {x | x ∉ x}, we would need an existing set to restrict to, and that set would have to contain all sets — which is exactly what we cannot assume.

**Why this is enough.** Every construction in mathematics that looks like "the set of all X with property φ" can be rephrased as "the subset of some larger, known set A with property φ." The integers satisfying a polynomial equation live in ℤ. The continuous functions with a certain property live in C([0,1]). We always have a containing set to separate from.

## Axiom 7: Infinity

**Axiom of Infinity.** There exists an inductive set — a set I that contains ∅ and is closed under the successor operation x ↦ x ∪ {x}:

∃I. (∅ ∈ I ∧ ∀x. (x ∈ I → x ∪ {x} ∈ I))

The natural numbers are defined as the smallest such set. By Separation (applied to the intersection of all inductive subsets of I), we get ω = {∅, {∅}, {∅, {∅}}, ...} — the von Neumann encoding of ℕ.

**Why we need a special axiom.** Without the Axiom of Infinity, ZFC proves only facts about hereditarily finite sets — sets whose elements, and whose elements' elements, etc., are all finite. This is consistent but too weak for analysis or algebra.

## Axiom 8: Replacement

**Axiom Schema of Replacement.** If φ(x, y) is a formula that defines a function (for every x, there is at most one y with φ(x, y)), then for any set A, the image {y | ∃x ∈ A. φ(x, y)} is a set:

∀A. (∀x ∈ A. ∃!y. φ(x, y)) → ∃B. ∀y. (y ∈ B ↔ ∃x ∈ A. φ(x, y))

**Motivation (Fraenkel's contribution).** Zermelo's original axioms cannot prove that the set {ω, 𝒫(ω), 𝒫(𝒫(ω)), ...} exists. Separation can only take subsets of existing sets; it cannot build new sets by applying a function. Replacement says: if you have a set A and a definable function F, the image F[A] is also a set.

**Consequence.** Replacement dramatically increases the strength of ZF. With it, all ordinals can be proved to exist as sets, all cardinal arithmetic becomes tractable, and the cumulative hierarchy V is shown to extend beyond Vω₊ω (which is all that Zermelo's axioms alone guarantee).

## Axiom 9: Foundation (Regularity)

**Axiom of Foundation.** Every non-empty set has an element disjoint from it:

∀A. (A ≠ ∅ → ∃x. (x ∈ A ∧ x ∩ A = ∅))

**Consequences.** Foundation implies:
- No set contains itself: ¬∃x. x ∈ x.
- No 2-cycles: ¬∃x.∃y. (x ∈ y ∧ y ∈ x).
- More generally, there are no infinite descending membership chains: no sequence x₀ ∋ x₁ ∋ x₂ ∋ ....

**Philosophical role.** Foundation is the axiom that makes the universe of sets *well-founded*. It ensures that the cumulative hierarchy picture is correct: every set lives at some stage Vα. Non-well-founded set theories (which allow circular membership) exist and are studied, but ordinary mathematics does not need them.

In type theory, well-foundedness is built into the definition of inductive types: every element of an inductive type is built from constructors applied to strictly smaller elements. There are no circular types.

## Axiom 10: Choice

**Axiom of Choice (AC).** For any set A of non-empty sets, there exists a *choice function* f such that f(B) ∈ B for every B ∈ A:

∀A. ((∀B ∈ A. B ≠ ∅) → ∃f. ∀B ∈ A. f(B) ∈ B)

**Why it is controversial.** For finite families, choice functions obviously exist — just pick one. For countably infinite families of finite sets, you can often define a choice function explicitly (choose the smallest element, or apply some rule). But for uncountably infinite families of arbitrary non-empty sets, there may be *no rule* for choosing. The Axiom of Choice asserts that choices exist even when we cannot describe them.

**Independence.** Gödel (1938) showed that if ZF is consistent, then ZFC (ZF + Choice) is consistent. Cohen (1963) showed that if ZF is consistent, then ZF + ¬AC is also consistent. So Choice is genuinely independent of the other axioms: you can use it or not without contradiction.

**Constructive mathematics.** In constructive mathematics (including the type theory underlying HoTT), Choice as stated is too strong. However, a *dependent choice* principle — that allows sequential choices where each depends on the previous — is acceptable. Full AC implies the existence of non-measurable sets of real numbers, which conflicts with the constructive principle that all sets are measurable.

## The Cumulative Hierarchy

These ten axioms, taken together, describe a universe of sets with a beautiful structure: the *von Neumann cumulative hierarchy* V = ⋃_α Vα, where:

- V₀ = ∅
- Vα₊₁ = 𝒫(Vα)
- Vλ = ⋃_{α<λ} Vα (limit ordinals)

Foundation ensures every set is in some Vα. Infinity ensures V reaches and surpasses Vω. Replacement ensures V surpasses Vω₊ω and beyond. Power Set provides the growth at successor stages.

This is the official picture of the set-theoretic universe. It is vast, well-founded, and (we believe) consistent. Whether it is *true* — whether there is a mathematical reality this hierarchy is accurately describing — is a philosophical question that has occupied set theorists and philosophers of mathematics for a century.

That question is not answered by any axiom. It is one of the deep open problems at the intersection of mathematics and philosophy. HoTT, by taking a more constructive approach, sidesteps some of it — but creates new philosophical questions of its own.
