# 1.1 Two Intervals and the Hom Type

## The Cubical vs. Simplicial Intervals

We have already met the cubical interval $\mathbb{I}$ in Chapters 22–23. Let's contrast it with the simplicial interval $\mathbf{2}$.

**The cubical interval $\mathbb{I}$.** This is the interval of cubical type theory. It has:
- Two endpoints: $0 : \mathbb{I}$ and $1 : \mathbb{I}$
- A complement $\sim r : \mathbb{I}$ (the De Morgan negation)
- Meet $\wedge$ and join $\vee$

The complement is what makes $\mathbb{I}$ *undirected*: given a path $p : a \to b$ in $\mathbb{I}$, the reverse path `sym p = λ i → p (~ i)` automatically exists. Every path is reversible. This is the right structure for homotopy theory, where paths model homotopies between points.

**The simplicial interval $\mathbf{2}$.** The simplicial interval models the totally ordered poset $\{0 < 1\}$. It has:
- Two terms: $0_\mathbf{2}$ and $1_\mathbf{2}$
- A comparison: there is a morphism from $0_\mathbf{2}$ to $1_\mathbf{2}$, but *not* from $1_\mathbf{2}$ to $0_\mathbf{2}$
- **No complement**: there is no $\sim r$ for elements of $\mathbf{2}$

The absence of complement is what makes $\mathbf{2}$ *directed*: a path in $\mathbf{2}$ goes one way and there's no automatic reversal. This is the right structure for category theory, where morphisms model directed arrows.

Geometrically:
- $\mathbb{I}$: the undirected interval $[0, 1]$ with a reflection symmetry
- $\mathbf{2}$: the directed interval $0 \to 1$ with no symmetry

## Simplicial Type Theory: The Framework

Simplicial type theory extends ordinary HoTT with:
1. The simplicial interval $\mathbf{2}$ as a new primitive
2. *Extension types* for specifying partial functions over simplicial shapes
3. The *Segal condition* as a property types can have

The system has two levels:
- The *outer* level: an extensional type theory (used for the shapes/simplices)
- The *inner* level: HoTT (used for the types)

The outer level reasons about combinatorial shapes (simplices, their faces). The inner level reasons about spaces (types with homotopy structure). This two-level structure is explicit in STT and in the Rzk proof assistant.

## The Hom Type

The central construction is the *hom type*, which captures directed paths:

**Definition.** For a type $A$ and elements $a, b : A$, the *hom type* is:

$$\mathsf{hom}_A(a, b) :\equiv \left\{ f : \mathbf{2} \to A \;\middle|\; f(0_\mathbf{2}) = a \text{ and } f(1_\mathbf{2}) = b \right\}$$

An element of $\mathsf{hom}_A(a, b)$ is a directed path from $a$ to $b$ — a map from the directed interval $\mathbf{2}$ to $A$ with specified endpoints.

Using extension types (discussed below), this is written:

$$\mathsf{hom}_A(a, b) :\equiv \left\langle \partial \mathbf{2} \to [0 \mapsto a, 1 \mapsto b] \right\rangle_{\mathbf{2} \to A}$$

where $\partial \mathbf{2} = \{0_\mathbf{2}, 1_\mathbf{2}\}$ is the boundary of $\mathbf{2}$ and we extend to all of $\mathbf{2}$.

**Asymmetry.** The types $\mathsf{hom}_A(a, b)$ and $\mathsf{hom}_A(b, a)$ are *different*. An element of one does not give an element of the other. This is directedness.

**Contrast with the path type.** For the undirected path type $a =_A b$, we have:
- `sym : (a =_A b) → (b =_A a)` (automatic symmetry)
- `a =_A b ≃ b =_A a` (the path space is symmetric)

For the hom type $\mathsf{hom}_A(a, b)$:
- No automatic reversal
- $\mathsf{hom}_A(a, b)$ and $\mathsf{hom}_A(b, a)$ can be very different (one empty, one not)

## Extension Types

Extension types are the key technical primitive in STT. They generalize partial functions to the simplicial setting.

**The extension type.** Given:
- A *shape* $\psi$ (a simplicial shape, e.g., the standard $n$-simplex $\Delta^n$)
- A *sub-shape* $\phi \subseteq \psi$ (a horn, a face, etc.)
- A type family $A : \psi \to \mathsf{Type}$
- A partial section $f : \Pi_{t:\phi} A(t)$

The extension type is:

$$\left\langle \phi \to f \right\rangle_{\psi \to A}$$

the type of *sections* of $A$ over $\psi$ that extend $f$ over $\phi$.

**Special cases:**
- $\phi = \emptyset$: the extension type is $\Pi_{t:\psi} A(t)$ (no constraints)
- $\phi = \psi$: the extension type is the singleton $\{f\}$ (unique extension — $f$ itself)
- $\phi = \{0, 1\}$, $\psi = [0,1]$, $f = [0 \mapsto a, 1 \mapsto b]$: the extension type is $\mathsf{hom}_A(a, b)$ (paths with fixed endpoints)

**The horn filling condition.** For a type $A$ to be "Kan" (to have well-behaved composition), we need horn inclusions to have unique extensions. This is the Segal condition, discussed in Section 2.

## Simplices and Shapes

In STT, the simplicial shapes are described using the outer (extensional) level:

- **$\mathbf{1}$**: a single point (the 0-simplex, $\Delta^0$)
- **$\mathbf{2}$**: the directed interval ($\Delta^1$, a directed edge)
- **$\Delta^2$**: the standard 2-simplex (a filled triangle with directed edges)
- **$\Lambda^2_1$**: the inner horn (two composable directed edges, without the composite)

These are not just combinatorial objects; they appear as types (or rather, as "topes" in the outer level).

**The 2-simplex $\Delta^2$.** Elements of $\Delta^2$ parametrize 2-simplices in a type. $\Delta^2$ is defined as:

$$\Delta^2 = \{ (t_1, t_2) : \mathbf{2} \times \mathbf{2} \mid t_1 \leq t_2 \}$$

(pairs in the directed interval with the ordering condition). An element $(t_1, t_2) : \Delta^2$ represents a point in the 2-simplex.

**The inner horn $\Lambda^2_1$.** This is the sub-shape of $\Delta^2$ missing the "composite edge":

$$\Lambda^2_1 = \{ (t_1, t_2) : \Delta^2 \mid t_1 = 0 \text{ or } t_2 = 1 \}$$

An element of $\Lambda^2_1 \to A$ specifies:
- A morphism $f : \mathsf{hom}_A(a, b)$ (from $t_1 = 0$ face)
- A morphism $g : \mathsf{hom}_A(b, c)$ (from $t_2 = 1$ face)

This is a "composable pair" — $f$ and $g$ are composable because they share the middle vertex $b$.

An element of $\Delta^2 \to A$ extending this composable pair is:
- A 2-simplex in $A$ — a triangle with vertices $a, b, c$
- A composite morphism $h : \mathsf{hom}_A(a, c)$ (the hypotenuse)
- Plus the homotopy $g \circ f \sim h$ (the interior of the triangle)

The Segal condition says: every composable pair has a *unique* (up to contractibility) 2-simplex filling it.

## The Identity Morphism

In a Segal type $A$, the identity morphism at $a : A$ is the constant path:

$$\mathsf{id}_a :\equiv \lambda t. a : \mathsf{hom}_A(a, a)$$

Check: $\mathsf{id}_a(0) = a$ and $\mathsf{id}_a(1) = a$. ✓

The identity is definable without any axiom — it's just the constant function from $\mathbf{2}$ to $A$.

**Identity laws.** For $f : \mathsf{hom}_A(a, b)$:
- $\mathsf{id}_b \circ f = f$ (left identity)
- $f \circ \mathsf{id}_a = f$ (right identity)

These follow from the Segal condition: composition is unique, and the compositions $\mathsf{id}_b \circ f$ and $f$ both fill the same horn (the horn with edges $f$ and $\mathsf{id}_b$). Since the filler is unique, they're equal.

## From Paths to Morphisms

The key question: how do undirected paths in HoTT relate to directed morphisms in STT?

**Every path gives a morphism.** If $p : a =_A b$ (an undirected path), then we can forget the direction and get a morphism $\mathsf{hom}_A(a, b)$. In STT, there is a function:

$$\alpha : (a =_A b) \to \mathsf{hom}_A(a, b)$$

mapping undirected paths to directed morphisms.

**Not every morphism is a path.** In a directed type, there may be morphisms $f : \mathsf{hom}_A(a, b)$ that don't come from any path. These are non-invertible morphisms.

**Invertible morphisms.** A morphism $f : \mathsf{hom}_A(a, b)$ is an *isomorphism* if there exists $g : \mathsf{hom}_A(b, a)$ with $g \circ f = \mathsf{id}_a$ and $f \circ g = \mathsf{id}_b$. The type of isomorphisms from $a$ to $b$ is:

$$\mathsf{Iso}_A(a, b) :\equiv \{ (f, g, \_, \_) \mid f : \mathsf{hom}_A(a,b), g : \mathsf{hom}_A(b,a), g \circ f = \mathsf{id}_a, f \circ g = \mathsf{id}_b \}$$

**The Rezk condition** (Section 3) says: in a "good" Segal type, isomorphisms correspond exactly to paths: $\mathsf{Iso}_A(a, b) \simeq (a =_A b)$. This is the categorical analogue of univalence.

## Summary: HoTT vs. STT

| Concept | HoTT | STT |
|---------|------|-----|
| Path $a = b$ | Undirected, symmetric | $=_A b$ still exists |
| Morphism $a \to b$ | Path (symmetric) | $\mathsf{hom}_A(a, b)$ (directed) |
| Interval | $\mathbb{I}$ with $\sim$ | $\mathbf{2}$ without $\sim$ |
| Reversal | Automatic (via $\sim$) | Only for invertible morphisms |
| Identity | $\mathsf{refl}$ | $\mathsf{id}_a = \lambda t. a$ |
| Composition | Path concatenation $\cdot$ | Unique 2-simplex (Segal) |
| "Good types" | All types | Segal types (∞-categories) |
| "Coherent types" | All types | Rezk types (univalent ∞-cats) |

Simplicial type theory doesn't replace HoTT — it *extends* it. Every type in STT is still an ∞-groupoid (with the undirected path type). But some types have additional Segal structure making them ∞-categories.
