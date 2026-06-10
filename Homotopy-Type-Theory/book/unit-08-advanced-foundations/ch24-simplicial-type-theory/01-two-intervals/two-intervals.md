# 24.1 Two Intervals: Directed and Undirected

## The Problem with One Interval

Ordinary HoTT — including CCHM cubical type theory — has one interval $\mathbb{I}$ equipped with a complement operation $\sim$. This complement is essential for homotopy theory: it gives path reversal. A path from $a$ to $b$ can always be reversed to give a path from $b$ to $a$.

But this automatic symmetry is exactly wrong for category theory. In a category, morphisms go one way. The existence of a morphism $f : a \to b$ does not imply the existence of a morphism $b \to a$. Morphisms compose but need not be invertible. The identity $\mathsf{id}_a : a \to a$ is special because it is the unit for composition — not because it is its own inverse.

If we want types that behave like ∞-categories — with directed morphisms, non-invertible composition, and no automatic reversal — we need an interval that lacks complement. The directed interval $\mathbf{2}$ is exactly this: an interval with two endpoints and an ordering, but no way to reverse direction.

## The Two Intervals Side by Side

**The undirected interval $\mathbb{I}$** (also called the cubical interval):
- Endpoints $i0, i1 : \mathbb{I}$
- Complement $\sim i$ with $\sim\, i0 = i1$ and $\sim\, i1 = i0$
- Meet $\wedge$ and join $\vee$
- A *path* $p : a =_A b$ has a reverse `sym p = λ i → p (~ i)` definitionally
- The path space is *symmetric*: $a =_A b \simeq b =_A a$
- Undirected: no distinction between "going from $a$ to $b$" and "going from $b$ to $a$"

**The directed interval $\mathbf{2}$** (also called the simplicial interval):
- Endpoints $0_\mathbf{2}, 1_\mathbf{2} : \mathbf{2}$
- An ordering: there is a morphism $0_\mathbf{2} \leq 1_\mathbf{2}$, but *not* $1_\mathbf{2} \leq 0_\mathbf{2}$
- **No complement**: there is no $\sim$ operation
- A *directed path* $f : \mathsf{hom}_A(a, b)$ from $a$ to $b$ has no automatic reverse
- The hom type is *asymmetric*: $\mathsf{hom}_A(a, b)$ and $\mathsf{hom}_A(b, a)$ are genuinely different types
- Directed: going from $a$ to $b$ is distinct from going from $b$ to $a$

Simplicial type theory has *both* intervals. The undirected interval $\mathbb{I}$ handles ordinary homotopy theory (HoTT paths, equivalences, the Univalence Axiom). The directed interval $\mathbf{2}$ handles categorical morphisms (functors, natural transformations, adjunctions).

## The Two-Level Structure

Simplicial type theory (STT) has a layered architecture:

**The outer level** (also called the *tope layer* or *shape layer*): An extensional type theory that reasons about *combinatorial shapes* — simplices, their faces and horns, and the directed interval. The outer level has *strict* equality (decidable, computational). Types at this level are the shapes: $\mathbf{1}$ (a point), $\mathbf{2}$ (the directed interval), $\Delta^2$ (the 2-simplex), $\Lambda^2_1$ (the inner horn), etc.

**The inner level** (the *homotopy layer*): Ordinary HoTT, possibly extended with the undirected interval $\mathbb{I}$. Types at this level are spaces: ∞-groupoids, Segal types, Rezk types. The inner level has *homotopical* equality (paths, homotopies, equivalences).

The outer level provides the *index shapes* for the inner level constructions. A type in the inner level is "indexed by" shapes from the outer level via the extension type construction.

In Rzk, this two-level structure is explicit in the syntax: outer-level expressions appear inside `#!rzk` blocks or in specific position within type annotations.

## The Directed Interval $\mathbf{2}$ in Detail

The directed interval $\mathbf{2}$ models the totally ordered poset $\{0 < 1\}$. In the outer level, it has:

- Two terms: $0_\mathbf{2} : \mathbf{2}$ and $1_\mathbf{2} : \mathbf{2}$
- A comparison: the proposition $(t \leq s)$ for $t, s : \mathbf{2}$, with:
  - $0_\mathbf{2} \leq 0_\mathbf{2}$, $1_\mathbf{2} \leq 1_\mathbf{2}$ (reflexivity)
  - $0_\mathbf{2} \leq 1_\mathbf{2}$ (the single non-trivial order relation)
  - *Not* $1_\mathbf{2} \leq 0_\mathbf{2}$ (antisymmetry)

The interval $\mathbf{2}$ is *not* a type in the inner level — it is a shape in the outer level. Functions from $\mathbf{2}$ into an inner type $A$ produce the morphisms of $A$.

## The Hom Type

The central construction of simplicial type theory:

**Definition.** For a type $A$ and elements $a, b : A$, the *hom type* is:

$$\mathsf{hom}_A(a, b) :\equiv \{ f : \mathbf{2} \to A \mid f(0_\mathbf{2}) = a \text{ and } f(1_\mathbf{2}) = b \}$$

An element of $\mathsf{hom}_A(a, b)$ is a *directed path* from $a$ to $b$ — a map from the directed interval to $A$ with specified endpoints.

In the notation of extension types (defined below):
$$\mathsf{hom}_A(a, b) :\equiv \langle \partial \mathbf{2} \to [0_\mathbf{2} \mapsto a, 1_\mathbf{2} \mapsto b] \rangle_{\mathbf{2} \to A}$$

where $\partial \mathbf{2} = \{0_\mathbf{2}, 1_\mathbf{2}\}$ is the boundary of the directed interval.

**Key asymmetry**: $\mathsf{hom}_A(a, b)$ and $\mathsf{hom}_A(b, a)$ are *different types*. An element of one does not automatically give an element of the other. This is the directionality of categorical morphisms, built into the type theory.

**The identity morphism**: For any $a : A$, the constant function $\mathsf{id}_a :\equiv \lambda t. \, a : \mathsf{hom}_A(a, a)$. At $0_\mathbf{2}$: $a$. At $1_\mathbf{2}$: $a$. ✓

## Extension Types

*Extension types* are the technical primitive that ties the outer and inner levels together.

**Definition.** Given:
- An outer-level shape $\psi$ (e.g., $\Delta^n$, $\Lambda^2_1$, $\mathbf{2}$)
- A sub-shape $\phi \subseteq \psi$
- An inner-level type family $A : \psi \to \mathsf{Type}$
- A partial section $f : (t : \phi) \to A(t)$

The *extension type* is:
$$\langle \phi \to f \rangle_{\psi \to A}$$
the type of sections $s : (t : \psi) \to A(t)$ such that $s(t) = f(t)$ for all $t : \phi$.

**Special cases:**
- When $\phi = \emptyset$: the extension type is $(t : \psi) \to A(t)$ (no constraint)
- When $\phi = \psi$: the extension type is the singleton $\{f\}$ (the unique extension is $f$ itself)
- When $\phi = \partial \mathbf{2}$ and $\psi = \mathbf{2}$: the extension type is $\mathsf{hom}_A(a, b)$ (paths from $a$ to $b$)

Extension types are how "horn-filling conditions" are stated. The Segal condition, the Rezk condition, and all horn-filling requirements are instances of extension type equivalences.

## Simplices and Horns

The key shapes for category theory:

**The 2-simplex $\Delta^2$**:
$$\Delta^2 = \{ (t_1, t_2) : \mathbf{2} \times \mathbf{2} \mid t_1 \leq t_2 \}$$

A map $\Delta^2 \to A$ is a *2-simplex in $A$*: a triangle with three vertices $a_{00}, a_{01}, a_{11}$ (the corners at $(0,0), (0,1), (1,1)$ — noting $\Delta^2$ lives in the square $\mathbf{2}^2$ restricted by $t_1 \leq t_2$) and three directed edges:
- $(t_1 = 0)$-face: the edge from $(0, 0)$ to $(0, 1)$ — a morphism $f : a_{00} \to a_{01}$
- $(t_2 = 1)$-face: the edge from $(0, 1)$ to $(1, 1)$ — a morphism $g : a_{01} \to a_{11}$
- Diagonal: the hypotenuse from $(0, 0)$ to $(1, 1)$ — a morphism $h : a_{00} \to a_{11}$

The interior of the 2-simplex provides a coherence: a "witness" that $h$ is the composite of $f$ then $g$.

**The inner horn $\Lambda^2_1$**:
$$\Lambda^2_1 = \{ (t_1, t_2) : \Delta^2 \mid t_1 = 0 \text{ or } t_2 = 1 \}$$

A map $\Lambda^2_1 \to A$ gives exactly the "two legs" of a composable pair:
- $f : \mathsf{hom}_A(a, b)$ (from the $t_1 = 0$ face)
- $g : \mathsf{hom}_A(b, c)$ (from the $t_2 = 1$ face)

The *hypotenuse* — the composite morphism $g \circ f : \mathsf{hom}_A(a, c)$ — is *missing*. An extension of this map to all of $\Delta^2$ provides exactly the hypotenuse and witnesses that it is the composite.

## From Paths to Morphisms

A fundamental question in STT: how are the two kinds of paths related?

**Every HoTT path gives a morphism.** If $p : a =_A b$ (an undirected path), there is a function:
$$\alpha : (a =_A b) \to \mathsf{hom}_A(a, b)$$
that "forgets the direction" — or rather, interprets the path as a morphism going one way. In the two-level framework, this is given by a coercion from the undirected interval to the directed interval.

**Not every morphism is a path.** In a Segal type, there may be morphisms $f : \mathsf{hom}_A(a, b)$ that do not come from any HoTT path. These are the genuinely non-invertible morphisms — the arrows that have no categorical inverse.

**Invertible morphisms.** A morphism $f : \mathsf{hom}_A(a, b)$ is an isomorphism if there exists $g : \mathsf{hom}_A(b, a)$ with $g \circ f = \mathsf{id}_a$ and $f \circ g = \mathsf{id}_b$. The type of isomorphisms from $a$ to $b$ in a Segal type is:
$$\mathsf{Iso}_A(a, b) :\equiv \Sigma_{f : \mathsf{hom}_A(a, b)}\; \Sigma_{g : \mathsf{hom}_A(b, a)}\; (g \circ f = \mathsf{id}_a) \times (f \circ g = \mathsf{id}_b)$$

The *Rezk condition* (Section 3) says that in a Rezk type, $\alpha$ is an equivalence onto the isomorphisms: every isomorphism comes from a path.

## Rzk: The Implementation

In the Rzk proof assistant, the two-interval structure appears as follows:

```rzk
-- The directed interval
#def 2 : Type := Fin 2  -- or defined as a primitive

-- The hom type for a type A and elements x y : A
#def hom (A : Type) (x y : A) : Type
  := (t : 2) -> A [ ∂ t |-> recBOT ]  -- with endpoint conditions
```

In practice, Rzk's syntax is designed to make extension types and horn-filling conditions readable:

```rzk
-- A 2-simplex in A (a composable pair with a filler)
#def Δ² (A : Type) : Type
  := (t : Δ²) -> A

-- The inner horn map (restriction)
#def horn-restriction (A : Type)
  : (Δ² -> A) -> (Λ²₁ -> A)
  := \ f -> \ t -> f t
```

The Segal condition then says this restriction is an equivalence.

## Why Directionality Is the Right Primitive

One might wonder: why not define categories as types equipped with extra structure (a composition map, associativity proofs, etc.)? This is the classical approach.

The answer: the classical approach has a *coherence problem*. When you define a bicategory or a tricategory by adding structure, you must specify associators, unitors, and verify all the coherence conditions (pentagon identity, etc.). As the dimension grows, the coherence conditions explode combinatorially.

The simplicial approach avoids this by making composition *unique* (not just specified): the Segal condition says there is *exactly one* (up to contractibility) filler for any composable pair. When composition is uniquely determined by the data, there are no coherence conditions to verify — uniqueness takes care of all coherences automatically.

The directed interval $\mathbf{2}$ is what makes this work. By encoding direction into the foundation, the entire apparatus of coherence problems is replaced by a single, clean, compositional condition on horn-fillers.
