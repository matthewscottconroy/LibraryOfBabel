# 2.1 Cohesive HoTT

## What is Cohesion?

The word "cohesion" in this context means: spaces that have both a *discrete* (purely set-like) aspect and a *continuous* (topological/smooth) aspect, with the two related in a coherent way.

Classical examples of cohesive structures:
- **Smooth manifolds**: they have an underlying topological space (the homotopy type) but also a smooth structure (differential forms, connections)
- **Topological spaces**: they have an underlying set of points plus a topology
- **Algebraic varieties**: they have an underlying scheme plus an analytic structure

The Lawvere-Schreiber-Shulman axioms for cohesive HoTT capture this structure via modalities. The key insight: the relationship between "continuous" and "discrete" is captured by adjunctions between the cohesive type theory and ordinary set theory (or ∞-groupoid theory).

## The Cohesion Axioms

Cohesive HoTT extends ordinary HoTT with three modalities satisfying the *cohesion axioms*:

**Axiom 25.1 (Cohesion).** There are three modalities with specified adjunctions:

$$\int \quad \dashv \quad \flat \quad \dashv \quad \sharp$$

satisfying:
1. $\int$ (shape) is a left adjoint (it has a right adjoint $\flat$)
2. $\flat$ (flat) is both a left and right adjoint
3. $\sharp$ (sharp) is a right adjoint (it has a left adjoint $\flat$)
4. $\int$ preserves finite products
5. $\flat$ is fully faithful: $\flat A \to A$ is a monomorphism
6. $\sharp$ is fully faithful: $A \to \sharp A$ is a monomorphism

**The unit maps:**
- Shape unit: $\iota : A \to \int A$ (every space maps to its homotopy type)
- Flat counit: $\varepsilon : \flat A \to A$ (every discrete space is a subspace of $A$)
- Sharp unit: $\eta : A \to \sharp A$ (every space maps into its codiscrete version)

**The relationships:**
- $\flat(\int A) \simeq \flat A$ (the discrete points of the shape of $A$ are the same as the discrete points of $A$)
- $\int(\flat A) \simeq \flat A$ (the shape of a discrete space is the same discrete space)

## The Three Modalities

### The Shape Modality $\int$

$\int A$ is the *shape* of $A$ — its underlying homotopy type, forgetting all geometric structure.

**Intuition:** Given a smooth manifold $M$, $\int M$ is the topological space underlying $M$ (no smooth structure), further considered up to homotopy. So $\int \mathbb{R}^n \simeq \mathbf{1}$ (the real $n$-space is contractible), $\int S^1 \simeq S^1$ (the circle's shape is the circle), $\int T^2 \simeq T^2$ (the torus's shape is the torus).

**The shape is the "de Rham fundamental group" approach.** The de Rham theorem says cohomology of a manifold depends only on its homotopy type. The shape modality captures exactly this: $\int M$ is what determines the de Rham cohomology.

**Types with trivial shape.** A type $A$ has contractible shape ($\int A \simeq \mathbf{1}$) if $A$ is "cohesively contractible" — it can be continuously deformed to a point. For example, $\int \mathbb{R} \simeq \mathbf{1}$.

**Shape-modal types.** A type $A$ is *shape-modal* if $\iota : A \to \int A$ is an equivalence. These are types that already "are" their homotopy type — types with no extra smooth/geometric structure beyond their topology. Discrete types are shape-modal.

### The Flat Modality $\flat$

$\flat A$ is the *underlying discrete space* of $A$ — the same elements but with no cohesive structure.

**Intuition:** $\flat \mathbb{R}$ is the set of real numbers with the discrete topology. There are no continuous paths in $\flat \mathbb{R}$ — every path is constant. $\flat \mathbb{R}$ has the same elements as $\mathbb{R}$ but no geometry.

**Theorem.** A type $A$ is *flat-modal* ($\flat A \simeq A$) iff every map $\mathbb{R} \to A$ is null-homotopic (factors through a point).

*Intuition:* A type has no cohesive structure iff it can't detect "continuity from $\mathbb{R}$." Discrete types can't detect continuous deformations.

**The flat counit.** The map $\varepsilon : \flat A \to A$ is the "inclusion of the discrete part." It sends a discrete element (a point with no neighborhood structure) to the corresponding cohesive point.

**Crisp variables.** In the implementation of cohesive HoTT (spatial type theory), variables of type $\flat A$ are called *crisp* — they have no cohesive structure. Working with crisp variables is like working in a purely discrete setting.

### The Sharp Modality $\sharp$

$\sharp A$ is the *codiscrete* type associated to $A$ — a type into which every map is "automatically continuous."

**Intuition:** $\sharp A$ is $A$ with the "indiscrete topology" (every map is continuous). Elements of $\sharp A$ remember all possible structure, making every function into them continuous.

**Theorem.** A type $A$ is *sharp-modal* ($\sharp A \simeq A$) iff $A$ is codiscrete — it has "too much" structure so that all paths in $A$ are automatically trivial.

The sharp modality is harder to have a direct intuition for than $\flat$ or $\int$. Its main role is as the right adjoint to $\flat$: it encodes the "formal neighborhoods" of points.

## The Adjunction Triangle

The three modalities satisfy the adjunction chain $\int \dashv \flat \dashv \sharp$, giving natural maps:

$$\flat A \xrightarrow{\varepsilon} A \xrightarrow{\eta} \sharp A$$
$$A \xrightarrow{\iota} \int A$$

and natural equivalences:
$$\int(\flat A) \simeq \flat A \simeq \flat(\int A)$$

**Geometric picture:** The flat part $\flat A$ is the "points" of $A$ (discrete, no structure). The full $A$ is the "space" (with cohesive structure). The sharp $\sharp A$ is the "all-at-once" version (codiscrete). The shape $\int A$ is the "homotopy type" (forget smooth structure, keep topology).

The natural map $\varepsilon : \flat A \to A$ includes the points into the space. The natural map $\eta : A \to \sharp A$ includes the space into the codiscrete cloud.

## The Discrete and Codiscrete Spectra

The cohesion axioms give a *spectrum* of types between the discrete and codiscrete extremes:

$$\text{Discrete types} \;\subset\; \text{All types} \;\subset\; \text{Codiscrete types}$$

- Discrete types: $\flat$-modal, no cohesive structure
- Codiscrete types: $\sharp$-modal, "maximal" structure

The shape $\int$ sends every type to its "most discrete" equivalent (its homotopy type). The flat $\flat$ sends every type to its "truly discrete" equivalent (forget all non-discrete structure).

## Real-Cohesive HoTT

*Real-cohesive HoTT* (Shulman 2018) is cohesive HoTT with a specific axiom about the real numbers:

**Axiom 25.2 (Real Cohesion).** For any type $A$:
$$A \text{ is } \sharp\text{-modal} \iff \text{every continuous path } [0,1] \to A \text{ is constant}$$

This axiom pins down the cohesive structure to be "generated by the real line." A type is codiscrete (sharp-modal) iff it has no interesting continuous paths indexed by $[0,1]$.

**Consequence.** In real-cohesive HoTT:
- $\int \mathbb{R} \simeq \mathbf{1}$ (the real line is contractible as a cohesive space)
- $\int S^1 \simeq S^1$ (the circle's shape is the HoTT circle)
- $\flat \mathbb{Z} \simeq \mathbb{Z}$ (the integers are already discrete)
- $\pi_1(S^1) = \mathbb{Z}$ follows from the covering space $\mathbb{R} \to S^1 = \mathbb{R}/\mathbb{Z}$

**The circle from the real line.** The HoTT circle $S^1$ appears naturally as the quotient $\mathbb{R}/\mathbb{Z}$ in cohesive HoTT:
$$\int(\mathbb{R}/\mathbb{Z}) \simeq S^1$$
The shape of the "smooth circle" (the quotient of $\mathbb{R}$ by integer translations) is the HoTT circle. This connects the cohesive and synthetic approaches.

## Differential Cohomology

One of the main applications of cohesive HoTT is to *differential cohomology* — the combination of topological and geometric data.

**Setup.** For a cohesive type $A$ and an abelian group $G$:
- $H^n(A, G)$ — the *ordinary cohomology* (depends only on $\int A$)
- $H^n_{dR}(A)$ — the *de Rham cohomology* (differential forms, depends on the smooth structure)
- $\hat{H}^n(A, G)$ — the *differential cohomology* (combines both)

**Definition (differential cohomology group):**
$$\hat{H}^n(A, G) :\equiv H^n(\flat A, G) \times_{H^n(\int A, G)} H^n_{dR}(A)$$

The pullback over the ordinary cohomology group $H^n(\int A, G)$ ensures that the flat part and the de Rham part agree on their underlying topological invariant.

**Theorem (de Rham theorem in cohesive HoTT).** For a smooth type $A$:
$$H^n_{dR}(A) \simeq H^n(\int A, \mathbb{R})$$

This is the de Rham theorem: the de Rham cohomology depends only on the shape.

The proof is almost definitional from the cohesion axioms: the de Rham cohomology is defined to respect the cohesive structure, and the shape $\int A$ captures exactly the homotopy-invariant information.

## The Brouwer Fixed-Point Theorem

One of the showcase theorems of real-cohesive HoTT:

**Theorem (Brouwer, synthetic).** Every continuous function $D^2 \to D^2$ (from the closed unit disk to itself) has a fixed point.

*Proof sketch in cohesive HoTT.*
1. The closed disk $D^2$ is cohesively defined as $\{ x : \mathbb{R}^2 \mid \|x\| \leq 1 \}$.
2. $\int D^2 \simeq \mathbf{1}$ (the disk is contractible).
3. By the cohesion axioms, if $f : D^2 \to D^2$ had no fixed point, we could construct a retraction $D^2 \to S^1$ (the boundary).
4. But $\int D^2 \simeq \mathbf{1}$ means $\pi_1(D^2) = 0$, and a retraction would give an injection $\pi_1(S^1) \hookrightarrow \pi_1(D^2)$, contradicting $\pi_1(S^1) = \mathbb{Z} \neq 0$.

The key: the cohesion axioms and the shape modality give enough geometric structure to run this classical argument synthetically.

## Implementing Cohesive HoTT

The implementation of cohesive HoTT in Cubical Agda uses the `--cohesion` flag (experimental):

```agda
{-# OPTIONS --cohesion --flat-split #-}
module CohesiveHoTT where

-- The flat modality (postulated or via the --cohesion flag)
postulate
  ♭ : {ℓ : Level} → Type ℓ → Type ℓ
  ♭-counit : {ℓ : Level} {A : Type ℓ} → ♭ A → A
  -- ♭-intro requires a "flat/crisp" context

-- The shape modality
postulate
  ∫ : {ℓ : Level} → Type ℓ → Type ℓ
  ∫-unit : {ℓ : Level} {A : Type ℓ} → A → ∫ A
  -- Universal property of ∫: maps ∫ A → B where B is ♭-modal
```

The implementation is ongoing work, as the cohesion axioms require careful treatment of context restrictions (crisp vs. cohesive variables).

## Summary: What Cohesion Adds

Cohesive HoTT adds:
1. **Geometric intuition**: types are spaces with cohesive structure, not just abstract ∞-groupoids
2. **De Rham cohomology**: differential forms and smooth functions have a synthetic definition
3. **Gauge theory**: principal bundles with connections are naturally defined
4. **The Brouwer fixed-point theorem**: a synthetic proof using the shape modality

The cohesion axioms are not contradictory to standard HoTT — every theorem of HoTT remains valid. Cohesive HoTT simply adds more structure, enabling geometric reasoning.
