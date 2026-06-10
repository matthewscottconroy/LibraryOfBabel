# 25.1 Modalities in Homotopy Type Theory

## What Is a Modality?

A *modality* is a universal construction: a way of "reflecting" types into a special subcategory, preserving as much structure as possible while satisfying some definability or closure condition.

The concept comes from modal logic, where modalities like $\Box$ (necessity) and $\Diamond$ (possibility) modify propositions. In modal logic, $\Box P$ means "$P$ is necessarily true" and $\Diamond P$ means "$P$ is possibly true." These operators satisfy certain axioms (K, T, 4, etc.) depending on the modality.

In type theory, modalities are more general: they act not just on propositions but on all types. The type-theoretic notion captures a wide class of universal constructions that appear throughout mathematics: truncation, localization, completion, sheafification, and the geometric modalities (shape, flat, sharp) of cohesive HoTT.

## The Abstract Definition

**Definition (Modality).** A *modality* $(\bigcirc, \eta)$ in HoTT consists of:

1. An operation $\bigcirc : \mathsf{Type} \to \mathsf{Type}$ assigning to each type $A$ a *modal type* $\bigcirc A$
2. A natural map $\eta_A : A \to \bigcirc A$ (the *modal unit* or *localization map*)
3. An *idempotency* condition: $\eta_{\bigcirc A} : \bigcirc A \to \bigcirc(\bigcirc A)$ is an equivalence
4. A *universal property*: for any $\bigcirc$-modal type $B$ (a type where $\eta_B : B \to \bigcirc B$ is an equivalence), every map $f : A \to B$ extends uniquely through $\eta_A$:
$$(\bigcirc A \to B) \xrightarrow{\;\simeq\;} (A \to B)$$
(by precomposition with $\eta_A$)

A type $B$ is $\bigcirc$-*modal* if $\eta_B : B \to \bigcirc B$ is an equivalence.

**Idempotency**: The condition says that applying $\bigcirc$ twice gives the same thing as applying it once: $\bigcirc(\bigcirc A) \simeq \bigcirc A$. This is the "once is enough" property of reflective constructions.

**The universal property**: $\bigcirc A$ is the "best approximation to $A$ from within the subcategory of $\bigcirc$-modal types." Any map from $A$ to a modal type factors uniquely through the localization $\eta_A : A \to \bigcirc A$.

## Modalities as Reflective Subcategories

**Theorem.** Modalities on the ∞-topos of types are in bijection with full subcategories $\mathcal{E}_\bigcirc \subseteq \mathsf{Type}$ that are:
1. *Closed under limits*: pullbacks, products, and the terminal type of modal types are modal
2. *Reflective*: the inclusion has a left adjoint $\bigcirc : \mathsf{Type} \to \mathcal{E}_\bigcirc$

The modal types form a reflective subcategory; the modality is the reflector (the left adjoint to the inclusion).

This gives a clean way to specify a modality: describe the subcategory of modal types, and the modality is the reflection into that subcategory.

## Key Examples

### Propositional Truncation ($\|-\|$)
- $\bigcirc A :\equiv \|A\|$: the propositional truncation (the "mere existence" of an element of $A$)
- $\eta_A :\equiv |{-}| : A \to \|A\|$
- Modal types: propositions (h-props)
- Universal property: maps $A \to P$ where $P$ is a proposition factor through $\|A\|$
- This is the modality collapsing proofs: it retains only "is $A$ inhabited?" not "which element of $A$?"

### $n$-Truncation ($\|-\|_n$)
- $\bigcirc A :\equiv \|A\|_n$: the $n$-truncation (the $n$-th Postnikov section)
- Modal types: $n$-truncated types (h-levels $\leq n$)
- Universal property: maps $A \to B$ where $B$ is $n$-truncated factor through $\|A\|_n$
- For $n = -1$: propositional truncation; for $n = 0$: set-truncation; for $n = -2$: the constant map to $\mathbf{1}$

### Localization at a Map
- Given a map $f : A \to B$, the *$f$-localization* $L_f$ makes $f$ into an equivalence
- $\bigcirc X :\equiv L_f X$: the $f$-local type (types where every map from $B$ that precomposes with $f$ has a unique extension from $A$)
- Modal types: $f$-local types ($f$-morphism is an equivalence in the hom-space)
- Example: $f$ is $S^1 \to \mathbf{1}$ — localizing kills $\pi_1$

### Nullification ($\mathsf{Null}_B$)
- $\bigcirc A :\equiv \mathsf{Null}_B(A)$: the $B$-nullification of $A$
- Modal types: types where every map $B \to X$ is null-homotopic (factors through $\mathbf{1}$)
- Universal property: nullification is the reflection into $B$-null types
- Special case $B = S^n$: nullifying at spheres gives Postnikov truncation

Nullification is the most general way to produce modalities: every modality in HoTT is equivalent to a nullification for some (possibly large) type $B$.

## Left Exact (Lex) Modalities

Not all modalities preserve the logical structure of types. *Left exact* (lex) modalities are the well-behaved ones:

**Definition.** A modality $\bigcirc$ is *left exact* (lex) if it preserves finite limits:
1. $\bigcirc \mathbf{1} \simeq \mathbf{1}$ (the terminal type is modal)
2. $\bigcirc(A \times_C B) \simeq \bigcirc A \times_{\bigcirc C} \bigcirc B$ (pullbacks preserved)

Equivalently: $\bigcirc$ preserves the identity type: $\bigcirc(a =_A b) \simeq (\bigcirc a =_{\bigcirc A} \bigcirc b)$.

**Why lex matters**:
- Lex modalities preserve propositions: if $A$ is a proposition, $\bigcirc A$ is a proposition
- Lex modalities preserve h-sets, h-groupoids, etc.
- Lex modalities commute with $\Sigma$-types: $\bigcirc(\Sigma_{x:A} B(x)) \simeq \Sigma_{x:\bigcirc A} \bigcirc B(x)$ (under conditions)

**Examples**:
- Propositional truncation: NOT lex (it collapses types in a way that doesn't preserve pullbacks)
- $n$-truncation for $n \geq 0$: lex (preserves pullbacks)
- The shape modality $\int$: lex (it preserves products: $\int(A \times B) \simeq \int A \times \int B$)
- The sharp modality $\sharp$: lex (being a right adjoint, it preserves all limits)

## Orthogonal Factorization Systems

Every modality $\bigcirc$ gives an *orthogonal factorization system* (OFS) on maps:

- **$\bigcirc$-connected maps**: maps $f : A \to B$ where the fibers $f^{-1}(b)$ are all $\bigcirc$-connected (i.e., $\bigcirc$-unit is an equivalence on fibers)
- **$\bigcirc$-modal maps**: maps where the fibers are all $\bigcirc$-modal

**The factorization**: Every map $f : A \to B$ factors as $A \xrightarrow{c} E \xrightarrow{m} B$ where $c$ is $\bigcirc$-connected and $m$ is $\bigcirc$-modal. This factorization is unique up to equivalence.

**Example**: For $n$-truncation, the connected maps are $(n+1)$-connected maps and the modal maps are $n$-truncated maps. The factorization is: $A \to \|A\|_{n+1} \to B$ — first surject onto the $(n+1)$-truncation, then map modally.

## Modalities and Adjunctions

Every adjunction $L \dashv R$ between ∞-toposes gives a modality on the right topos: $\bigcirc = R \circ L$, with unit $\eta_A : A \to R(L(A))$.

In the cohesive setting:
- The flat modality: $\flat = \text{Discrete} \circ \Gamma$ where $\Gamma$ = global sections
- The sharp modality: $\sharp = \text{Codiscrete} \circ \Gamma$
- The shape modality: $\int = \text{Discrete} \circ \Pi_0$ where $\Pi_0$ = connected components / geometric realization

Each comes from an adjunction between the cohesive topos and the base topos (the topos of discrete spaces). The adjoint triple $\int \dashv \flat \dashv \sharp$ means $\int$ and $\flat$ are adjoint, and $\flat$ and $\sharp$ are adjoint.

## The Formal Definition of Modal Operators

In HoTT, modalities can be axiomatized within the type theory itself. Shulman's *Modalities in Homotopy Type Theory* (2023) gives a comprehensive account.

**The key data for a lex modality** (in a type-theoretic presentation):
1. A type operator $\bigcirc : \mathsf{Type} \to \mathsf{Type}$
2. A unit $\eta : \Pi_{A:\mathsf{Type}} A \to \bigcirc A$
3. A "modal elimination rule": for any $\bigcirc$-modal family $P : \bigcirc A \to \mathsf{Type}$ and section $s : \Pi_{a:A} P(\eta(a))$, there exists a unique extension $\bar{s} : \Pi_{x:\bigcirc A} P(x)$

The elimination rule captures the universal property: maps from $A$ to modal types extend uniquely to $\bigcirc A$.

**Closure conditions**: A lex modality further satisfies:
- $\bigcirc \mathbf{1} = \mathbf{1}$ (or is contractible)
- If $P : \bigcirc A \to \mathsf{Type}$ with each $P(x)$ modal, then $\Pi_{x:\bigcirc A} P(x)$ is modal
- The dependent product of modal types is modal

These closure conditions ensure the modality behaves nicely with all type constructors.

## Implementation in Cubical Agda

In Cubical Agda, modalities are implemented using universe polymorphism and postulated or derived operations:

```agda
{-# OPTIONS --cubical #-}
module Modalities where
open import Cubical.Foundations.Prelude

-- Propositional truncation: a modality
open import Cubical.HITs.PropositionalTruncation

-- The modal type for propositional truncation
-- ∥ A ∥₁ : Type
-- The unit: ∣_∣₁ : A → ∥ A ∥₁
-- The universal property: rec₁ : isProp B → (A → B) → ∥ A ∥₁ → B

-- n-truncation: a modality
open import Cubical.HITs.Truncation
-- ∥ A ∥ n : Type  (n-truncation of A)
-- The unit: ∣_∣ : A → ∥ A ∥ n
-- Universal property: rec : isOfHLevel n B → (A → B) → ∥ A ∥ n → B

-- Cohesive modalities (experimental --cohesion flag)
{-# OPTIONS --cohesion --flat-split #-}
postulate
  ♭ : {ℓ : Level} → Type ℓ → Type ℓ
  ♭-counit : {ℓ : Level} {A : Type ℓ} → ♭ A → A
```

The `--cohesion` flag in Cubical Agda enables experimental support for the flat modality $\flat$. The shape $\int$ and sharp $\sharp$ are currently postulated axioms.

## Why Modalities Matter for Geometry

The power of modal type theory for geometry is that modalities capture the relationship between different *levels of structure*:

- A smooth manifold $M$ has elements (points), a topology (paths), and a smooth structure (differential forms)
- The flat modality $\flat M$ retains only the points — no paths, no geometry
- The shape $\int M$ retains only the topology — no smooth structure
- The sharp $\sharp M$ provides the codiscrete version — a "formal space"

The relationships between $\flat M$, $M$, $\int M$, and $\sharp M$ encode the entire geometric structure of $M$. Differential geometry is the study of these relationships — of what is preserved when you pass from $M$ to $\flat M$ (locally constant functions), from $M$ to $\int M$ (homotopy invariants), and back.

The modalities are the language in which geometry is expressed internally to type theory. Not via coordinates. Not via charts. Via the adjoint triple $\int \dashv \flat \dashv \sharp$.
