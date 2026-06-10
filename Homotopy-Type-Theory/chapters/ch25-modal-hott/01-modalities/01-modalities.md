# 1.1 Modalities in Type Theory

## What is a Modality?

A *modality* is a universal construction in type theory: given a type $A$, the modal type $\bigcirc A$ is the "reflection" of $A$ into a subcategory of types satisfying some property.

The abstract definition captures this universally:

**Definition.** A *modality* $(\bigcirc, \eta)$ in HoTT consists of:
1. For each type $A$, a type $\bigcirc A$ and a map $\eta_A : A \to \bigcirc A$ (the *unit*)
2. A type $A$ is *$\bigcirc$-modal* if $\eta_A : A \to \bigcirc A$ is an equivalence
3. For any $\bigcirc$-modal type $B$, every map $f : A \to B$ extends uniquely through $\eta_A$:

$$A \xrightarrow{\eta_A} \bigcirc A \xrightarrow{\;\exists! \bar{f}\;} B$$

In short: $\bigcirc A$ is the $\bigcirc$-modal type that $A$ maps into, in a universal way. Equivalently, $\bigcirc$ is a *left adjoint* to the inclusion of $\bigcirc$-modal types into all types.

**The universal property explicitly:** Precomposition with $\eta_A$ is an equivalence:

$$(\bigcirc A \to B) \xrightarrow{\; (-) \circ \eta_A \;} (A \to B)$$

for any $\bigcirc$-modal type $B$.

## Familiar Examples

### Propositional Truncation

The propositional truncation $\|-\| : \mathsf{Type} \to \mathsf{Prop}$ is a modality.

- $\bigcirc A = \|A\|$ (the propositional truncation)
- $\eta_A = |{-}| : A \to \|A\|$ (the constructor)
- Modal types: propositions ($\bigcirc B \simeq B$ iff $B$ is a proposition)
- Universal property: any map $A \to P$ where $P$ is a proposition extends uniquely through $\|A\|$

### n-Truncation

The $n$-truncation $\|-\|_n : \mathsf{Type} \to n\text{-}\mathsf{Type}$ is a modality.

- $\bigcirc A = \|A\|_n$
- Modal types: $n$-truncated types
- Universal property: maps out of $\|A\|_n$ into $n$-types correspond to maps out of $A$

### Localization

For a set of maps $S = \{f_i : A_i \to B_i\}$, the *localization* $L_S A$ is the universal type making all maps in $S$ into equivalences.

- If $S = \{f : A \to B\}$: $L_S$ makes $f$ an equivalence
- Special case: $f$ is the map $\mathbf{2} \to \mathbf{1}$ (collapsing two points) — localizing makes $\mathbf{2}$ contractible, which turns the type theory into a version where all propositions are trivial

The localization modality is the source of many important modalities in homotopy theory.

## Left Exact Modalities

Not all modalities are equal. *Left exact* (lex) modalities are particularly well-behaved — they preserve the structure of types.

**Definition.** A modality $\bigcirc$ is *left exact* if it preserves pullbacks: for any pullback

$$\begin{array}{ccc} P & \xrightarrow{g'} & B \\ \downarrow_{f'} & & \downarrow_f \\ A & \xrightarrow{g} & C \end{array}$$

the natural map $\bigcirc P \to \bigcirc A \times_{\bigcirc C} \bigcirc B$ is an equivalence.

**Equivalently:** $\bigcirc$ is lex iff it preserves:
- The identity type: $\bigcirc(a =_A b) \simeq (\bigcirc a =_{\bigcirc A} \bigcirc b)$
- Pullbacks: $\bigcirc(A \times_C B) \simeq \bigcirc A \times_{\bigcirc C} \bigcirc B$
- The terminal type: $\bigcirc \mathbf{1} \simeq \mathbf{1}$

**Why lex matters:** Lex modalities preserve logical structure. If $\bigcirc$ is lex:
- $\bigcirc(A \text{ is a proposition}) \iff \bigcirc A$ is a proposition
- $\bigcirc(\Sigma_{x:A} P(x)) \simeq \Sigma_{x:\bigcirc A} \bigcirc P(x)$ (under conditions)
- Types remain "the same shape" after applying $\bigcirc$

**Examples:**
- Propositional truncation: lex? No — it collapses information in a way that doesn't preserve pullbacks
- $n$-truncation: lex for $n \geq 0$, but not lex for $n = -1$
- Shape modality $\int$: lex (as part of the cohesion axioms)
- Sharp modality $\sharp$: lex (being a right adjoint, it preserves all limits)

## Modalities from Adjunctions

Every adjunction $L \dashv R$ between ∞-toposes gives a modality: the composition $\bigcirc = R \circ L$, with unit $\eta_A : A \to R(L(A))$.

In the cohesive setting:
- The flat modality $\flat = i \circ \Gamma$ where $\Gamma$ is the "global sections" functor and $i$ is the discrete inclusion
- The sharp modality $\sharp = i \circ \text{CoDisc}$ where $\text{CoDisc}$ is the codiscrete functor
- The shape modality $\int = i \circ \Pi$ where $\Pi$ is the geometric realization

The adjunction structure $\int \dashv \flat \dashv \sharp$ encodes the cohesive geometry.

## Modalities and Reflective Subcategories

Modalities correspond exactly to *reflective subcategories* of the ∞-topos.

**Theorem.** Modalities on an ∞-topos $\mathcal{E}$ correspond to full subcategories $\mathcal{E}_\bigcirc \subseteq \mathcal{E}$ that are:
1. *Closed under limits*: if $\{X_i\}$ is a diagram in $\mathcal{E}_\bigcirc$, its limit in $\mathcal{E}$ is in $\mathcal{E}_\bigcirc$
2. *Reflective*: the inclusion $\mathcal{E}_\bigcirc \hookrightarrow \mathcal{E}$ has a left adjoint $\bigcirc : \mathcal{E} \to \mathcal{E}_\bigcirc$

The modal types are the objects of $\mathcal{E}_\bigcirc$.

**Examples:**
- Discrete types: all sets with the discrete topology; $\bigcirc = $ discretization
- Hausdorff spaces: $\bigcirc = $ Hausdorff-ification (the largest Hausdorff quotient)
- Sheaves: $\bigcirc = $ sheafification (in a Grothendieck topology)

## Composing Modalities

Two modalities $\bigcirc_1$ and $\bigcirc_2$ can be composed, giving a modality $\bigcirc_1 \bigcirc_2$. But the composition is not symmetric in general.

**Idempotency:** $\bigcirc \bigcirc \simeq \bigcirc$ (applying the modality twice is the same as once). This is automatic from the universal property.

**The factorization system:** Every modality gives an *orthogonal factorization system* on maps:
- The "$\bigcirc$-connected" maps: maps whose $\bigcirc$-image is trivial
- The "$\bigcirc$-modal" maps: maps into $\bigcirc$-modal fibers

Every map factors as a $\bigcirc$-connected map followed by a $\bigcirc$-modal map, and this factorization is unique up to equivalence.

## Modalities in Lean 4 and Cubical Agda

In Lean 4, modalities can be postulated as axioms or defined from existing constructions:

```lean
-- Propositional truncation (a modality in Lean 4's type system)
-- Via Nonempty
def modal : Type → Prop := Nonempty

-- The unit
def unit : {A : Type} → A → Nonempty A := Nonempty.intro

-- Universal property: maps A → P where P : Prop factor through Nonempty A
def lift {A : Type} {P : Prop} (h : A → P) : Nonempty A → P :=
  fun ⟨a⟩ => h a
```

In Cubical Agda, modalities can be defined more generally:

```agda
{-# OPTIONS --cubical #-}
module Modalities where
open import Cubical.Foundations.Prelude
open import Cubical.HITs.PropositionalTruncation

-- Propositional truncation is a modality
-- Modal types: isProp
-- Unit: ∣_∣₁ : A → ∥ A ∥₁
-- Universal property: rec₁

-- n-Truncation is a modality
open import Cubical.HITs.Truncation
-- ∥ A ∥ n is the n-truncation
-- Modal types: isOfHLevel n
-- Unit: ∣_∣
-- Universal property: rec
```

## Nullification: A Universal Construction

For any type $B$, the *$B$-nullification* $\mathsf{Null}_B(A)$ is the universal type that maps from $A$ and has the property that every map $B \to \mathsf{Null}_B(A)$ is constant.

- $B$-nullification is a modality
- $B$-modal types: types $X$ where every map $B \to X$ is constant
- Unit: $A \to \mathsf{Null}_B(A)$

Special cases:
- $B = S^n$ (the $n$-sphere): $S^n$-nullification makes all $\pi_k$ for $k \leq n$ trivial — this is the Postnikov truncation
- $B = \mathbb{R}$: $\mathbb{R}$-nullification makes all paths factoring through $\mathbb{R}$ trivial — this is related to the flat modality in cohesive HoTT

Nullification is the most general way to produce modalities, and all modalities arising in algebraic topology (localization, completion, etc.) are instances of nullification.
