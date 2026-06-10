# 25.2 Cohesive HoTT

## The Geometry Hidden in Adjunctions

Every adjunction tells a story. The adjunction $\mathsf{Free} \dashv \mathsf{Forget}$ between free groups and sets tells the story of what algebraic structure you gain by taking free objects, and what you lose by forgetting it. The adjunction between globalization and sheafification tells the story of how local data assembles into global structure.

The adjoint triple $\int \dashv \flat \dashv \sharp$ tells the story of geometry: how a cohesive space relates to its underlying set of points, how its continuous structure relates to its discrete shadow, and how the homotopy type of a space relates to its geometric structure.

Cohesive HoTT is the type theory in which this story is told at the foundational level. By adding three modalities with specified adjoint relationships, we obtain a type theory that naturally speaks the language of differential geometry, gauge theory, and smooth manifolds — without ever mentioning coordinates or transition functions.

## The Cohesion Axioms

Cohesive HoTT extends ordinary HoTT with three modalities and their adjunction data:

**Axiom (Cohesion).** There are modalities $\int, \flat, \sharp$ with unit/counit maps:
$$\varepsilon^\flat : \flat A \to A \quad \text{(flat counit)}$$
$$\eta^\int : A \to \int A \quad \text{(shape unit)}$$
$$\eta^\sharp : A \to \sharp A \quad \text{(sharp unit)}$$

satisfying:
1. **Adjunction $\int \dashv \flat$**: The map $\eta^\int : A \to \int A$ is the unit of an adjunction with right adjoint $\flat$
2. **Adjunction $\flat \dashv \sharp$**: The map $\varepsilon^\flat : \flat A \to A$ is the counit of an adjunction with right adjoint $\sharp$
3. **$\flat$ is fully faithful**: The counit $\varepsilon^\flat : \flat A \to A$ is a monomorphism
4. **$\sharp$ is fully faithful**: The unit $\eta^\sharp : A \to \sharp A$ is a monomorphism
5. **$\int$ preserves finite products**: $\int(A \times B) \simeq \int A \times \int B$
6. **$\flat$ preserves all limits**: (being a right adjoint)
7. **$\int \circ \flat = \flat = \flat \circ \int$**: The discrete points of the shape are the same discrete points (and vice versa)

The last axiom encodes the relationship between the two adjunctions: $\flat$ sits between $\int$ and $\sharp$ in a coherent way.

## The Three Modalities in Detail

### Shape: $\int A$

The *shape* of $A$ is its underlying homotopy type — the space $A$ with all geometric structure forgotten, retaining only topology and homotopy.

**Operational intuition**: $\int A$ is the answer to "what is $A$'s homotopy type?" For a smooth manifold $M$: $\int M$ is the topological space underlying $M$, considered as an ∞-groupoid.

**Key calculations**:
- $\int \mathbb{R}^n \simeq \mathbf{1}$ (the real $n$-space is contractible — its shape has no interesting homotopy)
- $\int S^n \simeq S^n$ (the $n$-sphere's shape is the HoTT $n$-sphere)
- $\int(M \times N) \simeq \int M \times \int N$ (products preserved — from the cohesion axiom)

**Shape-modal types**: A type $A$ is *shape-modal* if $\eta^\int : A \to \int A$ is an equivalence. These are types that already "are" their homotopy type — they have no extra geometric structure beyond topology. Discrete types are shape-modal.

**The counit**: $\flat A \to \int(\flat A) \simeq \flat A$ — the counit of the $\int \dashv \flat$ adjunction, applied to $\flat A$, is an equivalence. Discrete spaces have the same shape as their own underlying set.

### Flat: $\flat A$

The *flat* of $A$ is its underlying discrete space — the same elements but with all geometric structure erased. Points of $\flat A$ are completely isolated.

**Operational intuition**: $\flat A$ is "the points of $A$, with no way to go between them continuously." For $\mathbb{R}$: $\flat \mathbb{R}$ is the set of real numbers with the discrete topology.

**Crisp variables**: In the implementation of cohesive HoTT (*spatial type theory*), variables of type $\flat A$ are called *crisp* — they range over elements of $A$ that have no cohesive structure. A crisp real number is a specific real number, but you can't do smooth analysis with it; it's discrete.

**The counit $\varepsilon^\flat : \flat A \to A$**: This map takes a discrete point and views it as a point in the cohesive space $A$. It is the "inclusion of the discrete points." Not every point of $A$ need be in the image (the topology might have "virtual points" from the sheaf perspective), but the map is always well-defined.

**Flat-modal types**: $A$ is flat-modal ($\flat A \simeq A$) iff every path in $A$ is constant — iff there are no interesting continuous maps $\mathbb{R} \to A$ (in the real-cohesive setting).

### Sharp: $\sharp A$

The *sharp* of $A$ is its codiscrete version — a type into which all maps from all cohesive spaces are automatically continuous.

**Operational intuition**: $\sharp A$ has the *indiscrete topology* — every map is continuous because there are no non-trivial open sets to check continuity against. It is in some sense "maximally geometric": it accepts all possible continuous structure, making every function into it trivially smooth.

**The unit $\eta^\sharp : A \to \sharp A$**: Every cohesive type maps into its codiscrete version. This map is a monomorphism (fully faithful).

**Sharp-modal types**: $A$ is sharp-modal ($\sharp A \simeq A$) iff $A$ is codiscrete — every map from any cohesive space to $A$ is automatically continuous. In the real-cohesive setting, this means every $\mathbb{R}$-path in $A$ is constant.

**The deep asymmetry**: $\flat$ and $\sharp$ are dual in a precise sense. $\flat A$ has "fewer" paths (the paths that are actually present in the discrete shadow). $\sharp A$ has "more" paths (everything maps in continuously). $A$ itself is in between.

## The Adjoint Triple as a Geometric Story

The three modalities and their adjunctions form a *triple* $\int \dashv \flat \dashv \sharp$. Here is the geometric story this triple tells:

$$\flat A \xrightarrow{\;\varepsilon^\flat\;} A \xrightarrow{\;\eta^\sharp\;} \sharp A$$
$$A \xrightarrow{\;\eta^\int\;} \int A$$

Starting from a cohesive space $A$:
- $\varepsilon^\flat : \flat A \to A$ includes the discrete points into the space
- $\eta^\sharp : A \to \sharp A$ includes the space into its codiscrete version
- $\eta^\int : A \to \int A$ maps the space to its homotopy type

The relationships:
- $\int(\flat A) \simeq \flat A$: the shape of a discrete space is itself (discrete things have no interesting homotopy beyond their set-level structure)
- $\flat(\int A) \simeq \flat A$: the discrete points of the shape of $A$ are the same as the discrete points of $A$ (shape doesn't create new points)

These coherences make the adjoint triple into a well-behaved geometric system.

## Real-Cohesive HoTT

The abstract cohesion axioms admit many models. To specify differential geometry as opposed to, say, algebraic geometry or topological dynamics, we add the *real cohesion axiom*:

**Axiom (Real Cohesion, Shulman 2018).** A type $A$ is $\sharp$-modal (codiscrete) if and only if every map $\mathbb{R} \to A$ is null-homotopic.

Equivalently: the $\sharp$-modal types are exactly the types that cannot "detect" continuity from the real line.

**What this pins down**: The real cohesion axiom says that the cohesive structure of our type theory is *generated by $\mathbb{R}$*. A type has non-trivial cohesive structure iff it has non-trivial $\mathbb{R}$-valued continuous functions (or, equivalently, non-constant $\mathbb{R}$-paths).

**Consequences** (in real-cohesive HoTT):
- $\int \mathbb{R} \simeq \mathbf{1}$: the real line is contractible as a cohesive space (it can be continuously retracted to a point — via the homotopy $(r, t) \mapsto r(1-t)$)
- $\int S^1 \simeq S^1$: the smooth circle has the same shape as the HoTT circle
- $\pi_1^{\mathsf{cohesive}}(S^1) = \mathbb{Z}$: the fundamental group of the smooth circle is $\mathbb{Z}$, proved using the covering $\mathbb{R} \to S^1$

The last result is particularly striking. The classical proof that $\pi_1(S^1) = \mathbb{Z}$ requires constructing the universal cover of $S^1$ (the real line, winding around) and computing its fiber. In real-cohesive HoTT, this proof becomes almost definitional: $S^1 = \mathbb{R}/\mathbb{Z}$ (the circle is the quotient of $\mathbb{R}$ by integer translations), $\int \mathbb{R} \simeq \mathbf{1}$, and the long exact sequence of the fibration gives $\pi_1(S^1) = \mathbb{Z}$.

## Spatial Type Theory (Crisp Variables)

The practical implementation of cohesive HoTT requires distinguishing between *cohesive* and *crisp* variables:

- A *cohesive* variable $x : A$ ranges over elements of the cohesive space $A$ — it has access to the full geometric structure
- A *crisp* variable $x :: A$ (notation from spatial type theory) ranges over elements of $\flat A$ — it is a "discrete" element, with no cohesive structure

The distinction is necessary because some constructions require variables to be crisp (discrete) — they make sense only for elements that don't depend on the cohesive structure.

**Crisp elimination**: The main rule for the $\flat$ modality requires a crisp input:
$$\frac{\Gamma \mid \Delta, x :: A \vdash t : B}{\Gamma \mid \Delta \vdash \mathsf{flat.elim}(t) : \flat A \to B}$$
where the left of $\mid$ is the crisp context and the right is the cohesive context.

In Cubical Agda with `--flat-split`, crisp variables are marked with `@♭`:
```agda
{-# OPTIONS --cohesion --flat-split #-}
-- Crisp variable: x is an element of ♭ A
crisp-example : ∀ {@♭ ℓ : Level} {@♭ A : Type ℓ} → ♭ A → A
crisp-example (@♭ x) = ♭-counit x
```

## The Brouwer Fixed-Point Theorem Synthetically

One of the showcase results of real-cohesive HoTT is a synthetic proof of the Brouwer fixed-point theorem:

**Theorem.** Every continuous function $f : D^2 \to D^2$ (the closed unit disk to itself) has a fixed point.

**Synthetic proof sketch.**
1. Define $D^2 = \{ x : \mathbb{R}^2 \mid \|x\| \leq 1 \}$ as a cohesive type.
2. Since $\mathbb{R}^2$ is contractible as a cohesive space ($\int \mathbb{R}^2 \simeq \mathbf{1}$), and $D^2 \subseteq \mathbb{R}^2$ is also contractible, we have $\int D^2 \simeq \mathbf{1}$.
3. The boundary $\partial D^2 = S^1$ has shape $\int S^1 \simeq S^1$ (the HoTT circle), with $\pi_1(S^1) = \mathbb{Z} \neq 0$.
4. If $f$ had no fixed point, we could construct a continuous retraction $r : D^2 \to \partial D^2$ (by taking the ray from $f(x)$ through $x$ to the boundary).
5. By functoriality of $\int$, $\int r : \int D^2 \to \int \partial D^2$ would be a retraction of $\int(S^1)$ from $\mathbf{1}$ — giving a surjection $\mathbf{1} \to S^1$.
6. But $\pi_1(\mathbf{1}) = 0 \neq \mathbb{Z} = \pi_1(S^1)$, so the surjection is impossible.
7. Contradiction. So $f$ has a fixed point. $\square$

The proof is synthetic: it uses only the cohesion axioms and the real cohesion axiom, without ever mentioning triangulations, simplicial approximations, or analytic topology. The geometry is internal to the type theory.

## Models of Cohesive HoTT

The cohesion axioms have multiple models:

**Smooth sets**: The topos of smooth sets (sheaves on the site of Cartesian spaces with smooth maps) gives a model. Types are sheaves; the shape $\int$ sends a smooth space to its geometric realization; $\flat$ sends a smooth space to its underlying discrete set.

**Smooth ∞-groupoids**: The ∞-topos of smooth ∞-groupoids (∞-sheaves on Cartesian spaces) is the primary model for Schreiber's physics program. Types are smooth ∞-groupoids; the shape is the ∞-categorical geometric realization.

**Synthetic differential geometry**: The well-adapted models for synthetic differential geometry (models satisfying the Kock-Lawvere axioms) satisfy the cohesion axioms with appropriate additional structure.

**Algebraic models**: The topos of algebraic spaces (schemes) or the ∞-topos of derived algebraic spaces gives an algebraic cohesion. The shape $\int$ would be the étale homotopy type; $\flat$ would be the underlying discrete algebra.

The existence of multiple models ensures that cohesive HoTT is not a theory about one specific geometric setting, but a general framework applicable wherever there is a meaningful distinction between "cohesive structure" and "discrete underlying set."
