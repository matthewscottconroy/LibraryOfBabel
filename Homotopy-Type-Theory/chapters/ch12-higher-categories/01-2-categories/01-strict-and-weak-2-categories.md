# 1.1 Strict and Weak 2-Categories

## The Starting Point: What's Missing from Ordinary Categories?

Let's think about the category **Cat** of small categories. Its objects are categories, its morphisms are functors. So far, ordinary category theory.

But here's a natural question: what are the "morphisms between morphisms" in **Cat**? A functor $F : \mathcal{C} \to \mathcal{D}$ is a morphism. Given two functors $F, G : \mathcal{C} \to \mathcal{D}$, a *natural transformation* $\alpha : F \Rightarrow G$ is a systematic way to transform one into the other. That's a morphism between morphisms.

**Cat** isn't just a category. It's a *2-category*: there are objects (categories), 1-morphisms (functors), and 2-morphisms (natural transformations). You can compose functors (1-morphism composition) and you can compose natural transformations in two ways:
- *Vertically*: if $\alpha : F \Rightarrow G$ and $\beta : G \Rightarrow H$, compose them to get $\beta \circ \alpha : F \Rightarrow H$
- *Horizontally*: if $\alpha : F \Rightarrow F'$ (transforming $\mathcal{C} \to \mathcal{D}$) and $\beta : G \Rightarrow G'$ (transforming $\mathcal{D} \to \mathcal{E}$), combine them to get $\beta \star \alpha : G \circ F \Rightarrow G' \circ F'$

This two-dimensional structure — with two kinds of composition that interact coherently — is the essence of 2-categories.

## Strict 2-Categories: The Definition

Let's nail down the definition carefully.

**Definition 1.1 (Strict 2-Category).** A *strict 2-category* $\mathcal{C}$ consists of:

- A collection of *objects* (0-cells): $A, B, C, \ldots$
- For each pair of objects $A, B$: a *category* $\mathcal{C}(A, B)$ whose:
  - Objects are *1-cells* $f : A \to B$
  - Morphisms are *2-cells* $\alpha : f \Rightarrow g$ (for $f, g : A \to B$)
  - Composition in $\mathcal{C}(A,B)$ is *vertical composition* of 2-cells
- For each triple $A, B, C$: a *composition functor* $\circ_{A,B,C} : \mathcal{C}(B,C) \times \mathcal{C}(A,B) \to \mathcal{C}(A,C)$

This composition functor does two things at once:
1. On objects (i.e., on 1-cells): $g \circ f : A \to C$ for $f : A \to B, g : B \to C$ — ordinary 1-cell composition
2. On morphisms (i.e., on 2-cells): $\beta \star \alpha : g \circ f \Rightarrow g' \circ f'$ for $\alpha : f \Rightarrow f', \beta : g \Rightarrow g'$ — *horizontal composition*

The "functor" condition on $\circ_{A,B,C}$ is exactly the *interchange law*:
$$(\beta' \circ \beta) \star (\alpha' \circ \alpha) = (\beta' \star \alpha') \circ (\beta \star \alpha)$$

In words: composing vertically and then horizontally is the same as composing horizontally and then vertically. This is what makes the two compositions coherent.

The "strict" part means all associativity and unit laws hold as *equalities*:
- $h \circ (g \circ f) = (h \circ g) \circ f$ (strict associativity of 1-cells)
- $\mathsf{id}_B \circ f = f = f \circ \mathsf{id}_A$ (strict units)

## Visualizing 2-Cells

It helps to draw these as diagrams. A 1-cell $f : A \to B$ is an arrow:
$$A \xrightarrow{f} B$$

A 2-cell $\alpha : f \Rightarrow g$ is a "double arrow" between two parallel arrows:
$$A \underset{g}{\overset{f}{\rightrightarrows}} B \quad \curvearrowright \quad \alpha : f \Rightarrow g$$

Or drawn as a 2-dimensional region:
$$\begin{array}{c} A \xrightarrow{f} B \\ \Downarrow \alpha \\ A \xrightarrow{g} B \end{array}$$

Vertical composition stacks these vertically:
$$\begin{array}{c} A \xrightarrow{f} B \\ \Downarrow \alpha \\ A \xrightarrow{g} B \\ \Downarrow \beta \\ A \xrightarrow{h} B \end{array} \quad \leadsto \quad \begin{array}{c} A \xrightarrow{f} B \\ \Downarrow \beta \circ \alpha \\ A \xrightarrow{h} B \end{array}$$

Horizontal composition places them side by side:
$$\begin{array}{c} A \xrightarrow{f} B \xrightarrow{g} C \\ \Downarrow \alpha \quad\quad \Downarrow \beta \end{array} \quad \leadsto \quad \begin{array}{c} A \xrightarrow{g \circ f} C \\ \Downarrow \beta \star \alpha \end{array}$$

## The Main Examples

**Example 1.2 (Cat).** The 2-category of small categories:
- Objects: small categories
- 1-cells: functors
- 2-cells: natural transformations
- Vertical composition: composition of natural transformations component-wise
- Horizontal composition: whiskering (defined in Chapter 10)

This is the motivating example for the whole theory.

**Example 1.3 (Grpd).** The full sub-2-category of Cat consisting of groupoids. Objects are groupoids (categories where every morphism is invertible), 1-cells are functors, 2-cells are natural transformations. This is important because groupoids model homotopy types.

**Example 1.4 (2-cell = homotopy).** For topological spaces, consider:
- Objects: topological spaces $X$
- 1-cells: continuous maps $f : X \to Y$
- 2-cells from $f$ to $g$: homotopies $H : X \times [0,1] \to Y$ with $H(-,0) = f$, $H(-,1) = g$

Vertical composition: stack homotopies end-to-end. Horizontal composition: compose homotopies. But wait — homotopy concatenation is only associative up to homotopy! So this is not a strict 2-category, just a bicategory. (We'll come back to this.)

**Example 1.5 (One-object 2-category = monoidal category).** A 2-category with one object has:
- One object $*$
- 1-cells: these are $\mathsf{Hom}(*,*)$, forming a category
- 2-cells between 1-cells

The 1-cells compose (giving a monoidal structure) and the 2-cells transform them. A one-object 2-category is exactly a *monoidal category* (with the single hom-category being the monoidal category, and 1-cell composition being the tensor product).

This is a nice pattern: just as a one-object category is a monoid, a one-object 2-category is a monoidal category.

## Bicategories: When Strictness Fails

In practice, the most natural 2-categorical structures are not strict. Composition is associative and unital only up to coherent isomorphism. This leads to *bicategories*, introduced by Bénabou in 1967.

**Definition 1.6 (Bicategory).** A *bicategory* $\mathcal{B}$ consists of:
- Objects: $A, B, C, \ldots$
- For each pair $A, B$: a category $\mathcal{B}(A, B)$ (1-cells and 2-cells between them)
- Composition: for each triple $A, B, C$, a functor $\otimes : \mathcal{B}(B,C) \times \mathcal{B}(A,B) \to \mathcal{B}(A,C)$
- Unit: for each object $A$, a distinguished 1-cell $\mathsf{id}_A \in \mathcal{B}(A,A)$

But now instead of strict equalities for associativity and units, we have natural isomorphisms:
- **Associator:** $\alpha_{h,g,f} : (h \otimes g) \otimes f \xrightarrow{\sim} h \otimes (g \otimes f)$ for composable $f, g, h$
- **Left unitor:** $\lambda_f : \mathsf{id}_B \otimes f \xrightarrow{\sim} f$
- **Right unitor:** $\rho_f : f \otimes \mathsf{id}_A \xrightarrow{\sim} f$

These isomorphisms must satisfy coherence conditions:

**The Pentagon Identity:** For four composable 1-cells $f, g, h, k$:
$$\begin{array}{c} \text{The diagram of five ways to reassociate} (k \otimes h) \otimes (g \otimes f) \text{ must commute.} \end{array}$$

**The Triangle Identity:** The associator, left unitor, and right unitor must be compatible:
$$\alpha_{g, \mathsf{id}_B, f} \circ (\rho_g \otimes f) = g \otimes \lambda_f$$

These coherence conditions ensure that any two ways to "reparenthesize" a composite of 1-cells (using associators and unitors) give the same result. This is the bicategorical analog of Mac Lane's coherence theorem for monoidal categories.

## Why Coherence Conditions?

Here's the key question: why do we need these specific coherence conditions? What goes wrong without them?

The short answer: without coherence conditions, the associator isomorphisms might not compose consistently. Different ways of reparenthesizing could give different results, making the structure unusable.

Mac Lane's coherence theorem (for monoidal categories) says: given the pentagon and triangle, *all* possible ways to reparenthesize a composite agree. This is the content of coherence: from finitely many conditions, you get infinitely many consequences for free.

**Coherence theorem for bicategories (Mac Lane-Bénabou):** In any bicategory, all diagrams of associators and unitors that could conceivably commute do commute.

This is a deep theorem, but its effect is practical: you can move associators and unitors around freely in proofs, knowing that any two ways of doing so will agree.

## Spans as a Bicategory

Let's work out the example of spans, which is a natural bicategory that isn't strict.

**Definition 1.7 (Span).** A *span* from $A$ to $B$ is a diagram $A \xleftarrow{s} C \xrightarrow{t} B$ (an object $C$ mapping to both $A$ and $B$).

The bicategory of spans (let's call it **Span**):
- Objects: sets $A, B, C, \ldots$
- 1-cells $A \to B$: spans $A \xleftarrow{s} C \xrightarrow{t} B$
- 2-cells: morphisms of spans (maps $C \to C'$ compatible with the legs)
- Composition of spans: given spans $A \xleftarrow{s} C \xrightarrow{t} B$ and $B \xleftarrow{u} D \xrightarrow{v} E$, their composite is the pullback:

$$A \xleftarrow{} C \times_B D \xrightarrow{} E$$

The pullback gives the composite span. But pullbacks are only defined up to isomorphism, not on the nose. So composition is only associative up to isomorphism — **Span** is a bicategory, not a strict 2-category.

Why study spans? They model:
- Relations (a relation $R \subseteq A \times B$ is the span $A \xleftarrow{\pi_1} R \xrightarrow{\pi_2} B$)
- Multilinear maps (in suitable settings)
- Correspondences in algebraic geometry

The bicategory of spans is a prototype for the kinds of weak 2-categorical structures that arise throughout mathematics.

## The Interchange Law as a Pasting Lemma

One of the most useful features of 2-categories is that you can reason with 2-dimensional diagrams ("pasting diagrams"). The interchange law is what makes this work.

Here's a small pasting diagram:
$$\begin{array}{ccccc}
A & \xrightarrow{f} & B & \xrightarrow{g} & C \\
 & \Downarrow \alpha & & \Downarrow \beta & \\
A & \xrightarrow{f'} & B & \xrightarrow{g'} & C
\end{array}$$

You can compute the overall 2-cell from $g \circ f$ to $g' \circ f'$ in two ways:
1. Horizontally: $\beta \star \alpha$
2. Vertically, after horizontally: $(\beta \star \mathsf{id}_{f'}) \circ (\mathsf{id}_g \star \alpha)$ (whisk first, then compose)

The interchange law says these are equal: $\beta \star \alpha = (\beta \star \mathsf{id}_{f'}) \circ (\mathsf{id}_g \star \alpha)$.

This pasting lemma is crucial for practical calculations in 2-categories. It says: in a pasting diagram, the order in which you compose doesn't matter — you'll always get the same result.

## 2-Functors and 2-Natural Transformations

Just as ordinary categories have functors and natural transformations, 2-categories have 2-functors and 2-natural transformations (and even 3-cells called modifications).

**2-Functor:** Sends objects to objects, 1-cells to 1-cells, 2-cells to 2-cells, preserving all composition and units strictly (for strict 2-categories) or up to coherence (for bicategories, where they're called *homomorphisms of bicategories*).

**2-Natural transformation:** For 2-functors $F, G : \mathcal{C} \to \mathcal{D}$, a 2-natural transformation $\alpha : F \Rightarrow G$ assigns to each object $A$ a 1-cell $\alpha_A : F(A) \to G(A)$ and for each 1-cell $f : A \to B$, a 2-cell filling a naturality square (not just making it commute — filling it with a 2-cell).

This is the beginning of the tower: 2-categories, 2-functors, 2-natural transformations, modifications... and then it keeps going.

## Summary

| Structure | Strict 2-Category | Bicategory |
|---|---|---|
| Objects | ✓ | ✓ |
| 1-Cells | ✓ | ✓ |
| 2-Cells | ✓ | ✓ |
| 1-Cell composition | Strict | Strict |
| Associativity of 1-cells | On the nose | Up to associator isomorphism |
| Units of 1-cells | On the nose | Up to unitor isomorphisms |
| Coherence | Automatic | Pentagon + triangle |

Strict 2-categories are simpler but less natural. Bicategories are more flexible and capture the examples that actually arise (spans, homotopies, functors, correspondences). The coherence conditions ensure the structure is usable.

For the purposes of HoTT, the key takeaway is: composition in higher categories is typically weak (up to isomorphism, up to coherence), not strict. This mirrors the fact that paths in a type compose up to higher paths, not literally on the nose.
