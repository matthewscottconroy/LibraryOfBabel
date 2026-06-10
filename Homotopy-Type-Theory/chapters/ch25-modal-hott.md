# Chapter 25: Modal Homotopy Type Theory — Cohesion and Geometry

## Introduction

Standard HoTT lives in an abstract mathematical world: every type is an ∞-groupoid, paths are paths, and there is no notion of "continuous" versus "discrete." The type theory cannot distinguish between a function from $\mathbb{R}$ to $\mathbb{R}$ that is continuous and one that is not — or rather, every function is "automatically" continuous in the sense that it is a morphism of types.

*Modal HoTT* extends homotopy type theory with *modalities* — operators on types that capture additional geometric or logical structure. The most important example is *cohesive HoTT*, introduced by Urs Schreiber and Mike Shulman, which adds modalities capturing the *cohesive* structure of spaces: the distinction between points and paths, between discrete and continuous structures, between local and global.

This chapter introduces modalities in type theory, the specific modalities of cohesive HoTT (flat $\flat$, sharp $\sharp$, and shape $\int$), and applications to geometry, physics, and differential cohomology.

---

## 1. Modalities in Type Theory

### 1.1 What is a Modality?

A *modality* in type theory is a type-level operator $\bigcirc$ together with:
- A *unit*: a natural map $\eta_A : A \to \bigcirc A$
- A *universal property*: maps $A \to B$ factor through $\bigcirc A$ when $B$ is "$\bigcirc$-modal"

**Definition 25.1 (Modality).** A *modality* $(\bigcirc, \eta)$ consists of:
1. For each type $A$, a type $\bigcirc A$ and a map $\eta_A : A \to \bigcirc A$
2. For each $\bigcirc$-modal type $B$ (a type satisfying $\bigcirc B \simeq B$), every map $f : A \to B$ factors uniquely through $\eta_A$:
   $$A \xrightarrow{\eta_A} \bigcirc A \xrightarrow{\exists! \bar{f}} B$$

This says: $\bigcirc A$ is the "best approximation" of $A$ that is $\bigcirc$-modal.

### 1.2 Examples of Modalities

**Propositional truncation:** $\bigcirc A = \|A\|$ (Chapter 17). Modal types are mere propositions. The unit $A \to \|A\|$ is the constructor. The universal property: any map $A \to P$ where $P$ is a proposition factors through $\|A\|$.

**$n$-Truncation:** $\bigcirc A = \|A\|_n$. Modal types are $n$-types.

**Double negation:** $\bigcirc A = \neg\neg A$ (in classical logic). Modal types satisfy $\neg\neg B \to B$.

**Localizations:** Given a map $f : A \to B$, the localization at $f$ makes $f$ into an equivalence.

### 1.3 Left Exact Modalities

The most natural modalities for geometry are *left exact* (or *lex*) modalities: those that preserve finite limits (in particular, preserve pullbacks).

**Definition 25.2 (Lex Modality).** A modality $\bigcirc$ is *left exact* if for any pullback square:
$$\begin{array}{ccc} A \times_C B & \to & B \\ \downarrow & & \downarrow \\ A & \to & C \end{array}$$

the induced map $\bigcirc(A \times_C B) \to \bigcirc A \times_{\bigcirc C} \bigcirc B$ is an equivalence.

Lex modalities are important because they preserve the logical structure of types: the modal version of a proposition is still a proposition, and so on.

### 1.4 Modalities from Reflective Subcategories

In the ∞-topos model, a modality corresponds to a *reflective subcategory* $\mathcal{E}_\bigcirc \subseteq \mathcal{E}$. The unit $\eta_A : A \to \bigcirc A$ is the *reflection* (the universal map from $A$ to the modal subcategory).

Examples:
- Sheaves in a Grothendieck topology form a reflective subcategory of presheaves.
- Hausdorff spaces form a reflective subcategory of topological spaces.

---

## 2. Cohesive HoTT

### 2.1 Cohesive ∞-Toposes

A *cohesive ∞-topos* is an ∞-topos $\mathcal{H}$ equipped with a string of adjunctions:

$$\Pi \dashv \flat \dashv \sharp$$

where:
- $\Pi : \mathcal{H} \to \infty\mathsf{Grpd}$ is the *shape* functor (geometric realization)
- $\flat : \infty\mathsf{Grpd} \to \mathcal{H}$ is the *flat* (discrete) embedding
- $\sharp : \infty\mathsf{Grpd} \to \mathcal{H}$ is the *sharp* (codiscrete) embedding

The cohesion conditions:
1. $\Pi \dashv \flat$: every cohesive object maps to its discrete shell
2. $\flat \dashv \sharp$: discrete structures embed in codiscrete structures
3. $\Pi$ preserves finite products
4. The counit $\epsilon : \flat \Pi X \to X$ is a monomorphism (discrete spaces are subsets of cohesive ones)

**Physical intuition:** The objects of $\mathcal{H}$ are "cohesive" spaces — they have both a topological/smooth structure and an underlying set. The functors $\Pi$, $\flat$, and $\sharp$ extract or impose these structures.

### 2.2 The Cohesion Axioms in Type Theory

Schreiber and Shulman axiomatize cohesion internally in HoTT. The theory is called *cohesive HoTT* or *real-cohesive HoTT* (in the model of real numbers).

**Axiom 25.3 (Cohesion).** The type theory has three additional modalities:
- $\int A$ (shape / "pure homotopy type of $A$"): the shape of $A$
- $\flat A$ (flat / discrete): the underlying discrete space of $A$
- $\sharp A$ (sharp / codiscrete): the codiscrete space associated to $A$

together with adjunctions $\int \dashv \flat \dashv \sharp$ and the usual cohesion conditions.

**The unit maps:**
- $\iota : A \to \int A$ — every cohesive space maps to its shape
- $\varepsilon : \flat A \to A$ — every discrete space maps into the corresponding cohesive space

### 2.3 What the Modalities Capture

**Shape $\int A$:** The shape of a cohesive space is its "homotopy type" — what you get by contracting all cohesive structure down to just topology. For the real line $\mathbb{R}$:
- $\int \mathbb{R}$ is the homotopy type of $\mathbb{R}$ = contractible = $\mathbf{1}$

For the circle $S^1$ (as a cohesive space, i.e., with its smooth structure):
- $\int S^1$ is the homotopy type of $S^1$ = the usual $S^1$ in HoTT

The shape modality "forgets" the smooth/geometric structure and retains only the homotopy type.

**Flat $\flat A$:** The flat modality gives the "underlying discrete space." An element of $\flat A$ is an element of $A$ with no cohesive structure — a point in $A$ with no neighborhood or continuity conditions.

A function $\flat A \to B$ is a function that only depends on the *value* of an element of $A$, not on its smooth/cohesive structure. This is what it means for a function to be "constant on connected components."

**Sharp $\sharp A$:** The sharp modality gives the "codiscrete" space — every map into $\sharp A$ is continuous. An element of $\sharp A$ is "maximally codiscrete" — it remembers all structure.

---

## 3. Differential Cohomology via Modal HoTT

### 3.1 Recovering Differential Geometry

A major application of cohesive HoTT is to recover differential geometry synthetically. The key insight (going back to Lawvere's synthetic differential geometry) is:

> **The tangent space at a point $x$ of a smooth type $A$ is the type of infinitesimal paths at $x$.**

In cohesive HoTT (with suitable axioms about infinitesimals), the tangent space $T_x A$ is defined internally, and differential forms arise as maps from tangent spaces.

### 3.2 De Rham Cohomology

In classical differential geometry, de Rham cohomology $H^n_{dR}(M, \mathbb{R})$ of a smooth manifold $M$ is computed from differential forms using exterior differentiation.

In cohesive HoTT:
- The de Rham complex of a smooth type $A$ is a chain complex of types
- The cohomology $H^n_{dR}(A)$ is defined as a type
- The de Rham theorem ($H^n_{dR}(A) \cong H^n(A, \mathbb{R})$) becomes a theorem in cohesive HoTT

**The shape interpretation:** The de Rham theorem says that de Rham cohomology depends only on the shape of a manifold:
$$H^n_{dR}(A) \simeq H^n(\int A, \mathbb{R})$$

In cohesive HoTT, this is almost definitional: $\int A$ is the shape, and cohomology of $\int A$ is the homotopy-theoretic cohomology.

### 3.3 Differential Cohomology Groups

*Differential cohomology* refines ordinary cohomology by mixing topological data (homotopy classes) with geometric data (differential forms). In cohesive HoTT, differential cohomology groups are defined as homotopy pullbacks:

$$\hat{H}^n(A) :\equiv H^n(\flat A) \times_{H^n(\int A)} H^n_{dR}(A)$$

where $H^n(\flat A)$ is the "flat cohomology" (discrete, no geometry) and $H^n_{dR}(A)$ is the de Rham cohomology.

This definition recovers the classical Cheeger-Simons differential cohomology groups synthetically.

---

## 4. The Flat and Sharp Modalities in Practice

### 4.1 Flat Types and Constant Maps

A type $A$ is *flat-modal* (or *discrete*) if $\flat A \simeq A$ — the underlying discrete space is equivalent to $A$ itself. This means $A$ has no cohesive structure: it is already "maximally discrete."

**Examples:**
- $\mathbb{N}$, $\mathbb{Z}$, $\mathbb{Q}$ (with their discrete topologies) are flat-modal.
- $\mathbb{R}$ (with the usual smooth structure) is *not* flat-modal.
- Every ordinary set (h-set with no extra structure) is flat-modal.

**Theorem 25.4.** A type $A$ is flat-modal iff every map $\mathbb{R} \to A$ is constant.

*Intuition:* A type has no cohesive structure iff it has no "continuous deformations" — every path in $A$ is constant.

### 4.2 Sharp Types and Codiscrete Spaces

A type $A$ is *sharp-modal* (or *codiscrete*) if $\sharp A \simeq A$. Codiscrete spaces have "too much" structure — every map into them is continuous.

**Theorem 25.5 (Codiscrete Types are ∞-Groupoids).** Every sharp-modal type $A$ is a codiscrete ∞-groupoid: the identity types of $A$ are all contractible.

*Intuition:* Codiscrete spaces are "totally disconnected" in a strong sense — there are no nontrivial paths.

### 4.3 The Adjunction Triangle

The three modalities form a triangle of adjunctions:

$$\int \dashv \flat \dashv \sharp$$

with natural maps:
$$\flat A \xrightarrow{\varepsilon} A \xrightarrow{\eta} \sharp A$$

and the shape map:
$$A \xrightarrow{\iota} \int A$$

These are related by: $\int(\flat A) \simeq \flat A$ (the shape of a discrete space is itself) and $\flat(\int A) \simeq \flat A$ (the flat of a shape is still flat).

---

## 5. Real-Cohesive HoTT

### 5.1 The Axiom of Real Cohesion

*Real-cohesive HoTT* (Shulman 2018) is the specific version of cohesive HoTT with the axiom that the real numbers $\mathbb{R}$ generate the cohesion:

**Axiom 25.6 (Axiom of Real Cohesion).** For every type $A$ and every function $f : \mathbb{R} \to A$, if $A$ is $\sharp$-modal, then $f$ is constant.

This axiom says: codiscrete types are those with no non-trivial "continuous paths" indexed by $\mathbb{R}$.

**Theorem 25.7 (Brouwer Fixed Point Theorem, synthetic).** In real-cohesive HoTT, every continuous function $D^2 \to D^2$ (where $D^2$ is the unit disk, defined as a cohesive type) has a fixed point.

This follows from the cohesive structure: the shape $\int D^2$ is contractible (the disk is contractible), and Brouwer's theorem is a statement about the shape.

### 5.2 Recovering the Fundamental Group of $S^1$

In real-cohesive HoTT, the shape of the real line modulo $\mathbb{Z}$ is the circle:

$$\int(\mathbb{R}/\mathbb{Z}) \simeq S^1$$

and the fundamental group of $S^1$ in cohesive HoTT is:

$$\pi_1(S^1, \mathsf{base}) = \mathbb{Z}$$

But now this is proved differently: $\pi_1(S^1)$ is computed from the cohesive structure of $\mathbb{R}$, using the adjunction between $\int$ and $\flat$.

**The covering space perspective:** The universal covering space $\mathbb{R} \to \mathbb{R}/\mathbb{Z} = S^1$ is a cohesive construction — $\mathbb{R}$ is the "universal flat cover" of the circle. The monodromy is $\pi_1(S^1) = \pi_0(\flat \mathbb{R}) = \mathbb{Z}$ (the flat points of $\mathbb{R}$ are the integers).

---

## 6. Differential Cohesion

### 6.1 Infinitesimals and the Jet Bundle

*Differential cohesion* extends cohesive HoTT with additional modalities capturing infinitesimal structure:

**The infinitesimal shape modality $\Im$:** For a smooth type $A$, $\Im A$ is $A$ "with all finite-distance structure collapsed," retaining only infinitesimal neighborhoods. An element of $\Im A$ is an equivalence class of elements of $A$ that agree to all finite orders.

**The de Rham shape modality $\mathcal{R}$:** $\mathcal{R} A$ is the "de Rham stack" of $A$, where all infinitesimal paths in $A$ are collapsed.

These modalities enable the synthetic definition of:
- Jet bundles $J^\infty_A B$ (formal neighborhood of the diagonal in $A \times A$)
- Crystals (de Rham spaces)
- D-modules (sheaves on the de Rham stack)

### 6.2 The Jet Monad

In differential cohesion, the *jet monad* $\mathcal{J}^\infty$ sends a bundle $E \to M$ to its "∞-jet bundle" $J^\infty E \to M$. A section of $J^\infty E$ is a formal solution to a PDE.

The connection between differential cohesion and PDEs:
- A PDE on sections of $E \to M$ defines a sub-bundle $\mathcal{S} \subseteq J^\infty E$.
- A formal solution is a section of $\mathcal{S}$.
- The question of integrability (do formal solutions lift to actual solutions?) is a cohesion question: does $\flat \mathcal{S} \to \mathcal{S}$ surject?

This gives a *synthetic* theory of PDEs using the modalities of differential cohesion.

---

## 7. Applications to Physics

### 7.1 Gauge Theory and Principal Bundles

In physics, gauge theories are described by *principal bundles with connection*. In cohesive HoTT:

**Principal $G$-bundle on $M$:** A map $M \to \mathbf{B}G$ where $\mathbf{B}G :\equiv \mathsf{BAut}(G)$ is the *classifying type* of $G$-bundles.

**Principal bundle with connection:** A lift to $\mathbf{B}G_\nabla$ (the cohesive classifying type, not just the discrete one). The difference between $\mathbf{B}G$ and $\mathbf{B}G_\nabla$ is precisely the *flat cohomology* — connections are the extra cohesive structure.

**The moduli stack of connections:** The type of all principal $G$-bundles with connection on $M$ is:
$$\mathbf{Conn}_G(M) :\equiv (M \to \mathbf{B}G_\nabla)$$

This is automatically a homotopy type (an ∞-groupoid), whose:
- Objects are connections
- Morphisms are gauge transformations
- Higher morphisms are gauge-of-gauge transformations

### 7.2 Chern-Weil Theory

*Chern-Weil theory* computes the characteristic classes of a principal bundle from the curvature of a connection. In cohesive HoTT:

The Chern character is a map:
$$\mathsf{ch} : (M \to \mathbf{B}G_\nabla) \to \Pi_{n}\, \hat{H}^{2n}(M)$$

from connections to differential cohomology classes. The cohesive structure makes this map natural and well-defined without choosing local coordinates.

**Theorem 25.8 (Chern-Weil in cohesive HoTT).** For any compact manifold $M$ and Lie group $G$, the Chern-Weil homomorphism:
$$\mathsf{ch} : \pi_0(\mathbf{Conn}_G(M)) \to \bigoplus_n H^{2n}_{dR}(M)$$

is natural in $M$ and $G$.

This follows from the functoriality of the cohesive modalities and the natural transformation $\int \to \flat$ (shape maps naturally to flat).

### 7.3 String Theory and Higher Gauge Theory

Higher gauge theories — where the gauge field is a 2-form (as in B-field in string theory) or a 3-form (as in M-theory) — require *higher principal bundles*: principal ∞-bundles with ∞-connection.

In cohesive HoTT:
- A 2-form gauge field is a map $M \to \mathbf{B}^2 U(1)_\nabla$ (classifying type for circle 2-bundles with connection).
- A 3-form gauge field is a map $M \to \mathbf{B}^3 U(1)_\nabla$.

These higher gauge theories are defined *synthetically* in cohesive HoTT — the underlying ∞-groupoid structure is built into the type theory.

---

## 8. Pyknotic and Condensed Objects

### 8.1 The Condensed Mathematics Program

*Condensed mathematics* (Scholze-Clausen, 2019) is a new foundation for algebra and geometry that replaces topological spaces with *condensed sets* — sheaves on the site of profinite sets. This framework handles algebraic objects with topological structure better than classical approaches.

In the homotopy-theoretic setting, condensed objects become *pyknotic objects* (Barwick-Haine) — sheaves of ∞-groupoids on the site of compact Hausdorff spaces.

### 8.2 Pyknotic Types as a Modal HoTT

*Pyknotic type theory* (Anel, 2022) proposes a version of modal HoTT where the modality captures the condensed structure:

**The pyknotic modality $\mathbb{P}$:** For a type $A$, $\mathbb{P}A$ is the "pyknotic completion" — the "condensed" version of $A$ that knows about all compact Hausdorff spaces mapping into it.

This approach aims to bring the benefits of condensed mathematics into the synthetic type-theoretic setting.

---

## 9. Implementing Modal HoTT

### 9.1 The Agda Implementation

Several aspects of modal HoTT have been implemented in Cubical Agda:

```agda
-- The sharp modality (codiscrete types)
postulate
  ♯ : (A : Type) → Type
  ♯-unit : {A : Type} → A → ♯ A
  ♯-elim : {A B : Type} → isCodeiscrete B → (A → B) → ♯ A → B

-- The flat modality (discrete types)
postulate
  ♭ : (A : Type) → Type
  ♭-counit : {A : Type} → ♭ A → A
  ♭-intro : {A : Type} → A → ♭ A  -- only valid for flat types
```

The challenge: the flat modality $\flat$ requires a *counit* (not a unit), and its introduction rule is only valid when the context is "flat" — a non-standard context restriction.

### 9.2 The Spatial Type Theory Approach

*Spatial type theory* (Sterling-Harper, 2021) is a more syntactically careful approach to modal type theory that handles context restrictions properly. It distinguishes between:
- **Cohesive contexts**: the usual HoTT contexts
- **Crisp contexts**: contexts where all variables are flat (discrete)

A variable $x :: A$ (with double colon) is *crisp* — it can be used in a flat context. A variable $x : A$ is *cohesive* — it has full cohesive structure.

This distinction allows the flat modality to be introduced properly: $\flat A$ is the type of crisp elements of $A$.

---

## Exercises

**25.1.** Show that propositional truncation $\| - \|$ is a modality. Identify the unit map, the modal types, and the universal property. Is it left exact?

**25.2.** In cohesive HoTT with the real line $\mathbb{R}$, explain why $\int \mathbb{R} \simeq \mathbf{1}$ (the shape of $\mathbb{R}$ is contractible). Use the cohesion axioms.

**25.3.** Show that the adjunction $\int \dashv \flat$ implies: for any cohesive type $A$, the space of maps $\int A \to B$ (where $B$ is discrete) is equivalent to the space of maps $A \to \flat B$.

**25.4.** Define the *moduli type* of flat $G$-bundles on a cohesive type $M$ as $M \to \mathbf{B}G$. What is the homotopy type of this space? (*Hint:* Use the shape modality to compute $\pi_0$ and $\pi_1$.)

**25.5.** In the de Rham cohomology application: explain why $\int(\mathbb{R}/\mathbb{Z}) \simeq S^1$. What is the cohesive structure of $\mathbb{R}/\mathbb{Z}$ that gets captured by the shape?

**25.6.** The adjunction $\flat \dashv \sharp$ gives a map $\flat A \to \sharp A$ for every type $A$. Describe this map geometrically: what does it do to the cohesive structure?

**25.7 (Research).** Read Schreiber-Shulman, "Quantum Gauge Field Theory in Cohesive Homotopy Type Theory" (2012). Identify how the cohesive modalities are used to define the moduli stack of gauge fields. What is the role of the flat modality in defining "flat connections"?
