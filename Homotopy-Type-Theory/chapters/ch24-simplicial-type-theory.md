# Chapter 24: Simplicial Type Theory — Synthetic ∞-Category Theory

## Introduction

Homotopy type theory is the internal language of ∞-groupoids. Every type is an ∞-groupoid — all paths are invertible, every morphism is an equivalence. This is appropriate for homotopy theory, but it means HoTT cannot directly reason about directed structure: categories, where morphisms may not be invertible; functors, which may not be equivalences; natural transformations, which form a strict order.

*Simplicial type theory*, developed by Emily Riehl and Michael Shulman (2017 and ongoing), is an extension of HoTT designed to be the internal language of *∞-categories* — where morphisms are directed and may not be invertible. The central innovation is a second interval, the *simplicial interval*, which enables directed paths and Segal conditions.

This chapter introduces simplicial type theory (STT), the notion of Segal types (types that behave like ∞-categories), Rezk types (complete Segal spaces), and the synthetic Yoneda lemma. We also discuss the **Rzk** proof assistant, which implements simplicial type theory.

---

## 1. Two Intervals

### 1.1 The Cubical Interval $\mathbb{I}$

We have seen the cubical interval $\mathbb{I}$ in Chapters 22–23. It models *undirected* paths: if there is a path from $a$ to $b$, there is automatically a path from $b$ to $a$ (via complement $\sim$). The path type $a =_A b$ is symmetric.

The cubical interval corresponds to the topological interval $[0,1]$ with its symmetry (reversal).

### 1.2 The Simplicial Interval $\mathbf{2}$

Simplicial type theory introduces a second interval $\mathbf{2}$ — the *simplicial interval*, modeled by the poset $\{0 < 1\}$.

**Properties of $\mathbf{2}$:**
- Two terms: $0_\mathbf{2} : \mathbf{2}$ and $1_\mathbf{2} : \mathbf{2}$
- A *comparison*: there is a term comparing them in one direction only
- **Not symmetric**: unlike $\mathbb{I}$, the interval $\mathbf{2}$ has no complement. The "paths" in $\mathbf{2}$ are directed.

**The hom type:** For $a, b : A$, the *hom type* is:
$$\mathsf{hom}_A(a, b) :\equiv \Sigma_{f : \mathbf{2} \to A}\, f(0_\mathbf{2}) = a \times f(1_\mathbf{2}) = b$$

An element of $\mathsf{hom}_A(a, b)$ is a *directed path* from $a$ to $b$ in $A$ — a map from the directed interval $\mathbf{2}$ to $A$ with the correct endpoints.

**Key asymmetry:** $\mathsf{hom}_A(a, b)$ and $\mathsf{hom}_A(b, a)$ are *different* types. An element of one does not automatically give an element of the other. Directed paths are not automatically reversible.

---

## 2. Extension Types

### 2.1 The Extension Type Construction

The most important primitive in simplicial type theory is the *extension type*:

$$\langle \phi \to f \rangle_A$$

Given:
- A shape $\phi \subseteq \psi$ (an inclusion of simplicial shapes)
- A type family $A : \psi \to \mathsf{Type}$
- A partial section $f : \Pi_{t:\phi} A(t)$ (defined on the smaller shape)

The extension type $\langle \phi \to f \rangle_A$ is the type of *sections* of $A$ over $\psi$ that extend $f$.

**Intuition:** This is the type of ways to fill in the missing part of a partial section. It generalizes both dependent function types (when $\phi = \emptyset$) and the Kan horn-filling conditions (when $\phi$ is a horn).

### 2.2 The Connection to Kan Conditions

In classical simplicial homotopy theory, a Kan fibration has the property that horn inclusions $\Lambda^n_k \hookrightarrow \Delta^n$ lift. In simplicial type theory, this lifting property is expressed using extension types:

$$\mathsf{isKan}(A) :\equiv \Pi_{n}\, \Pi_{k}\, \Pi_{f : \Lambda^n_k \to A}\, \langle \Lambda^n_k \to f \rangle_{\Delta^n \to A}$$

A Kan type is one where all horn inclusions have extensions. The simplicial type theory axioms guarantee that all types satisfy this condition.

### 2.3 Hom Types Revisited

Using extension types, the hom type $\mathsf{hom}_A(a, b)$ can be expressed as:

$$\mathsf{hom}_A(a, b) :\equiv \langle \partial \mathbf{2} \to [0_\mathbf{2} \mapsto a, 1_\mathbf{2} \mapsto b] \rangle_{\mathbf{2} \to A}$$

where $\partial \mathbf{2} = \{0_\mathbf{2}, 1_\mathbf{2}\}$ is the boundary and the extension is over all of $\mathbf{2}$.

---

## 3. Segal Types

### 3.1 The Segal Condition

The Segal condition captures the notion of "a type with a well-behaved composition of morphisms."

**Definition 24.1 (Segal Type).** A type $A$ is *Segal* if for every composable pair of morphisms — two arrows $f : \mathsf{hom}_A(a, b)$ and $g : \mathsf{hom}_A(b, c)$ — there exists a unique (up to a contractible space of choices) composite $g \circ f : \mathsf{hom}_A(a, c)$.

More precisely, $A$ is Segal if the restriction map:
$$\mathsf{comp}_A : (\mathbf{2} \times \mathbf{2} \to A) \to \langle \Lambda^2_1 \to ... \rangle$$

is an equivalence, where $\Lambda^2_1$ is the "inner horn" — two composable arrows without the composite.

**Equivalent formulation:** $A$ is Segal iff for every $a, b, c : A$, the natural map:
$$\mathsf{hom}_A(a, c) \to \Sigma_{b:A}\, \mathsf{hom}_A(a, b) \times \mathsf{hom}_A(b, c)$$

is... (this version is not quite right — the Segal condition is about the equivalence of composable pairs with the composition).

**Correct formulation:** $A$ is Segal iff the map:
$$\langle \Delta^2 \to A \rangle \to \langle \Lambda^2_1 \to A \rangle$$
is an equivalence (restricting 2-simplices to their "spine" — the two composable edges).

### 3.2 Segal Types as ∞-Categories

A Segal type $A$ behaves like an ∞-category:
- Objects: elements $a : A$
- Morphisms: $f : \mathsf{hom}_A(a, b)$
- Composition: given by the Segal condition (unique up to contractibility)
- Associativity: automatic from the simplicial structure
- Identity: the diagonal $a \mapsto \mathsf{id}_a \in \mathsf{hom}_A(a, a)$

**The identity morphism:** In a Segal type $A$, the identity at $a : A$ is:
$$\mathsf{id}_a :\equiv \lambda t. a : \mathsf{hom}_A(a, a)$$

the constant path from $a$ to $a$.

**Remark:** Every ∞-groupoid (every type in ordinary HoTT) is a Segal type — composition is given by path concatenation. But Segal types are more general: morphisms need not be invertible.

### 3.3 Examples of Segal Types

**Example 24.2 (Sets).** The type $\mathsf{Set}$ of all sets (in an appropriate universe) is Segal. The hom type $\mathsf{hom}_\mathsf{Set}(A, B)$ is the type of functions $A \to B$ (not necessarily bijective). Composition is function composition.

**Example 24.3 (Posets).** Any poset $(P, \leq)$ gives a Segal type. The hom type $\mathsf{hom}_P(a, b)$ is the type of proofs that $a \leq b$ — either contractible (if $a \leq b$) or empty (if not). Composition is transitivity.

**Example 24.4 (∞-Groupoids).** Every type $X$ in HoTT is Segal (with the undirected paths as morphisms). But it also satisfies the stronger Rezk condition (see below).

**Example 24.5 (The Universe).** $\mathsf{Type}$ is a Segal type. The hom type $\mathsf{hom}_\mathsf{Type}(A, B)$ is the type of functions $A \to B$ (not necessarily equivalences). Composition is function composition. This models the ∞-category of types and functions.

---

## 4. Rezk Types

### 4.1 The Completeness Condition

A Segal type is like an ∞-category, but not every ∞-category is the "right" notion. We also need a completeness condition: an invertible morphism should correspond to a path between objects.

**Definition 24.6 (Rezk Type / Complete Segal Space).** A Segal type $A$ is *Rezk* (or *complete*) if for every $a, b : A$, the natural map:
$$(a = b) \to \mathsf{Iso}_A(a, b)$$

is an equivalence, where $\mathsf{Iso}_A(a, b)$ is the type of isomorphisms from $a$ to $b$ (morphisms with an inverse).

**Intuition:** In a Rezk type, two objects are *equal* iff they are *isomorphic*. This is exactly the univalence principle — but now for ∞-categories rather than types. A Rezk type is "univalent" in the categorical sense.

### 4.2 Rezk Types as ∞-Categories with Univalence

The Rezk condition is the categorical analogue of univalence:
- For types (∞-groupoids): univalence says equivalences = paths.
- For Segal types (∞-categories): the Rezk condition says isomorphisms = paths.

**Theorem 24.7.** Every type $A$ in HoTT (viewed as a Segal type) is automatically Rezk.

*Proof sketch.* In an ∞-groupoid, every morphism is invertible, so $\mathsf{Iso}_A(a,b) = \mathsf{hom}_A(a,b) = (a = b)$. $\square$

**Theorem 24.8.** The universe $\mathsf{Type}$ with the Segal structure above is *not* Rezk — because it would require all functions to be equivalences. The Rezk completion of $\mathsf{Type}$ (with isomorphisms = equivalences) satisfies the Rezk condition.

### 4.3 The Rezk Completion

Given any Segal type $A$, its *Rezk completion* $\hat{A}$ is the Segal type obtained by forcing the Rezk condition: two objects in $\hat{A}$ are equal iff they are isomorphic in $A$.

The Rezk completion is the categorical analogue of the propositional truncation: it imposes a condition on equality while preserving the universal property.

---

## 5. Functors, Natural Transformations, and Adjunctions

### 5.1 Functors Between Segal Types

**Definition 24.9 (Functor).** A *functor* between Segal types $A$ and $B$ is simply a function $F : A \to B$ (a term of the dependent function type). No extra condition is needed — every function between Segal types is automatically a functor.

*Why?* Because functions preserve all type-theoretic structure, including the Segal structure. The functoriality conditions (preservation of identity and composition) follow from how functions interact with the simplicial structure.

**Explicitly:** If $f : \mathsf{hom}_A(a, b)$, then $\mathsf{ap}_F(f) : \mathsf{hom}_B(F(a), F(b))$ — functoriality is just applying $F$ to a directed path. Identity preservation: $\mathsf{ap}_F(\mathsf{id}_a) = \mathsf{id}_{F(a)}$ (constant path goes to constant path).

### 5.2 Natural Transformations

**Definition 24.10 (Natural Transformation).** A *natural transformation* from $F : A \to B$ to $G : A \to B$ is an element of $\mathsf{hom}_{A \to B}(F, G)$ — a directed path from $F$ to $G$ in the function type.

Unfolding: a natural transformation $\alpha : \mathsf{hom}_{A \to B}(F, G)$ is a map $\alpha : \mathbf{2} \to (A \to B)$ with $\alpha(0) = F$ and $\alpha(1) = G$. By currying, this is a map $\alpha : A \to (\mathbf{2} \to B)$, i.e., for each $a : A$, a morphism $\alpha_a : \mathsf{hom}_B(F(a), G(a))$.

**Naturality:** The naturality condition — that for every $f : \mathsf{hom}_A(a,b)$, the square $\alpha_b \circ F(f) = G(f) \circ \alpha_a$ commutes — is *automatic*! It follows from the fact that $\alpha$ is a function and hence compatible with the simplicial structure.

This is the synthetic advantage: naturality is not a condition but a consequence.

### 5.3 The Yoneda Lemma Synthetically

**Theorem 24.11 (Synthetic Yoneda Lemma).** For a Segal type $A$, an object $a : A$, and a functor $F : A \to \mathsf{Type}$:
$$\mathsf{hom}_{A \to \mathsf{Type}}(\mathsf{hom}_A(a, -), F) \simeq F(a)$$

*Proof sketch.* A natural transformation $\alpha : \mathsf{hom}_{A \to \mathsf{Type}}(\mathsf{hom}_A(a,-), F)$ assigns to each $b : A$ a function $\alpha_b : \mathsf{hom}_A(a,b) \to F(b)$, naturally in $b$.

Define $\Phi(\alpha) = \alpha_a(\mathsf{id}_a) : F(a)$.

Define $\Psi(u)_b(f) = F(f)(u)$ for $u : F(a)$ and $f : \mathsf{hom}_A(a,b)$.

The naturality of $\Psi(u)$ follows from functoriality of $F$. The verification that $\Phi \circ \Psi = \mathsf{id}$ and $\Psi \circ \Phi = \mathsf{id}$ uses the Segal condition (unique composition). $\square$

**Corollary 24.12.** The representable functor $\mathsf{hom}_A(a,-) : A \to \mathsf{Type}$ is the "free" functor on the point $a$: any natural transformation from it to $F$ is determined by its value at the identity.

### 5.4 Adjunctions

**Definition 24.13 (Adjunction).** An *adjunction* between Segal types $A$ and $B$ consists of functors $F : A \to B$ and $G : B \to A$ together with a natural equivalence:

$$\mathsf{hom}_B(F(a), b) \simeq \mathsf{hom}_A(a, G(b))$$

natural in $a : A$ and $b : B$.

In simplicial type theory, this is expressed as: the function:

$$\Phi_{a,b} : \mathsf{hom}_B(F(a), b) \to \mathsf{hom}_A(a, G(b))$$

is an equivalence, and the collection $\{\Phi_{a,b}\}$ forms a natural transformation in both arguments.

The naturality again follows automatically from the function type structure.

---

## 6. The Rzk Proof Assistant

### 6.1 Overview

**Rzk** (Nikolai Kudasov, 2023) is a proof assistant implementing simplicial type theory. It is designed specifically for formalizing results in synthetic ∞-category theory.

**Key features:**
- Built on a variant of Riehl-Shulman simplicial type theory
- Uses TOPE (type of positions with extension types)
- Can formalize the Yoneda lemma, Segal/Rezk conditions
- Syntax designed to closely follow the mathematical notation of the Riehl-Shulman papers

### 6.2 Basic Rzk Syntax

```rzk
-- The 2-simplex shape (the standard 2-simplex)
#define Δ² := (t : 2 × 2 | t.1 ≤ t.2)

-- A type is Segal if inner horn filling is unique
#define isSegal (A : U) : U :=
  (x : A) → (y : A) → (z : A) →
  (f : hom A x y) → (g : hom A y z) →
  isContr (Σ (h : hom A x z), Δ² → A [
    (t.1 ≡ 0) ↦ f,
    (t.2 ≡ 1) ↦ g,
    (t.1 ≡ t.2) ↦ \t → id A (if ... else ...)
  ])

-- The Yoneda lemma statement
#define yoneda
  (A : U)
  (isSegalA : isSegal A)
  (a : A)
  (C : A → U)
  (isFibC : ...)  -- covariant fibration
  : (hom A a → C a) ≃ ((x : A) → hom A a x → C x)
  := ...
```

### 6.3 Formalizing the Yoneda Lemma in Rzk

The Riehl-Shulman paper "A Synthetic Theory of ∞-Categories in Homotopy Type Theory" (2017) contains a complete synthetic proof of the Yoneda lemma. The Rzk proof assistant can verify this proof.

**Key steps:**
1. Define the notion of a *covariant fibration* $C : A \to \mathsf{Type}$ (the right notion of "functor" in the directed setting).
2. State the Yoneda lemma: $\mathsf{hom}(a,-) \to C$ is equivalent to $C(a)$.
3. Prove via explicit construction of $\Phi$ and $\Psi$, verifying naturality synthetically.

---

## 7. Two-Sided Fibrations and the Arrow Category

### 7.1 The Arrow Category

For a Segal type $A$, the *arrow category* $A^\mathbf{2}$ is the function type $\mathbf{2} \to A$:

$$A^\mathbf{2} :\equiv \mathbf{2} \to A$$

An element $f : A^\mathbf{2}$ is a directed path in $A$, i.e., a morphism. This is the ∞-categorical analogue of the arrow category.

**Objects of $A^\mathbf{2}$:** Morphisms $f : \mathsf{hom}_A(a,b)$ in $A$.

**Morphisms of $A^\mathbf{2}$:** Commutative squares in $A$ (2-dimensional directed paths).

**Theorem 24.14.** If $A$ is Segal, so is $A^\mathbf{2}$.

### 7.2 Left and Right Fibrations

In classical ∞-category theory, a *left fibration* (or *right fibration*) is a map with special lifting properties modeling a covariant (or contravariant) functor to spaces.

In simplicial type theory, these are defined using the extension type:

**Definition 24.15 (Left Fibration).** A map $p : E \to A$ of Segal types is a *left fibration* if for every lifting problem (a horn with the incoming morphism), there is a unique (contractible space of) lifts.

Left fibrations model covariant functors $A \to \mathsf{Type}$: given such a functor $F$, the *Grothendieck construction* $\Sigma_{a:A} F(a)$ maps to $A$ via the projection, and this projection is a left fibration.

---

## 8. ∞-Topos Theory Synthetically

### 8.1 The ∞-Topos Axioms in Simplicial TT

An *∞-topos* is an ∞-category satisfying certain exactness conditions (Giraud axioms for ∞-categories). In simplicial type theory, the ambient type theory is the internal language of an ∞-topos — so ∞-topos conditions can be expressed as type-theoretic axioms.

**Key axioms for an ∞-topos:**
1. **Presentable**: Every type is a colimit of basic types.
2. **Locally cartesian closed**: For every map $f : A \to B$, the pullback functor $f^* : B/\mathsf{Type} \to A/\mathsf{Type}$ has a right adjoint (dependent product along $f$).
3. **Descent**: Colimits are "universal" (stable under base change).

In simplicial type theory, (2) and (3) are axioms of the type theory, and (1) is established via a generating set of types.

### 8.2 Sheaf ∞-Toposes

A key class of ∞-toposes is *sheaf ∞-toposes*: given a site $(C, J)$ (a small ∞-category with a Grothendieck topology), the $\infty$-topos of sheaves $\mathsf{Sh}(C, J)$ satisfies all the above axioms.

HoTT is the internal language of the *terminal* ∞-topos — the ∞-groupoids.

Simplicial type theory is designed to be the internal language of *all* ∞-toposes simultaneously, abstracting over their particular structure.

---

## 9. The Riehl-Shulman Program

### 9.1 The Vision

The Riehl-Shulman program aims to develop a type theory that is to ∞-category theory what HoTT is to homotopy theory:
- **HoTT**: Internal language of ∞-groupoids. Every type is an ∞-groupoid.
- **Simplicial TT**: Internal language of ∞-toposes. Every Segal type is an ∞-category.

The goal is to prove theorems of ∞-category theory synthetically — without point-set models of ∞-categories (quasi-categories, complete Segal spaces), but directly from the axioms of the type theory.

### 9.2 Results Formalized So Far

The following results have been formalized in simplicial type theory (either in Rzk or on paper):

1. **Yoneda Lemma** (Riehl-Shulman 2017): the synthetic Yoneda lemma for Segal types.
2. **Adjunctions** (Riehl-Shulman): the unit-counit characterization of adjunctions.
3. **Limits and Colimits** (Riehl-Shulman): terminal objects, initial objects, (co)products.
4. **∞-Groupoid Completion** (Riehl 2022): the Rezk completion construction.
5. **Slices and Overcategories** (Kudasov 2023): in Rzk.

### 9.3 Open Problems

1. **Presentability**: How to state and prove presentability (generators and relations for ∞-categories) in simplicial type theory.
2. **Grothendieck construction**: Formalizing the correspondence between left fibrations and covariant functors internally.
3. **Stable ∞-categories**: Adding structure for stable homotopy theory ($\Omega$-spectra, triangulated categories).
4. **Sheaf theory**: Defining Grothendieck topologies and sheaves in simplicial type theory.

---

## 10. Connections to Other Areas

### 10.1 Directed Univalence

The Rezk condition (completeness) is the directed analogue of univalence. A natural question: is there a *directed* version of the univalence axiom?

**Conjecture (directed univalence):** In an appropriate setting, the type of functors between Segal types $A$ and $B$ is equivalent to the type of "directed equivalences" from $A$ to $B$ — fully faithful and essentially surjective functors.

This would be the exact analogue of univalence for Segal types.

### 10.2 The Relation to Cubical Type Theory

Cubical type theory handles the *undirected* case (HoTT) with computational content. Simplicial type theory handles the *directed* case (∞-category theory) but currently lacks a proof of canonicity.

A major open problem: can simplicial type theory be given computational content analogous to cubical type theory? This would require:
- A computational interpretation of the simplicial interval $\mathbf{2}$
- Computation rules for the extension types
- A proof of canonicity

---

## Exercises

**24.1.** Verify that in a Segal type $A$, the identity morphism $\mathsf{id}_a = \lambda t. a : \mathsf{hom}_A(a,a)$ is a left and right unit for composition. (*Hint:* Use the Segal condition on composable pairs involving the constant path.)

**24.2.** Show that for any type $A$ (in ordinary HoTT), viewing $A$ as a Segal type with $\mathsf{hom}_A(a,b) = (a =_A b)$, the Rezk condition reduces to the statement: "every self-equivalence of $A$ corresponds to a path in $A$." How does this relate to univalence?

**24.3.** Let $P$ be a preorder (a type with a relation $\leq$ that is reflexive and transitive). Define a Segal type from $P$ and describe the hom types, composition, and the Rezk condition. When is a preorder Rezk?

**24.4.** State the Yoneda lemma (Theorem 24.11) in Rzk syntax. What is the type of the Yoneda map $\Phi$? (*Reference:* The Riehl-Shulman paper, Section 8.)

**24.5.** A natural transformation $\alpha : F \Rightarrow G$ is a *natural isomorphism* if each $\alpha_a : \mathsf{hom}_B(F(a), G(a))$ is an isomorphism. Show that $\alpha$ is a natural isomorphism iff $\alpha : \mathsf{hom}_{A \to B}(F, G)$ is an isomorphism in the functor type.

**24.6.** Describe the arrow category $A^\mathbf{2}$ when $A$ is:
  (a) A set (discrete Segal type)
  (b) A group (viewed as a one-object Segal type)
  (c) The type $\mathsf{Type}$ of all types

**24.7 (Research).** Read Riehl-Shulman, "A Synthetic Theory of ∞-Categories in Homotopy Type Theory" (2017). Identify the exact statement of the Yoneda lemma (their Theorem 5.5) and the proof strategy. How does the proof differ from the classical Yoneda proof in ordinary category theory? What is the role of the Segal condition?
