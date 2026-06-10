# Applications: Synthetic Homotopy Theory

## Application 1: Formal Verification of Distributed Consensus Protocols

Distributed systems face a fundamental problem: nodes must agree on a shared value despite unreliable communication. Protocols like Paxos, Raft, and PBFT solve consensus, but their correctness proofs are notoriously subtle and error-prone. Several bugs in published protocols have been discovered only years after "formal" proofs were accepted.

Synthetic homotopy theory offers a new framework for understanding these protocols. The key insight: the space of possible protocol states is not a simple set but a higher-dimensional type, and the correctness condition is a statement about which loops (sequences of state transitions that return to the start) are contractible.

**The connection.** A consensus protocol defines a type State of protocol states. A "run" of the protocol is a path in State. Correctness conditions like "any two correct runs agree on the committed value" translate to: any two paths in State with the same endpoints are homotopic (the space State is "sufficiently path-connected").

The encode-decode method then becomes the proof technique: define a code family code : State → Type that captures the "consensus value" at each state, and prove that transport along any correct run gives the agreed value. The winding number computation from π₁(S¹) = Z is a toy model: the code family counts how many times the protocol cycles, and the theorem is that any complete run agrees with this count.

**Concrete example.** Raft's log replication can be modeled as: each leader term corresponds to a loop in a State type. The "election term" counter is exactly the winding number — transport along the term loop increments by 1. The correctness of log replication (committed entries are never overwritten) becomes the statement that the code family is injective on committed states.

This is not merely a metaphor. The HoTT-Agda library has been used to formally verify simplified consensus protocols by Angiuli et al. (2016). The encode-decode method gives a modular proof structure: prove the code family is correct, then derive all safety properties by transport.

**Why it matters.** Classical verification of distributed protocols uses TLA+ or model checking, which enumerate states. These methods cannot handle parametric protocols (with unbounded numbers of nodes). The HoTT approach handles parametric protocols naturally: the State type is defined abstractly, and the encode-decode proof works for any number of nodes.

## Application 2: Topological Data Analysis and Persistent Homotopy

Topological data analysis (TDA) studies the "shape" of high-dimensional data by computing topological invariants. The core tool is persistent homology: as you scale a threshold parameter ε, topological features (connected components, loops, voids) appear and disappear, and the "persistence" of a feature measures how robust it is.

The Freudenthal Suspension Theorem appears in TDA in a surprising way: it governs when topological features in a filtration stabilize as the dimension of the ambient space grows.

**The connection.** Given a point cloud X in Rⁿ, the Vietoris-Rips filtration produces a sequence of simplicial complexes V_ε(X). As ε grows, the homology H_k(V_ε(X)) changes. The persistence diagram records when each homology class appears (birth) and disappears (death).

For data embedded in Rⁿ, the Freudenthal theorem predicts that the persistent homotopy groups stabilize as you increase the ambient dimension: πₖ(V_ε(X)) → πₖ(V_ε(ΣX)) is an isomorphism in a range. This "stable" range is where topological features are most robustly detected.

Synthetic homotopy theory provides the right language for this: instead of working with specific spaces embedded in Rⁿ, you work with the type of all filtrations, and the Freudenthal theorem applies universally. The encode-decode method can then be applied to compute persistent fundamental groups directly from the combinatorial structure of the filtration.

**Concrete implementation.** The Python library Ripser computes persistent homology. Future HoTT-based tools could compute persistent homotopy groups (not just homology) using the encode-decode method. For π₁, this amounts to computing the van Kampen theorem at each filtration level — a computation the HoTT approach makes modular and formally verifiable.

**Why it matters.** Homology detects certain topological features but misses others (it cannot distinguish the torus from the Klein bottle in H₁ over Z/2Z). Homotopy groups detect more, but are harder to compute. The synthetic approach to homotopy computation opens the possibility of more powerful TDA tools that formally verify their topological claims.

## Application 3: Formal Proof Assistants and Homotopy Theory Libraries

The Brunerie computation — the formal verification that β = 2 and therefore π₄(S³) = Z/2Z — is a landmark in the history of formal mathematics. It demonstrated that a cutting-edge theorem in algebraic topology, using techniques that were barely understood by any human expert at the time, could be formally verified in a proof assistant.

This achievement rests on the synthetic approach. The theorem is not formalized by translating classical topology into Coq or Lean — that would require formalizing covering spaces, CW complexes, simplicial sets, and spectral sequences, a project of enormous scope. Instead, the theorem is proved synthetically in Cubical Agda, using only the HIT definitions and the encode-decode method.

**The library ecosystem.** Three libraries support synthetic homotopy theory:

1. **HoTT-Agda** (https://github.com/HoTT/HoTT-Agda): The original library, developed 2011-2018. Contains the encode-decode proof of π₁(S¹) = Z, the van Kampen theorem (Favonia-Shulman), and early approaches to the Hopf fibration.

2. **Cubical Agda** (https://github.com/agda/cubical): The modern library, using Cubical type theory for computable Univalence. Contains the Brunerie number computation (Ljungström-Mörtberg 2023), the Hopf fibration, and the complete proof of π₄(S³) = Z/2Z.

3. **UniMath** (https://github.com/UniMath/UniMath): The Voevodsky-initiated library in Coq, formalizing a broad range of mathematics on univalent foundations. Contains formal developments of algebra, topology, and category theory using the synthetic approach.

**Impact on mathematics.** The formal verification of Brunerie's theorem established that:
- Computer verification of cutting-edge research mathematics is achievable in practice, not just in principle.
- The synthetic approach makes formalization dramatically more feasible than the classical-translation approach.
- Proof assistants are now genuine mathematical collaborators, not just verification tools.

Future targets include formal proofs of the Barratt-Priddy-Quillen theorem (relating stable homotopy to symmetric groups), the Adams spectral sequence, and eventually the computation of all 2-primary stable homotopy groups through dimension 64 (currently known to classical homotopy theorists but not formally verified).

## Application 4: Programming Language Semantics and Quotient Types

In programming language theory, quotient types — types modulo an equivalence relation — appear throughout: free monoids modulo reordering, abstract syntax trees modulo alpha-renaming, concurrent processes modulo bisimulation. Getting quotient types right in dependent type theory is technically challenging.

The HoTT approach via HITs provides the correct treatment. A quotient type A/R (where R : A → A → Prop is an equivalence relation) is the pushout:

```
Σ(a b:A). R(a,b) ⇉ A → A/R
```

(two maps sending each pair (a, b, r) to a and b respectively, forcing a = b in the quotient).

**The encode-decode connection.** The fundamental group of A/R is computed by van Kampen: loops in A/R correspond to chains of R-related elements in A. The encode-decode method computes π₁(A/R) from the combinatorial data of R. This is exactly how group presentations work: a group G = ⟨generators | relations⟩ is the fundamental group of the CW complex with one 0-cell, one 1-cell per generator, and one 2-cell per relation.

**Programming language application.** In a dependent type system, program equivalence can be modeled as an equivalence relation on programs, and the quotient type Program/Equiv is the type of programs-up-to-equivalence. The identity types in this quotient capture which programs are provably equivalent. The Brunerie-style computation corresponds to computing the "homotopy complexity" of the equivalence relation.

**Representation independence.** A major application of the Univalence Axiom in programming languages is representation independence: if two data structure implementations are equivalent, they are equal. The encode-decode method is the proof technique for establishing this equivalence formally. This has been applied to verify that the ArrayList and LinkedList implementations of lists satisfy the same interface (Angiuli-Harper-Wilson, 2021).

## Application 5: Gauge Theory and Mathematical Physics

In physics, gauge theory studies fields that are defined up to local symmetry transformations. The mathematical framework is a principal G-bundle over spacetime M — a fiber bundle with fiber the gauge group G and structure group G. Two field configurations that differ by a gauge transformation are physically identical.

The Hopf fibration is the prototypical example of a non-trivial principal bundle. The complex Hopf fibration S¹ → S³ → S² is the principal U(1)-bundle over S². The Dirac magnetic monopole is a U(1)-gauge field on S² with a non-trivial bundle structure characterized by the Hopf invariant.

**The synthetic connection.** In HoTT, a principal G-bundle over a type B is exactly the Hopf family construction: a type family H : B → Type where each H(x) is equivalent to G and the total space Σ(x:B).H(x) is the total bundle. The Hopf family H : S² → Type (with H(base) = S¹ = U(1)) is exactly the Hopf bundle.

The long exact sequence of the Hopf fibration corresponds to the long exact sequence in gauge theory relating the topology of the gauge group, the total bundle space, and the base space. The computation π₃(S²) = Z corresponds to the quantization of magnetic monopole charge.

**Quantum computing application.** Topological quantum computing uses anyons — quasiparticles with non-abelian statistics — to implement fault-tolerant quantum gates. The computation theory of anyons is precisely the representation theory of the braid group B_n = π₁(Conf_n(C)), the fundamental group of the configuration space of n particles in the plane.

The van Kampen theorem computes π₁ of configuration spaces. For topological quantum gates, the relevant computation is π₁(Conf_n(C)) = B_n (the braid group). This computation follows from van Kampen applied to the complement of the discriminant locus in configuration space.

The synthetic HoTT approach gives a formally verified framework for this computation, with direct application to quantum error correction: a fault-tolerant gate corresponds to a path in configuration space that is not contractible (has non-trivial winding number), and the encode-decode method computes which paths give which gates.

## Application 6: Algebraic K-Theory and Stable Homotopy

Algebraic K-theory is one of the deepest connections between algebraic topology and algebra. For a ring R, the K-groups K_n(R) are defined as the homotopy groups of an infinite loop space: K_n(R) = πₙ(BGL(R)⁺), where GL(R) is the infinite general linear group and the + denotes Quillen's plus-construction.

The Freudenthal theorem is foundational to this: it ensures that the K-groups stabilize and that the "stable" K-theory is accessible. The stable homotopy groups πₙˢ appear as K-groups of specific rings (the sphere spectrum), giving a direct connection between the stable homotopy of spheres and algebraic K-theory.

**The synthetic connection.** In HoTT, Eilenberg-MacLane spaces K(G, n) are defined directly as HITs. The K-groups of a ring can be defined synthetically using the loop space and suspension: Kₙ(R) = πₙ(K(GL(R), 1)⁺). This avoids the technical setup of model categories and simplicial sets required in the classical definition.

The Freudenthal theorem then gives: for n ≥ 3, Kₙ(R) → Kₙ₊₁(ΣR) is an isomorphism (in the stable range), where ΣR is the suspension of R as a ring spectrum. This is the K-theory analog of the stable homotopy stabilization.

**Why it matters for research.** The formalization of algebraic K-theory in HoTT is a major open project. A proof assistant that can formally verify K-theory computations would have direct applications to number theory (the Quillen-Lichtenbaum conjecture relates K-theory of number fields to special values of L-functions) and arithmetic geometry (motivic cohomology and the Bloch-Kato conjecture, proved by Voevodsky).

Voevodsky himself was motivated to develop HoTT partly by the need for formal verification in algebraic K-theory and motivic homotopy theory. The full realization of his vision would produce formally verified proofs of the deepest results in arithmetic algebraic geometry.

## Application 7: Machine Learning on Topological Spaces

Graph neural networks (GNNs) and topological machine learning (TML) process data with geometric structure. The question of which machine learning architectures can capture which topological properties is increasingly important as these tools are applied to molecular biology, materials science, and physical simulations.

The fundamental group is a topological invariant that GNNs typically cannot distinguish: two graphs with the same vertex and edge counts but different loop structures have the same GNN representations despite different π₁. The encode-decode method gives a constructive procedure for computing π₁ of a graph complex, providing both a theoretical tool for analyzing GNN expressivity and a practical algorithm for topology-aware machine learning.

**The synthetic connection.** A graph G can be viewed as a 1-dimensional CW complex (or HIT: vertices as point constructors, edges as path constructors). The fundamental group π₁(G) is computed by the encode-decode method applied to a code family on the graph's vertices. For a spanning tree T ⊂ G, the code family assigns to each vertex v the set of T-reduced paths from the base vertex to v, and the encode function computes the winding number around each non-tree edge.

This computation is exactly the classical algorithm for computing π₁ of a graph (the Schreier coset graph method), now formalized in HoTT and provably correct.

**Practical implementation.** The PyTorch Geometric library supports graph-level machine learning. A HoTT-inspired extension would compute provably correct topological features:
1. Construct the graph as a HIT.
2. Apply the encode-decode method to compute the fundamental group.
3. Use the group structure (e.g., abelianization H₁(G)) as a topological feature vector.
4. Train machine learning models on these formally verified features.

This approach guarantees that the topological features are true topological invariants (invariant under graph isomorphism and homotopy equivalence), unlike purely combinatorial features (degree sequences, etc.) that are not homotopy invariants.

The Brunerie computation suggests the horizon: if π₄(S³) = Z/2Z can be formally verified by computer, the computation of topological invariants of moderate complexity is within reach of practical machine learning pipelines.
