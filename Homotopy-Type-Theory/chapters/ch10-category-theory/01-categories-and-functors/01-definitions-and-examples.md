# 1.1 Categories and Functors

## The Definition of a Category

A *category* is one of the simplest mathematical structures: objects (things) and morphisms (arrows between things), with a composition law and identity arrows. The simplicity is deceptive — the abstraction captures something essential about mathematical structure.

**Definition.** A *category* $\mathcal{C}$ consists of:
- A collection $\mathsf{Ob}(\mathcal{C})$ of *objects*, written $A, B, C, \ldots$
- For each pair of objects $A, B$, a set $\mathsf{Hom}_\mathcal{C}(A, B)$ of *morphisms* (or *arrows*) from $A$ to $B$, written $f : A \to B$
- For each triple $A, B, C$, a *composition* operation: given $f : A \to B$ and $g : B \to C$, their composite $g \circ f : A \to C$
- For each object $A$, an *identity morphism* $\mathsf{id}_A : A \to A$

These must satisfy:
- **Associativity:** $(h \circ g) \circ f = h \circ (g \circ f)$ for all composable $f, g, h$
- **Left identity:** $\mathsf{id}_B \circ f = f$ for all $f : A \to B$
- **Right identity:** $f \circ \mathsf{id}_A = f$ for all $f : A \to B$

That's it. Three types of data (objects, morphisms, composition + identities) and three axioms (associativity, left and right unit laws). The simplicity is intentional.

**Size.** A category is *small* if $\mathsf{Ob}(\mathcal{C})$ is a set and all hom-sets $\mathsf{Hom}(A, B)$ are sets. It is *locally small* if all hom-sets are sets (even if $\mathsf{Ob}(\mathcal{C})$ is a proper class). Most naturally occurring categories ($\mathbf{Set}$, $\mathbf{Grp}$, $\mathbf{Top}$) are locally small but not small.

## Examples from Everywhere

Part of what makes category theory powerful is the diversity of examples. The same formal structure appears in radically different mathematical domains.

**Set.** The most fundamental example: objects are sets, morphisms are functions, composition is function composition, identity is the identity function. This is $\mathbf{Set}$.

**Grp.** Objects are groups, morphisms are group homomorphisms. A group homomorphism $f : G \to H$ satisfies $f(g_1 \cdot g_2) = f(g_1) \cdot f(g_2)$ and $f(e_G) = e_H$.

**Top.** Objects are topological spaces, morphisms are continuous maps. Continuity is required because we care about the topological structure, not just the set-theoretic structure.

**Vect$_k$.** Objects are vector spaces over a field $k$, morphisms are linear maps.

**Type (in MLTT).** Objects are types $A : \mathsf{Type}$, morphisms from $A$ to $B$ are functions $f : A \to B$, composition is function composition, identity is $\lambda x. x$. This is the "walking category" of dependent type theory.

**The category of a preorder.** A preorder $(P, \leq)$ gives a category: objects are elements of $P$; $\mathsf{Hom}(a, b) = \{*\}$ if $a \leq b$, else $\mathsf{Hom}(a, b) = \emptyset$. Transitivity gives composition; reflexivity gives identity. This is a category where every hom-set has at most one element.

In a preorder category, "morphism from $a$ to $b$" means "a proof that $a \leq b$." Composition is exactly the proof of transitivity.

**The category of a monoid.** A monoid $(M, \cdot, e)$ gives a one-object category $\mathbf{B}M$: the single object is $\star$; $\mathsf{Hom}(\star, \star) = M$; composition is monoid multiplication; identity is $e$.

So a monoid is a category with one object. And a group is a one-object category where every morphism is an isomorphism. Category theory subsumes monoids and groups as special cases.

**Discrete category.** For any set $S$, the *discrete* category on $S$ has elements of $S$ as objects and only identity morphisms (no non-identity morphisms). Composition is trivial.

**Opposite category.** For any category $\mathcal{C}$, the *opposite* $\mathcal{C}^{op}$ has the same objects but reversed morphisms: $\mathsf{Hom}_{\mathcal{C}^{op}}(A, B) = \mathsf{Hom}_\mathcal{C}(B, A)$. Composition reverses too: $f \circ_{op} g = g \circ f$.

The opposite category is why category theory has a *duality principle*: every theorem has a dual obtained by reversing all arrows. For example, the dual of "products" is "coproducts."

## Isomorphisms

**Definition.** A morphism $f : A \to B$ is an *isomorphism* if there exists $g : B \to A$ (the *inverse*) such that $g \circ f = \mathsf{id}_A$ and $f \circ g = \mathsf{id}_B$.

When $f$ is an isomorphism, $A$ and $B$ are *isomorphic*, written $A \cong B$.

Isomorphisms in different categories:
- In $\mathbf{Set}$: bijections
- In $\mathbf{Grp}$: group isomorphisms
- In $\mathbf{Top}$: homeomorphisms
- In $\mathbf{Vect}_k$: linear isomorphisms
- In a preorder: $a \cong b$ iff $a \leq b$ and $b \leq a$ (i.e., $a = b$ in a partial order)
- In a monoid category: every invertible element of $M$

**Key property:** The inverse of an isomorphism is unique. If $g$ and $g'$ are both inverses of $f$, then $g = g \circ \mathsf{id}_B = g \circ (f \circ g') = (g \circ f) \circ g' = \mathsf{id}_A \circ g' = g'$.

## Functors: Maps Between Categories

A *functor* is a map between categories that preserves the categorical structure.

**Definition.** A *functor* $F : \mathcal{C} \to \mathcal{D}$ consists of:
- A function on objects: $F(A) \in \mathsf{Ob}(\mathcal{D})$ for each $A \in \mathsf{Ob}(\mathcal{C})$
- A function on morphisms: $F(f) : F(A) \to F(B)$ in $\mathcal{D}$ for each $f : A \to B$ in $\mathcal{C}$

satisfying:
- $F(\mathsf{id}_A) = \mathsf{id}_{F(A)}$ (identities preserved)
- $F(g \circ f) = F(g) \circ F(f)$ (composition preserved)

A functor that preserves the direction of arrows is *covariant*. A *contravariant functor* $F : \mathcal{C} \to \mathcal{D}$ reverses arrows: $F(f) : F(B) \to F(A)$ for $f : A \to B$. (Equivalently, a functor $\mathcal{C}^{op} \to \mathcal{D}$.)

## Examples of Functors

**Forgetful functor $U : \mathbf{Grp} \to \mathbf{Set}$.** Send each group to its underlying set; send each homomorphism to the underlying function. The functor "forgets" the group structure.

More generally, any map that forgets structure is a functor: rings to groups (forget multiplication), topological spaces to sets (forget topology), etc.

**Free functor $F : \mathbf{Set} \to \mathbf{Grp}$.** Send each set $S$ to the free group $F(S)$ on $S$; send each function $f : S \to T$ to the unique group homomorphism $F(f) : F(S) \to F(T)$ induced by $f$. This functor "adds" free structure.

**Fundamental group $\pi_1 : \mathbf{Top}_* \to \mathbf{Grp}$.** This is the key functor connecting topology to algebra. A pointed topological space $(X, x_0)$ maps to its fundamental group $\pi_1(X, x_0)$; a based continuous map $f : (X, x_0) \to (Y, y_0)$ maps to the induced group homomorphism $f_* : \pi_1(X, x_0) \to \pi_1(Y, y_0)$.

The functoriality of $\pi_1$ is central to algebraic topology: $\pi_1(f \circ g) = \pi_1(f) \circ \pi_1(g)$ and $\pi_1(\mathsf{id}) = \mathsf{id}$.

**The Hom functor $\mathsf{Hom}(A, -) : \mathcal{C} \to \mathbf{Set}$.** For a fixed object $A$, send each $B$ to the set $\mathsf{Hom}(A, B)$; send each $f : B \to C$ to the postcomposition function $f_* : \mathsf{Hom}(A, B) \to \mathsf{Hom}(A, C)$, $g \mapsto f \circ g$.

**The contravariant Hom functor $\mathsf{Hom}(-, B) : \mathcal{C}^{op} \to \mathbf{Set}$.** For a fixed $B$, send each $A$ to $\mathsf{Hom}(A, B)$; send $f : A' \to A$ to precomposition $f^* : \mathsf{Hom}(A, B) \to \mathsf{Hom}(A', B)$, $g \mapsto g \circ f$.

**ap in type theory.** The action-on-paths function $\mathsf{ap}_f : (a = b) \to (f(a) = f(b))$ from Section 4.1 of Chapter 9 is exactly the functor action of $f$ on the morphisms of the fundamental groupoid of $A$. Every function $f : A \to B$ in MLTT is a functor between the fundamental groupoids of $A$ and $B$.

## Functors as Structure-Preserving Maps

Functors are the structure-preserving maps for categories. Just as group homomorphisms preserve group operations and continuous maps preserve topology, functors preserve categorical structure: composition and identities.

This leads to the category of all (small) categories, $\mathbf{Cat}$, where objects are categories and morphisms are functors. This is a well-defined category: functor composition is associative (since function composition is), and the identity functor $\mathsf{Id}_\mathcal{C}$ is the identity morphism.

**Fully faithful functors.** A functor $F : \mathcal{C} \to \mathcal{D}$ is:
- *Full* if $F : \mathsf{Hom}(A, B) \to \mathsf{Hom}(F(A), F(B))$ is surjective for all $A, B$
- *Faithful* if $F : \mathsf{Hom}(A, B) \to \mathsf{Hom}(F(A), F(B))$ is injective for all $A, B$
- *Fully faithful* if it's both full and faithful

A fully faithful functor is an "embedding": it identifies $\mathcal{C}$ with a full subcategory of $\mathcal{D}$. The Yoneda embedding (Section 3) is fully faithful.

## The Connection to Type Theory

In MLTT, the types and functions form a category. But there's more: every context $\Gamma$ in MLTT gives a *slice category*: the category of types over $\Gamma$ (types dependent on $\Gamma$). A morphism in the slice category is a term of one type given the other.

The dependent type theory operations correspond to categorical operations:
- Π types correspond to *right adjoints* in the slice category
- Σ types correspond to *left adjoints* (composition with the projection)
- Substitution corresponds to *pullback* along a morphism

This correspondence — between dependent type theory and locally cartesian closed categories — will be developed in Section 7.

## Diagrams and Commutativity

A *diagram* in a category is a graph where nodes are objects and edges are morphisms. A diagram *commutes* if any two paths between the same nodes compose to the same morphism.

For example, a commuting triangle is three objects $A, B, C$ with morphisms $f : A \to B$, $g : B \to C$, $h : A \to C$ such that $h = g \circ f$. A commuting square has four objects and four morphisms where both paths from the top-left to the bottom-right give the same composite.

Commuting diagrams are the bread and butter of category theory. They express equalities between morphisms in a visually clear way. In proofs, "by the commutativity of the diagram" means "by chasing the diagram."

In type theory, the groupoid laws (Section 3 of Chapter 9) can be expressed as commuting diagrams in the fundamental groupoid of a type. The naturality of transport and ap are also commuting squares.
