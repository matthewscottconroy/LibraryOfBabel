# Chapter 12: Higher Category Theory and the Homotopy Hypothesis

## Introduction

Ordinary categories have objects and morphisms. But morphisms themselves can have morphisms between them — *2-morphisms* — and those can have 3-morphisms, and so on. This is the world of higher category theory.

For HoTT, higher category theory is not a distant abstraction — it is the very thing that the identity types are computing. The key theorem (the *homotopy hypothesis*) says that ∞-groupoids are the same as homotopy types. Since types in HoTT are ∞-groupoids (by their iterated identity type structure), this connects HoTT directly to homotopy theory.

This chapter introduces higher categories, explains the homotopy hypothesis, and previews the theory of (∞,1)-categories — which is the subject of Chapter 24 (Simplicial Type Theory), where these ideas become the object of formal investigation.

---

## 1. 2-Categories

### 1.1 Strict 2-Categories

**Definition 12.1 (Strict 2-Category).** A *strict 2-category* $\mathcal{C}$ consists of:
- A collection of *objects* (or *0-cells*): $A, B, C, \ldots$
- For each pair of objects, a *category* $\mathcal{C}(A, B)$ of *1-cells* and *2-cells*:
  - *1-cells*: $f : A \to B$ (morphisms)
  - *2-cells*: $\alpha : f \Rightarrow g$ (morphisms between morphisms, for $f, g : A \to B$)
- *Composition of 1-cells*: $g \circ f : A \to C$ for $f : A \to B$, $g : B \to C$
- *Vertical composition of 2-cells*: $\beta \circ \alpha : f \Rightarrow h$ for $\alpha : f \Rightarrow g$, $\beta : g \Rightarrow h$
- *Horizontal composition of 2-cells*: $\beta \star \alpha : g \circ f \Rightarrow g' \circ f'$ for $\alpha : f \Rightarrow f'$, $\beta : g \Rightarrow g'$

All associativity and unit laws hold *strictly* (as equalities).

**Example 12.2 (Cat).** The strict 2-category of (small) categories, functors, and natural transformations.

**Example 12.3 (Grpd).** The 2-category of groupoids, functors, and natural transformations.

### 1.2 Bicategories (Weak 2-Categories)

In practice, the higher composition laws often only hold *up to coherent isomorphism*, not strictly. This leads to *bicategories*.

**Definition 12.4 (Bicategory).** A *bicategory* $\mathcal{B}$ has:
- Objects, 1-cells, 2-cells as above
- Composition of 1-cells $g \otimes f$ (using $\otimes$ to distinguish from strict $\circ$)
- *Associator*: $\alpha_{h,g,f} : (h \otimes g) \otimes f \cong h \otimes (g \otimes f)$ (a natural 2-isomorphism)
- *Left/right unitors*: $\lambda_f : 1_B \otimes f \cong f$, $\rho_f : f \otimes 1_A \cong f$
- *Coherence conditions*: the Mac Lane pentagon and triangle identities (ensuring different ways to reassociate 1-cells give the same result)

The difference from a strict 2-category: associativity holds *up to a chosen isomorphism*, not on the nose.

**Example 12.5 (Spans).** Objects are sets; 1-cells $A \to B$ are *spans* (diagrams $A \leftarrow C \rightarrow B$); 2-cells are morphisms of spans. Composition is by pullback. This is only associative up to isomorphism (pullbacks compose up to isomorphism).

---

## 2. Groupoids

### 2.1 Definition

**Definition 12.6 (Groupoid).** A *groupoid* is a category in which every morphism is an isomorphism. Equivalently, it is a small category $\mathcal{G}$ where every morphism has a two-sided inverse.

**Key fact:** In a groupoid, the "set" of morphisms from $x$ to $y$ is either empty or a torsor for the automorphism group $\mathsf{Aut}(x) = \mathsf{Hom}(x, x)$.

**Examples:**
- Every group $G$ is a one-object groupoid $\mathbf{B}G$: one object $*$ and $\mathsf{Hom}(*,*) = G$.
- The *discrete groupoid* on a set $S$: only identity morphisms.
- The *pair groupoid* $S \times S$ with one morphism between any two elements: completely connected.
- The *fundamental groupoid* $\Pi_1(X)$ of a topological space $X$ (see below).

### 2.2 The Fundamental Groupoid

**Definition 12.7 (Fundamental Groupoid).** For a topological space $X$, the *fundamental groupoid* $\Pi_1(X)$ is the groupoid where:
- Objects are points $x \in X$
- Morphisms $x \to y$ are homotopy classes of paths from $x$ to $y$
- Composition is path concatenation (well-defined on homotopy classes)
- Identity at $x$ is the constant path
- Inverses are reversed paths

**Theorem 12.8.** $\Pi_1(X)$ is indeed a groupoid: every path-class has an inverse (the reversed path).

The *fundamental group* $\pi_1(X, x_0)$ of a space at a basepoint $x_0$ is the automorphism group $\mathsf{Aut}_{\Pi_1(X)}(x_0)$ in the fundamental groupoid.

**Why groupoids are better than groups:** The fundamental group requires a choice of basepoint, and the group depends on this choice (different basepoints give isomorphic but not equal groups). The fundamental groupoid is basepoint-free and captures the same information — and more (it sees the connectivity between different components).

### 2.3 Groupoids in MLTT

In MLTT, every type $A$ has a natural groupoid structure:
- Objects: terms $a : A$
- Morphisms $a \to b$: terms $p : a =_A b$
- Composition: path concatenation (defined via J)
- Identities: reflexivity $\mathsf{refl}_a$
- Inverses: path inversion (defined via J)

**Theorem 12.9 (Types are Groupoids).** Every type in MLTT is a groupoid (the groupoid of its identity proofs).

But types are not just groupoids — the identity proofs themselves form a type, with its own identity proofs, and so on. This gives a tower of structure: types are *∞-groupoids*.

---

## 3. The Homotopy Hypothesis

### 3.1 Grothendieck's Conjecture

In a famous 1983 letter to Quillen, Grothendieck conjectured:

> "The study of $n$-truncated homotopy types ... is essentially equivalent to the study of $n$-groupoids."

More precisely: *homotopy types are the same as ∞-groupoids*. A *homotopy type* is a topological space, considered up to weak homotopy equivalence (a map inducing isomorphisms on all homotopy groups). An *∞-groupoid* is an ∞-category in which every $k$-morphism (for all $k$) is invertible.

The homotopy hypothesis says these two notions are equivalent.

### 3.2 Evidence and Formulations

**The simplicial set formulation (standard):** The homotopy hypothesis is realized by:
- *Geometric realization:* $|{-}| :$ (simplicial sets) $\to$ (topological spaces)
- *Singular complex:* $\mathsf{Sing}(-)$ : (topological spaces) $\to$ (simplicial sets)

These are adjoint and induce an equivalence between:
- *Kan complexes* (simplicial sets satisfying the horn-filling condition)
- *Homotopy types* (topological spaces, up to weak equivalence)

Kan complexes are the simplicial set model of ∞-groupoids.

**The MLTT/HoTT formulation:** Types in MLTT/HoTT *are* ∞-groupoids. The groupoid structure comes from:
- Objects: terms $a : A$
- 1-morphisms: paths $p : a = b$
- 2-morphisms: paths between paths $H : p = q$ (where $p, q : a = b$)
- $n$-morphisms: iterated identity types

The univalence axiom adds: equivalent types are equal, which turns this into a fully homotopy-theoretic system.

### 3.3 Consequences for HoTT

The homotopy hypothesis means that every theorem of homotopy theory has a *type-theoretic formulation*, and every construction in type theory has a *topological interpretation*.

**Examples:**
- $\pi_1(S^1) = \mathbb{Z}$ (the fundamental group of the circle is the integers) has a precise type-theoretic formulation and proof in HoTT.
- The suspension of a 0-connected type has trivial $\pi_0$ — provable in HoTT.
- The Hopf fibration $S^1 \to S^3 \to S^2$ can be constructed as a HIT in HoTT.

---

## 4. (∞,1)-Categories

A *k-tuple groupoid* has invertible morphisms at all levels above $k$. The two extreme cases:
- *∞-groupoids:* all morphisms invertible (these model homotopy types)
- *Strict ω-categories:* no invertibility requirement at any level

The most important intermediate case is *(∞,1)-categories*: all $k$-morphisms for $k \geq 2$ are invertible, but 1-morphisms may not be.

### 4.1 Quasi-Categories (Joyal)

The most tractable model of (∞,1)-categories is via *quasi-categories* (Joyal, Lurie).

**Definition 12.10 (Quasi-Category).** A *quasi-category* is a simplicial set $X$ satisfying the *inner horn filling condition*: for every inner horn inclusion $\Lambda^n_k \hookrightarrow \Delta^n$ (with $0 < k < n$), every map $\Lambda^n_k \to X$ extends to a map $\Delta^n \to X$.

(Inner horns: remove the interior of $\Delta^n$ and one of its faces except the $k$-th. "Inner" means we don't remove the outer faces, just an inner face.)

**Intuition:** The inner horn $\Lambda^n_k$ represents a "composable sequence" of morphisms with one composition missing. The filling condition says that composition can always be performed.

The difference from Kan complexes: Kan complexes fill *all* horns (including outer horns $\Lambda^n_0$ and $\Lambda^n_n$), which corresponds to inverses existing for all morphisms.

### 4.2 Objects, Morphisms, Compositions

In a quasi-category $X$:
- **Objects:** 0-simplices $x : \Delta^0 \to X$
- **Morphisms:** 1-simplices $f : \Delta^1 \to X$ (with source $f(0)$ and target $f(1)$)
- **Composition:** Given composable $f : x \to y$ and $g : y \to z$ (i.e., a map from $\Lambda^2_1$), the horn filling gives a 2-simplex $\sigma : \Delta^2 \to X$ with faces $g$, $h$, and $f$ — where $h : x \to z$ is the composite. The composite is *not unique*, but it is unique *up to contractible choice*.

This is the key feature of (∞,1)-categories: composition is defined *up to homotopy*, and the space of choices is contractible.

### 4.3 Nerve of a Category

Every ordinary category $\mathcal{C}$ gives a quasi-category via its *nerve* $N(\mathcal{C})$:
- $n$-simplices of $N(\mathcal{C})$ are composable chains $A_0 \xrightarrow{f_1} A_1 \xrightarrow{f_2} \cdots \xrightarrow{f_n} A_n$
- Face maps are composition or deletion of objects at ends

**Theorem 12.11.** $N(\mathcal{C})$ is a quasi-category. Moreover, it fills inner horns *uniquely* — compositions are unique, as expected for ordinary categories.

Ordinary categories are the *discrete* case of (∞,1)-categories (where all 2-cells are trivial).

---

## 5. ∞-Groupoids: Multiple Definitions

The homotopy hypothesis requires a notion of ∞-groupoid. Several equivalent definitions exist:

**1. Kan complexes:** Simplicial sets satisfying all horn-filling conditions. The most tractable model.

**2. Batanin-Leinster globular sets with operations:** A combinatorial definition using globes (generalizations of spheres) and filling operations.

**3. Strict ω-groupoids:** Globular sets with strict composition laws. These *don't* model all homotopy types (only the "linear" ones, roughly).

**4. Types in MLTT/HoTT:** The internal notion. A type $A$ is an ∞-groupoid via its iterated identity types.

**5. Complete Segal spaces (Rezk):** Simplicial spaces satisfying Segal and completeness conditions.

The equivalence of these definitions (where they are in fact equivalent) is a non-trivial theorem of higher category theory.

---

## 6. Connection to HoTT

The relationship between HoTT and higher category theory is the following:

**HoTT is the internal language of ∞-toposes.**

An *∞-topos* (Lurie) is a presentable (∞,1)-category with enough "topos-like" structure. Just as an elementary topos has an internal logic (intuitionistic higher-order logic), an ∞-topos has an internal language — and this language is HoTT.

The precise statement:
- The *∞-topos of spaces* (Kan complexes) models HoTT with univalence.
- Constructing a new type in HoTT corresponds to constructing a new object in the ∞-topos.
- Proving a theorem in HoTT corresponds to constructing a section (global element) in the ∞-topos.

This is the deepest connection in the curriculum: HoTT is not just a logical system that *happens to* model homotopy types. It is the *inherent logic* of ∞-toposes, the same way that intuitionistic logic is the inherent logic of 1-toposes.

---

## Exercises

**12.1.** Show that the category $\mathbf{Grpd}$ of groupoids is itself a 2-category. What are the 2-morphisms?

**12.2.** Compute the fundamental groupoid $\Pi_1(X)$ for:
  - $X = S^1$ (the circle): what are the morphisms from the basepoint to itself?
  - $X = [0,1]$ (the interval): what morphisms exist between distinct points?
  - $X = \mathbb{R}^n$ (Euclidean space): how many morphisms between any two points?

**12.3.** Show that in MLTT, the groupoid laws (associativity, unit, inverse) hold for path concatenation. Give the explicit J-proof for the left unit law: $\mathsf{refl} \cdot p = p$.

**12.4.** A *2-groupoid* is a 2-category in which every 1-cell and every 2-cell is an isomorphism. Show that every topological space $X$ gives a 2-groupoid $\Pi_2(X)$ with:
  - Objects: points of $X$
  - 1-cells: paths (up to which equivalence?)
  - 2-cells: homotopies between paths (up to which equivalence?)

**12.5.** The nerve $N(G)$ of a group $G$ (viewed as a one-object category): describe the simplices of $N(G)$ explicitly. Show that $N(G)$ is a Kan complex (so $G$, as a one-object groupoid, gives an ∞-groupoid).

**12.6.** Inner horn filling: write out explicitly what a map $\Lambda^2_1 \to X$ consists of (i.e., what data it provides). Why does filling it give a "composite"?

**12.7 (Research).** Read the statement of the homotopy hypothesis as formalized by Grothendieck and Ara. What is the current status of the fully general conjecture? (Hint: it's been proved in various cases — for 1-groupoids (Quillen), for strict ω-groupoids (Brown-Higgins), but the full weak case remains open in full generality.)
