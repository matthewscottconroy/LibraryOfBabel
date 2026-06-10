# 24.2 Segal Types

## What Makes a Type a Category?

In ordinary HoTT, every type is an ∞-groupoid: the path structure gives composition, inverses, and coherences up to all higher homotopies, and every morphism (path) is automatically invertible. The homotopy type theory is *completely symmetric*: there is no meaningful sense in which paths go one way rather than another.

But not every mathematical object is an ∞-groupoid. A category has morphisms that are not invertible. A poset has morphisms that are unique (either there is exactly one morphism from $a$ to $b$ or there is none). A monoid is a one-object category where not every element need be invertible. The classical mathematical world is full of directed, asymmetric structure.

The Segal condition is the condition that captures exactly when a type has this directed categorical structure. It is a condition on how the simplicial shapes fit together inside a type — not extra structure added on top, but a property that either holds or doesn't.

## The Segal Condition

**Definition.** A type $A$ in simplicial type theory is *Segal* if the restriction map:
$$\mathsf{Seg}_A : (\Delta^2 \to A) \xrightarrow{\;\;\simeq\;\;} (\Lambda^2_1 \to A)$$
is an equivalence.

The map is:
- **Domain** $\Delta^2 \to A$: the type of 2-simplices in $A$ — triangles with three vertices, three directed edges, and a coherence interior.
- **Codomain** $\Lambda^2_1 \to A$: the type of inner horns in $A$ — composable pairs $(f : \mathsf{hom}_A(a,b), g : \mathsf{hom}_A(b,c))$ without a specified composite.
- **The map**: restriction — take the full triangle and forget the hypotenuse.

Saying this map is an *equivalence* means two things:

1. **Existence**: Every composable pair has a composite (a triangle filling the horn).
2. **Uniqueness up to contractibility**: The space of composites is contractible — any two triangles filling the same horn are connected by a path of triangles. The composite is essentially unique.

This is stronger than just saying "composition exists." It says composition is *uniquely determined* by the composable pair, up to a contractible space of choices.

## Why Contractibility, Not Uniqueness?

In a strict category, composition is *strictly unique*: there is exactly one composite of two composable morphisms, no choice involved. In an ∞-category, composition is *uniquely determined up to contractibility*: there may be multiple composites, but they are all connected by higher-dimensional morphisms that are themselves contractibly unique.

The contractibility is the infinite-categorical version of uniqueness. A contractible space is "essentially a single point" — it has a unique inhabitant up to homotopy. So the Segal condition says: the space of composites is essentially a single point, i.e., composition is essentially unique.

This is exactly right for ∞-categories: composition is defined up to homotopy, and the homotopy is itself defined up to homotopy, and so on. The contractibility condition packages all these coherences in a single assertion.

## Examples

**Example 1: Any type in HoTT.** For any type $X$ in ordinary HoTT, define $\mathsf{hom}_X(a, b) :\equiv (a =_X b)$ — the path type. Path concatenation gives composition, and the Segal condition holds: the space of path-composites filling a composable pair is contractible (since paths have a unique concatenation up to homotopy). So every ∞-groupoid is Segal. HoTT embeds into STT.

But every morphism (path) is invertible. The HoTT types are Segal but they are more: they are ∞-groupoids, and the Segal structure is "thin" in the sense that all morphisms are isomorphisms.

**Example 2: Sets and functions.** The universe of sets $\mathsf{Set} = \Sigma_{A:\mathsf{Type}} \mathsf{isSet}(A)$ is Segal with $\mathsf{hom}_\mathsf{Set}(A, B) = (A \to B)$ (functions, not just bijections). A composable pair $(f : A \to B, g : B \to C)$ has a unique composite $g \circ f : A \to C$, so the Segal condition holds.

This is a genuinely non-groupoid Segal type: there are many functions $A \to B$ and $B \to A$, but only some are inverses. The type $\mathsf{hom}_\mathsf{Set}(\mathbb{N}, \mathbb{N})$ is the type of functions from naturals to naturals — most of which are not invertible.

**Example 3: A poset.** Let $(P, \leq)$ be a poset. Define $A$ as the type of elements of $P$, with $\mathsf{hom}_A(a, b) :\equiv (a \leq b)$ (a proposition: either there is one morphism from $a$ to $b$ or there is none). The Segal condition holds: if $a \leq b$ and $b \leq c$, then $a \leq c$ by transitivity, and the composite is unique (being a proposition). So posets are Segal types.

**Example 4: The universe $\mathsf{Type}$.** With $\mathsf{hom}_\mathsf{Type}(A, B) = (A \to B)$ (functions), the universe is Segal. Function composition is uniquely defined. This example is important because it is the "universe of ∞-categories" — a Segal type whose objects are types.

## Composition in a Segal Type

From the Segal condition, we get a composition operation:

$$\circ : \mathsf{hom}_A(b, c) \times \mathsf{hom}_A(a, b) \to \mathsf{hom}_A(a, c)$$

defined as: given $f : \mathsf{hom}_A(a, b)$ and $g : \mathsf{hom}_A(b, c)$, let $g \circ f$ be the hypotenuse of the essentially unique 2-simplex filling the horn $(f, g)$.

More precisely: the Segal condition gives an equivalence $(\Delta^2 \to A) \simeq (\Lambda^2_1 \to A)$. The quasi-inverse of the restriction map fills horns. Given the composable pair $(f, g)$, we apply the quasi-inverse to get a full 2-simplex, and then extract the hypotenuse.

**Associativity**: Composition is automatically associative — up to homotopy. The homotopy arises from the 3-dimensional Segal condition: the two composites $(h \circ g) \circ f$ and $h \circ (g \circ f)$ both fill the same 3-dimensional horn, and by the contractibility of 3-simplices in a Segal type, they are homotopic.

**Unit laws**: The identity morphism $\mathsf{id}_a = \lambda t. a : \mathsf{hom}_A(a, a)$ satisfies $f \circ \mathsf{id}_a = f$ and $\mathsf{id}_b \circ f = f$ — from the contractibility of the filler that witnesses these as equal composites.

## The Spine of a Simplex

The Segal condition can be generalized to all dimensions:

**The $n$-spine** $\mathsf{Sp}[n]$ of $\Delta^n$ is the sequence of $n$ composable edges:
$$\mathsf{Sp}[n] = \{(t_1, \ldots, t_n) : \mathbf{2}^n \mid t_1 \leq t_2 \leq \cdots \leq t_n\}$$

The *full Segal condition* for all $n$ says:
$$(\Delta^n \to A) \xrightarrow{\;\;\simeq\;\;} (\mathsf{Sp}[n] \to A)$$

This says: an $n$-simplex in $A$ is completely determined by its $n$ composable edges (its spine). All the higher-dimensional data (the faces, the interior) is uniquely filled in.

For $n = 2$: the original Segal condition.
For $n = 3$: gives associativity as a contractible choice.
For all $n$: gives all higher coherences.

A type satisfying all spine conditions is precisely an ∞-category in the synthetic sense.

## Covariant Fibrations: Functors to Spaces

In classical category theory, a *functor* $F : \mathcal{C} \to \mathsf{Set}$ is a rule assigning a set to each object and a function to each morphism, consistently with composition. In STT, the analogue is a *covariant fibration*:

**Definition.** A type family $C : A \to \mathsf{Type}$ over a Segal type $A$ is a *covariant fibration* if the Segal condition holds for the total space $\Sigma_{a:A} C(a)$ over $A$.

More concretely: $C$ is covariant if for every morphism $f : \mathsf{hom}_A(a, b)$ and element $c : C(a)$, there is a unique (up to contractibility) element $f_*(c) : C(b)$ — the "pushforward" of $c$ along $f$.

Covariant fibrations are the correct notion of "functors from a Segal type to spaces" in STT. The Grothendieck construction relates covariant fibrations to genuine functors: $C : A \to \mathsf{Type}$ is a covariant fibration iff the projection $\Sigma_{a:A} C(a) \to A$ is a left fibration.

## Discrete Types and Posetal Segal Types

At the "bottom" of the complexity spectrum:

**Discrete Segal types**: A type $A$ is *discrete* if every morphism $f : \mathsf{hom}_A(a, b)$ is an isomorphism, and the isomorphism is unique (i.e., $A$ is an ∞-groupoid). Equivalently, the hom type $\mathsf{hom}_A(a, b)$ is equivalent to the path type $a =_A b$ for all $a, b$.

**Posetal Segal types**: A Segal type $A$ is *posetal* if every hom type $\mathsf{hom}_A(a, b)$ is a proposition (at most one morphism between any two objects). These are the synthetic preorders. A posetal Segal type is additionally a partial order (antisymmetric) iff it satisfies the Rezk condition (next section).

## The Coherence-Free Miracle

The classical approach to higher categories is haunted by the *coherence problem*. To define a bicategory, you need associators, unitors, and the pentagon identity. To define a tricategory, the coherence conditions fill pages. As the dimension increases, the number of coherence conditions grows super-exponentially.

In simplicial type theory, *there are no coherence conditions to specify*. A Segal type has composition, and all higher coherences are automatic consequences of the contractibility of horn-fillers. You don't prove associativity — associativity holds because any two 3-simplices filling the same horn are contractibly equal. You don't specify an associator — the associator is the contractible path between the two composites.

This is the deepest payoff of the synthetic approach: the type theory absorbs the coherence problem into the Segal condition. Instead of specifying a tower of coherences, you just check the horn-filling condition and get all coherences for free.

This is not a trick. It is a consequence of working with contractibility rather than strict uniqueness. Contractible choices carry all the coherence information implicitly, in the topology of the space of choices.

## Segal Types in Rzk

In Rzk, the Segal condition is formalized as:

```rzk
#def isSegal (A : Type) : Type
  := (x y z : A)
  -> (f : hom A x y) -> (g : hom A y z)
  -> isContr (Σ (h : hom A x z), 
              Σ (_ : Δ² -> A),
              ...)  -- the 2-simplex with hypotenuse h
```

The `isContr` condition captures the "unique up to contractibility" requirement. Given a composable pair $(f, g)$, the type of composites (2-simplices with spine $(f, g)$) is contractible.

From this definition, one can prove in Rzk:
- Composition is a function: `comp A isSegal-A f g : hom A x z`
- Composition is associative: a path between the two compositions
- Identity is the unit: paths witnessing the unit laws

All of this follows from the single Segal condition — no additional axioms are needed.
