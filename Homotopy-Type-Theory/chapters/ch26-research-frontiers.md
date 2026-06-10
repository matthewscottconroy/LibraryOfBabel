# Chapter 26: Research Frontiers — Open Problems and the Path Forward

## Introduction

This final chapter maps the frontier of active research in homotopy type theory and related areas. It is organized not as a survey of settled mathematics, but as a guide to *open problems* — questions that remain unsolved and where new contributions are genuinely possible. The chapter also describes the practical steps for beginning independent research: how to read papers, where to find problems, how to engage with the community, and what the realistic timeline looks like for making a first research contribution.

By this point in the curriculum, the reader has mastered:
- The foundations (logic, set theory, algebra, analysis, proof theory)
- The type-theoretic core (MLTT, dependent types, Curry-Howard)
- The categorical perspective (category theory, categorical logic, higher categories)
- The topology (algebraic topology, simplicial sets, homotopy theory)
- HoTT proper (identity types, h-levels, univalence, HITs)
- Synthetic homotopy theory (encode-decode, $\pi_1(S^1) = \mathbb{Z}$, Freudenthal, Hopf)
- Proof assistants (Lean 4, Cubical Agda)
- Advanced foundations (cubical type theory, simplicial type theory, modal HoTT)

What comes next is research.

---

## 1. The Open Problems Landscape

### 1.1 Canonical Open Problems

The following problems are recognized by the community as major open questions. Progress on any of these would be a significant result.

**Problem 1: A simpler proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ (Brunerie's problem)**

Brunerie's 2016 thesis proved $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ entirely within HoTT, but the proof term was too large for human verification — it required a computer to check. The open problem:

> *Find a proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ in HoTT that can be checked by a human (or at least that has a clear conceptual structure).*

Recent work by Ljungström and Mörtberg (2023) has made progress using Cubical Agda, giving a more computational proof. But a completely transparent conceptual proof remains open.

**Problem 2: Canonicity for Book HoTT**

In standard (axiomatic) HoTT, univalence is an axiom with no computation rule. This means:

> *Is it true that every closed term of type $\mathbb{N}$ in Book HoTT (with univalence as an axiom) is definitionally equal to a numeral?*

This is the *canonicity conjecture* for Book HoTT. It is known to hold for cubical type theory (Chapter 23) but unknown for the axiomatic system. The difficulty is that `ua` introduces stuck terms that cannot be reduced.

**Problem 3: Coherence for Higher Inductive Types**

The current theory of HITs (Chapter 19) gives their *specification* (what constructors they have) but not a general *syntax* for all possible HITs. Specifically:

> *Give a uniform syntax and semantics for all higher inductive types, with a general theorem ensuring that any HIT satisfying the syntactic conditions has a well-defined type theory.*

Various proposals exist (Lumsdaine-Shulman, van den Berg-Garner), but a fully general and computationally satisfactory answer remains open.

**Problem 4: Univalence in HoTT without Axioms**

Both axiomatic HoTT (with `ua` as an axiom) and cubical type theory (with `ua` as a theorem) coexist. A deeper question:

> *Is there a type theory (other than cubical TT) in which univalence is a theorem rather than an axiom, with a different computational interpretation?*

This is connected to the question of what the "right" computational foundation for HoTT is.

### 1.2 Synthetic Homotopy Theory Frontiers

**Problem 5: Computation of $\pi_n(S^n)$ for all $n$**

We know $\pi_n(S^n) = \mathbb{Z}$ for all $n \geq 1$. This is a theorem in classical algebraic topology, and the $n=1$ case is proved in HoTT (Chapter 19). For $n \geq 2$, the proof requires Freudenthal (Chapter 20):

> *Formalize the proof of $\pi_n(S^n) = \mathbb{Z}$ for all $n$ in Cubical Agda or Lean 4, using the Freudenthal suspension theorem.*

This is within reach: Freudenthal is in the Cubical Agda library, and the inductive argument is known. The challenge is filling in the formalization details.

**Problem 6: Higher Homotopy Groups of Spheres Computationally**

The homotopy groups of spheres $\pi_k(S^n)$ for $k > n$ are the central mystery of algebraic topology. The first few are known:
- $\pi_3(S^2) = \mathbb{Z}$ (Hopf, proved in HoTT)
- $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ (Brunerie, proved in HoTT but hard)
- $\pi_5(S^4) = \mathbb{Z}/2\mathbb{Z}$ (classical, not yet in HoTT)
- $\pi_6(S^3) = \mathbb{Z}/12\mathbb{Z}$ (classical, far from HoTT)

> *Prove $\pi_5(S^4) = \mathbb{Z}/2\mathbb{Z}$ in HoTT using the EHP sequence or another method.*

**Problem 7: The Blakers-Massey Bound is Sharp**

The Blakers-Massey theorem gives a connectivity bound on the map $C \to A \times_{A \sqcup_C B} B$. The current synthetic proofs (Chapter 20) establish the bound. The sharpness:

> *Show in HoTT that the bound in Blakers-Massey is sharp — i.e., that there exist $m$-connected and $n$-connected maps for which the connectivity of the comparison map is exactly $m+n$.*

### 1.3 Foundations and Proof Theory

**Problem 8: Directed Univalence**

In simplicial type theory (Chapter 24), the Rezk condition (completeness) says isomorphisms are paths. A directed analogue of univalence would say:

> *Is there a directed univalence axiom for simplicial type theory, asserting that the type of functors between Segal types is equivalent to the type of "biequivalences"?*

This is analogous to the classical fact that every equivalence of categories is a biequivalence, but the precise statement and its computational content in STT are unknown.

**Problem 9: Canonicity for Simplicial Type Theory**

As noted in Chapter 24, simplicial type theory currently lacks a proof of canonicity:

> *Does every closed term of type $\mathbb{N}$ in simplicial type theory reduce to a numeral?*

This would require a computational interpretation of the extension types and the two-interval structure.

**Problem 10: The Universe of Segal Types**

In ordinary HoTT, the universe $\mathsf{Type}$ itself is a type (and a Rezk type). In simplicial type theory:

> *Define a universe of "all Segal types" as a Segal type in simplicial TT, and show it satisfies the Rezk condition.*

This would give a synthetic model of the ∞-topos of ∞-categories.

---

## 2. Formalization Frontiers

### 2.1 Mathlib and the Lean 4 Ecosystem

Mathlib4 contains hundreds of thousands of formalized theorems, but it remains incomplete. Specific HoTT-relevant gaps:

**HoTT in Lean 4 (Mathlib or standalone):**
- The full Seifert-van Kampen theorem (currently absent)
- Freudenthal suspension theorem (absent)
- Blakers-Massey theorem (absent)
- The Hopf fibration as a formalized fibration (absent)

**Contributing to Mathlib:**
The Mathlib contribution process (Chapter 21) is the standard path. For HoTT-specific results, a standalone library (analogous to the Cubical Agda library) would be valuable.

### 2.2 The Cubical Agda Library

The Cubical Agda library (github.com/agda/cubical) is more directly HoTT-compatible. Current gaps:

- **$\pi_4(S^3)$**: Brunerie's result is in the library, but the proof is computational, not conceptual.
- **Freudenthal**: Present in the library, but the generalized version for non-spheres is not fully developed.
- **Eilenberg-MacLane spaces**: Partially present; systematic treatment incomplete.
- **Spectra**: The sphere spectrum is defined, but stable homotopy theory is largely absent.
- **K-theory**: Algebraic K-theory in HoTT is virtually absent.

### 2.3 The Rzk Library

The Rzk proof assistant (Chapter 24) is young. The main library (sHoTT) is small:

- The Yoneda lemma: formalized.
- Adjunctions: partially formalized.
- Limits and colimits: mostly absent.
- Grothendieck construction: absent.
- Stable ∞-categories: absent.

Contributions here are especially impactful because the library is early in development.

---

## 3. Type-Theoretic Foundations Research

### 3.1 New Type Theory Designs

Beyond CCHM and simplicial TT, several new type theories are under development:

**Displayed type theory (dTT):** (Aagaard, North, Veltri) extends DTT with "displayed" types, designed to make fibered reasoning more natural. Applications to synthetic category theory.

**Multimodal type theory (MTT):** (Gratzer, Kavvos, Nuyts, Birkedal) gives a general framework for modal type theories, encompassing both cohesive HoTT and directed type theories. The open problem: give a full model for MTT that validates all the desired axioms.

**Parametric type theory:** (Nuyts, Vezzosi, Ahman) extends type theory with *parametricity* axioms, enabling internal reasoning about relational parametricity. Applications to program correctness and abstract data types.

**Internal languages of new ∞-toposes:** Different ∞-toposes (equivariant, motivic, $p$-adic) require specialized type theories. The general question: what is the internal language of an ∞-topos with specific properties (e.g., the ∞-topos of $G$-spaces for a group $G$)?

### 3.2 Proof Theory of HoTT

The proof theory of HoTT (decidability, complexity, proof size) is largely unexplored:

**Decidability of type checking:** Is type checking in cubical type theory decidable? The answer is yes (normalization gives a decision procedure), but the complexity is unknown.

**Proof complexity:** How long must a proof of $\pi_n(S^n) = \mathbb{Z}$ be in HoTT? Are there results that have short classical proofs but only long HoTT proofs?

**Proof mining:** Classical proofs of theorems often contain constructive content that can be extracted (Kreisel's proof mining). For HoTT proofs: is there content in HoTT proofs beyond what is visible classically?

---

## 4. Connections to Computer Science

### 4.1 Homotopy Type Theory for Programming Languages

HoTT has direct applications to programming language theory:

**Quotient types and data abstraction:** HITs give a precise meaning to "abstract data types" — types where certain equalities are forced. A queue and a deque may have the same abstract behavior (same quotient) even with different implementations.

**Parametricity via HoTT:** The Reynolds parametricity theorem (every polymorphic function is a natural transformation) has a HoTT interpretation: the identity type in System F is the HoTT path type. This gives a new proof of the theorems of free theorems.

**Observational type theory:** (McBride, Altenkirch, Swierstra) is a type theory where equality is defined *by observation* — two terms are equal if they are observationally indistinguishable. This is closely related to HoTT.

### 4.2 Verified Compilation and Program Semantics

**Denotational semantics via HoTT:** The denotational semantics of a programming language is a functor from syntax (a category) to semantics (types). HoTT gives a natural setting for this: the functor is a map of Segal types, and semantic equivalence is path equality in the target.

**Domain theory in HoTT:** Classical domain theory uses partially ordered sets as semantic domains. In HoTT, domains should be types with a suitable ∞-category structure. The development of *HoTT domain theory* is an active research area.

**Verified compilation:** Compilers that are proved correct via HoTT proofs. The CompCert project (in Coq) shows this is possible; the next step is using HoTT-specific techniques (univalence, parametricity) to simplify and automate correctness proofs.

### 4.3 Synthetic Domain Theory

**Domain theory** is the mathematics of computation: domains model recursive types and fixed points, and the denotational semantics of programming languages is given by functors on domains.

In synthetic domain theory (Hyland, Phoa, Taylor), domains are modeled as types in a category satisfying certain axioms. HoTT offers a new setting:

**Lifting monad $L$:** In HoTT, the *lifting* of a type $A$ is the type $LA :\equiv \|A + \mathbf{1}\|_?$ — elements of $A$ together with a "bottom" element, with suitable continuity conditions. This is a monad for partiality.

**Problem:** Give a full synthetic domain theory in HoTT: define the category of domains, prove the existence of fixed points, and derive the semantics of a programming language with general recursion.

---

## 5. Connections to Mathematics

### 5.1 Algebraic K-Theory

Algebraic K-theory assigns to a ring $R$ a sequence of groups $K_n(R)$ that measure algebraic invariants (projective modules, stable isomorphisms of free modules, etc.).

In HoTT, K-theory is naturally a sequence of homotopy groups of a certain space (the K-theory space). The HoTT formulation:

$$K_n(R) :\equiv \pi_n(|BGL(R)^+|)$$

where $BGL(R)^+$ is the Quillen plus-construction on the classifying space of $GL(R)$.

**In HoTT:** $BGL(R)$ can be defined as a HIT (the classifying type of $R$-module chains). The plus-construction adds path constructors killing the perfect normal subgroup of $\pi_1$.

**Open problem:** Formalize algebraic K-theory in Cubical Agda or Lean 4, starting with the definition of $K_0$ (projective modules) and $K_1$ (units of $R$).

### 5.2 Topological Field Theories and Cobordism

A *topological field theory* (TFT) assigns algebraic data to manifolds: a vector space to each compact $(n-1)$-manifold and a linear map to each $n$-cobordism.

In HoTT:
- Manifolds are types with specific cohesive structure
- TFTs are functors from the cobordism ∞-category to a symmetric monoidal ∞-category
- The classification of TFTs (the Baez-Dolan cobordism hypothesis) is a deep theorem about ∞-categories

**The cobordism hypothesis in HoTT:** The fully extended cobordism hypothesis (Lurie 2009) classifies extended TFTs. A synthetic proof in simplicial type theory would be a major achievement:

> *Prove the cobordism hypothesis in simplicial type theory: fully extended $n$-dimensional TFTs valued in a symmetric monoidal ∞-category $\mathcal{C}$ are classified by the $n$-fold dualizable objects in $\mathcal{C}$.*

### 5.3 Chromatic Homotopy Theory

Chromatic homotopy theory organizes the homotopy groups of spheres by "chromatic height" — using the theory of formal group laws and modular forms. The chromatic filtration:
- Height 0: rational homotopy theory
- Height 1: complex K-theory, Adams operations
- Height 2: elliptic cohomology, topological modular forms
- Height $n$: Morava K-theory, lubin-Tate spectra

**In HoTT:** Chromatic homotopy theory requires spectra (Chapter 19 appendix), which are defined in HoTT but not well-developed. The p-completion and localization modalities of modal HoTT (Chapter 25) are the starting point.

**Open problem:** Define the sphere spectrum and rationalization/chromatic localization in HoTT, and prove the chromatic convergence theorem (that the $p$-local sphere is the hocolimit of its chromatic localizations).

---

## 6. How to Begin Contributing

### 6.1 Reading the Literature

**Essential papers (with recommended order):**

1. **Univalent Foundations Program, "Homotopy Type Theory"** (2013) — the HoTT Book. This is the primary text. Chapters 1-6 are essential; chapters 7-10 are the research content.

2. **Awodey-Warren, "Homotopy-Theoretic Models of Identity Types"** (2009) — the groupoid model and why UIP fails. Foundational for understanding what HoTT is doing.

3. **Cohen-Coquand-Huber-Mörtberg, "Cubical Type Theory"** (2015) — the CCHM cubical type theory. Read this after understanding Book HoTT.

4. **Riehl-Shulman, "A Synthetic Theory of ∞-Categories"** (2017) — simplicial type theory. Best read after Chapter 24.

5. **Brunerie, "On the Homotopy Groups of Spheres in HoTT"** (2016, PhD thesis) — the $\pi_4(S^3)$ computation. Dense but essential for synthetic homotopy theory research.

6. **Anel-Biedermann-Finster-Joyal, "A Generalized Blakers-Massey Theorem"** (2017) — the synthetic Blakers-Massey. Important for understanding modern techniques.

7. **Shulman, "Brouwer's Fixed-Point Theorem in Real-Cohesive HoTT"** (2018) — cohesive HoTT in action.

**For formalization:**
- Cubical Agda library: github.com/agda/cubical (read the source)
- Mathlib documentation: leanprover-community.github.io/mathlib4_docs
- Rzk documentation: rzk-lang.github.io

### 6.2 Finding Your Problem

The best research problems are those where:
1. The problem is *clearly stated* and has a known answer in classical mathematics
2. The HoTT proof would require *new ideas*, not just translation
3. The problem is *connected to the broader program* (univalence, HITs, synthetic homotopy)

**How to find your problem:**
- Read the HoTT Book and identify which exercises are open problems (marked as such or with incomplete proofs)
- Check the Cubical Agda library issues on GitHub for "wanted theorems"
- Read recent papers (2020-2026) and look at "future work" sections
- Ask on the HoTT Zulip chat (hott.zulipchat.com) — the community is welcoming and responsive

**Concrete starter problems (in increasing difficulty):**
1. Formalize $\pi_n(S^n) = \mathbb{Z}$ for $n \geq 2$ using Freudenthal in Cubical Agda.
2. Formalize the Mayer-Vietoris sequence for pushouts in Cubical Agda.
3. Prove the Seifert-van Kampen theorem in Lean 4 (Mathlib contribution).
4. Formalize one more case of the long exact sequence of homotopy groups.
5. Compute $\pi_5(S^4)$ synthetically.

### 6.3 Engaging with the Community

**Conferences:**
- **HoTTEST** (online): homotopytype.theory/seminars — free online seminar series
- **TYPES** (annual European conference): the main European venue for dependent types
- **LICS** (Logic in Computer Science): broader CS logic venue
- **ITP** (Interactive Theorem Proving): for formalization results
- **IJCAR/CADE**: for automated reasoning

**Online communities:**
- **HoTT Zulip**: hott.zulipchat.com — the most active HoTT community
- **HoTT Google Group**: announcements and discussions
- **MathOverflow/CSTheory StackExchange**: for specific technical questions
- **Lean4 Zulip**: leanprover.zulipchat.com — for Lean/Mathlib questions

**Workshop proceedings:** Many cutting-edge results first appear as workshop papers at LICS, POPL, or HoTT workshops before journal publication.

### 6.4 Writing and Publishing

**Proof assistant results:** If you formalize a new result in Agda or Lean, the primary output is the formalized proof (a GitHub repository). Write a paper explaining the formalization — what was hard, what techniques you used, what the mathematical content is.

**Venues for formalization papers:**
- ITP (Interactive Theorem Proving)
- Conferences associated with the proof assistant (e.g., Agda workshops)
- LICS and POPL (for foundational results)
- Archive of Formal Proofs (for Isabelle/HOL, analogous projects exist for Lean)

**Venues for type theory papers:**
- LICS, POPL, FSCD (Formal Structures in Computation and Deduction)
- TYPES proceedings (informal, good for early results)
- MSCS (Mathematical Structures in Computer Science) for mathematical content

**Arxiv:** Post all preprints to arxiv.org in cs.LO (logic in computer science) or math.LO or math.AT (algebraic topology). The HoTT community uses arxiv heavily.

---

## 7. The Longer View

### 7.1 Why HoTT Matters

The significance of HoTT extends beyond the specific results:

**For mathematics:** HoTT provides a foundation where mathematical practice (identifying isomorphic structures, working invariantly) is validated formally. As proof assistants become more powerful, HoTT gives the "right" foundation for mechanized mathematics.

**For computer science:** HoTT connects programming language theory (dependent types, parametricity) with topology (homotopy groups, fibrations) and gives a unified framework for reasoning about programs and their correctness.

**For physics:** Cohesive HoTT (Chapter 25) provides a synthetic foundation for the geometric objects that appear in gauge theory and string theory. This connection between physics and type theory is deep and mostly unexplored.

**For foundations:** HoTT is a serious alternative to ZFC set theory as a foundation for mathematics — one that is both expressive (all of mathematics can be formalized) and intrinsically meaningful (types have geometric content).

### 7.2 The Next Decade

The trajectory of the field:

**Near term (2025-2028):**
- Computational improvements to Cubical Agda and related tools
- A clearer proof of $\pi_4(S^3)$ (Brunerie's problem)
- The first major Lean 4 HoTT library (complementing Mathlib)
- Simplicial type theory with canonicity

**Medium term (2028-2033):**
- Synthetic algebraic K-theory in Cubical Agda
- A verified compiler in HoTT
- The cobordism hypothesis formalized in simplicial type theory
- HoTT textbooks and courses at graduate level

**Long term:**
- HoTT as the standard foundation for computer-verified mathematics
- Automated theorem proving at the level of graduate coursework
- The HoTT perspective reshaping how algebraic topology and category theory are taught and practiced

### 7.3 Your Place in the Story

This curriculum is a starting point, not an endpoint. The field is young — the HoTT Book was published in 2013, and many of the papers in this curriculum are from 2015-2024. The open problems above are not obscure curiosities; they are the central questions driving an active research community.

The tools are now available: Cubical Agda with a mature library, Lean 4 with Mathlib, Rzk for directed type theory. The mathematical background in this curriculum gives you what you need to read current papers and identify where you can contribute.

The next result could be yours.

---

## Exercises

**26.1.** Look up the current state of Brunerie's problem. Has there been progress since 2023? What is the current state of the proof of $\pi_4(S^3)$ in Cubical Agda?

**26.2.** Browse the open issues on the Cubical Agda library (github.com/agda/cubical/issues). Identify two issues that are labeled "wanted theorem" or "enhancement" and that you would be able to work on given the contents of this curriculum.

**26.3.** Read the abstract and introduction of Riehl-Shulman (2017). Identify one result in their paper that is not yet formalized in Rzk (check the Rzk library). State the result precisely.

**26.4.** The Seifert-van Kampen theorem is in Chapter 20 (stated) but not in Lean 4 (Mathlib). Outline the steps needed to formalize it: what definitions are needed, what lemmas, and what is the main proof strategy?

**26.5.** Register for the HoTT Zulip and find a conversation about an open problem that interests you. Summarize the current state of that problem in one paragraph.

**26.6.** Choose one paper from the "Essential papers" list (Section 6.1) that you have not yet read. Read it and write a 2-page summary: what is the main result, what are the key ideas, and what open questions does the paper raise?

**26.7 (Project).** Choose one of the "Concrete starter problems" in Section 6.2. Over the course of a month, attempt to formalize it in Cubical Agda or Lean 4. Document:
  - What worked
  - What was harder than expected
  - What new ideas or lemmas you needed
  - Whether you succeeded, and if not, what the obstacle was

Submit the formalized proof (if successful) or a detailed account of the obstacles (if not) as a written report. This is research.
