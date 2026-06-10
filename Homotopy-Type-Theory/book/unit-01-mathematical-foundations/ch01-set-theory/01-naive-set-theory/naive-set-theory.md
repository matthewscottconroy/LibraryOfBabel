# Naive Set Theory

## Cantor's Original Vision

Georg Cantor did not set out to found mathematics. He was trying to solve a specific problem: can a function on an interval have different trigonometric series representations, and if so, where must the representations agree? Answering this required understanding the structure of arbitrary sets of real numbers — not just intervals, not just finite unions, but genuinely arbitrary infinite subsets.

The tools did not exist. Cantor invented them.

His 1874 paper established the uncountability of the real numbers. His 1891 paper gave the cleaner diagonal argument and generalized it: for *any* set A, the power set 𝒫(A) — the set of all subsets of A — has strictly greater cardinality than A. There is no largest infinity. The transfinite is stratified.

Cantor's definition of a set was simple and natural: "A set is a collection M of definite, well-distinguished objects m of our intuition or our thought (which are called the 'elements' of M) into a whole." This definition is informal but clear. A set is a collection. An element is a member. A set is determined by its members.

From this starting point, Cantor built an elaborate theory:

**Equality.** Two sets are equal iff they have the same elements (extensionality).

**Subset.** A ⊆ B iff every element of A is an element of B.

**Union.** A ∪ B = {x | x ∈ A or x ∈ B}.

**Intersection.** A ∩ B = {x | x ∈ A and x ∈ B}.

**Difference.** A \ B = {x | x ∈ A and x ∉ B}.

**Power set.** 𝒫(A) = {B | B ⊆ A}.

**Cartesian product.** A × B = {(a, b) | a ∈ A and b ∈ B}.

These operations are straightforward. The trouble starts with comprehension.

## The Comprehension Principle

**Naive Comprehension.** For any property φ(x), there exists a set {x | φ(x)} containing exactly the elements satisfying φ.

This principle seems like the defining feature of set theory: sets are defined by properties. It underlies all of Cantor's work and Frege's foundational program.

Examples:
- The set of even natural numbers: {n ∈ ℕ | n is even}
- The set of prime numbers: {p ∈ ℕ | p is prime}
- The set of continuous functions: {f | f is a continuous function ℝ → ℝ}

These examples seem perfectly fine. The trouble is that naive comprehension permits *any* property — including self-referential ones.

## Russell's Paradox

**The paradox.** Let R = {x | x ∉ x}. Then:
- If R ∈ R, then R satisfies the defining property x ∉ x, so R ∉ R. Contradiction.
- If R ∉ R, then R satisfies x ∉ x, so by the definition of R, R ∈ R. Contradiction.

Both cases lead to contradiction. Therefore R cannot be a set. But naive comprehension says it must be. Naive comprehension is inconsistent.

**Diagnosis.** The problem is self-reference. The property "x ∉ x" refers to the membership relation applied to x with x as both the set and the element. When we form {x | x ∉ x} and ask whether it is a member of itself, we are led in a circle.

This is the same structure as the Liar paradox ("This sentence is false"): a sentence or set refers to itself in a way that produces a contradiction. Gödel would later use the same self-referential structure to prove his incompleteness theorems — by creating a sentence in arithmetic that says "I am not provable in this system."

**Other paradoxes.** Russell's paradox is not the only one.

*Cantor's paradox.* Let V = {x | x is a set}. Then 𝒫(V) ⊆ V (since 𝒫(V) consists of sets). So |𝒫(V)| ≤ |V|. But Cantor's theorem says |𝒫(V)| > |V| for any set V. Contradiction. So V cannot be a set.

*Burali-Forti paradox.* Let Ω = {x | x is an ordinal}. If Ω is an ordinal, then Ω ∈ Ω (since Ω is the largest ordinal, and ordinals are transitive). But ordinals are well-founded — no ordinal contains itself. Contradiction.

All three paradoxes arise from the same source: unrestricted comprehension applied to self-referential or "too large" properties. The universe of all sets is not itself a set. The collection of all ordinals is not an ordinal.

## Responses to the Paradoxes

Three main responses were developed in the decades following Russell's letter.

**Russell's type theory.** Objects are assigned to a hierarchy of *types*: type 0 contains individuals, type 1 contains sets of individuals, type 2 contains sets of sets of individuals, and so on. The membership relation x ∈ y is only well-formed if y is exactly one type higher than x. Self-referential constructions are ruled out because R = {x | x ∉ x} requires x and {x | ...} to be the same type — which is forbidden.

Type theory solves the paradoxes but introduces significant complications. *Principia Mathematica* required a *ramified* type theory with additional restrictions on how properties are defined, to avoid more subtle paradoxes (involving impredicative definitions in arithmetic). The result was technically correct but unwieldy.

**Zermelo's axiomatic approach.** Rather than a general comprehension principle, allow only *restricted* comprehension: from any existing set A and any property φ, form the subset {x ∈ A | φ(x)}. Since we start from an existing set and only carve out subsets, we cannot form {x | x ∉ x} — we would need an initial set to restrict to, and that initial set itself would have to contain R, which is circular.

This is the approach that became ZFC. The comprehension axiom becomes the *Separation* axiom (Aussonderung): for any set A and property φ, the set {x ∈ A | φ(x)} exists. Other axioms (Pairing, Union, Power Set, Infinity, Replacement) provide the existence of specific sets that do not follow from Separation alone.

**Von Neumann-Bernays-Gödel (NBG) set theory.** Introduce two sorts: *sets* and *proper classes*. Every set is a class, but not every class is a set. Proper classes (like V = "the class of all sets") exist as mathematical objects but cannot themselves be members of other classes. This avoids the paradoxes because R = {x | x ∉ x}, if it were a proper class, could not be asked whether it is a member of itself — membership requires the left-hand side to be a *set*.

NBG is conservative over ZFC for statements about sets: any sentence about sets provable in NBG is provable in ZFC, and vice versa. It is sometimes more convenient for category theory (which deals with "the category of all sets," a proper class) but adds conceptual complexity.

## The Cumulative Hierarchy

The ZFC approach — axiomatic restriction — comes with a natural picture of what sets look like: the *cumulative hierarchy* V.

**Definition.** Define sets Vα for each ordinal α:
- V₀ = ∅
- Vα₊₁ = 𝒫(Vα) (the power set of the previous stage)
- Vλ = ⋃_{α < λ} Vα for limit ordinals λ

The *axiom of foundation* (or *regularity*) says every set belongs to some Vα. This gives a stratified picture: sets are built up in stages, and no set can contain itself because a set at stage α+1 can only contain sets from earlier stages.

This picture is consistent with all the ZFC axioms and provides a beautiful mental model: the set-theoretic universe is a well-founded hierarchy, like a tree growing outward from ∅, where each new stage adds all the subsets of what already exists.

The analogy to type theory is direct. In Martin-Löf Type Theory, types are introduced at *universe levels* Uᵢ: small types live in U₀, types that involve U₀ live in U₁, and so on. The universe hierarchy prevents self-referential types (Type : Type would allow Girard's paradox). The set-theoretic cumulative hierarchy and the type-theoretic universe hierarchy solve the same problem — preventing self-referential collapse — by the same structural idea: a well-founded stratification.

## What Naive Set Theory Gets Right

Before moving to the axioms, it is worth noting what naive set theory gets right. The intuition behind set comprehension — that properties define collections — is mathematically productive. Category theory requires "large" collections (the category of all groups, all topological spaces) that are not sets in ZFC. Most mathematicians work with set comprehension informally, never forming pathological sets, and get correct results.

The lesson is not that Cantor's intuition was wrong. It is that it must be disciplined. The ZFC axioms are that discipline: they codify the constructions mathematicians actually need, block the pathological constructions, and provide a system that is (we believe, though it cannot prove itself) consistent.

In HoTT, the discipline is different: types are defined by their *constructors* (inductive definitions), and self-reference is controlled by universe levels. The shift from "defined by any property" to "defined by explicit constructors" is one of the deepest changes between set-theoretic and type-theoretic foundations.
