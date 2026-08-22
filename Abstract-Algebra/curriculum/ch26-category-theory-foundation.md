# Chapter 26 — Category Theory as Foundation

**Part VI: Foundations of Mathematics**
*Prerequisites: [Chapter 14](ch14-category-theory.md), [Chapter 24](ch24-set-theory-logic.md)*
*Next: [Chapter 27 — Topos Theory and Homotopy Type Theory](ch27-topos-homotopy-type-theory.md)*

---

## Learning Objectives

- Understand toposes as generalized universes (categories that behave like **Set**)
- See how logic can be formulated internally to a category
- Understand sheaves as variable sets and their role in geometry and logic
- Understand Lawvere's categorical axioms for set theory (ETCS)
- Begin to see how categorical logic unifies algebra, geometry, and logic

---

## 26.1 Limits of ZFC as a Foundation

### 26.1.1 The "Collection Problem"

ZFC has sets and proper classes (like "all groups") but the distinction is rigid and artificial. Category theory constantly works with large collections — the category of all groups, the category of all topological spaces — which are not sets in ZFC.

**Universes:** One fix: assume Grothendieck universes (large cardinals). Another: use a structural set theory.

### 26.1.2 The Structural Approach

In ZFC, sets are built "materially" — every set has a definite internal $\in$-structure. But in practice, mathematicians care about sets only up to isomorphism.

**Structural set theory** (ETCS, Lawvere): axioms talk about functions between sets, not membership. Functions are primitive; elements are derived (as maps from a singleton).

---

## 26.2 Toposes

### 26.2.1 Elementary Toposes

A **Grothendieck topos** is a category equivalent to the category of sheaves $\mathrm{Sh}(\mathcal{C}, J)$ on a site. More generally, an **elementary topos** is a category $\mathcal{E}$ satisfying:

1. **Finite limits exist** (and are preserved by pullback functors)
2. **Exponentials exist:** For any $A, B \in \mathcal{E}$, the "function object" $B^A$ (right adjoint to $- \times A$)
3. **Subobject classifier exists:** An object $\Omega$ with a map $\top: 1 \to \Omega$ such that every monomorphism $A \hookrightarrow B$ is a pullback of $\top$ along a unique "characteristic map" $B \to \Omega$

$\Omega$ is the "object of truth values." In **Set**, $\Omega = \{0, 1\}$ (true/false).

### 26.2.2 Examples of Toposes

| Topos | Description | Logic |
|-------|-------------|-------|
| **Set** | Sets and functions | Classical, 2-valued |
| $\mathrm{Sh}(X)$ | Sheaves on topological space $X$ | Intuitionistic, multi-valued |
| $[\mathcal{C}^{op}, \mathbf{Set}]$ | Presheaves on $\mathcal{C}$ | Intuitionistic |
| $G\text{-Set}$ | Sets with $G$-action | Classical, boolean |
| $\mathrm{Set}^{\Delta^{op}}$ | Simplicial sets | Homotopy-theoretic |
| $\mathrm{Sh}(\mathrm{Spec}\, R, \mathrm{\acute{e}t})$ | Étale sheaves on $\mathrm{Spec}\, R$ | Arithmetic geometry |

### 26.2.3 Internal Logic of a Topos

Every topos has an **internal language** — a higher-order typed logic in which you can reason about objects as if they were sets, but the reasoning is valid in any topos.

**Key feature:** The internal logic may be **intuitionistic** (law of excluded middle fails) in general toposes; it is **classical** in Boolean toposes.

**Subobject classifier:**
- In **Set**: $\Omega = \{T, F\}$
- In $\mathrm{Sh}(X)$: $\Omega(U) = \{$open subsets of $U\}$ — truth values are open sets
- In $G$-**Set**: $\Omega = \{$subgroups of $G\}$

---

## 26.3 Sheaves

### 26.3.1 Sheaves on a Topological Space

A **presheaf** $F$ on a topological space $X$ is a functor $F: \mathcal{O}(X)^{op} \to \mathbf{Set}$ — for each open set $U$, a set $F(U)$ of "sections over $U$," with restriction maps.

$F$ is a **sheaf** if it satisfies the gluing axiom: sections that agree on overlaps glue to a unique global section.

**Examples:**
- Continuous functions: $F(U) = \mathcal{C}(U, \mathbb{R})$
- Locally constant functions
- Regular functions on a variety (the structure sheaf $\mathcal{O}_X$)

### 26.3.2 Sheaves as Variable Sets

A sheaf $F$ on $X$ assigns to each open set $U$ a "local set" $F(U)$. This is like a set that "varies" across the space $X$. In the topos $\mathrm{Sh}(X)$:

- Objects (sheaves) = "variable sets"
- Morphisms = compatible families of functions

**Logic in $\mathrm{Sh}(X)$:** A proposition $P$ is "locally true" — true on some open cover, possibly not globally. This is intuitionistic logic.

### 26.3.3 Grothendieck Sites and Generalized Sheaves

A **Grothendieck topology** (site) generalizes "open covers" to an arbitrary category. A **sheaf** on a site $(\mathcal{C}, J)$ is a presheaf satisfying a descent condition.

**Key sites:**
- **Zariski site** on schemes — algebraic geometry
- **Étale site** — finer topology; étale sheaves compute $\ell$-adic cohomology
- **Crystalline site** — $p$-adic cohomology
- **Flat site** — descent theory

---

## 26.4 Lawvere's ETCS

### 26.4.1 Elementary Theory of the Category of Sets (ETCS)

Lawvere (1964) gave axioms for the category of sets without $\in$:

1. **Category axioms** (composition, identities)
2. **Terminal object** $1$ (one-element set)
3. **Products** (Cartesian products)
4. **Exponentials** (function sets $B^A$)
5. **Subobject classifier** $\Omega$ (truth values)
6. **Natural number object** $\mathbb{N}$ (Peano induction)
7. **Axiom of choice** (epimorphisms split)
8. **Well-pointedness** ($1$ is a generator)

ETCS is equiconsistent with ZFC (with appropriate replacement). It axiomatizes sets **structurally** — elements are maps $1 \to X$, not primitive.

### 26.4.2 The Internal Language

The **internal language** of ETCS (or any topos) is a higher-order intuitionistic type theory. Statements in this language can be interpreted in any topos — a form of "portable mathematics."

**Example:** The statement "every surjection has a section" is true in ETCS + AC (= **Set**) but false in $\mathrm{Sh}(X)$ for non-trivial $X$ (where AC fails). This detects fundamental differences in logical universes.

---

## 26.5 Categorical Logic

### 26.5.1 Theories as Categories

A **Lawvere theory** for a single-sorted algebraic theory (groups, rings, modules, etc.) is a category $\mathbb{T}$ with objects $\{n \mid n \in \mathbb{N}\}$ (powers of a generating object) and morphisms being the "operations."

A **model** of $\mathbb{T}$ in a category $\mathcal{C}$ is a product-preserving functor $\mathbb{T} \to \mathcal{C}$.

**Example:** $\mathbb{T}_\mathbf{Grp}$: models in **Set** are groups; models in **Ab** are abelian groups; models in **Top** are topological groups.

### 26.5.2 Classifying Toposes

For a geometric theory $\mathbb{T}$, there exists a topos $\mathrm{Set}[\mathbb{T}]$ (the **classifying topos**) such that:
$$\text{Topos morphisms } \mathcal{E} \to \mathrm{Set}[\mathbb{T}] \cong \text{models of } \mathbb{T} \text{ in } \mathcal{E}$$

The classifying topos "represents" all models of $\mathbb{T}$. This is the geometric/categorical analogue of the functor of points in algebraic geometry.

---

## 26.6 Cohomology via Sheaves

Sheaf cohomology unifies many cohomology theories:
- **De Rham cohomology:** $H^n_{dR}(X) = H^n(X, \mathcal{O}_{cl})$ (closed forms / exact forms)
- **Singular cohomology:** $H^n(X, \underline{A})$ for constant sheaf $\underline{A}$
- **Algebraic coherent cohomology:** $H^n(X, \mathcal{F})$ for a coherent sheaf $\mathcal{F}$ on a scheme

The **derived category** $D(\mathrm{Sh}(X))$ provides the correct framework: cohomology is just $\mathrm{R}\Gamma$ (derived global sections).

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Yoneda lemma (for toposes) | Every topos is a "universe" satisfying higher-order logic |
| Diaconescu's theorem | AC in a topos $\Rightarrow$ Boolean logic |
| Comparison theorem | Coherent toposes classify coherent theories |
| Barr's theorem | Every topos has a surjective Boolean cover (so classical arguments can apply) |

---

## Milestone Exercises

1. Verify that **Set** is an elementary topos by checking the three axioms.

2. Show that in the topos $G$-**Set** (sets with $G$-action), $\Omega$ = set of subgroups of $G$. What is the internal logic?

3. Define a sheaf on the topological space $\mathbb{R}$ by $F(U) =$ locally bounded functions on $U$. Check the sheaf axioms.

4. Show that Diaconescu's theorem holds: if every epimorphism in a topos splits (AC), then the topos is Boolean.

5. Define the Lawvere theory for commutative rings. What are its models in **Set**? In **Top**?

6. Show that a presheaf $F$ on a poset $(P, \leq)$ (with the Alexandrov topology) satisfies the sheaf axiom for every open cover iff it satisfies the sheaf axiom for two-element covers.

7. What does "the internal real numbers" mean in the topos $\mathrm{Sh}(\mathbb{R})$? What does continuity mean internally?

---

## Connections Forward

- **Chapter 27:** $\infty$-categories generalize toposes by allowing morphisms between morphisms to all orders; homotopy type theory internalizes homotopy theory in foundations.

---

*Next: [Chapter 27 — Topos Theory and Homotopy Type Theory](ch27-topos-homotopy-type-theory.md)*
