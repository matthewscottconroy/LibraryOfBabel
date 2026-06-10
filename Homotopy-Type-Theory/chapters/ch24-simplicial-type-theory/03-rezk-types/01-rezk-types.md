# 3.1 Rezk Types and Directed Univalence

## The Problem with Segal Types Alone

A Segal type has well-defined composition, but it may identify "equality" and "isomorphism" in the wrong way. Consider the Segal type $\mathsf{Type}$ (with hom = functions):

- Two types $A$ and $B$ can be *isomorphic* as objects in $\mathsf{Type}$ (there exist functions $f : A \to B$ and $g : B \to A$ with $g \circ f = \mathsf{id}$ and $f \circ g = \mathsf{id}$) — this means $A$ and $B$ are equivalent as types.
- But are $A$ and $B$ *equal* (as types, in the universe)? By univalence, yes — but this requires *univalence*, not just the Segal structure.

The issue: in a Segal type, the path type $a =_A b$ (undirected paths from $a$ to $b$) and the isomorphism type $\mathsf{Iso}_A(a, b)$ (invertible morphisms from $a$ to $b$) are separate. The Segal condition doesn't require them to agree.

The *Rezk condition* (completeness) requires that isomorphisms and paths coincide. This is the categorical analogue of univalence: just as univalence says "equivalent types are equal," the Rezk condition says "isomorphic objects are equal."

## Isomorphisms in a Segal Type

**Definition.** In a Segal type $A$, a morphism $f : \mathsf{hom}_A(a, b)$ is an *isomorphism* if there exists an *inverse* $f^{-1} : \mathsf{hom}_A(b, a)$ with:
$$f^{-1} \circ f = \mathsf{id}_a \quad \text{and} \quad f \circ f^{-1} = \mathsf{id}_b$$

The type of isomorphisms from $a$ to $b$ is:
$$\mathsf{Iso}_A(a, b) :\equiv \Sigma_{f : \mathsf{hom}_A(a,b)}\, \Sigma_{g : \mathsf{hom}_A(b,a)}\, (g \circ f = \mathsf{id}_a) \times (f \circ g = \mathsf{id}_b)$$

**The canonical map.** There is always a function:
$$\alpha_{a,b} : (a =_A b) \to \mathsf{Iso}_A(a, b)$$

mapping paths to isomorphisms. It sends `refl` to the identity isomorphism $(\mathsf{id}_a, \mathsf{id}_a, -, -)$.

This map always exists — the question is whether it's an equivalence.

## The Rezk Condition

**Definition.** A Segal type $A$ is *Rezk* (or *complete*) if for all $a, b : A$:
$$\alpha_{a,b} : (a =_A b) \xrightarrow{\;\simeq\;} \mathsf{Iso}_A(a, b)$$

is an equivalence.

In words: $A$ is Rezk iff equality between objects coincides with isomorphism between objects.

This is the categorical completeness condition. It says: the only way for two objects to be "the same" in $A$ is for them to be isomorphic via an isomorphism that comes from a path.

## Examples

**Every ∞-groupoid is Rezk.** In an ∞-groupoid (ordinary HoTT type), every morphism is a path, and every path is a morphism. So $\mathsf{hom}_A(a,b) = (a =_A b)$ and $\mathsf{Iso}_A(a,b) = (a =_A b)$. The map $\alpha_{a,b}$ is the identity. ✓

**The universe $\mathsf{Type}$ is Rezk if and only if univalence holds.** The Rezk condition for $\mathsf{Type}$ says:
$$(A =_\mathsf{Type} B) \simeq \mathsf{Iso}_\mathsf{Type}(A, B)$$

The isomorphisms in $\mathsf{Type}$ (with $\mathsf{hom} = $ functions) are exactly the equivalences $A \simeq B$. So the Rezk condition for $\mathsf{Type}$ is exactly the univalence axiom! This is the key connection:

**Univalence is the Rezk condition for the universe.**

**Posets.** A poset $(P, \leq)$ gives a Segal type where $\mathsf{hom}_P(a, b) = (a \leq b)$. This is Rezk iff:
$$a = b \iff a \leq b \text{ and } b \leq a$$
i.e., iff $P$ is an *antisymmetric* poset (= a partial order, not just a preorder). A preorder that is not antisymmetric is not Rezk.

**The complete Segal space.** In classical homotopy theory, a "complete Segal space" is a Segal space satisfying a completeness condition equivalent to the Rezk condition. Rezk types are the synthetic type-theoretic version.

## Rezk Completion

Given a Segal type $A$ that is not Rezk, we can construct its *Rezk completion* $\hat{A}$ — the "smallest Rezk type" containing $A$.

**Construction.** The Rezk completion is obtained by forcing the map $\alpha_{a,b}$ to be an equivalence. This is done by identifying isomorphic objects: we add paths for every isomorphism, forcing equality to equal isomorphism.

More precisely, $\hat{A}$ is the *groupoid completion* of $A$ in the directed sense — we freely invert all morphisms.

**Universal property.** $\hat{A}$ is the initial Rezk type with a functor from $A$. Any functor $F : A \to B$ where $B$ is Rezk factors through the completion:
$$A \to \hat{A} \xrightarrow{\bar{F}} B$$

## The Rezk Condition and Univalence: The Deep Connection

Let's make the connection between Rezk and univalence precise.

In ordinary HoTT with univalence:
- The universe $\mathsf{Type}$ satisfies: $A = B \iff A \simeq B$
- This is univalence: equality in the universe = equivalence

In simplicial type theory:
- The Segal type $\mathsf{Type}$ (with hom = functions) satisfies the Segal condition
- The Rezk condition for $\mathsf{Type}$ says: $A = B \iff A \simeq B$ (equivalences = isomorphisms in $\mathsf{Type}$)
- This is exactly univalence!

So **univalence = Rezk condition for $\mathsf{Type}$**.

More generally:
- **Univalence**: "For the universe, isomorphism = equality"
- **Rezk**: "For a Segal type, isomorphism = equality"

Univalence is the instance of the Rezk condition for the specific Segal type $\mathsf{Type}$.

This illuminates why univalence is the "right" axiom: it's the condition making the universe into a *complete* (Rezk) Segal type, i.e., a well-behaved ∞-category.

## Directed Univalence

The analogy suggests a *directed* version of univalence:

**Conjecture (Directed Univalence).** There is a Segal type $\mathsf{Type}^{cat}$ (of ∞-categories) such that the "isomorphisms" in $\mathsf{Type}^{cat}$ are the *fully faithful and essentially surjective functors* (the ∞-categorical equivalences), and the Rezk condition holds: two ∞-categories are equal in $\mathsf{Type}^{cat}$ iff they are equivalent as ∞-categories.

This would be the analogue of univalence for ∞-categories: "equivalent ∞-categories are equal."

This conjecture is an open problem in simplicial type theory (as of 2025). It requires:
1. A good type $\mathsf{Type}^{cat}$ of ∞-categories in STT
2. A characterization of equivalences between ∞-categories
3. Proving the Rezk condition

## Rezk Fibrations

For completeness theory:

**Definition.** A *Rezk fibration* (or *right fibration*) $p : E \to A$ over a Rezk type $A$ is a map such that the fibers $E_a = p^{-1}(a)$ are contractible when $a$ has a path to another object with a filler.

This is the simplicial analogue of a principal fibration in topology.

Rezk fibrations classify objects "up to isomorphism": two sections of a Rezk fibration represent the same object iff they are isomorphic.

## Working with Rezk Types

In practice, working with Rezk types means:
1. Choose a Segal type $A$ (the "∞-category")
2. Verify the Rezk condition or take the Rezk completion
3. Use the fact that equality and isomorphism coincide

Many common types are Rezk:
- Any ∞-groupoid (ordinary HoTT type) is automatically Rezk
- The universe $\mathsf{Type}$ is Rezk by univalence
- The type of groups is Rezk (two groups are equal iff isomorphic — this is structure invariance from Chapter 18)

The Rezk condition is the appropriate notion of "universe-like" Segal type — one where the type theory's notion of equality matches the categorical notion of isomorphism.

## From Sets to ∞-Categories: The Hierarchy

Just as HoTT has the h-level hierarchy (Prop < Set < Groupoid < ...), STT has an analogous hierarchy:

| Level | HoTT term | Segal/Rezk term |
|-------|-----------|-----------------|
| -1 | hProp | Partial order (trivial morphisms) |
| 0 | hSet | Poset or category with discrete objects |
| 1 | Groupoid | ∞-groupoid |
| 1+ | 1-Type | Segal 1-type (1-category) |
| $\infty$ | ∞-Type | Segal ∞-type (∞-category) |

The Rezk condition sits at each level: a Rezk Segal 1-type is a (strict/classical) category; a Rezk Segal ∞-type is an ∞-category; the universe $\mathsf{Type}$ is a Rezk Segal ∞-type (by univalence).

This hierarchy unifies the classical notions of categories at different levels with the type-theoretic notion of h-levels.
