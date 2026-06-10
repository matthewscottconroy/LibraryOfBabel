# The Axiom of Choice

## What It Says

The Axiom of Choice says something modest-sounding: given any collection of non-empty sets, you can simultaneously choose one element from each. If you have a collection of pairs of shoes, you can choose one shoe from each pair. If you have infinitely many boxes, each containing at least one ball, there is a function picking one ball from each box.

For finite collections, this is trivially true — just choose. For countably infinite collections where each set has a definable "smallest" element, you can choose algorithmically. The content of Choice appears when the collection is uncountably infinite and the sets have no canonical structure — no natural "first" element, no algorithmic selection rule. Even then, Choice asserts: a simultaneous selection exists.

Formally:

**Axiom of Choice (AC).** For every set A of non-empty sets, there exists a *choice function* f with domain A such that f(B) ∈ B for every B ∈ A.

∀A. ((∀B ∈ A. B ≠ ∅) → ∃f: A → ⋃A. ∀B ∈ A. f(B) ∈ B)

## Why It Is Controversial

Every other ZFC axiom asserts the existence of a set that can be concretely described. Extensionality: the empty set is unique. Pairing: {a, b} is the set containing exactly a and b. Union: ⋃A contains exactly the elements of elements of A. Each axiom constructs a unique, fully specified set.

Choice is different. It asserts existence without uniqueness or description. For uncountably infinite families of arbitrary sets, there may be *no* definable choice function — no rule, formula, or algorithm that specifies which element to pick. Choice says: pick anyway.

This non-constructive character is what disturbed mathematicians like Borel, Baire, and Lebesgue (the "semi-intuitionists") who objected that mathematical existence should imply constructability. It is also what makes Choice unavoidable in classical analysis: without it, you cannot prove that every vector space has a basis, that every field has an algebraic closure, that every compact Hausdorff space is normal.

## Equivalent Formulations

Remarkably, the Axiom of Choice is equivalent (within ZF) to several statements that look completely different.

**Zorn's Lemma.** Let P be a non-empty partially ordered set in which every chain (totally ordered subset) has an upper bound in P. Then P has a maximal element.

*Equivalence to AC.* Zorn's Lemma → AC: given a collection A of non-empty sets, form the poset of all partial choice functions (choice functions defined on subsets of A), ordered by extension. Every chain has an upper bound (the union of the chain). By Zorn, there is a maximal partial choice function f. By maximality, f must be defined on all of A (if not, extend it to one more set — contradicting maximality). So f is a full choice function.

AC → Zorn's Lemma: given a chain-bounded poset P with no maximal element, use Choice to build an increasing sequence that goes beyond every element — eventually reaching a contradiction with the chain bound.

**Well-Ordering Theorem.** Every set can be well-ordered. (A well-ordering is a linear order where every non-empty subset has a least element.)

*Equivalence to AC.* This is the most natural direction: given a well-ordering of a set, the choice function picks the least element from each non-empty subset. Conversely, from a choice function on all non-empty subsets, we can build a well-ordering by a transfinite process of "choose the first element, then the first element of the remainder, ..."

**Tychonoff's Theorem.** The product of any collection of compact topological spaces is compact.

The Tychonoff theorem for *Hausdorff* spaces is provable without full AC (using the Boolean Prime Ideal Theorem). But for general compact spaces (not necessarily Hausdorff), the Tychonoff theorem is equivalent to AC over ZF.

**Basis Theorem.** Every vector space has a Hamel basis (a linearly independent spanning set).

*Without AC.* It is consistent with ZF that ℝ, viewed as a vector space over ℚ, has no Hamel basis. The existence of a Hamel basis for ℝ over ℚ is equivalent to AC (roughly).

**Comparability of Cardinals.** For any two cardinals κ, λ, either κ ≤ λ or λ ≤ κ.

Without Choice, it is consistent that there exist sets whose cardinalities are incomparable — neither is embeddable in the other. Choice resolves all cardinalities into a linear order.

## What Choice Enables

With Choice (i.e., in ZFC), mathematics has access to a rich toolkit:

**Infinite combinatorics.** Every infinite set has a countably infinite subset. Every infinite set can be partitioned into pairs. These basic combinatorial facts require Choice.

**Measure theory.** The Hahn-Banach theorem (there are linear extensions of bounded functionals) requires Choice. The existence of Borel regular measures and the Riesz representation theorem require Choice. The Lebesgue measure can be defined in ZF, but its key properties require Choice.

**Algebra.** Every commutative ring with unity has a maximal ideal (a consequence of Zorn's Lemma). Every field has an algebraic closure. These are the basic tools of algebraic geometry and number theory.

**Topology.** Alexander's subbase theorem (a space is compact iff every open cover from a subbase has a finite subcover) requires the Boolean Prime Ideal Theorem, which follows from AC. Tychonoff's theorem as above.

## What Choice Produces (Paradoxes)

Choice also implies results that feel geometrically impossible.

**Vitali sets.** Consider the real numbers ℝ modulo the equivalence relation "x ~ y iff x - y ∈ ℚ." This partitions ℝ into uncountably many equivalence classes, each dense in [0,1]. Using Choice, pick one representative from each class in [0,1]. The result is a *Vitali set* V — a subset of [0,1] that is not Lebesgue measurable. Its measure cannot be defined consistently.

**Banach-Tarski paradox.** Using the Axiom of Choice, you can decompose a ball in ℝ³ into finitely many (non-measurable) pieces, then reassemble them into two balls of the same size as the original. This "paradox" (it is actually a theorem) shows that AC produces sets that have no consistent notion of volume.

These results do not make AC inconsistent — they show that non-measurable sets exist, and that the usual geometric intuition about measure breaks down for arbitrary subsets. Measure theory, properly done, restricts attention to *measurable* sets and avoids the paradoxes.

## Choice in Constructive Mathematics

The Axiom of Choice is not available in constructive mathematics, including the type theory underlying HoTT. Here is the reason: a constructive proof of ∃x. P(x) must exhibit a witness — an explicit x with P(x). A constructive choice function, for a family indexed by a set A, must provide, for each B ∈ A, an explicit element of B. Without a *rule* for choosing, this is impossible constructively.

However, two weaker principles are available:

**Dependent Choice (DC).** If R is a relation on a set X such that for every x ∈ X there exists y ∈ X with R(x, y), then for every x₀ ∈ X there is a sequence x₀, x₁, x₂, ... with R(xₙ, xₙ₊₁) for all n.

This is a sequential, algorithmic form of choice: at each step, you pick one element, but the choice depends on the previous. DC is acceptable in many constructive frameworks and suffices for most of analysis.

**Countable Choice (AC_ω).** For any sequence of non-empty sets A₀, A₁, A₂, ..., there is a choice function. This is AC restricted to countable families and is acceptable constructively in many frameworks.

Full AC — uncountable choice from arbitrary families — requires non-constructive existence and is incompatible with certain constructive principles (like "every function ℝ → ℝ is measurable"). In HoTT specifically, it is a theorem that the Axiom of Choice holds for sets (in the sense of homotopy-level-0 types), because sets in HoTT are well-behaved enough that choice functions can be constructed. But for higher types, AC fails in general.

This is one of the most striking features of HoTT: it resolves the constructive/classical debate at a new level. For propositions and sets, HoTT behaves classically (Choice holds). For higher-dimensional types, constructive principles apply. The distinction between the homotopy levels of types determines which logical principles hold.
