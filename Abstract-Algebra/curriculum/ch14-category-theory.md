# Chapter 14 — Category Theory

**Part III: Abstract Algebra**
*Prerequisites: [Chapter 12](ch12-module-theory.md), [Chapter 13](ch13-field-theory-galois.md)*
*Next: [Chapter 15 — Complexes, Homology, and Exact Sequences](ch15-homological-algebra-basics.md)*

---

## Learning Objectives

- Define categories, functors, and natural transformations
- Understand representable functors and the Yoneda lemma
- Define adjoint functors and recognize them throughout algebra
- Understand limits and colimits as universal constructions
- Recognize categories as the language that makes "sameness" precise
- Apply categorical thinking to unify constructions across algebra

---

## 14.1 Categories

### 14.1.1 Definition

A **category** $\mathcal{C}$ consists of:
- A collection of **objects**: $\mathrm{ob}(\mathcal{C})$
- For each pair $A, B \in \mathrm{ob}(\mathcal{C})$, a set of **morphisms** $\mathrm{Hom}_\mathcal{C}(A, B)$
- **Composition**: $\mathrm{Hom}(B, C) \times \mathrm{Hom}(A, B) \to \mathrm{Hom}(A, C)$, $(g, f) \mapsto g \circ f$
- **Identity morphisms**: $\mathrm{id}_A \in \mathrm{Hom}(A, A)$

Satisfying:
- **Associativity:** $h \circ (g \circ f) = (h \circ g) \circ f$
- **Identity laws:** $f \circ \mathrm{id}_A = f$ and $\mathrm{id}_B \circ f = f$

### 14.1.2 Examples

| Category | Objects | Morphisms |
|----------|---------|-----------|
| **Set** | Sets | Functions |
| **Grp** | Groups | Group homomorphisms |
| **Ab** | Abelian groups | Group homomorphisms |
| **Ring** | Rings | Ring homomorphisms |
| **$R$-Mod** | Left $R$-modules | $R$-linear maps |
| **Vect$_F$** | $F$-vector spaces | Linear maps |
| **Top** | Topological spaces | Continuous maps |
| **$G$** (a group) | One object $\bullet$ | Elements of $G$ |
| **$P$** (a poset) | Elements of $P$ | Unique arrow $a \to b$ if $a \leq b$ |

### 14.1.3 Special Morphisms

- **Monomorphism (mono):** $f: A \to B$ with $f \circ g = f \circ h \Rightarrow g = h$ (left-cancellable). Generalizes injections.
- **Epimorphism (epi):** Right-cancellable. Generalizes surjections.
- **Isomorphism:** $f$ with a two-sided inverse $f^{-1}$.

**Warning:** In **Ring**, the inclusion $\mathbb{Z} \hookrightarrow \mathbb{Q}$ is an epimorphism but not surjective.

---

## 14.2 Functors

### 14.2.1 Definition

A **functor** $F: \mathcal{C} \to \mathcal{D}$ assigns:
- To each object $A \in \mathcal{C}$: an object $F(A) \in \mathcal{D}$
- To each morphism $f: A \to B$: a morphism $F(f): F(A) \to F(B)$

Preserving:
- $F(\mathrm{id}_A) = \mathrm{id}_{F(A)}$
- $F(g \circ f) = F(g) \circ F(f)$

This is a **covariant** functor. A **contravariant** functor reverses arrows: $F(f): F(B) \to F(A)$.

### 14.2.2 Examples

| Functor | Domain | Codomain | What it does |
|---------|--------|----------|--------------|
| Free group $F$ | **Set** | **Grp** | $S \mapsto $ free group on $S$ |
| Forgetful $U$ | **Grp** | **Set** | $(G, \cdot) \mapsto G$ (forget structure) |
| Abelianization | **Grp** | **Ab** | $G \mapsto G/[G,G]$ |
| $- \otimes_R M$ | $R$-**Mod** | $R$-**Mod** | Tensor with $M$ |
| $\mathrm{Hom}_R(M, -)$ | $R$-**Mod** | **Ab** | Hom from $M$ |
| $\pi_1$ | **Top** | **Grp** | Fundamental group |

### 14.2.3 Faithful and Full Functors

- $F$ is **faithful** if each $\mathrm{Hom}(A,B) \to \mathrm{Hom}(FA, FB)$ is injective
- $F$ is **full** if each is surjective
- $F$ is **fully faithful** if each is bijective

---

## 14.3 Natural Transformations

### 14.3.1 Definition

Given functors $F, G: \mathcal{C} \to \mathcal{D}$, a **natural transformation** $\eta: F \Rightarrow G$ assigns to each object $A \in \mathcal{C}$ a morphism $\eta_A: F(A) \to G(A)$ such that for every $f: A \to B$:

$$G(f) \circ \eta_A = \eta_B \circ F(f)$$

(The diagram commutes: the square with $\eta_A, \eta_B, F(f), G(f)$ commutes.)

If each $\eta_A$ is an isomorphism, $\eta$ is a **natural isomorphism** ($F \cong G$).

### 14.3.2 Why Natural Transformations Matter

Natural transformations capture "canonical" maps — maps that don't depend on arbitrary choices.

**Example:** The double-dual map $V \to V^{**}$ is natural (no basis required), but the isomorphism $V \cong V^*$ is not (requires a choice of inner product or basis).

**Functors, natural transformations, and categories form a 2-category.** The collection of all functors $\mathcal{C} \to \mathcal{D}$ is itself a category $[\mathcal{C}, \mathcal{D}]$ with natural transformations as morphisms.

---

## 14.4 The Yoneda Lemma

### 14.4.1 Representable Functors

For each $A \in \mathcal{C}$, define the **hom-functor**:
$$\mathrm{h}^A = \mathrm{Hom}_\mathcal{C}(A, -): \mathcal{C} \to \mathbf{Set}, \quad B \mapsto \mathrm{Hom}_\mathcal{C}(A, B)$$

A functor $F: \mathcal{C} \to \mathbf{Set}$ is **representable** if $F \cong \mathrm{h}^A$ for some $A$.

### 14.4.2 Yoneda Lemma

**Lemma (Yoneda):** For any functor $F: \mathcal{C} \to \mathbf{Set}$ and any object $A$:
$$\mathrm{Nat}(\mathrm{h}^A, F) \cong F(A)$$

Natural transformations from $\mathrm{h}^A$ to $F$ correspond bijectively to elements of $F(A)$, and this bijection is natural in both $A$ and $F$.

**Yoneda embedding:** The functor $\mathcal{C} \to [\mathcal{C}^{\mathrm{op}}, \mathbf{Set}]$, $A \mapsto \mathrm{h}^A$, is fully faithful. Every category embeds fully faithfully into a category of presheaves.

**Philosophy:** An object is determined by all the maps into it (or out of it). To understand $A$, understand $\mathrm{Hom}(-, A)$.

---

## 14.5 Adjoint Functors

### 14.5.1 Definition

Functors $F: \mathcal{C} \to \mathcal{D}$ and $G: \mathcal{D} \to \mathcal{C}$ are **adjoint** ($F \dashv G$, $F$ is left adjoint to $G$) if:

$$\mathrm{Hom}_\mathcal{D}(F(A), B) \cong \mathrm{Hom}_\mathcal{C}(A, G(B))$$

naturally in $A \in \mathcal{C}$ and $B \in \mathcal{D}$.

### 14.5.2 Examples of Adjunctions

| Left adjoint $F$ | Right adjoint $G$ | Context |
|-----------------|-------------------|---------|
| Free group | Forgetful | **Set** $\leftrightarrow$ **Grp** |
| Free module $R^{(-)}$ | Forgetful | **Set** $\leftrightarrow$ $R$-**Mod** |
| $- \otimes_R M$ | $\mathrm{Hom}_R(M, -)$ | $R$-**Mod** $\leftrightarrow$ $R$-**Mod** |
| Abelianization $G \mapsto G/[G,G]$ | Inclusion | **Ab** $\hookrightarrow$ **Grp** |
| Suspension $\Sigma$ | Loop space $\Omega$ | **Top** $\leftrightarrow$ **Top** |
| Colimit | Diagonal | $[\mathcal{J}, \mathcal{C}]$ $\leftrightarrow$ $\mathcal{C}$ |

### 14.5.3 Unit and Counit

Every adjunction $F \dashv G$ has:
- A **unit** $\eta: \mathrm{Id}_\mathcal{C} \Rightarrow G \circ F$ (natural transformation)
- A **counit** $\varepsilon: F \circ G \Rightarrow \mathrm{Id}_\mathcal{D}$

satisfying triangle identities: $(G\varepsilon) \circ (\eta G) = \mathrm{id}_G$ and $(\varepsilon F) \circ (F\eta) = \mathrm{id}_F$.

---

## 14.6 Limits and Colimits

### 14.6.1 Universal Constructions

Many constructions in algebra are characterized by universal properties:

| Construction | Universal property type |
|-------------|------------------------|
| Product $A \times B$ | Limit over discrete diagram |
| Coproduct $A \sqcup B$ | Colimit over discrete diagram |
| Kernel $\ker f$ | Equalizer (limit) |
| Cokernel $\mathrm{coker}\, f$ | Coequalizer (colimit) |
| Fiber product $A \times_C B$ | Pullback (limit) |
| Pushout | Colimit over span |
| Direct limit | Filtered colimit |
| Inverse limit | Cofiltered limit |

### 14.6.2 Limits

A **limit** of a diagram $D: \mathcal{J} \to \mathcal{C}$ is an object $\varprojlim D$ with maps to each $D(j)$ such that any other such cone factors uniquely through it.

A category has all small limits iff it has all products and equalizers.

### 14.6.3 Colimits

Dual notion. A **colimit** $\varinjlim D$ is an object with maps from each $D(j)$ such that any cocone factors uniquely through it.

**Example:** The quotient $M/N$ of a module is a coequalizer; the tensor product $M \otimes_R N$ is a colimit.

### 14.6.4 Adjoints Preserve Limits

**Theorem:** Right adjoints preserve limits. Left adjoints preserve colimits.

**Applications:**
- $\mathrm{Hom}(M, -)$ (right adjoint to $- \otimes M$) preserves limits, hence products and kernels — explaining why Hom is left-exact.
- $- \otimes M$ (left adjoint) preserves colimits, hence coproducts and cokernels — explaining why tensor is right-exact.

---

## 14.7 Abelian Categories

An **abelian category** is a category where:
- $\mathrm{Hom}(A, B)$ is an abelian group and composition is bilinear
- Finite products and coproducts exist and coincide (biproducts)
- Every mono is a kernel; every epi is a cokernel

**Examples:** **Ab**, $R$-**Mod**, sheaves of abelian groups.

Abelian categories are the natural home for homological algebra — the kernels, images, and exact sequences of Chapter 15 live here.

**Freyd–Mitchell embedding theorem:** Every small abelian category embeds fully faithfully into $R$-**Mod** for some ring $R$. This lets us do "diagram chases" in any abelian category.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Yoneda lemma | $\mathrm{Nat}(\mathrm{h}^A, F) \cong F(A)$, naturally |
| Adjoints preserve limits | Right adjoints preserve limits; left adjoints preserve colimits |
| Freyd–Mitchell | Every small abelian category embeds in $R$-Mod |

---

## Milestone Exercises

1. Verify the category axioms for **Set** and for a group $G$ viewed as a one-object category.

2. Show that the forgetful functor $U: \mathbf{Grp} \to \mathbf{Set}$ is faithful but not full.

3. Prove the Yoneda lemma.

4. Exhibit the unit and counit for the adjunction between free $\mathbb{Z}$-modules and abelian groups.

5. Show that products in **Set** are Cartesian products, and that the Cartesian product satisfies the universal property of the categorical product.

6. Prove that right adjoints preserve limits by using the bijection $\mathrm{Hom}(F(-), -)$.

7. Show that $\ker$ and $\mathrm{coker}$ of a map of abelian groups satisfy the universal properties of equalizer and coequalizer.

---

## Connections Forward

- **Chapter 15:** Homological algebra is done in abelian categories; chain complexes, exact sequences, and diagram lemmas are categorical.
- **Chapter 16:** Derived functors are the canonical way to extend left/right exact functors to exact functors using resolutions.
- **Chapter 26:** Category theory as a foundation; toposes as generalized set theories; logical content of categorical structure.
- **Chapter 27:** $\infty$-categories (Lurie's framework) are the proper setting for homotopy theory, derived algebraic geometry, and the Langlands program.

---

*Next: [Chapter 15 — Complexes, Homology, and Exact Sequences](ch15-homological-algebra-basics.md)*
