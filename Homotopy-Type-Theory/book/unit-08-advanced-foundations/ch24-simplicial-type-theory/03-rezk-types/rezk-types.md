# 24.3 Rezk Types and Directed Univalence

## Segal Types Are Not Enough

A Segal type has well-defined composition. But Segal alone does not say that *equality* and *isomorphism* agree. And in any reasonable mathematical framework, they should: two objects are "the same" iff there is an invertible morphism between them.

Consider the universe $\mathsf{Type}$ with hom = functions. It is Segal: function composition is uniquely defined. But now ask: when are two types $A$ and $B$ *equal* in $\mathsf{Type}$? And when are they *isomorphic* (as objects of the Segal type $\mathsf{Type}$, meaning: related by an invertible function)?

The answer to the second question: $A$ and $B$ are isomorphic in $\mathsf{Type}$ (as a Segal type) iff there exist functions $f : A \to B$ and $g : B \to A$ with $g \circ f = \mathsf{id}_A$ and $f \circ g = \mathsf{id}_B$ — iff $A \simeq B$.

The answer to the first question: $A = B$ in $\mathsf{Type}$ iff they are related by a path in the universe — which, *by univalence*, is exactly when $A \simeq B$.

So for $\mathsf{Type}$: isomorphism and equality coincide. But this uses univalence — it is not a consequence of the Segal condition alone. Without univalence, you might have two equivalent types that are not *equal* in the universe.

The *Rezk condition* is the general statement that isomorphism implies equality. It is the categorical analogue of univalence, and it is exactly the additional condition needed to go from "Segal type" to "∞-category in the complete sense."

## Isomorphisms in a Segal Type

Given a Segal type $A$, a morphism $f : \mathsf{hom}_A(a, b)$ is an *isomorphism* if it has a two-sided categorical inverse:

**Definition.** $f : \mathsf{hom}_A(a, b)$ is an *isomorphism* if there exists $f^{-1} : \mathsf{hom}_A(b, a)$ with:
$$f^{-1} \circ f = \mathsf{id}_a \quad \text{and} \quad f \circ f^{-1} = \mathsf{id}_b$$

The type of isomorphisms from $a$ to $b$:
$$\mathsf{Iso}_A(a, b) :\equiv \Sigma_{f : \mathsf{hom}_A(a, b)}\, \Sigma_{g : \mathsf{hom}_A(b, a)}\, (g \circ f = \mathsf{id}_a) \times (f \circ g = \mathsf{id}_b)$$

**The canonical map.** There is always a function:
$$\alpha_{a,b} : (a =_A b) \to \mathsf{Iso}_A(a, b)$$

sending `refl_a` to the identity isomorphism $(\mathsf{id}_a, \mathsf{id}_a, -, -)$. This is defined by path induction: for `refl`, the identity is its own inverse.

The Rezk condition asks that $\alpha_{a,b}$ is an *equivalence* — not just that it exists.

## The Rezk Condition

**Definition.** A Segal type $A$ is *Rezk* (or *complete*) if for all $a, b : A$, the canonical map:
$$\alpha_{a,b} : (a =_A b) \xrightarrow{\;\;\simeq\;\;} \mathsf{Iso}_A(a, b)$$
is an equivalence.

In words: $A$ is Rezk iff every isomorphism between objects comes from a (unique) path, and every path gives an isomorphism. Isomorphism = equality.

This is a strong condition. In a non-Rezk Segal type, there might be isomorphisms without corresponding paths — objects that are "the same up to isomorphism" but are genuinely different objects in the type.

## The Fundamental Connection: Rezk and Univalence

**Theorem.** The universe $\mathsf{Type}$ (with $\mathsf{hom}_\mathsf{Type}(A, B) = (A \to B)$) satisfies the Rezk condition if and only if the Univalence Axiom holds.

*Proof.* The isomorphisms in $\mathsf{Type}$ are:
$$\mathsf{Iso}_\mathsf{Type}(A, B) = \Sigma_{f:A \to B}\, \Sigma_{g:B \to A}\, (g \circ f = \mathsf{id}_A) \times (f \circ g = \mathsf{id}_B) = (A \simeq B)$$
(quasi-invertible functions = equivalences).

So the Rezk condition for $\mathsf{Type}$ says: $(A =_\mathsf{Type} B) \simeq (A \simeq B)$. This is exactly the Univalence Axiom. $\square$

**The upshot**: Univalence is the Rezk condition for the specific Segal type $\mathsf{Type}$. This tells us that univalence is not an extra axiom *about types* — it is the instance, for the universe, of a completely general categorical principle: *isomorphic objects are equal*.

This illuminates why univalence is the right axiom. It is not an arbitrary choice; it is the only choice consistent with treating the universe as a well-behaved ∞-category.

## Examples

**Every ∞-groupoid is Rezk.** In an ∞-groupoid, $\mathsf{hom}_A(a, b) = (a =_A b)$ and every path is an isomorphism. So $\mathsf{Iso}_A(a, b) = (a =_A b)$ and the map $\alpha_{a,b}$ is the identity. All ordinary HoTT types are Rezk.

**Posets: Rezk iff antisymmetric.** For a preorder $(P, \leq)$:
- $\mathsf{Iso}_P(a, b) = (a \leq b) \times (b \leq a)$ (a morphism in each direction)
- The Rezk condition: $(a = b) \simeq (a \leq b) \times (b \leq a)$
- This is exactly the antisymmetry condition: $a \leq b$ and $b \leq a$ implies $a = b$

So a preorder is Rezk iff it is a partial order (antisymmetric). The Rezk condition converts preorders to the mathematically "correct" notion.

**Sets with functions: Rezk iff univalent.** The Segal type $\mathsf{Set}$ (with hom = functions) is Rezk iff isomorphic sets are equal. In HoTT with univalence: yes. Without univalence: not in general.

## Rezk Completion

Every Segal type $A$ has a *Rezk completion* $\hat{A}$ — the "smallest Rezk type" that $A$ maps into.

**Construction.** $\hat{A}$ is formed by forcing $\alpha_{a,b}$ to be an equivalence. Concretely: add a path for every isomorphism in $A$ that doesn't yet have one. This is a HIT-like construction (or a reflective localization) that:
1. Inverts all isomorphisms in $A$
2. Adds the necessary paths
3. Satisfies the universal property

**Universal property.** $\hat{A}$ is the initial Rezk type with a functor $\iota : A \to \hat{A}$. Every functor $F : A \to B$ where $B$ is Rezk factors uniquely through $\hat{A}$:
$$A \xrightarrow{\;\iota\;} \hat{A} \xrightarrow{\;\bar{F}\;} B$$

This universal property is the categorical version of "freely inverting the isomorphisms."

**Example.** A preorder $P$ has a Rezk completion $\hat{P}$ which is the partial order obtained by identifying $a$ and $b$ whenever $a \leq b$ and $b \leq a$. The Rezk completion collapses each "isomorphism class" to a single point.

## Directed Univalence

The deep analogy between Rezk and Univalence suggests a *directed* version:

**Directed Univalence Principle (informal).** For a suitable type $\mathsf{Type}^{cat}$ of ∞-categories, two ∞-categories are equal in $\mathsf{Type}^{cat}$ if and only if they are equivalent as ∞-categories (related by a fully faithful and essentially surjective functor).

This would be the analogue of univalence for ∞-categories: the statement that the Segal type $\mathsf{Type}^{cat}$ is Rezk, where "isomorphism" in $\mathsf{Type}^{cat}$ means "categorical equivalence."

Formalizing this requires:
1. A definition of $\mathsf{Type}^{cat}$ (the type of Segal types, or Rezk types) in STT
2. The notion of categorical equivalence (fully faithful and essentially surjective functor) as the isomorphisms
3. Proving the Rezk condition for this universe

This is an *open problem* in simplicial type theory (as of 2025). The Riehl-Shulman program has laid the groundwork, but directed univalence is not yet proved.

## The Adjoint Functor Theorem Direction

The Rezk condition connects to classical category theory in another way: it is the condition under which the *adjoint functor theorem* can be formulated.

The classical adjoint functor theorem says (in one version): a functor $R : \mathcal{D} \to \mathcal{C}$ has a left adjoint if and only if $R$ preserves limits and $\mathcal{C}$ has a small cogenerating set. In STT, an analogous theorem should hold for functors between Rezk types.

The key point: adjunctions between Segal types can be characterized using the Rezk condition. A functor $L : A \to B$ is a left adjoint to $R : B \to A$ iff the types $\mathsf{hom}_B(L(a), b)$ and $\mathsf{hom}_A(a, R(b))$ are *equivalent* — and this equivalence, being an isomorphism in the Segal type of hom-types, corresponds (by the Rezk condition) to an equality.

## The Hierarchy: HoTT, Segal, Rezk

Here is the full picture of the type hierarchy in STT:

| Condition | Name | Classical analogue |
|-----------|------|-------------------|
| None | Type | ∞-groupoid (all morphisms invertible) |
| Segal | Segal type | ∞-category (composition unique) |
| Segal + Rezk | Rezk type | Complete ∞-category (isomorphism = equality) |

The bottom row (ordinary HoTT types) satisfies all three conditions, since in an ∞-groupoid every morphism is an isomorphism and the map $\alpha_{a,b}$ is the identity.

The important new types are the Segal-but-not-all-groupoid types, and among those, the Rezk types are the "well-behaved" ones.

## Working with Rezk Types in Practice

In Rzk, the Rezk condition is formalized and used as follows:

```rzk
#def isRezk (A : Segal-type) : Type
  := (a b : A) -> isEquiv (a = b) (Iso A a b) (alpha A a b)

-- The universe is Rezk by univalence
#def Type-isRezk : isRezk Type
  := \ A B -> univalence A B  -- uses the Univalence Axiom
```

With the Rezk condition, one can prove in Rzk:
- Isomorphic objects can be substituted: if $f : \mathsf{Iso}_A(a, b)$ and $P : A \to \mathsf{Type}$, then $P(a) \simeq P(b)$.
- The Rezk completion exists for any Segal type.
- Rezk types are "closed" under the standard categorical constructions (functor types, over-categories, etc.).

The Rezk condition is the final piece that makes simplicial type theory a complete framework for ∞-category theory: it identifies the right notion of categorical equivalence as the synthetic identity.
