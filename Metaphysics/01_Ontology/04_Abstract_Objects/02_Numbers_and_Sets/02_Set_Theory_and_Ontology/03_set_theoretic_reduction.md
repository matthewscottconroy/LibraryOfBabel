# Set-Theoretic Reduction

One of the central projects of late nineteenth and early twentieth century foundations of mathematics was to show that all of mathematics could be reduced to set theory. Numbers, functions, ordered pairs, relations — all could be identified with sets or sets of sets. If this reduction works, then the ontological commitments of mathematics are just commitments to the existence of sets.

The reduction strategy begins with defining the natural numbers as sets, then defining operations (addition, multiplication) on those sets, proving that these satisfy the Peano axioms, and then showing that real numbers, complex numbers, functions, and other mathematical entities can all be defined in terms of sets.

Paul Benacerraf's famous 1965 paper "What Numbers Could Not Be" raised a decisive problem for the identification of numbers with particular sets. There are many ways to define the natural numbers as sets — von Neumann's and Zermelo's among them — and all of them satisfy the Peano axioms equally well. There is no non-arbitrary reason to prefer one over another.

Benacerraf concluded that numbers are not sets at all. If there were a fact of the matter about which sets the numbers are, there would be a principled reason to prefer one reduction; since there is not, numbers must be something else. His positive suggestion was that numbers are positions in an abstract structure — a view that motivates mathematical structuralism.

The set-theoretic reduction remains valuable as a way of showing that mathematics can be formalized and that mathematical statements can be given precise truth-conditions within a single framework. But the philosophical lesson is that this formalization does not settle questions of mathematical ontology: what we are formalizing may not be what sets there are.

## The Success of Set-Theoretic Reduction

The set-theoretic reduction is one of the great achievements of nineteenth and twentieth century mathematics. The key reductions:

**Natural numbers**: Defined as sets using either the von Neumann ordinals (0 = ∅, n+1 = n ∪ {n}) or the Zermelo ordinals (0 = ∅, n+1 = {n}). Both satisfy the Peano axioms; arithmetic operations are defined set-theoretically.

**Integers**: Defined as equivalence classes of ordered pairs of natural numbers: the integer corresponding to n − m is the equivalence class [(n, m)] where (n₁, m₁) ~ (n₂, m₂) iff n₁ + m₂ = n₂ + m₁.

**Rationals**: Defined as equivalence classes of ordered pairs of integers (p, q) with q ≠ 0, where (p₁, q₁) ~ (p₂, q₂) iff p₁ × q₂ = p₂ × q₁.

**Real numbers**: Defined as Dedekind cuts (pairs of sets partitioning the rationals) or as equivalence classes of Cauchy sequences of rationals. Both constructions yield the complete ordered field with the least upper bound property.

**Functions**: A function f: A → B is defined as a set of ordered pairs — a subset of A × B in which every element of A appears exactly once as a first component.

**Ordered pairs**: The Kuratowski definition: ⟨a, b⟩ = {{a}, {a, b}}. This encodes the pair as a set in a way that recovers both components and preserves the ordering.

Each definition is mathematically correct — the defined sets satisfy all the expected axioms and theorems. The reduction is technically successful.

## The Benacerraf Problem

Benacerraf's argument focuses on the natural numbers:

**Step 1**: There are at least two successful set-theoretic definitions of the natural numbers:
- Von Neumann: 0 = ∅, 1 = {∅}, 2 = {∅, {∅}}, 3 = {∅, {∅}, {∅, {∅}}}, ...
- Zermelo: 0 = ∅, 1 = {∅}, 2 = {{∅}}, 3 = {{{∅}}}, ...

**Step 2**: Both definitions satisfy the Peano axioms, so both yield something deserving to be called "the natural numbers" from a mathematical standpoint.

**Step 3**: The two definitions come apart on non-arithmetic questions: on the von Neumann definition, 1 ∈ 3; on the Zermelo definition, 1 ∉ 3 (since 3 = {{{∅}}} and 1 = {∅}, and {∅} ≠ {{∅}}).

**Step 4**: If numbers are particular sets, then the statement "1 ∈ 3" has a determinate truth value. But no arithmetic consideration determines whether we should use the von Neumann or Zermelo definition. The choice is arbitrary.

**Conclusion**: Since no choice is privileged, numbers are not particular sets. If they exist at all, they are not particular objects with non-arithmetic properties — they have only arithmetic properties. This is the structuralist conclusion: numbers are positions in an arithmetic structure, not particular objects with further identity conditions.

## What the Reduction Accomplishes

Benacerraf's argument does not show that the set-theoretic reduction is useless — it shows that it accomplishes less than might be hoped. The reduction establishes:

1. **Consistency relative to set theory**: If set theory is consistent, then arithmetic is consistent. The reduction shows how to model arithmetic within set theory.

2. **Definitional independence**: Mathematical concepts can be made precise within a single foundational framework. The reduction provides a canonical, precise language for mathematics.

3. **Proof-theoretic gains**: Many mathematical proofs become tractable within set theory that would be difficult to formalize otherwise.

What the reduction does *not* establish:

1. **Ontological identity**: Numbers are not *identical to* sets. The reduction provides models of arithmetic within set theory, not identities.

2. **Unique ontology**: The multiplicity of reductions shows that set theory is not the unique home of arithmetic. Category theory, type theory, and other foundations can serve as equally valid homes for mathematical objects.

3. **Settling the metaphysical question**: Whether mathematical objects exist, what they are, and how we know about them are not settled by the technical success of the reduction.

## Category Theory and Structural Foundations

Category theory, developed by Eilenberg and Mac Lane in the 1940s, offers an alternative foundational perspective that is structuralist from the start. In category theory:

- The focus is on morphisms (structure-preserving maps) rather than on the internal constitution of objects.
- Objects are characterized by their categorical properties — by what morphisms go into and out of them — rather than by what they are "made of."
- The natural numbers are characterized by their universal property: ℕ is the initial object in the category of commutative monoids.

On this categorical foundation, the Benacerraf problem dissolves: there is no question about whether numbers are von Neumann or Zermelo ordinals, because the categorical characterization of ℕ does not fix any particular set-theoretic representation. Any structure satisfying the universal property is the natural numbers. Multiple set-theoretic models are allowed; none is privileged.

This aligns with structuralism: mathematical objects are characterized by their structural roles, not by their intrinsic constitution. The category-theoretic foundation makes this structuralist insight precise and provides tools for working with abstract mathematical structures directly, without the mediating role of set theory.
