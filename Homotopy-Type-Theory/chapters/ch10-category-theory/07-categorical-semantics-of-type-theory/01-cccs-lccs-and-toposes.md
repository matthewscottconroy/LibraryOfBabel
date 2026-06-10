# 7.1 Categorical Semantics of Type Theory

## The Semantics Program

Every formal system has models — mathematical objects that satisfy the axioms. For type theory, the models are *categories of a specific kind*. This connection — between type theories and their categorical models — is the *semantics program*, and it's fundamental to understanding why type theory works.

The key correspondences:

| Type Theory | Category |
|---|---|
| Simply typed lambda calculus (STLC) | Cartesian closed category (CCC) |
| Dependent type theory (MLTT) | Locally cartesian closed category (LCCC) |
| Higher-order logic / System F | Topos |
| HoTT | $\infty$-Topos |

These are not just analogies — they're equivalences. Every model of STLC determines a CCC, and every CCC determines a model of STLC. The correspondence is functorial.

## Cartesian Closed Categories (CCCs)

A *cartesian closed category (CCC)* is a category with:
- **Finite products:** Terminal object $\mathbf{1}$ and binary products $A \times B$ with projections and pairing
- **Exponentials:** For each $A$ and $B$, an object $B^A$ (or $[A, B]$) representing the "function type from $A$ to $B$," with evaluation and currying

**Exponential condition:** There's a natural bijection:
$$\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, B^A)$$

This is the product-exponential adjunction $(-) \times A \dashv (-)^A$.

**Examples of CCCs:**
- $\mathbf{Set}$: products are Cartesian products, $B^A = [A, B]$ (function sets)
- $\mathbf{Grp}$: Not a CCC (exponentials don't exist in general for groups)
- Presheaf categories $[\mathcal{C}^{op}, \mathbf{Set}]$: Always CCCs
- Any topos: CCCs (and more)

**The internal language of a CCC is STLC.** More precisely:
- Objects of $\mathcal{C}$ ↔ types of STLC
- Morphisms $A \to B$ ↔ terms $x : A \vdash t : B$ (up to $\beta$/$\eta$ equivalence)
- Products ↔ product types $A \times B$
- Exponentials ↔ function types $A \to B$
- Composition ↔ substitution / term composition
- Identity ↔ variable $x : A \vdash x : A$

Every theorem provable in STLC holds in every CCC, and every universal theorem about CCCs corresponds to a derivable rule in STLC.

## From STLC to Dependent Types: LCCCs

To model dependent types, we need a richer categorical structure. The key insight: dependent types correspond to *slices*.

For a morphism $f : A \to B$ in a category $\mathcal{C}$, the *slice category* $\mathcal{C}/A$ has:
- Objects: morphisms $g : C \to A$ in $\mathcal{C}$ (types "over $A$")
- Morphisms from $(g : C \to A)$ to $(h : D \to A)$: morphisms $k : C \to D$ with $h \circ k = g$

A type family $B : A \to \mathsf{Type}$ corresponds to an object in $\mathcal{C}/A$ — a morphism whose source is the "total space" $\sum_{a:A} B(a)$ and whose "fibers" are the $B(a)$.

**Locally Cartesian Closed Category (LCCC).** A category $\mathcal{C}$ is *locally cartesian closed* if every slice category $\mathcal{C}/A$ is a CCC.

This means: for any $A$, any two "types over $A$" (morphisms $B \to A$ and $C \to A$) have a "function type over $A$" (an exponential in $\mathcal{C}/A$).

**The internal language of an LCCC is dependent type theory.** More precisely:
- Objects of $\mathcal{C}$ ↔ types (or contexts)
- Morphisms $A \to B$ ↔ terms of type $B$ in context $A$
- Slice categories ↔ types in context (type families)
- Products in $\mathcal{C}/A$ ↔ Σ types (dependent pairs)
- Exponentials in $\mathcal{C}/A$ ↔ Π types (dependent functions)
- Substitution (pullback along $f : A \to B$) ↔ substitution in type theory

**Adjoint triple $\Sigma \dashv \Delta \dashv \Pi$:** For a morphism $f : A \to B$:
- $\Sigma_f : \mathcal{C}/A \to \mathcal{C}/B$ (compose with $f$): Σ type
- $f^* = \Delta_f : \mathcal{C}/B \to \mathcal{C}/A$ (pullback along $f$): substitution
- $\Pi_f : \mathcal{C}/A \to \mathcal{C}/B$ (right adjoint): Π type

The adjunctions $\Sigma_f \dashv f^* \dashv \Pi_f$ are the categorical statement of the typing rules for Σ and Π.

## Identity Types Categorically

The identity type $a =_A b$ doesn't directly correspond to an existing categorical notion in LCCCs. It requires additional structure.

One approach: *path objects*. A category has *path objects* if for every $A$, there's an object $\mathsf{Path}(A)$ fitting into a factorization:

$$A \xrightarrow{r} \mathsf{Path}(A) \xrightarrow{(s,t)} A \times A$$

where $r$ is a weak equivalence and $(s, t)$ is a fibration. The path object $\mathsf{Path}(A)$ is the categorical analog of the identity type: its elements are paths (or identity proofs) in $A$.

This factorization system is the essential structure of a *model category* (Quillen), the standard setting for abstract homotopy theory.

**Theorem (Awodey-Warren 2009).** Quillen model categories give models of intensional MLTT. The path objects model identity types; the factorization models the J rule.

This was the first precise connection between homotopy theory (model categories) and type theory (MLTT). It showed that intensional MLTT is naturally interpreted in homotopy-theoretic settings.

## Toposes

A *topos* (plural: toposes or topoi) is a category that shares key properties of $\mathbf{Set}$:

**Definition.** An *elementary topos* is a category $\mathcal{E}$ with:
1. Finite limits
2. Exponentials (so it's a CCC)
3. A *subobject classifier* $\Omega$: an object with a morphism $\mathsf{true} : \mathbf{1} \to \Omega$ such that for every monomorphism $m : A \hookrightarrow B$, there's a unique *classifying map* $\chi_m : B \to \Omega$ with $m = \chi_m^{-1}(\mathsf{true})$

The subobject classifier $\Omega$ plays the role of the "type of propositions": every subobject (subtype, predicate) corresponds to a morphism into $\Omega$.

**Examples of toposes:**
- $\mathbf{Set}$: the archetypal topos. $\Omega = \{\mathsf{true}, \mathsf{false}\}$.
- Presheaf categories $[\mathcal{C}^{op}, \mathbf{Set}]$: always toposes. $\Omega$ is the "sieve" presheaf.
- Sheaves on a topological space $X$: a topos. $\Omega$ is the sheaf of opens.
- Smooth sets (sheaves on smooth manifolds): a topos containing smooth spaces as a full subcategory.

**The internal language of a topos is higher-order intuitionistic logic.** In a topos, you can interpret:
- Types as objects
- Propositions (subtypes) as morphisms into $\Omega$
- $\forall$ and $\exists$ as adjunctions $\Sigma_f \dashv f^* \dashv \Pi_f$
- Equality as the diagonal morphism
- Implication as the exponential on $\Omega$

This is the *Mitchell-Bénabou* language — the internal language of a topos.

**The connection to logic:** Different toposes model different logical principles. $\mathbf{Set}$ (with Boolean $\Omega = \{0, 1\}$) models classical logic. Presheaf toposes model intuitionistic logic without LEM. The "effective topos" (due to Hyland) models constructive/realizability semantics.

## $\infty$-Toposes and HoTT

The semantics of HoTT requires going beyond 1-categorical toposes to *$\infty$-toposes*.

**$\infty$-categories.** An $\infty$-category is a category-like structure where morphisms exist at all levels: objects (0-morphisms), morphisms (1-morphisms), morphisms between morphisms (2-morphisms), and so on, with all morphisms at level $> 1$ being invertible (weakly). This is an $\infty$-groupoid.

The precise definition uses *Kan complexes* (simplicial sets with horn-filling conditions) or other models. The theory of $\infty$-categories was developed by Joyal and Lurie.

**$\infty$-Topos (Lurie 2009).** An *$\infty$-topos* is an $\infty$-category satisfying $\infty$-categorical analogs of the topos axioms: $\infty$-limits, an "object classifier" (the $\infty$-analog of $\Omega$), and descent (the $\infty$-analog of sheaf conditions).

Examples:
- The $\infty$-category of $\infty$-groupoids (Kan complexes): the analog of $\mathbf{Set}$
- The $\infty$-category of presheaves on a small $\infty$-category: analog of presheaf toposes
- The $\infty$-category of sheaves on a site: analog of sheaf toposes

**Theorem (Shulman, Rezk, et al.).** Every $\infty$-topos provides a model of HoTT (with Univalence and HITs). Conversely, HoTT's internal language is the "internal language of $\infty$-toposes."

This is the culmination of the semantics program for HoTT. Every theorem in HoTT holds in every $\infty$-topos. Different $\infty$-toposes model different additional axioms (classical logic, Axiom of Choice, etc.).

## Voevodsky's Simplicial Set Model

Voevodsky's 2006 construction (the first model of HoTT + Univalence) uses the $\infty$-topos of Kan complexes:

- Types ↔ Kan complexes (simplicial sets satisfying horn-filling conditions)
- Terms ↔ vertices/edges of simplicial sets
- Identity type $a = b$ ↔ path space (fibrant replacement of the diagonal)
- Univalence ↔ the universal Kan fibration classifies all Kan fibrations
- HITs ↔ specific constructions in the simplicial model (circle = the simplicial circle $S^1$, etc.)

The verification that Univalence holds in this model is Voevodsky's main theorem. It uses the fact that the "object classifier" in the $\infty$-category of Kan complexes satisfies the univalence condition.

## The Internal Language Correspondence

The precise theorem:

**Theorem (Language-Semantics correspondence).** There's an equivalence between:
- Models of [type theory] (up to equivalence)
- [Categorical structure] (up to equivalence)

| Type Theory | Categorical Structure |
|---|---|
| STLC | CCC |
| Dependent type theory (no universes) | LCCC |
| MLTT (with universes) | LCCC + universe objects |
| HoTT | $\infty$-Topos |

This correspondence is not just a bijection on objects — it's an equivalence of *categories*: the category of models of the type theory (with homomorphisms) is equivalent to the category of the appropriate categorical structures (with appropriate functors).

## Why This Matters

The categorical semantics of type theory is important for several reasons:

**Consistency proofs.** If you can construct a non-trivial model (one where $\mathbf{0}$ is not inhabited), the type theory is consistent. Voevodsky's simplicial set model shows HoTT is consistent.

**Independence proofs.** If an axiom holds in some models and fails in others, it's independent of the base theory. The failure of UIP in the groupoid model shows UIP is independent of intensional MLTT.

**New axioms.** The categorical perspective suggests new axioms. Univalence arises naturally from the $\infty$-topos semantics. Modal HoTT axioms arise from the theory of modalities in $\infty$-toposes.

**Connections to geometry.** Different $\infty$-toposes model different geometric settings (smooth geometry, algebraic geometry, etc.). HoTT statements that hold in all $\infty$-toposes are "synthetic" theorems that apply to all geometric contexts simultaneously.
