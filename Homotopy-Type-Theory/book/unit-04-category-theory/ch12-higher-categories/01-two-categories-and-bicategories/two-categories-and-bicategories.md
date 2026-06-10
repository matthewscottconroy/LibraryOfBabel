# 2-Categories and Bicategories

## One Dimension Up

Ordinary category theory has objects and morphisms. The category $\mathbf{Cat}$ of small categories has categories as objects and functors as morphisms. Between two functors $F, G : \mathcal{C} \to \mathcal{D}$, there are natural transformations. So $\mathbf{Cat}$ has three levels: objects (categories), morphisms (functors), and 2-morphisms (natural transformations).

This is the simplest example of a *2-category*: a category enriched in $\mathbf{Cat}$ itself. The presence of three levels — objects, 1-morphisms, 2-morphisms — defines the "2-categorical" or "bicategorical" structure.

Understanding 2-categories is the entry point for higher category theory. The passage from categories to 2-categories is where the key difficulties first appear: the distinction between strict and weak, the role of coherence conditions, and the need for higher-dimensional reasoning.

## Strict 2-Categories

**Definition.** A *strict 2-category* $\mathcal{C}$ consists of:
- A class of *objects* (0-cells): $A, B, C, \ldots$
- For each pair of objects $(A, B)$, a category $\mathcal{C}(A, B)$ whose objects are *1-morphisms* $f : A \to B$ and whose morphisms are *2-morphisms* $\alpha : f \Rightarrow g$
- *Horizontal composition*: a functor $\circ : \mathcal{C}(B, C) \times \mathcal{C}(A, B) \to \mathcal{C}(A, C)$ for each triple $(A, B, C)$
- *Identity 1-morphisms*: $\mathsf{id}_A \in \mathcal{C}(A, A)$ for each object $A$

satisfying:
- **Strict associativity:** $(h \circ g) \circ f = h \circ (g \circ f)$ as functors from $\mathcal{C}(C,D) \times \mathcal{C}(B,C) \times \mathcal{C}(A,B) \to \mathcal{C}(A,D)$
- **Strict units:** $\mathsf{id}_B \circ f = f = f \circ \mathsf{id}_A$

In a strict 2-category, the associativity and unit laws hold strictly (as equalities, not just isomorphisms).

**Two kinds of composition for 2-morphisms:**
- *Vertical composition*: if $\alpha : f \Rightarrow g$ and $\beta : g \Rightarrow h$, then $\beta \circ_v \alpha : f \Rightarrow h$ (composition within the category $\mathcal{C}(A,B)$)
- *Horizontal composition*: if $\alpha : f \Rightarrow g$ (1-morphisms $A \to B$) and $\beta : h \Rightarrow k$ (1-morphisms $B \to C$), then $\beta \circ_h \alpha : h \circ f \Rightarrow k \circ g$

**The interchange law:** Vertical and horizontal composition satisfy: $(\beta' \circ_v \beta) \circ_h (\alpha' \circ_v \alpha) = (\beta' \circ_h \alpha') \circ_v (\beta \circ_h \alpha)$. The order in which you compose doesn't matter.

## Examples of Strict 2-Categories

**$\mathbf{Cat}$:** The canonical example. Objects are small categories; 1-morphisms are functors; 2-morphisms are natural transformations. Horizontal composition of natural transformations is the whiskering operation; vertical composition is the pointwise composition of natural transformation components. $\mathbf{Cat}$ is a strict 2-category.

**$\mathbf{Grpd}$:** The 2-category of groupoids, functors, and natural transformations. A special case of $\mathbf{Cat}$. Important because groupoids model homotopy types.

**The 2-category of monads in $\mathcal{C}$:** Objects are monads $(T, \eta, \mu)$ on $\mathcal{C}$; 1-morphisms are monad morphisms (natural transformations compatible with the monad structure); 2-morphisms are modifications.

**The fundamental 2-groupoid $\Pi_2(X)$:** For a topological space $X$: objects are points $x \in X$; 1-morphisms are paths $\gamma : x \to y$ (continuous maps $[0,1] \to X$ with $\gamma(0) = x$ and $\gamma(1) = y$); 2-morphisms are homotopies of paths (relative to endpoints). This is a strict 2-groupoid (all 2-morphisms are invertible).

## Bicategories: The Weak Version

In a strict 2-category, composition is associative on the nose. But most naturally occurring examples are only associative *up to isomorphism*. The right notion is a *bicategory*.

**Definition.** A *bicategory* $\mathcal{B}$ consists of:
- A class of objects
- For each pair of objects, a category $\mathcal{B}(A, B)$ (of 1-morphisms and 2-morphisms)
- Composition functors $\circ : \mathcal{B}(B, C) \times \mathcal{B}(A, B) \to \mathcal{B}(A, C)$
- Identity 1-morphisms $\mathsf{id}_A$

together with *coherence isomorphisms*:
- *Associator:* a natural isomorphism $\alpha_{f,g,h} : (h \circ g) \circ f \xrightarrow{\sim} h \circ (g \circ f)$
- *Left unitor:* $\lambda_f : \mathsf{id}_B \circ f \xrightarrow{\sim} f$
- *Right unitor:* $\rho_f : f \circ \mathsf{id}_A \xrightarrow{\sim} f$

satisfying:
- **Pentagon axiom:** a coherence equation for the associator ensuring that $(h \circ g) \circ (f \circ e) \cong h \circ (g \circ (f \circ e))$ computed in two ways gives the same result
- **Triangle axiom:** a coherence equation ensuring that $f \circ \mathsf{id} \circ g \cong f \circ g$ computed via the unitors gives the same result as the direct comparison

A strict 2-category is a bicategory where all the coherence isomorphisms are identities.

## The Coherence Theorem

**Theorem (MacLane, Bénabou).** Every bicategory is equivalent (in an appropriate 2-categorical sense) to a strict 2-category.

This theorem says: even though bicategories are weaker than strict 2-categories, you can always "strictify" them — find an equivalent strict 2-category. So for many purposes, you can work with strict 2-categories without loss of generality.

*Caveat*: the strictification only works for 2-categories, not for higher-dimensional cases. For n-categories with n ≥ 3, there are coherence phenomena that cannot be strictified. This is one reason why higher category theory is genuinely harder in higher dimensions.

## Why Weakness Is Necessary

The coherence theorem might suggest: "just use strict 2-categories, since every bicategory is equivalent to one." But this misses the point. The *natural* examples are weak, and working with their natural presentations is often cleaner.

**Span bicategory.** Objects are sets; 1-morphisms from $A$ to $B$ are *spans* (diagrams $A \leftarrow C \rightarrow B$); 2-morphisms are morphisms of spans. Composition of spans is by pullback. Since pullbacks are only defined up to isomorphism, composition is only associative up to isomorphism — this is a bicategory, not a strict 2-category. The strictification exists but is more complex to describe.

**The bicategory $\mathbf{Mod}$.** Objects are rings; 1-morphisms from $R$ to $S$ are $(R,S)$-bimodules; 2-morphisms are bimodule homomorphisms. Composition is tensor product of bimodules, which is only associative up to natural isomorphism. This is the natural setting for Morita theory.

**Types as a bicategory.** In MLTT with the identity type, types form a bicategory: objects are types; 1-morphisms are functions; 2-morphisms are homotopies (elements of identity types of function types). The associativity of composition of functions holds strictly, but the groupoid structure on the identity type makes this a degenerate bicategory where the only 2-morphisms are homotopies. Going to the full ∞-groupoid structure requires all the higher identity types.

## 2-Functors and Pseudofunctors

Just as functors are maps between categories, 2-functors are maps between 2-categories. A *strict 2-functor* $F : \mathcal{C} \to \mathcal{D}$ sends objects, 1-morphisms, and 2-morphisms to their counterparts, preserving all composition and identities strictly.

A *pseudofunctor* (or *homomorphism of bicategories*) is the weak version: it sends objects, 1-morphisms, and 2-morphisms to their counterparts, but preserves composition only up to specified coherent isomorphisms (the *compositor* and *unitor* of the pseudofunctor).

Every functor $F : \mathcal{C} \to \mathcal{D}$ between ordinary categories extends to a strict 2-functor $\mathbf{B}F : \mathbf{B}\mathcal{C} \to \mathbf{B}\mathcal{D}$ (where $\mathbf{B}$ promotes a category to a 2-category with only trivial 2-morphisms). Pseudofunctors arise naturally in the study of moduli problems and in the theory of stacks.

## The Road to Higher Categories

The passage from categories to 2-categories generalizes to $n$-categories (for any $n$) and ultimately to ∞-categories.

For each $n$:
- An $n$-category has morphisms at all levels $0, 1, \ldots, n$
- Composition is associative and unital at each level
- There are coherence conditions at each level

For *strict* $n$-categories, all composition laws hold strictly. For *weak* $n$-categories, they hold up to isomorphisms at the next level, with coherence conditions.

The combinatorics of coherence conditions for weak $n$-categories become rapidly complex as $n$ increases. For $n = 2$: the pentagon and triangle axioms. For $n = 3$: the Stasheff associahedra. For general $n$: infinite families of coherence cells.

The breakthrough: instead of specifying all the coherence conditions explicitly, use a *simplicial* or *globular* model that encodes them implicitly. This is the approach of quasi-categories (Joyal, Lurie) for ∞-categories, and of globular sets (Batanin) for ω-categories.

In the simplicial approach, the composition and coherence data are encoded in the *horn-filling conditions* of simplicial sets. A Kan complex — a simplicial set where all horns fill — is an ∞-groupoid. A quasi-category — a simplicial set where only inner horns fill — is an (∞,1)-category.

This is the language we need for HoTT's homotopy hypothesis.
