# Applications: Cubical Agda and Computational HoTT in the Wider World

Cubical Agda is younger than Lean 4 and has fewer industrial deployments. But its applications are at the frontier: not just verifying known mathematics, but making new mathematics *computable*. This section surveys six concrete applications, from the verification of abstract mathematics to the practical consequences of having a type theory where univalence has a computation rule.

---

## Application 1: Synthetic Algebraic Topology as Computation

The classical computation of π₄(S³) = ℤ/2ℤ requires:
- The Hopf fibration (a non-trivial fiber bundle)
- The Blakers-Massey theorem (a connectivity bound)
- The Freudenthal suspension theorem (stabilization of homotopy groups)
- A careful calculation of a cup product structure

Each of these is a non-trivial theorem in algebraic topology. Formalizing them classically (in Lean 4 or Coq with classical topology) would require hundreds of pages of infrastructure. In Cubical Agda, using synthetic homotopy theory, Brunerie's formalization is around 10,000 lines of code — and the result computes.

The computational content is the application. The `encode : (base ≡ base) → ℤ` function is the winding number algorithm. The Hopf fibration in Cubical Agda is a fiber sequence that you can evaluate at specific inputs. These are programs that classical algebraic topology cannot provide — it gives existence proofs, not algorithms.

For automated theorem proving, this is the key: Cubical Agda proofs generate algorithms as a byproduct. A proof of $\pi_n(X) = G$ is an algorithm for computing the $n$-th homotopy class of a map $X \to X$. These algorithms can be extracted and used in geometric computing, computer graphics (computing homotopy classes of deformations), and robotics (motion planning in configuration spaces).

---

## Application 2: Verified Compilers and Language Semantics

The CompCert compiler (Lean 4 application, Chapter 21) has a counterpart in the HoTT setting: language semantics where equality of programs is not just propositional but homotopy-theoretic. This sounds abstract, but it has concrete consequences.

In standard semantics, two programs are "equal" if they have the same observable behavior. In HoTT semantics, the *paths between proofs* that two programs are equal can carry information: they correspond to the *rewrites* or *refactorings* that transform one program into another. The homotopy type of the space of equivalent programs is a measure of the "shape" of program equivalence.

The **MTT** (Multimodal Type Theory) project and related work uses modal HoTT (including cubical structures) to give semantics to programming languages with effects, stages, and modalities. This is semantics where the computational content of HoTT is directly relevant: transport in a type family parametrized by program states corresponds to moving data between states; univalence corresponds to the ability to swap equivalent representations.

Cubical Agda is used as a testbed for these semantic ideas: define the semantics of a small language in Cubical Agda, prove its properties using HoTT tools (path induction, univalence, HITs), and extract computational content from the proofs.

---

## Application 3: Univalent Foundations for Mathematics

The Univalent Foundations program, initiated by Voevodsky, aims to rebuild mathematics on HoTT foundations — where equality between mathematical objects means isomorphism (or equivalence), not identity. In classical foundations (ZFC set theory), two mathematical structures that are isomorphic may nonetheless be "different objects" (they have different elements). In Univalent Foundations, isomorphic structures are *equal* by univalence: `ua` converts the isomorphism into a path.

This has practical consequences for formalization. When you define a mathematical structure in classical foundations, you must prove many "transport lemmas": if $G_1 \cong G_2$ (isomorphic groups) and $P$ holds of $G_1$, you must separately prove $P$ holds of $G_2$. In Univalent Foundations, this is automatic: `transport (ua iso) proof` does it for free, because the isomorphism is literally a path.

The **UniMath** library (originally for Coq, now with an Agda counterpart) formalizes mathematics in this univalent style. The **Cubical Agda** library takes this further: not only are isomorphic structures equal (by univalence), but the transport along the equality *computes*, so you can evaluate what "transport along the isomorphism" gives concretely.

This application is primarily mathematical: it changes the foundations of mathematical practice rather than deploying to industry. But its consequences for formalization are significant — fewer lemmas to prove, cleaner mathematical interfaces, and a library organization that respects the mathematical principle that "isomorphic is good enough."

---

## Application 4: Type-Safe Metaprogramming and Reflection

Lean 4 has a macro system: code that generates code. Cubical Agda can extend this to *type-theoretically verified* metaprogramming, where the generated code is not just syntactically correct but semantically correct — provably so.

The idea: if you have a decision procedure for a class of goals (e.g., "is this goal provable by ring normalization?"), you can run the procedure and produce a proof certificate. In standard type theory, the certificate must be checked by the type checker. In Cubical Agda, with computable transport and univalence, the certificate itself can be a *computable* object: running the procedure produces a term, and the term reduces to the proof.

A concrete instance: **reflected proofs of ring identities**. You have a ring expression `e`, evaluate it symbolically to its normal form `n`, then produce a path `e ≡ n` by computation (not by a separate proof). This is possible in Cubical Agda because evaluation is definitional: the path holds by `refl`, not by a proof that required an axiom.

This is not yet fully deployed in practice, but it is a direction being explored in the Cubical Agda research community. The goal: a "proof by reflection" infrastructure for Cubical Agda that is both more powerful and more efficient than the analogous infrastructure in Lean 4 or Coq.

---

## Application 5: Cohomology Theories and Data Science

In classical algebraic topology, cohomology theories (singular cohomology, K-theory, cobordism, ...) classify topological spaces up to various equivalence relations. In Cubical Agda, the Eilenberg-MacLane spaces $K(G, n)$ can be defined as HITs, and cohomology can be defined synthetically:

```
Hⁿ(X; G) := ∥ X → K(G, n) ∥₀
```

(where $\|\cdot\|_0$ is the set-truncation). The Cubical library is developing this infrastructure, with the long-term goal of computing cohomology rings synthetically.

The practical application is in data science and applied topology. **Topological data analysis (TDA)** uses persistent homology to detect "topological features" (connected components, loops, voids) in datasets. The theoretical foundation of TDA is algebraic topology. A synthetic, computational formulation of these theories could provide:

1. Verified algorithms for computing persistent homology (where "verified" means machine-checked correctness, not just testing)
2. New algorithms derived from HoTT proofs (the computational content of cohomology theorems)
3. A type-theoretic language for specifying TDA algorithms that is automatically verifiable

This application is long-term and partially speculative, but the direction is clear: as Cubical Agda's cohomology library matures, it will become a platform for verified TDA algorithms.

---

## Application 6: Quantum Computing and Linear Type Theory

Quantum computing requires careful management of quantum states: you cannot copy a quantum bit (the no-cloning theorem), and operations must be unitary. This is precisely the kind of structure that dependent type theory can enforce at the type level.

**Proto-Quipper** and related languages use linear dependent type theory to give type-safe semantics to quantum circuits. The linear structure ensures that quantum bits are used exactly once (not copied, not discarded unless via explicit operations). The dependent structure allows the type to track the number of qubits and the shape of circuits.

Cubical Agda is relevant here for two reasons:
1. The *semantics* of these quantum languages can be given in terms of HoTT: the space of quantum circuits forms a type, and the homotopy of this type corresponds to circuit equivalence.
2. Univalence provides a principled framework for "program equivalence": two quantum programs that implement the same unitary transformation are equal by univalence, and this equality carries computational content (the specific transformations that relate the two implementations).

This application is at the research frontier. Groups at CMU (Bob Harper's group), Edinburgh (Sam Staton), and Oxford (Chris Heunen) are working on the intersection of quantum computing and dependent type theory. Cubical Agda provides the proof assistant infrastructure for exploring this space.

---

## Application 7: Program Extraction and Verified Scientific Computing

The Curry-Howard correspondence promises: from a proof that a function exists with certain properties, you can extract a program that computes it. In systems with canonicity (like Cubical Agda), this extraction is not just theoretically possible but practically efficient: the extracted program is the proof itself, evaluated.

For scientific computing, this means:
- Prove that a numerical algorithm converges (a mathematical theorem)
- Extract the algorithm as a Haskell program (Agda compiles to Haskell)
- Run the extracted program on actual data

The extracted program is guaranteed to have the convergence property, because the proof guarantees it and the extraction preserves the computational content.

Cubical Agda compiles to Haskell via GHC, and the resulting code can be efficient for non-trivial programs. Groups exploring this include the MathComp project (originally for Coq, now exploring Agda) and the Cubical Agda team for HoTT-specific computations.

The dream: a pipeline from mathematical theorem to verified scientific code, where the formalization and the implementation are the same artifact. This is the computational version of the Curry-Howard correspondence, realized at scale. Cubical Agda, with its computational univalence and HITs, is the most advanced current platform for exploring it.

---

## The Big Picture

Cubical Agda's applications are at a different scale and maturity than Lean 4's. Lean 4 and Mathlib are deployed in mathematical research (Fermat's Last Theorem project) and beginning to influence industry (verified compilers, cryptographic protocols). Cubical Agda's applications are more speculative and more research-oriented.

But the speculative applications are significant. A synthetic algebraic topology that computes. A verified framework for quantum computing. A foundation for mathematics where isomorphism implies equality, and the transport along that equality is an algorithm. These are not incremental improvements to existing technology. They are new kinds of mathematics.

The gap between where we are and where these applications promise to take us is exactly the gap that makes this field exciting. Cubical Agda is young. Its library is smaller than Mathlib. Its automation is weaker. But its conceptual foundations are deeper, and its long-term potential — as a tool for turning abstract mathematics into verified computation — is enormous.
