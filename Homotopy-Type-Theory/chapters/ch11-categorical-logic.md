# Chapter 11: Categorical Logic and the Semantics of Type Theory

## Introduction

Categorical logic studies the relationship between categories and logical systems. The central idea is the *internal language* of a category: every sufficiently structured category has an associated logical system, and conversely, every logical system has an associated category of models. Understanding this correspondence is essential for:

1. **Understanding why type theory works:** The rules of dependent type theory are exactly the rules that hold in locally cartesian closed categories.
2. **Building models:** To prove consistency or independence results, we construct categorical models.
3. **Connecting HoTT to homotopy theory:** The category of Kan simplicial sets models HoTT (with univalence as a theorem rather than an axiom).

This chapter develops the categorical semantics of type theory from the ground up.

---

## 1. Cartesian Closed Categories and STLC

### 1.1 Cartesian Categories

**Definition 11.1.** A category $\mathcal{C}$ is *cartesian* if it has:
- A terminal object $\mathbf{1}$
- Binary products $A \times B$ for all objects $A, B$

The terminal object and binary products give $n$-ary products for all $n$: $A_1 \times \cdots \times A_n$ (with $A_0 = \mathbf{1}$ as the nullary product).

In a cartesian category, we can model contexts: a context $\Gamma = x_1 : A_1, \ldots, x_n : A_n$ corresponds to the product type $A_1 \times \cdots \times A_n$. A substitution $\sigma : \Gamma \to \Gamma'$ corresponds to a morphism $\sigma : A_1 \times \cdots \times A_n \to B_1 \times \cdots \times B_m$.

### 1.2 Cartesian Closed Categories

**Definition 11.2 (CCC).** A cartesian category $\mathcal{C}$ is *cartesian closed* if for every object $A$, the functor $(-) \times A : \mathcal{C} \to \mathcal{C}$ has a right adjoint, written $[A, -]$ or $A \Rightarrow -$ (the *internal hom* or *exponential object*).

The adjunction says:
$$\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A, B])$$
naturally in $C$ and $B$. The isomorphism is *currying*.

**Theorem 11.3.** The category $\mathbf{Set}$ is a CCC. The exponential $[A, B] = B^A$ is the set of all functions from $A$ to $B$.

**Theorem 11.4.** The internal language of a CCC is the *simply typed lambda calculus*.

More precisely:
- Objects of $\mathcal{C}$ correspond to types.
- Morphisms $f : A \to B$ correspond to terms $x : A \vdash f(x) : B$ (terms in context).
- Terminal object $\mathbf{1}$ corresponds to the unit type.
- Products $A \times B$ correspond to product types.
- Exponentials $[A, B]$ correspond to function types $A \to B$.
- Currying corresponds to $\to$-introduction.
- Application corresponds to $\to$-elimination.

Every model of STLC is a CCC, and every CCC gives a model of STLC.

---

## 2. Locally Cartesian Closed Categories and Dependent Types

### 2.1 Slice Categories

**Definition 11.5 (Slice Category).** Given a category $\mathcal{C}$ and object $I \in \mathcal{C}$, the *slice category* $\mathcal{C}/I$ has:
- Objects: morphisms $f : A \to I$ in $\mathcal{C}$ (pairs $(A, f)$)
- Morphisms $(A, f) \to (B, g)$: morphisms $h : A \to B$ with $g \circ h = f$

The slice category $\mathcal{C}/I$ models "types in context $I$": an object of $\mathcal{C}/I$ is a family of types indexed by $I$.

**Example 11.6.** In $\mathbf{Set}/I$: an object is a function $f : A \to I$, which corresponds to the family of sets $\{f^{-1}(i)\}_{i \in I}$ — a type family indexed by $I$.

### 2.2 Locally Cartesian Closed Categories

**Definition 11.7 (LCCC).** A category $\mathcal{C}$ is *locally cartesian closed* if every slice category $\mathcal{C}/I$ is a cartesian closed category.

**Theorem 11.8.** The category $\mathbf{Set}$ is locally cartesian closed.

In $\mathbf{Set}/I$:
- Terminal object: $(I, \mathsf{id}_I)$ (the identity, modeling the trivial family)
- Products: $(A, f) \times (B, g) = (A \times_I B, \pi)$ (the pullback over $I$, modeling Σ types)
- Exponentials: $[(A, f), (B, g)]$ in $\mathbf{Set}/I$ models Π types

**Theorem 11.9 (Seely, Hofmann, Dybjer).** The internal language of a locally cartesian closed category is *dependent type theory* (specifically, a version without identity types).

The correspondence:
- Slice $\mathcal{C}/I$ corresponds to types in context $\Gamma$ (where $\Gamma$ is the object $I$)
- Products in $\mathcal{C}/I$ correspond to Σ types
- Exponentials in $\mathcal{C}/I$ correspond to Π types
- The reindexing functor $f^* : \mathcal{C}/J \to \mathcal{C}/I$ (for $f : I \to J$) corresponds to substitution

### 2.3 The Substitution Problem

There is a subtlety in making this correspondence precise: in categorical models, composition is *strictly* associative; in type theory, substitution is only associative *up to definitional equality*. This gives the *coherence problem*.

**Solutions:**
1. *Contextual categories (Cartmell):* A stricter structure that matches the syntactic structure of type theory exactly.
2. *Categories with families (Dybjer):* A generalized algebraic structure.
3. *Comprehensive factorization / display map categories (Taylor, Jacobs):* Using fibrations.

---

## 3. Fibered Categories

Fibered categories provide a clean way to model dependent types categorically, without the coherence problem of LCCC semantics.

### 3.1 Grothendieck Fibrations

**Definition 11.10 (Fibration).** A functor $p : \mathcal{E} \to \mathcal{B}$ is a *Grothendieck fibration* if for every morphism $f : I \to J$ in $\mathcal{B}$ and every object $X$ with $p(X) = J$, there exists a *cartesian lift*: an arrow $\bar{f} : \bar{f}^*X \to X$ in $\mathcal{E}$ with $p(\bar{f}) = f$, which is universal in the appropriate sense.

The *fiber* over $I$ is $\mathcal{E}_I = p^{-1}(I)$ (the subcategory of objects over $I$).

**Example 11.11.** The *codomain fibration* $\text{cod} : \mathcal{C}^{\to} \to \mathcal{C}$ sends a morphism $f : A \to B$ to its codomain $B$. The fiber over $B$ is the slice category $\mathcal{C}/B$. This fibration is used to model families of types.

**Example 11.12.** The *fundamental fibration* over a topological space $X$: the total space is the pathspace $\{(x, y, p) \mid x, y \in X, p : x \to y \text{ a path}\}$, fibered over $X \times X$ via the endpoints. This is the direct topological model of the identity type!

### 3.2 Types as Fibrations

In the categorical semantics of dependent type theory:
- A *context* $\Gamma$ is an object of the base category $\mathcal{B}$.
- A *type* $\Gamma \vdash A \,\mathsf{type}$ is an object of the fiber $\mathcal{E}_\Gamma$.
- A *term* $\Gamma \vdash a : A$ is a *section*: a morphism $s : \Gamma \to A$ in $\mathcal{E}_\Gamma$ with $p(s) = \mathsf{id}_\Gamma$.
- *Substitution* $A[\sigma]$ corresponds to the reindexing functor $\sigma^* : \mathcal{E}_\Delta \to \mathcal{E}_\Gamma$ for $\sigma : \Gamma \to \Delta$.

---

## 4. Toposes

Toposes are categories that behave like the category of sets — but can model many different logical universes.

### 4.1 Elementary Toposes

**Definition 11.13 (Elementary Topos).** An *elementary topos* $\mathcal{E}$ is a category with:
1. Finite limits (equivalently: terminal object, pullbacks)
2. Power objects: for every $A$, a *subobject classifier* $\Omega$ and a bijection $\mathsf{Sub}(A) \cong \mathsf{Hom}(A, \Omega)$
3. Exponentials: $[A, B]$ for all $A, B$

The *subobject classifier* $\Omega$ classifies subobjects: every subobject $S \hookrightarrow A$ is classified by a unique morphism $\chi_S : A \to \Omega$.

**In $\mathbf{Set}$:** $\Omega = \{\text{true, false}\}$. The subobject classifier classifies subsets: $\chi_S(a) = \text{true}$ iff $a \in S$. This is the characteristic function.

**In other toposes:**
- Sheaves $\mathbf{Sh}(X)$ on a topological space: $\Omega$ is the sheaf of open subsets of $X$. The subobject classifier has a different logic — not classical!
- Presheaves $[\mathcal{C}^{op}, \mathbf{Set}]$: $\Omega$ is the presheaf of sieves. This models intuitionistic logic.

### 4.2 The Internal Logic of a Topos

Every topos has an *internal language*: an intuitionistic higher-order logic. The objects of $\mathcal{E}$ are the "types" and the subobjects of $A$ are the "propositions about $A$" (via the bijection $\mathsf{Sub}(A) \cong \mathsf{Hom}(A, \Omega)$).

**Key feature:** The internal logic of a topos is *intuitionistic* (LEM fails for a general topos). Classical toposes (satisfying LEM internally) are precisely Boolean toposes.

This explains why constructive logic is natural in categorical semantics: it is the logic that works in *all* toposes, not just $\mathbf{Set}$.

### 4.3 Presheaf Models

**Example 11.14.** For any category $\mathcal{C}$, the presheaf category $\widehat{\mathcal{C}} = [\mathcal{C}^{op}, \mathbf{Set}]$ is a topos. It models an *intuitionistic set theory with multiple stages of information*.

This is directly relevant to HoTT: the model of cubical type theory lives in a presheaf category over the cube category $\Box$. The objects of this category are "cubical sets" — sets with structure in each dimension, modeling the cube-like structure of path types in cubical type theory.

---

## 5. The Semantics of the Identity Type

### 5.1 The Problem

When we add the identity type to dependent type theory (making MLTT), the categorical semantics becomes subtler. The identity type $a =_A b$ is a type that depends on two terms $a, b : A$. Categorically, it should be an object in the fiber over $A \times A$.

**The path object semantics:** In a model category (a category with a well-behaved notion of homotopy), every object $A$ has a *path object* $\mathsf{Path}(A)$ — an object that models the type of paths in $A$:
$$\mathsf{Path}(A) = A^{[0,1]}$$
with two endpoint maps $s, t : \mathsf{Path}(A) \to A$ (source and target) and a reflexivity map $r : A \to \mathsf{Path}(A)$ (the constant path).

The identity type $a =_A b$ is modeled by the fiber of $(s, t) : \mathsf{Path}(A) \to A \times A$ over $(a, b)$.

### 5.2 The Awodey-Warren Theorem

**Theorem 11.15 (Awodey-Warren, 2009).** The groupoid model of MLTT (where types are groupoids and identity types are hom-sets) provides a model in which UIP fails.

This theorem, proved just before HoTT was formulated, was the first evidence that the identity type could have non-trivial semantics. It prepared the ground for Voevodsky's simplicial set model.

### 5.3 The Simplicial Set Model

**Theorem 11.16 (Voevodsky, 2006-2009).** The category of Kan simplicial sets provides a model of MLTT in which:
1. Every type is a Kan complex (a weak ∞-groupoid)
2. Identity types correspond to path spaces
3. The univalence axiom holds as a theorem
4. UIP fails in general

This theorem is the foundation of HoTT. It shows that Voevodsky's univalence axiom is *consistent* (relative to ZFC with large cardinals) — because it holds in the simplicial set model.

---

## Exercises

**11.1.** Show that the category $\mathbf{Set}$ is a CCC by:
  - Constructing the exponential $B^A$ explicitly
  - Showing the natural bijection $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, B^A)$ (currying/uncurrying)

**11.2.** The category $\mathbf{Pos}$ of posets and order-preserving functions: is it a CCC? If so, describe the exponential $[A, B]$.

**11.3.** Describe the slice category $\mathbf{Set}/\{0,1\}$ explicitly. What are its objects? What do products in this slice category correspond to for type families over $\{0, 1\}$?

**11.4.** Show that the product in the slice $\mathbf{Set}/I$ is the pullback in $\mathbf{Set}$:
$$\{(a, b) \in A \times B \mid f(a) = g(b)\}$$

**11.5.** In the internal logic of the topos $\widehat{\mathcal{C}}$ of presheaves on a category $\mathcal{C}$:
  - What does the subobject classifier $\Omega$ look like? ($\Omega(c) = $ sieves on $c$)
  - When does LEM fail internally?

**11.6.** The *subobject classifier* in $\mathbf{Set}$ is $\Omega = \{\mathsf{T}, \mathsf{F}\}$. Every subset $S \subseteq A$ is classified by a unique function $\chi_S : A \to \Omega$. Verify: $\chi_{S_1 \cap S_2} = \chi_{S_1} \wedge \chi_{S_2}$ (pointwise AND).

**11.7 (Challenge).** Sketch the construction of the groupoid model of MLTT (Awodey-Warren). Specifically:
  - What is a "type" in this model? (A groupoid)
  - What is a "term"? (An object of the groupoid)
  - What is the identity type $a =_G b$? (The set of morphisms from $a$ to $b$)
  - Why does UIP fail? (Hint: $\pi_1(S^1)$ — morphisms from the basepoint to itself are all integers)

---

## See Also

**In chapters/:**
- `ch10-category-theory` — Prerequisite. The categorical concepts (CCC, adjunctions, limits) that ch11 applies to type theory.
- `ch08-dependent-types` and `ch09-mltt` — The type theories whose categorical semantics ch11 develops. The central correspondence: dependent type theory ↔ locally Cartesian closed category (LCCC). Specifically: `Π_f B` (the dependent product along `f`) is right adjoint to `f*` (pullback along `f`).
- `ch12-higher-categories` — The categorical semantics of HoTT requires ∞-categories (or ∞-toposes), not just ordinary categories. The groupoid model (exercise 11.7) is the 1-categorical approximation to the full ∞-categorical semantics.
- `ch15-simplicial-sets` — The Kan simplicial set model is a specific locally Cartesian closed category (the category of Kan complexes) that models MLTT with univalence. The subobject classifier of sSet is the Kan complex of propositions.
- `ch18-univalence` — Univalence holds in any ∞-topos with a univalent universe. The topos-theoretic perspective makes precise what "universe" means and why univalence is a property of the universe object.

**In book/:**
- `book/unit-04-category-theory/ch11-categorical-logic/` — Extended narrative on categorical semantics, emphasizing the LCCC ↔ dependent type theory correspondence and its implications for the design of proof assistants.

**In demos/:**
- `demos/demo_categorical_logic.py` — Illustrates CCCs, Cartesian closed structure, and the internal language of a category.
- `demos/demo_categories.py` — Basic category theory; natural transformations and the Yoneda lemma (from ch10) are prerequisites for the fibration constructions here.

**The LCCC ↔ dependent type theory dictionary:**
| Type Theory | Category Theory |
|---|---|
| Context `Γ` | Object of the base category C |
| Type `Γ ⊢ A : Type` | Morphism `A → Γ` in C (a "display map") |
| Term `Γ ⊢ a : A` | Section of `A → Γ` |
| Weakening | Pullback: `f*A` for `f : Δ → Γ` |
| Substitution | Pullback of display maps |
| `Π_f A` (dependent product) | Right adjoint of `f*`: `Π_f ⊣ f*` |
| `Σ_f A` (dependent sum) | Left adjoint: `Σ_f ⊣ f*` |
| Identity type `Id_A` | Path-object `A^I → A × A` |
