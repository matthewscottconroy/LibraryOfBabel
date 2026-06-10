# 2.1 Segal Types

## What is a Segal Type?

A Segal type is the synthetic type-theoretic analogue of a complete Segal space (in classical homotopy theory) and of an ∞-category (in higher category theory). It's a type where morphisms have a well-defined composition operation.

The name "Segal" comes from Graeme Segal's work on configuration spaces and the *Segal condition* in simplicial spaces — the condition that ensures composition of morphisms is uniquely defined.

## The Segal Condition

**Definition.** A type $A$ is *Segal* if the restriction map:

$$\mathsf{comp}_A : (\Delta^2 \to A) \to (\Lambda^2_1 \to A)$$

is an equivalence.

Let's unpack. $\Lambda^2_1 \to A$ is the type of "composable pairs": pairs of morphisms $(f : \mathsf{hom}_A(a,b), g : \mathsf{hom}_A(b,c))$ — an inner horn. $\Delta^2 \to A$ is the type of 2-simplices in $A$: triangles with three vertices and three directed edges.

The restriction map takes a triangle and forgets the hypotenuse, leaving only the two "spine" edges. The Segal condition says this restriction is an equivalence — meaning:

1. **Existence:** Every composable pair $(f, g)$ has a composite (a triangle filling it)
2. **Uniqueness:** The composite is unique (up to contractibility) — any two fillings of the same horn are equal (up to a 2-path)

**In other words:** $A$ is Segal iff every composable pair of morphisms has a unique composite.

## Reformulating: Segal Types Have Composition

The Segal condition gives us a *composition function*:

$$\circ : \mathsf{hom}_A(b, c) \times \mathsf{hom}_A(a, b) \to \mathsf{hom}_A(a, c)$$

defined by taking the filler of the horn. Since the filler is unique (up to contractibility), this is well-defined.

More precisely, the composite $g \circ f$ is defined as the hypotenuse of the unique triangle filling the horn $(f, g)$:
$$g \circ f :\equiv \text{(hypotenuse of the unique filler of } (f, g) \text{)}$$

**Associativity.** Composition in a Segal type is automatically associative — up to homotopy. The homotopy comes from the Segal condition: $(h \circ g) \circ f$ and $h \circ (g \circ f)$ are both fillers of the same 3-dimensional horn, and the uniqueness of fillers (by a 3-dimensional Segal condition) makes them equal.

## Examples

**Example 1: Sets.** The universe of sets $\mathsf{Set}$ (types that are h-sets) is a Segal type. The hom type $\mathsf{hom}_\mathsf{Set}(A, B)$ is the type of functions $A \to B$ (not necessarily bijections). Composition is function composition. The Segal condition holds because function composition is uniquely defined.

**Example 2: Posets.** A poset $(P, \leq)$ gives a Segal type. The hom type $\mathsf{hom}_P(a, b)$ is the proof that $a \leq b$: it's either empty (if $a \not\leq b$) or contractible (if $a \leq b$, there's exactly one proof). Composition is transitivity.

This Segal type is very "thin": all 2-simplices are either present (if the three inequalities hold) or absent. There are no interesting 2-morphisms.

**Example 3: Every type in HoTT.** Any type $X$ in ordinary HoTT gives a Segal type by taking $\mathsf{hom}_X(a, b) = (a =_X b)$ (the undirected path type). Path concatenation gives composition. The Segal condition: every "composable pair of paths" $(p : a =_X b, q : b =_X c)$ has a unique composite $p \cdot q : a =_X c$.

But this is stronger than just Segal: since all morphisms (paths) are invertible, this Segal type is actually an ∞-groupoid, which satisfies the Rezk condition (see next section).

**Example 4: The universe $\mathsf{Type}$.** The type $\mathsf{Type}$ is Segal. The hom type $\mathsf{hom}_\mathsf{Type}(A, B)$ is the type of functions $A \to B$. Composition is function composition. The Segal condition: given $f : A \to B$ and $g : B \to C$, the composite $g \circ f : A \to C$ exists uniquely.

Note: $\mathsf{Type}$ is *not* Rezk in the strong sense (see next section), because not all isomorphisms between types correspond to paths in $\mathsf{Type}$. To make $\mathsf{Type}$ Rezk, we need to restrict to "univalent" types.

## The Spine of a Simplex

The *spine* of $\Delta^n$ is the sequence of $n$ composable edges:

$$\mathsf{Sp}[n] :\equiv \{(t_1, \ldots, t_n) : \mathbf{2}^n \mid t_1 \leq t_2 \leq \cdots \leq t_n \}$$

The Segal condition can be stated more generally: for all $n$, the restriction map

$$(\Delta^n \to A) \to (\mathsf{Sp}[n] \to A)$$

is an equivalence. This says: an $n$-simplex in $A$ (a chain of $n$ composable morphisms with all higher-dimensional data) is determined by its spine (just the $n$ composable morphisms).

For $n = 2$: this is the original Segal condition.
For $n = 3$: this gives associativity.
For higher $n$: these give all higher coherences.

A type satisfying all these spine conditions is exactly what's needed for an ∞-category.

## Segal vs. Strict Category

A key advantage of Segal types over strict categories: **there's no coherence problem**.

In classical category theory, when you define a (strict) 2-category or a bicategory, you need to specify associators and unitors and verify the pentagon and triangle identities. These coherence conditions are notorious for being complex.

In Segal type theory:
- Composition is *defined* as "fill the horn" — no coherence data to specify
- Associativity holds *automatically* from the contractibility of horn fillers
- All higher coherences are similarly automatic

This is the synthetic advantage: the coherence problem is absorbed into the contractibility of fillers, which the Segal condition guarantees.

## The ∞-Groupoid Case

When $A$ is an ∞-groupoid (every morphism is invertible), the Segal condition reduces to ordinary path concatenation. In this case:
- $\mathsf{hom}_A(a, b) = (a =_A b)$ (undirected paths)
- The unique composite of $(p, q)$ is $p \cdot q$
- The Segal condition follows from the fact that path concatenation is well-defined

So ordinary HoTT types are all Segal. The Segal condition is automatic. What STT adds is types that are Segal but *not* ∞-groupoids — types where morphisms aren't all invertible.

## Covariant Fibrations

A key notion for functors from a Segal type to $\mathsf{Type}$:

**Definition.** A *covariant fibration* (or *left fibration*) over $A$ is a type family $C : A \to \mathsf{Type}$ such that for every morphism $f : \mathsf{hom}_A(a, b)$ and element $c : C(a)$, there is a *transport* of $c$ along $f$ in $C(b)$, and this transport is unique in an appropriate sense.

More precisely, $C$ is a covariant fibration over $A$ if the Segal condition for $C$ (as a type over $A$) holds: every partial section over a horn extends uniquely to a section over the full simplex.

Covariant fibrations are the correct notion of "functor from a Segal type to spaces" in STT. They generalize the notion of a discrete fibration in classical category theory.

The Grothendieck construction connects covariant fibrations to functors: given $C : A \to \mathsf{Type}$, the total space $\Sigma_{a:A} C(a)$ with the projection to $A$ is a covariant fibration iff $C$ is a covariant fibration.

## Why Not Just Use Classical ∞-Category Theory?

Classical ∞-category theory (quasi-categories, complete Segal spaces, etc.) requires working with simplicial sets, Kan fibrations, and the elaborate machinery of model categories. This is powerful but:

- **Not foundational**: requires set theory as a foundation
- **Not synthetic**: statements about ∞-categories must be translated into statements about simplicial sets
- **Coherence-intensive**: proofs require complex coherence arguments

Simplicial type theory gives a *synthetic* approach:
- **Foundational**: built on type theory, not set theory
- **Synthetic**: ∞-categories are types, not simplicial sets
- **Coherence-free**: the type theory handles coherences automatically

This is analogous to how HoTT gives a synthetic approach to homotopy theory, avoiding the need to work with specific topological spaces.

## The Segal Condition in Rzk

In the Rzk proof assistant, the Segal condition is stated as:

```rzk
-- A type A is Segal if
-- the horn-filling map is an equivalence
#define isSegal (A : U) : U :=
  (x y z : A) → (f : hom A x y) → (g : hom A y z) →
  isContr (Σ (h : hom A x z), Δ² → A [ ... ])
```

The `isContr` says there's a unique composite up to contractibility — this is the Segal condition stated precisely.

In Rzk, one can then prove theorems about Segal types, including the associativity of composition and the unit laws.
