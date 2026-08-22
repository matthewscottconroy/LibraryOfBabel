# Chapter 27 — Topos Theory and Homotopy Type Theory

**Part VI: Foundations of Mathematics**
*Prerequisites: [Chapter 26](ch26-category-theory-foundation.md)*
*This is the final chapter — the top of the ladder.*

---

## Learning Objectives

- Understand $\infty$-categories (quasi-categories) as the correct setting for homotopy theory
- Understand the homotopy hypothesis: $\infty$-groupoids = spaces
- Understand homotopy type theory (HoTT) as a new foundation
- Understand the Univalence Axiom and its consequences
- See how HoTT makes proof assistants (Coq, Agda, Lean) mathematically correct
- Connect the entire curriculum to current research

---

## 27.1 The Inadequacy of Classical Categories for Homotopy

### 27.1.1 The Problem

Classical category theory identifies isomorphic objects. But in homotopy theory, spaces have maps between their homotopies, homotopies between those, etc. — an infinite tower of "higher" structure that cannot be captured by ordinary morphisms.

**Example:** Two chain complexes can be quasi-isomorphic (isomorphic on homology) without being isomorphic as complexes. The derived category $D(\mathcal{A})$ is constructed to force quasi-isomorphisms to become isomorphisms, but this loses homotopical information.

The correct framework is the **derived $\infty$-category** $\mathcal{D}(\mathcal{A})$, a specific $\infty$-category.

### 27.1.2 Homotopical Algebra

The solution is to work with **model categories** (Quillen, 1967) or $\infty$-categories:

- **Weak equivalences** replace isomorphisms
- **Cofibrant/fibrant replacements** replace projective/injective resolutions
- Homotopy limits and colimits replace ordinary limits and colimits

---

## 27.2 $\infty$-Categories

### 27.2.1 Quasi-Categories (Joyal–Lurie)

A **quasi-category** (or $(\infty,1)$-category) is a simplicial set $\mathcal{C}$ satisfying the **inner horn filling condition**:

For every $0 < k < n$ and every map $\Lambda^n_k \to \mathcal{C}$ (inner horn), there exists a filler $\Delta^n \to \mathcal{C}$.

- $\Delta^n$ = standard $n$-simplex
- $\Lambda^n_k$ = $n$-simplex with interior and the $k$-th face removed

**Intuition:** 0-simplices = objects, 1-simplices = morphisms, 2-simplices = "homotopies between compositions," $n$-simplices = higher homotopies. The filling condition says compositions exist but are only unique up to homotopy.

### 27.2.2 The Homotopy Hypothesis

**Grothendieck's homotopy hypothesis:** $\infty$-groupoids (all morphisms invertible at all levels) are "the same as" topological spaces (up to homotopy equivalence).

$$\{\text{topological spaces}\} / \text{homotopy} \simeq \{\infty\text{-groupoids}\}$$

This is a theorem (in various models of $\infty$-groupoids).

**Consequence:** Homotopy theory can be done purely algebraically, inside the world of $\infty$-categories.

### 27.2.3 Lurie's $\infty$-Topos Theory

An **$\infty$-topos** is an $\infty$-category satisfying $\infty$-categorical analogues of Giraud's axioms (the characterization of Grothendieck toposes):

- Colimits distributed over products
- Descent (sheaf condition at the $\infty$-categorical level)

**Examples:**
- $\mathcal{S}$ = $\infty$-category of spaces (the $\infty$-topos version of **Set**)
- $\mathrm{Sh}_\infty(X)$ = $\infty$-sheaves on a topological space $X$
- $\mathrm{Sh}_\infty(\mathrm{Spec}\, R, \mathrm{\acute{e}t})$ = $\infty$-étale sheaves (arithmetic geometry)

### 27.2.4 Stable $\infty$-Categories

A **stable $\infty$-category** is an $\infty$-category where suspension and loop are inverse equivalences. These generalize triangulated categories (fixing their non-functorial "cone" problem).

**Key examples:**
- $\mathcal{D}(R)$ = derived $\infty$-category of an abelian category
- $\mathrm{Sp}$ = stable $\infty$-category of spectra (the setting of stable homotopy theory)
- Perfect complexes on a scheme

---

## 27.3 Homotopy Type Theory (HoTT)

### 27.3.1 The Core Idea

**Martin-Löf Type Theory** is a formal system where:
- **Types** are the basic objects (replace "sets")
- **Terms** inhabit types ($t: A$ = "$t$ is a term of type $A$")
- **Propositions are types** (Curry-Howard correspondence)

**HoTT** (Voevodsky et al., 2013) adds:
- **Identity types** $\mathrm{Id}_A(a, b)$ — the "path space" between $a$ and $b$ in $A$
- $A$ is not just a set but a **homotopy type** (with a notion of path, path-of-path, etc.)

### 27.3.2 The Univalence Axiom

**Univalence:** For any two types $A, B$:
$$\mathrm{IsEquiv}(e) \simeq \mathrm{Id}_{\mathcal{U}}(A, B)$$

In English: **isomorphic types are identical.** An equivalence $A \simeq B$ is the same as a proof that $A = B$ in the universe $\mathcal{U}$.

**Consequence:** Any statement $P(A)$ that holds for a type $A$ automatically holds for any isomorphic type $B$ — without needing transport lemmas. This is "mathematics up to isomorphism" built into the foundations.

**Logical content:** Univalence is the axiom that makes HoTT a correct foundation for "structural" mathematics. It is independent of — and goes beyond — ZFC.

### 27.3.3 $n$-Types and Homotopy Levels

Types in HoTT have a **homotopy level**:
- $(-1)$-types (propositions): $\mathrm{Id}(a,b)$ is contractible for all $a, b$
- $0$-types (sets): $\mathrm{Id}(a,b)$ is a proposition for all $a, b$
- $1$-types (groupoids): $\mathrm{Id}(p, q)$ is a proposition for all paths $p, q$
- $\infty$-types: general spaces with arbitrary higher homotopy

**Sets in HoTT** are exactly the 0-types. Classical mathematics lives inside the 0-types. But higher types encode new mathematics (homotopy groups, higher categories).

---

## 27.4 Cubical Type Theory

### 27.4.1 The Computational Problem

HoTT with Univalence is not **constructive** in original formulations — Univalence is an axiom with no computational content.

**Cubical type theory** (Cohen, Coquand, Huber, Mörtberg, 2016) gives a constructive model for Univalence:
- Introduces an **interval type** $\mathbb{I}$ with endpoints $0, 1$
- Paths $a = b$ are functions $\mathbb{I} \to A$ with endpoints $a, b$
- Univalence is derivable from the structural rules

This gives a foundation that is:
- Constructive (has computational content)
- Models homotopy theory correctly
- Implemented in proof assistants (Cubical Agda)

---

## 27.5 Proof Assistants and Formalized Mathematics

### 27.5.1 The Connection

HoTT and type theory are the theoretical foundation behind modern proof assistants:

| Proof Assistant | Foundation | Key Library |
|----------------|-----------|-------------|
| Coq | Calculus of Constructions | Mathcomp, UniMath |
| Agda | Martin-Löf TT | Cubical Agda, HoTT-Agda |
| Lean 4 | Calculus of Constructions | Mathlib |
| Isabelle/HOL | Higher-order logic | Archive of Formal Proofs |

**Mathlib** (Lean 4) contains formalized proofs of graduate-level algebra, analysis, and number theory. The future of mathematics includes formal verification.

### 27.5.2 Voevodsky's Univalent Foundations Program

Vladimir Voevodsky (Fields Medal 1998) developed HoTT as a new foundation for mathematics, motivated by:
- His work in algebraic geometry (motives, $A^1$-homotopy theory) forced him to work with $\infty$-categories
- He was troubled by errors in published proofs; formalization via type theory provides machine-checked certainty
- Univalence makes the foundations correctly reflect how mathematicians actually work (up to isomorphism)

---

## 27.6 Higher Algebra and Derived Algebraic Geometry

### 27.6.1 $\mathbb{E}_\infty$-Rings

An **$\mathbb{E}_\infty$-ring** is a ring object in the $\infty$-category of spectra — a commutative ring up to coherent homotopies at all levels. This is the correct notion of "commutative ring" in stable homotopy theory.

**Examples:**
- The sphere spectrum $\mathbb{S}$ (the "absolute" base ring)
- $H\mathbb{Z}$ (the Eilenberg-MacLane spectrum — classical algebra)
- $\mathrm{ku}$ (complex K-theory spectrum)
- $\mathrm{TMF}$ (topological modular forms — connecting to number theory)

### 27.6.2 Derived Algebraic Geometry (DAG)

**Derived schemes** are spaces locally modeled on $\mathrm{Spec}\, A$ where $A$ is a simplicial commutative ring (or $\mathbb{E}_\infty$-ring).

**Applications:**
- Intersection theory without transversality assumptions
- Moduli spaces with correct (virtual) dimensions
- Geometric Langlands via $\infty$-categories
- Mirror symmetry

---

## 27.7 The View from the Top

You have climbed the ladder. Here is what the cathedral looks like from here:

**The four great connections:**

1. **Algebra ↔ Geometry:** Commutative algebra = algebraic geometry (Spec functor). Group theory = symmetry of geometry. Lie algebras = infinitesimal symmetry.

2. **Algebra ↔ Topology:** Homological algebra = algebraic topology (chain complexes, spectral sequences). Stable homotopy theory = $\mathbb{E}_\infty$-rings.

3. **Number Theory ↔ Representation Theory:** Langlands program. Galois representations ↔ automorphic forms. L-functions ↔ moduli spaces.

4. **Logic ↔ Geometry:** Toposes = generalized spaces = logical universes. Types = spaces (HoTT). Proofs = paths.

The foundations of mathematics are not below algebra — they are beside it, intertwined with it. Set theory, category theory, and type theory are not merely logical scaffolding; they are mathematical structures in their own right, with representation theory, geometry, and number theory woven through them.

**The cathedral is not finished.** The classification of $\infty$-categories, the Langlands program, the categorical foundations of quantum field theory, the homotopy theory of moduli spaces, the $p$-adic Langlands correspondence — these are open problems at the frontier. The curriculum you have completed gives you the language to read them.

---

## Milestone Exercises

1. Look up the statement of Voevodsky's Milnor conjecture (now theorem). Identify which parts of this curriculum provide the necessary background vocabulary.

2. In cubical type theory: what is the computational content of the proof that $\mathrm{Id}_{\mathbb{N}}(0, 1)$ is empty? (This should be a decision procedure.)

3. Formulate what "the integers" look like in HoTT. What is $\pi_1(S^1)$ in HoTT? (It should be $\mathbb{Z}$.)

4. What does Univalence say about the category of groups? What does it mean for two groups to be "equal" (vs. isomorphic)?

5. Read the introduction of Lurie's "Higher Algebra" (freely available). Identify three ideas from this curriculum that appear and three that are new.

6. The sphere spectrum $\mathbb{S}$ is called the "absolute base" ring. What is $\mathbb{S} \otimes_{\mathbb{S}} H\mathbb{Z}$? What does this say about the relationship between stable homotopy theory and classical algebra?

7. Final capstone: Choose one of the following and write a 3-5 page outline of the mathematical content and why it requires the foundations built in this curriculum: (a) The local Langlands correspondence for $GL_2(\mathbb{Q}_p)$, (b) The geometric Satake equivalence, (c) Derived intersection theory in algebraic geometry, (d) Topological modular forms $\mathrm{TMF}$.

---

## Research Entry Points

| Area | Starting references |
|------|-------------------|
| $\infty$-categories | Lurie, "Higher Topos Theory"; Rezk, "A model for the homotopy theory of homotopy theory" |
| HoTT | "Homotopy Type Theory" book (freely available); Cubical Agda documentation |
| Derived algebraic geometry | Lurie, "Derived Algebraic Geometry"; Toën–Vezzosi, "HAG" |
| Geometric Langlands | Frenkel, "Langlands Correspondence for Loop Groups"; Ben-Zvi–Nadler |
| Quantum groups and categorification | Chari–Pressley, "A Guide to Quantum Groups"; Khovanov homology |

---

*You have reached the top of the ladder. The cathedral continues upward.*

*Return to [Index](index.md)*
