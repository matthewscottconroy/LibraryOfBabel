# 1.1 Open Problems in HoTT

## The Landscape

Research in HoTT divides naturally into several areas, each with its own open problems:

1. **Synthetic homotopy theory**: computing homotopy groups, proving connectivity results
2. **Foundations**: canonicity, the syntax of HITs, new type theories
3. **Formalization**: building the library of machine-verified mathematics
4. **Connections to mathematics**: K-theory, cobordism, chromatic homotopy
5. **Connections to computer science**: program semantics, parametricity, domain theory
6. **Directed type theory**: ∞-category theory synthetically

## Problem 1: A Conceptual Proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$

**Status:** Proved by Brunerie (2016, PhD thesis), with a more computational proof by Ljungström-Mörtberg (2022-2023).

**The problem:** Brunerie's original proof was so large that it couldn't be checked by a human — only by computer. The 2022 simplification made it computationally checkable, but not necessarily conceptually transparent.

**What's wanted:** A proof with a clear conceptual structure, ideally one that a human can follow at each step without losing the thread in an enormous proof term.

**The key difficulty:** The computation involves the Hopf fibration ($S^1 \to S^3 \to S^2$), the EHP sequence, and a number — the Brunerie number $n$ — defined by the proof. Showing $n = 2$ requires a computation. Finding a "human-readable" version of this computation is the challenge.

**Entry points:** Understand Brunerie's thesis. Understand the Ljungström-Mörtberg simplification. Identify the step where the computation of $n$ becomes hard and look for an alternative approach.

## Problem 2: Canonicity for Book HoTT

**Status:** Proved for cubical type theory (Huber 2018). Open for Book HoTT (axiomatic univalence).

**The problem:** In Book HoTT, univalence is an axiom with no computation rule. Does every closed natural number in Book HoTT still reduce to a numeral?

**The difficulty:** Any proof of canonicity must handle terms of the form `transport (ua e) n`, which are stuck in Book HoTT. Either you need to show that such terms never appear at base types (which is false — they can via encoding), or you need a new computational interpretation.

**Current approaches:**
- Shulman's "homotopy canonicity" results (weaker than full canonicity)
- Searching for models of Book HoTT where all types have canonical elements

**Why it matters:** If canonicity fails for Book HoTT, it means that some valid theorems in Book HoTT have no constructive content — purely existential statements. This would be a fundamental limitation of the axiomatic approach.

## Problem 3: A General Syntax for HITs

**Status:** Partial answers exist. No fully general solution.

**The problem:** Each HIT is currently specified individually — the circle has its own definition, the suspension its own, the pushout its own. There's no general *grammar* for "all HITs" from which each specific HIT is an instance.

**What's wanted:** A general syntax (a type of "HIT specifications") and a semantics (a theorem saying any specification that satisfies the syntactic conditions gives a well-defined type theory with the specified computation rules).

**Current approaches:**
- Lumsdaine-Shulman: a semantics for many classes of HITs using left adjoints in ∞-toposes
- Van den Berg-Garner: a 2-cell-based syntax for HITs

**The challenge:** Higher-dimensional HITs (with path-between-path constructors, like $K(G, n)$ for $n > 1$) are much harder than 1-dimensional HITs (like the circle). The general case requires a complete understanding of coherence.

## Problem 4: Directed Univalence

**Status:** Conjectured. No proof in any setting.

**The problem:** In simplicial type theory, is there a "directed" version of univalence? Specifically:

> Is there a Segal type $\mathsf{Cat}$ of ∞-categories such that the natural map $(A =_\mathsf{Cat} B) \to \mathsf{Equiv}(A, B)$ (from paths to equivalences of ∞-categories) is an equivalence?

**What's known:** The Rezk condition for a specific Segal type $\mathcal{C}$ says isomorphisms = paths in $\mathcal{C}$. Directed univalence would be the Rezk condition for the Segal type of all Segal types.

**The difficulty:** We need a good definition of $\mathsf{Cat}$ (the type of ∞-categories in simplicial type theory), and we need the Rezk condition to be provable for it. This requires directed univalence to be consistent with the axioms of simplicial type theory.

## Problem 5: Canonicity for Simplicial Type Theory

**Status:** Open.

**The problem:** In simplicial type theory, does every closed natural number term reduce to a numeral?

**The difficulty:** The simplicial interval $\mathbf{2}$ doesn't obviously have computation rules. The extension types are defined by a universal property, not by an explicit construction. Making these computable requires a fundamentally new approach.

**Connection to Problem 4:** Directed univalence would be a major step toward canonicity, because it would give computation rules for the universe of categories.

## Problem 6: Formalize $\pi_n(S^n) = \mathbb{Z}$ for All $n$

**Status:** Known for $n = 1$ (Chapter 20). Proof sketch for all $n$ uses Freudenthal.

**The problem:** Write a complete formalization in Cubical Agda of:
$$\pi_n(S^n) = \mathbb{Z}$$
for all $n \geq 1$.

**The approach:** Use the Freudenthal suspension theorem (already in the Cubical library) and induction:
- Base case $n = 1$: $\pi_1(S^1) = \mathbb{Z}$ (formalized)
- Inductive step: Freudenthal gives $\pi_{n+1}(S^{n+1}) \cong \pi_n(S^n)$ for $n \geq 1$

**The difficulty:** Making the induction work cleanly in Cubical Agda, verifying all the connectivity conditions, and handling the group structure correctly.

**This is an approachable problem.** Someone who has completed this curriculum has the background to tackle it. It would be a solid contribution to the Cubical Agda library.

## Problem 7: Blakers-Massey Bound is Sharp

**Status:** The bound is proved. Sharpness unknown in HoTT.

**The problem:** The Blakers-Massey theorem says: if $f : A \to B$ is $m$-connected and $g : A \to C$ is $n$-connected, then the comparison map $A \to B \times_{B \sqcup_A C} C$ is $(m+n-1)$-connected.

*Sharpness:* For each $m, n$, exhibit an example where the connectivity is *exactly* $m+n-1$ (not higher).

**Classical answer:** The attaching maps $S^{m+n-1} \to S^m \vee S^n$ provide examples — these are the maps appearing in the James splitting. Formalizing these examples in HoTT is a non-trivial but tractable problem.

## Problem 8: Stable Homotopy Theory in HoTT

**Status:** Spectra are defined in HoTT. Systematic development minimal.

**The problem:** Develop stable homotopy theory synthetically in HoTT, including:
- The sphere spectrum $\mathbb{S}$
- Stable homotopy groups $\pi_k^s = \pi_{n+k}(S^n)$ for $n$ large
- The $p$-localization modality
- Chromatic filtration

**The approach:** The $p$-localization modality (from cohesive/modal HoTT) gives the start. The chromatic filtration requires the theory of Morava K-theories, which requires substantial algebraic machinery.

**Why it matters:** The stable homotopy groups of spheres are the "atoms" of homotopy theory. A synthetic development would give new geometric insight into these purely arithmetic invariants.

## Research Directions Beyond the Core Problems

The eight problems above are the named landmarks. Around them lie broader research programs where the landscape is less charted.

### New Type Theory Designs

Beyond CCHM and simplicial TT, several new type theories are under development:

**Displayed type theory (dTT):** (Aagaard, North, Veltri) extends DTT with "displayed" types, designed to make fibered reasoning more natural. Applications to synthetic category theory.

**Multimodal type theory (MTT):** (Gratzer, Kavvos, Nuyts, Birkedal) gives a general framework for modal type theories, encompassing both cohesive HoTT and directed type theories. The open problem: give a full model for MTT that validates all the desired axioms.

**Parametric type theory:** (Nuyts, Vezzosi, Ahman) extends type theory with *parametricity* axioms, enabling internal reasoning about relational parametricity. Applications to program correctness and abstract data types.

**Internal languages of new ∞-toposes:** Different ∞-toposes (equivariant, motivic, $p$-adic) require specialized type theories. The general question: what is the internal language of an ∞-topos with specific properties (e.g., the ∞-topos of $G$-spaces for a group $G$)?

### Proof Theory of HoTT

The proof theory of HoTT (decidability, complexity, proof size) is largely unexplored:

**Decidability of type checking:** Is type checking in cubical type theory decidable? The answer is yes (normalization gives a decision procedure), but the complexity is unknown.

**Proof complexity:** How long must a proof of $\pi_n(S^n) = \mathbb{Z}$ be in HoTT? Are there results that have short classical proofs but only long HoTT proofs?

**Proof mining:** Classical proofs of theorems often contain constructive content that can be extracted (Kreisel's proof mining). For HoTT proofs: is there content in HoTT proofs beyond what is visible classically?

### Connections to Computer Science

**Quotient types and data abstraction:** HITs give a precise meaning to "abstract data types" — types where certain equalities are forced. A queue and a deque may have the same abstract behavior (same quotient) even with different implementations.

**Parametricity via HoTT:** The Reynolds parametricity theorem (every polymorphic function is a natural transformation) has a HoTT interpretation: the identity type in System F is the HoTT path type. This gives a new proof of the theorems of free theorems.

**Observational type theory:** (McBride, Altenkirch, Swierstra) is a type theory where equality is defined *by observation* — two terms are equal if they are observationally indistinguishable. This is closely related to HoTT.

**Denotational semantics via HoTT:** The denotational semantics of a programming language is a functor from syntax (a category) to semantics (types). HoTT gives a natural setting for this: the functor is a map of Segal types, and semantic equivalence is path equality in the target.

**Verified compilation:** Compilers that are proved correct via HoTT proofs. The CompCert project (in Coq) shows this is possible; the next step is using HoTT-specific techniques (univalence, parametricity) to simplify and automate correctness proofs.

**Synthetic domain theory:** Domain theory is the mathematics of computation: domains model recursive types and fixed points. In synthetic domain theory (Hyland, Phoa, Taylor), domains are modeled as types in a category satisfying certain axioms. HoTT offers a new setting, starting from the *lifting monad* $L$: the lifting of a type $A$ is the type $LA :\equiv \|A + \mathbf{1}\|_?$ — elements of $A$ together with a "bottom" element, with suitable continuity conditions. This is a monad for partiality. The open problem: give a full synthetic domain theory in HoTT — define the category of domains, prove the existence of fixed points, and derive the semantics of a programming language with general recursion.

### Connections to Mathematics

**Algebraic K-theory:** K-theory assigns to a ring $R$ a sequence of groups $K_n(R)$ that measure algebraic invariants. In HoTT, K-theory is naturally a sequence of homotopy groups of the K-theory space:

$$K_n(R) :\equiv \pi_n(|BGL(R)^+|)$$

where $BGL(R)^+$ is the Quillen plus-construction on the classifying space of $GL(R)$. In HoTT, $BGL(R)$ can be defined as a HIT, and the plus-construction adds path constructors killing the perfect normal subgroup of $\pi_1$. Open problem: formalize algebraic K-theory in Cubical Agda or Lean 4, starting with $K_0$ (projective modules) and $K_1$ (units of $R$).

**Topological field theories and cobordism:** A TFT assigns algebraic data to manifolds: a vector space to each compact $(n-1)$-manifold and a linear map to each $n$-cobordism. In HoTT, TFTs are functors from the cobordism ∞-category to a symmetric monoidal ∞-category. The fully extended cobordism hypothesis (Lurie 2009) classifies extended TFTs; a synthetic proof in simplicial type theory would be a major achievement:

> *Prove the cobordism hypothesis in simplicial type theory: fully extended $n$-dimensional TFTs valued in a symmetric monoidal ∞-category $\mathcal{C}$ are classified by the $n$-fold dualizable objects in $\mathcal{C}$.*

## Triage: What to Work On

Some rough guidance on accessibility:

**Accessible to a motivated graduate student:**
- Problem 6: Formalize $\pi_n(S^n) = \mathbb{Z}$ for all $n$ (good first research project)
- Formalize Mayer-Vietoris in Cubical Agda
- Extend the Cubical library with new HITs or computations

**Requires significant research background:**
- Problem 2: Canonicity for Book HoTT
- Problem 3: General syntax for HITs
- Problem 5: Canonicity for STT

**Open at the research frontier:**
- Problem 1: A cleaner proof of $\pi_4(S^3)$ (active area, papers appearing regularly)
- Problem 4: Directed univalence (major open problem)
- Problem 8: Chromatic homotopy in HoTT (long-term)

The best strategy: start with Problem 6 or a Cubical Agda library contribution. This gives you hands-on experience with the tools, forces you to understand the mathematics deeply, and produces a concrete output. From there, the larger problems become more accessible.
