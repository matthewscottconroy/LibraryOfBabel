# Applications: Set Theory

## 1. Database Theory and Relational Algebra

The relational model of databases — invented by Edgar Codd in 1970 and now the dominant paradigm for data management — is built directly on set theory.

A *relation* in the database sense is a set of tuples. A table with columns Name, Age, Department is a set of triples (name, age, department). The basic database operations are set operations: SELECT is set filtering (Separation), JOIN is Cartesian product followed by filtering, UNION and INTERSECTION are the corresponding set operations.

The power of this formulation: once you identify database operations as set-theoretic, algebraic identities between set operations become query optimization rules. Two SQL queries that express the same set-theoretic computation will return the same results, and the database engine can choose the more efficient formulation.

Cantor-Bernstein and cardinality arguments appear in query complexity: the question "how many rows can this query return?" is a question about the cardinality of a certain set. Joins can produce up to |A| × |B| results (the full Cartesian product), and estimating this is a fundamental challenge in query optimization.

## 2. Descriptive Set Theory and Measurability

Descriptive set theory — the study of the definability and measurability of subsets of Polish spaces (complete separable metric spaces) — is a direct application of ZFC set theory to analysis.

The *Borel sets* are the smallest σ-algebra containing the open sets: they are built by countable unions, countable intersections, and complementation. The Borel hierarchy classifies sets by complexity: Σ⁰₁ (open sets), Π⁰₁ (closed sets), Σ⁰₂ (countable unions of closed sets), and so on, transfinitely.

*Analytic sets* (continuous images of Borel sets) are measurable and have the perfect set property (any uncountable analytic set contains a perfect subset, hence has cardinality continuum). *Co-analytic sets* are complements of analytic sets. Whether co-analytic sets are measurable depends on set-theoretic hypotheses beyond ZFC.

This is where set theory meets analysis in practice. The existence of non-measurable sets (from AC), the Vitali construction, and the distinction between "definable" and "arbitrary" sets — all of these are concrete consequences of the axioms, with direct impact on what kind of analysis is possible.

## 3. Cardinality in Computer Science: Undecidability

The cardinality arguments of set theory — specifically Cantor's diagonal argument — are the basis for the undecidability of the Halting Problem.

Turing showed in 1936 that no program can decide, for every program P and input x, whether P(x) halts. The proof is diagonal: suppose H(P, x) is such a decider. Define D(P) = "loop forever if H(P, P) says P(P) halts; halt otherwise." Does D(D) halt? If D(D) halts, H(D, D) says it does, so D(D) loops forever — contradiction. If D(D) loops forever, H(D, D) says it doesn't halt, so D(D) halts — contradiction.

This argument is Cantor's diagonal argument with programs replacing real numbers. The "size" of the set of all programs is countable (programs are finite strings). The "size" of the set of all problems (subsets of programs) is uncountable. So there are uncountably many problems and only countably many algorithms — most problems are undecidable.

Cardinality is not an abstract concern; it is the reason that computation has fundamental limits.

## 4. Ordinals in Proof Theory

The *proof-theoretic ordinal* of a formal system T is the supremum of the ordinals whose well-foundedness T can prove. It measures the "strength" of T.

- Primitive Recursive Arithmetic: proof-theoretic ordinal ωω.
- Peano Arithmetic: ε₀ (the smallest ordinal fixed by α ↦ ωα).
- Predicative Analysis: Γ₀ (the Feferman-Schütte ordinal).
- Full Second-Order Arithmetic: Π¹₁-CA₀ ordinal, much larger.

Gentzen's proof of the consistency of Peano Arithmetic uses transfinite induction up to ε₀ — an ordinal Peano Arithmetic itself cannot prove to be well-founded. This is the sense in which transfinite ordinals "transcend" formal systems.

For HoTT: the type-theoretic universes (U₀ ⊂ U₁ ⊂ U₂ ⊂ ...) correspond to stronger and stronger formal systems. The question of how many universes to assume — and what large-cardinal axioms are appropriate — is the type-theoretic analogue of the ordinal-strength hierarchy in proof theory.

## 5. Topology and Point-Set Theory

General topology is built on set theory. A topological space is a set X with a collection τ of subsets (the "open sets") satisfying: ∅ ∈ τ, X ∈ τ, τ is closed under finite intersections and arbitrary unions. Continuous functions, compactness, connectedness — all are defined in terms of set-theoretic operations on the open sets.

The Axiom of Choice appears throughout:
- Tychonoff's theorem (product of compact spaces is compact) is equivalent to AC.
- Every compact Hausdorff space is normal (T4) — requires AC.
- The existence of a well-ordering of every topological space's underlying set — requires AC.

Descriptive set theory (above) studies which topological statements depend on which set-theoretic principles. This makes the foundations of topology genuinely sensitive to the axioms of set theory — not all of topology is independent of set-theoretic choices.

## 6. Formal Verification of Mathematical Proofs

The Lean Mathematical Library (Mathlib) is a large-scale project to formalize mathematics in the Lean proof assistant, which is based on dependent type theory rather than ZFC. Yet the mathematics being formalized is largely set-theoretic: groups, rings, topological spaces, measure theory, algebraic geometry.

The translation from ZFC-based mathematics to type-theoretic formalization is non-trivial and illuminating. In ZFC, a group is a set with operations satisfying axioms. In Lean, a group is a type with a typeclass instance providing the operations and proofs of the axioms. The Kuratowski ordered pair encoding is replaced by the definitional product type. Functions are not sets of pairs; they are terms of function types.

The formalization process makes explicit all the informal conventions that ZFC-based mathematics relies on. Every isomorphism-as-equality inference must be replaced with explicit transport along an equivalence. Every "abuse of notation" must be justified. The labor of formalization reveals the actual structure of mathematical arguments and identifies where informal conventions were papering over genuine logical steps.

This is one practical motivation for HoTT: by building isomorphism-as-equality into the foundation (via Univalence), the gap between mathematical practice and formal foundation is reduced, making formalization less laborious.
