# 2.1 Natural Transformations

## What a Natural Transformation Is

Functors are maps between categories. What are the maps between functors? Natural transformations.

**Definition.** Given functors $F, G : \mathcal{C} \to \mathcal{D}$, a *natural transformation* $\alpha : F \Rightarrow G$ is a family of morphisms:

$$\alpha_A : F(A) \to G(A) \quad \text{for each object } A \in \mathcal{C}$$

satisfying the *naturality condition*: for every morphism $f : A \to B$ in $\mathcal{C}$, the following square commutes:

$$G(f) \circ \alpha_A = \alpha_B \circ F(f)$$

Drawn as a diagram:

$$\begin{array}{ccc} F(A) & \xrightarrow{\alpha_A} & G(A) \\ {}_{F(f)}\downarrow & & \downarrow_{G(f)} \\ F(B) & \xrightarrow{\alpha_B} & G(B) \end{array}$$

The morphisms $\alpha_A$ are the *components* of the natural transformation.

## Reading the Naturality Condition

The naturality condition says: it doesn't matter whether you apply $\alpha$ before or after applying $F$ or $G$ to a morphism. More precisely: applying $F$ to an arrow $f$ and then transforming with $\alpha_B$ gives the same result as transforming with $\alpha_A$ and then applying $G$ to $f$.

This "coherence" condition is what makes a natural transformation "natural" — the components at different objects are consistent with the structure of both categories.

**Why the naturality condition matters:** Without it, a family $\{\alpha_A\}$ would be just a collection of morphisms with no relationship to the categorical structure. The naturality condition ensures the family is compatible with the morphisms of $\mathcal{C}$.

## Examples

**The double dual embedding.** For vector spaces over a field $k$, define $\eta_V : V \to V^{**}$ by $\eta_V(v) = \text{ev}_v$ (evaluation at $v$: the linear functional sending $\phi \in V^*$ to $\phi(v)$).

This is a natural transformation $\mathsf{id}_{\mathbf{Vect}_k} \Rightarrow (-)^{**}$ (from the identity functor to the double-dual functor). Naturality says: for any linear map $f : V \to W$, the diagram

$$\begin{array}{ccc} V & \xrightarrow{\eta_V} & V^{**} \\ {}_{f}\downarrow & & \downarrow_{f^{**}} \\ W & \xrightarrow{\eta_W} & W^{**} \end{array}$$

commutes. This is the "natural" embedding: it uses no choice of basis.

The *single* dual embedding $V \to V^*$ is not natural (it requires choosing an isomorphism $V \cong V^*$, which depends on a basis). The double dual embedding is natural precisely because it doesn't make any choices.

This was the original motivating example for category theory! Eilenberg and Mac Lane invented categories to make "natural" precise.

**The determinant.** For $n \times n$ invertible matrices over a field $k$, $\det : \mathsf{GL}_n(k) \to k^*$ is a group homomorphism (natural transformation component). The determinant is natural in $k$: for any field map $\phi : k \to k'$, the appropriate square commutes.

**Homotopies as natural transformations.** In the fundamental groupoid of a topological space, objects are points and morphisms are homotopy classes of paths. A homotopy $H : f \simeq g$ (between continuous maps $f, g : X \to Y$) is a natural transformation between the induced functors $f_*, g_* : \pi_1(X) \to \pi_1(Y)$.

In MLTT, a homotopy $H : \prod_{x:A} f(x) = g(x)$ is a natural transformation between $f$ and $g$ viewed as functors between the fundamental groupoids of $A$ and $B$. The naturality condition for $H$ is exactly the "naturality square" we proved in Section 5.1 of Chapter 9: $H(a_1) \cdot \mathsf{ap}_g(p) = \mathsf{ap}_f(p) \cdot H(a_2)$.

## Composition and Identity

Natural transformations can be composed. Given $\alpha : F \Rightarrow G$ and $\beta : G \Rightarrow H$ (all functors $\mathcal{C} \to \mathcal{D}$), the *vertical composition* $\beta \circ \alpha : F \Rightarrow H$ is defined componentwise:

$$(\beta \circ \alpha)_A = \beta_A \circ \alpha_A : F(A) \to H(A)$$

The identity natural transformation $\mathsf{id}_F : F \Rightarrow F$ has components $(\mathsf{id}_F)_A = \mathsf{id}_{F(A)}$.

This makes functors $\mathcal{C} \to \mathcal{D}$ into a category — the *functor category* $[\mathcal{C}, \mathcal{D}]$ or $\mathcal{D}^{\mathcal{C}}$ — whose objects are functors and morphisms are natural transformations.

**Horizontal composition.** There's also a way to compose natural transformations "sideways." Given $\alpha : F \Rightarrow G$ (functors $\mathcal{C} \to \mathcal{D}$) and $\beta : H \Rightarrow K$ (functors $\mathcal{D} \to \mathcal{E}$), the *horizontal composition* $\beta * \alpha : H \circ F \Rightarrow K \circ G$ has components:

$$(\beta * \alpha)_A = \beta_{G(A)} \circ H(\alpha_A) = K(\alpha_A) \circ \beta_{F(A)}$$

(Both definitions agree by naturality of $\beta$.)

The two compositions satisfy the *interchange law*: $(\beta' \circ \beta) * (\alpha' \circ \alpha) = (\beta' * \alpha') \circ (\beta * \alpha)$.

This means categories with functors and natural transformations form a *2-category* $\mathbf{Cat}$: objects are categories, 1-morphisms are functors, 2-morphisms are natural transformations. This is the beginning of higher category theory.

## Natural Isomorphisms

A natural transformation $\alpha : F \Rightarrow G$ is a *natural isomorphism* if every component $\alpha_A : F(A) \to G(A)$ is an isomorphism in $\mathcal{D}$.

When $F$ and $G$ are naturally isomorphic (written $F \cong G$), we say they are "the same up to natural isomorphism." This is the correct notion of sameness for functors.

**Example.** The product $A \times B$ and $B \times A$ are naturally isomorphic (via the swap maps $\tau_{A,B} : A \times B \to B \times A$, $(a, b) \mapsto (b, a)$). Naturality means: for any $f : A \to A'$ and $g : B \to B'$, the swap is compatible with the maps.

In type theory: $A \times B \simeq B \times A$ via the equivalence $\lambda (a, b). (b, a)$, and this equivalence is natural in $A$ and $B$.

## Functor Categories and Presheaves

The functor category $[\mathcal{C}^{op}, \mathbf{Set}]$ (also written $\hat{\mathcal{C}}$ or $\mathsf{PSh}(\mathcal{C})$) consists of:
- Objects: functors $\mathcal{C}^{op} \to \mathbf{Set}$ (called *presheaves*)
- Morphisms: natural transformations between presheaves

Presheaves generalize sets indexed by objects of $\mathcal{C}$. They're important because:
1. Every category embeds in its presheaf category (Yoneda embedding, Section 3)
2. Presheaf categories are toposes (Chapter 11's categorical semantics)
3. Many models of type theory are built from presheaf categories

**Key example.** If $\mathcal{C}$ is a small category, a presheaf $F : \mathcal{C}^{op} \to \mathbf{Set}$ assigns to each object $c$ a set $F(c)$ and to each morphism $f : c \to c'$ a *restriction* map $F(f) : F(c') \to F(c)$ (note: direction reversed because the functor is contravariant). This is the notion of a sheaf on a site, generalized to arbitrary categories.

## Whiskering

*Whiskering* is a simpler form of horizontal composition, using an identity.

Given a natural transformation $\alpha : F \Rightarrow G$ (functors $\mathcal{C} \to \mathcal{D}$) and a functor $H : \mathcal{D} \to \mathcal{E}$, the *right-whiskered* transformation $H\alpha : H \circ F \Rightarrow H \circ G$ has components:

$$(H\alpha)_A = H(\alpha_A) : H(F(A)) \to H(G(A))$$

Similarly, given a functor $K : \mathcal{B} \to \mathcal{C}$ and $\alpha : F \Rightarrow G$, the *left-whiskered* transformation $\alpha K : F \circ K \Rightarrow G \circ K$ has components:

$$(\alpha K)_B = \alpha_{K(B)} : F(K(B)) \to G(K(B))$$

In type theory: whiskering appears when composing homotopies with functions. If $H : f \sim g$ (functions $A \to B$) and $k : B \to C$, then $k \circ H : k \circ f \sim k \circ g$ is the right-whiskered homotopy, with $(k \circ H)(x) = \mathsf{ap}_k(H(x))$.

## The 2-Category Structure of Cat

The existence of natural transformations makes $\mathbf{Cat}$ a 2-category:
- 0-cells (objects): categories
- 1-cells (morphisms): functors
- 2-cells (morphisms between morphisms): natural transformations

In a 2-category, you can compose 1-cells (functor composition), compose 2-cells vertically (vertical composition of natural transformations), and compose 2-cells horizontally (horizontal composition). The interchange law ensures these are compatible.

The 2-category structure of $\mathbf{Cat}$ is the stepping stone to:
- **Bicategories:** 2-categories where composition is associative only up to isomorphism
- **Higher categories:** $n$-categories for all $n$, and $\infty$-categories in the limit
- **$\infty$-groupoids:** The fully general case, which models HoTT

Types in HoTT are $\infty$-groupoids precisely because they have: elements (0-cells), paths between elements (1-cells), paths between paths (2-cells), and so on, with all levels of "morphisms" being invertible.

## Summary

| Concept | Definition | Example |
|---|---|---|
| Natural transformation $\alpha : F \Rightarrow G$ | Family of morphisms $\alpha_A : F(A) \to G(A)$ satisfying naturality | Double-dual embedding |
| Vertical composition | $(\beta \circ \alpha)_A = \beta_A \circ \alpha_A$ | Concatenation of homotopies |
| Natural isomorphism | Natural transformation with all components isomorphisms | $A \times B \cong B \times A$ |
| Functor category $[\mathcal{C}, \mathcal{D}]$ | Objects = functors, morphisms = nat. transforms | Presheaf category $\hat{\mathcal{C}}$ |
| Horizontal composition | Composing nat. transforms "sideways" | Whiskering |
| 2-category | Categories/functors/nat. transforms | $\mathbf{Cat}$ |

Natural transformations are the morphisms of the world of functors. They capture the notion of a "natural" (basis-independent, coherent) transformation between constructions. In HoTT, they appear as homotopies between functions — the type-theoretic analog of continuous deformations.
