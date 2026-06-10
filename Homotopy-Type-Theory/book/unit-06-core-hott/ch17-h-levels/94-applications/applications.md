# Applications: H-Levels and Truncations

## Application 1: Database Theory and Query Semantics

Database queries must answer questions of the form "does there exist a record satisfying condition P?" The h-level hierarchy provides the formal foundation for distinguishing between different strengths of this existential claim.

In a database system, there are (at least) three distinct things a query might return:
1. A specific record satisfying P (proof-relevant existence: Σ(r:Record).P(r))
2. The fact that some record satisfies P, without specifying which (mere existence: ‖Σ(r:Record).P(r)‖)
3. The set of all records satisfying P (a list/set: the n-element subset)

The h-level hierarchy formalizes exactly these distinctions. Proposition-level queries (‖−‖) return yes/no. Set-level queries return unordered collections. The difference matters for query optimization: a proposition-level query can short-circuit (find one answer), while a set-level query must find all.

In formal database theory (using the relational model), these distinctions correspond to the three types of query semantics: existence queries, counted queries, and enumeration queries. The propositional truncation is the formal counterpart of the SQL `EXISTS` clause, while proof-relevant existence corresponds to `SELECT` with a `LIMIT 1`. The h-level framework gives these a unified formal treatment.

## Application 2: Type-Safe Programming and Proof-Relevant APIs

In software engineering, the distinction between propositions and sets (or higher types) corresponds to the distinction between boolean results and data results.

A function `isPrime : N → Bool` computes a proof-irrelevant truth value. But `isPrime : N → Prop` (where Prop is the universe of propositions) gives a mere proposition — the property of primeness. These have the same "logical content" but different computational behaviors.

A *proof-relevant* version `isPrime : N → Type` would give, for each prime n, a specific proof of primeness — and different proofs might correspond to different primality certificates (different witnesses to the Fermat/Miller-Rabin/AKS primality test). Software that returns a primality certificate (not just true/false) operates at the proof-relevant level.

The h-level framework tells API designers exactly what they are committing to: a boolean API (h-level 0) collapses all proofs; a proof-relevant API (higher h-level) preserves them. The Rust and Haskell type systems, while not HoTT, are moving toward finer distinctions of this kind through indexed types and GADTs.

## Application 3: Homotopy-Based Machine Learning

In topological data analysis (TDA), the homology groups of a dataset capture its "shape" — connected components (H₀), loops (H₁), voids (H₂), and so on. These correspond precisely to the homotopy groups of a type:

- H₀ corresponds to π₀ = ‖A‖₀ (set of connected components, the set-truncation)
- H₁ corresponds to π₁ = ‖Ω(A)‖₀ (the fundamental group, the set-truncation of the loop space)
- H_n corresponds to π_n (the n-th homotopy group)

The h-level hierarchy determines which of these groups are trivial. A contractible space has all H_n = 0. A set (discrete space) has H_n = 0 for n ≥ 1. A groupoid has H_n = 0 for n ≥ 2 and H₁ = the fundamental group.

In machine learning applications of TDA, the question "what is the h-level of this dataset's shape?" corresponds to "how many topological features does this dataset have?" A dataset that is "essentially contractible" (h-level -2) has no features — it's a single blob. A dataset with interesting loops (h-level ≥ 1) has circular structure worth detecting. The h-level provides a hierarchy of complexity for dataset shapes, directly applicable to dimensionality reduction and anomaly detection.

## Application 4: Formal Verification of Mathematical Theorems

The Four Color Theorem was proved by Appel and Haken in 1976 using computer assistance — a proof that was initially controversial because it relied on checking millions of cases by computer. The Coq formalization by Gonthier in 2005 verified the proof formally, using a combination of higher-order logic and efficient combinatorial representations.

The h-level machinery is crucial for such large formalizations. Every object in the formal proof (graph, coloring, configuration, reducibility proof) must be assigned an h-level. Graphs are sets of vertices and edges — h-level 0. Colorings are functions from vertices to colors — also sets if colors form a set. The theorem "any planar graph is 4-colorable" lives at the propositional level (h-level -1): it is a proposition about the existence of a coloring.

The key point: the theorem need only produce a coloring (proof-relevant existence), but the *statement* of the theorem is propositional (mere existence of a coloring is sufficient). The h-level framework guides the formalization: work proof-relevantly inside the proof, state the conclusion propositionally.

## Application 5: Foundations of Category Theory

Category theory is naturally expressed at h-level 1 (groupoids) and above. A category is not just a set of morphisms — morphisms compose and the composition is associative (up to a propositional equality), making a category naturally a "groupoid-like" thing when you only keep the invertible morphisms.

The Rezk completeness condition for categories in HoTT says: a category (in the HoTT sense, called a *univalent category* or *Rezk category*) is one where the type of objects is a *1-type* (groupoid), and the condition `isEquiv : (a = b) → (a ≅ b)` (from equality of objects to isomorphism) is satisfied.

This is a direct application of the h-level hierarchy: categories naturally live at h-level 1 (their object types are groupoids). The condition that a category be "Rezk complete" or "univalent" is exactly the condition that object equality is the same as isomorphism — the category-theoretic analog of the Univalence Axiom.

The h-level hierarchy thus provides the foundation for a fully formal theory of categories in HoTT, with full proof assistant support, that is automatically invariant under equivalence of categories.

## Application 6: Quantum Computing and Quantum Error Correction

In quantum computing, a quantum state is not just a bit (true/false) — it is an element of a Hilbert space, with potentially complex superposition structure. Quantum measurement collapses this superposition to a classical bit — an operation analogous to propositional truncation.

More precisely: a quantum state in a 2-dimensional Hilbert space (a qubit) is an element of the unit sphere S³. Measurement collapses this to a classical bit (an element of Bool). The map S³ → Bool (projection) is many-to-one — many quantum states give the same measurement outcome. Propositional truncation in HoTT is the formal analog: ‖S³‖ = ‖1‖ = 1 (the sphere is contractible), but this is not the right truncation for qubits.

The h-level hierarchy suggests that quantum information naturally lives at h-level 1 (a groupoid of quantum states with unitaries as morphisms), while classical information lives at h-level 0 (a set of bits). Quantum error correction — the process of protecting quantum information against noise — can be understood as maintaining the h-level 1 structure against processes that would collapse it to h-level 0 (decoherence). This connection, while speculative, suggests that the h-level hierarchy may provide the right formal framework for understanding quantum information in type-theoretic terms.
