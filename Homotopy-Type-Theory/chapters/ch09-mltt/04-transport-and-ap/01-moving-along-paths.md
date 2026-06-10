# 4.1 Transport and ap: Moving Along Paths

## The Two Fundamental Derived Operations

From the J rule, we can derive many operations on identity proofs. But two are fundamental — they appear in almost every argument in type theory and HoTT:

1. **Transport** ($\mathsf{transport}$): moving an element of a type family along a path
2. **ap** (action on paths): applying a function to a path

Both are consequences of J alone. They express, in type-theoretic language, two basic homotopy-theoretic ideas: parallel transport along a path, and the functoriality of continuous maps.

## Transport

**Definition.** Given a type family $P : A \to \mathsf{Type}$ and a path $p : a =_A b$, there is a function:

$$\mathsf{transport}^P(p) : P(a) \to P(b)$$

**Construction via J:** Apply J with:
- Motive: $C(b', p') = P(a) \to P(b')$ — for each endpoint $b'$ and path $p'$ from $a$ to $b'$, the type of functions from $P(a)$ to $P(b')$
- Base case: $d = \mathsf{id}_{P(a)} : P(a) \to P(a)$ — the identity function at $a$

J gives: $\mathsf{J}(C, \mathsf{id}_{P(a)}) : \prod_{b:A} \prod_{p:a=b} P(a) \to P(b)$

Define $\mathsf{transport}^P(p) = \mathsf{J}(C, \mathsf{id}_{P(a)}, b, p)$.

**Computation:** $\mathsf{transport}^P(\mathsf{refl}_a) = \mathsf{id}_{P(a)}$ — transporting along the reflexivity path is the identity function.

## What Transport Does

Transport says: if $a$ and $b$ are identified (equal), then their "properties" are isomorphic. More precisely, the *fibers* of the family $P$ over $a$ and $b$ are connected by a function.

**Example 1: Transporting natural number properties.**

Let $P(n) = \mathsf{IsEven}(n)$ and $p : 4 = 4$ (reflexivity). Then $\mathsf{transport}^P(p) = \mathsf{id}$. Nothing interesting here.

But suppose $q : 2 + 2 = 4$ (a proof of this arithmetic identity). Then $\mathsf{transport}^P(q) : \mathsf{IsEven}(2 + 2) \to \mathsf{IsEven}(4)$: if $2 + 2$ is even, so is $4$ (as expected — they're equal!).

**Example 2: Transporting vectors along length proofs.**

Let $P(n) = \mathsf{Vec}(A, n)$ and $p : m = n$. Then $\mathsf{transport}^P(p) : \mathsf{Vec}(A, m) \to \mathsf{Vec}(A, n)$: a vector of length $m$ becomes a vector of length $n$, given a proof they're equal. The vector itself doesn't change — the proof is just being used to *retype* it.

**Example 3: Substitution of equals.**

Let $P(x) = (x =_A c)$ for a fixed $c : A$, and $p : a = b$. Then $\mathsf{transport}^P(p) : (a = c) \to (b = c)$: if $a = b$ and $a = c$, then $b = c$. This is substitution of equals — one of the most basic logical operations.

## Transport and the Leibniz Principle

The Leibniz Principle (or Leibniz's Law) in classical logic states: if $a = b$, then $a$ has a property if and only if $b$ has the property.

In type theory, transport makes this precise. For any type family $P : A \to \mathsf{Type}$:
- If $p : a = b$, then $\mathsf{transport}^P(p) : P(a) \to P(b)$ (forward direction)
- $\mathsf{transport}^P(p^{-1}) : P(b) \to P(a)$ (backward direction)

Moreover, these two functions are inverse: $\mathsf{transport}^P(p) \circ \mathsf{transport}^P(p^{-1}) = \mathsf{id}_{P(b)}$ and vice versa. So transport gives an *equivalence* $P(a) \simeq P(b)$ whenever $a = b$.

This is stronger than Leibniz's original formulation: not only do $a$ and $b$ have the same properties, but the evidence of $P(a)$ and $P(b)$ is explicitly *identified* via transport.

## Transport and Parallel Transport in Geometry

The geometric picture: a type family $P : A \to \mathsf{Type}$ is a fibration over the space $A$. Each point $a$ has a fiber $P(a)$ sitting over it. A path $p : a = b$ in the base space $A$ allows you to "lift" an element $u : P(a)$ in the fiber over $a$ to an element $\mathsf{transport}^P(p, u) : P(b)$ in the fiber over $b$.

This is *parallel transport* in differential geometry: given a connection on a fiber bundle, a path in the base space determines a linear map between the fibers at the endpoints. Type-theoretic transport is the algebraic version of this, with paths in the identity type playing the role of paths in the base space.

In HoTT, this is not just an analogy — it's the precise mathematical meaning. Types are spaces, type families are fibrations, and transport is exactly the parallel transport operation.

## Properties of Transport

**Transport along $\mathsf{refl}$ is identity:**
$$\mathsf{transport}^P(\mathsf{refl}_a) = \mathsf{id}_{P(a)}$$

**Transport respects concatenation:**
$$\mathsf{transport}^P(p \cdot q) = \mathsf{transport}^P(q) \circ \mathsf{transport}^P(p)$$

(First transport along $p$, then along $q$.)

**Transport respects inversion:**
$$\mathsf{transport}^P(p^{-1}) = (\mathsf{transport}^P(p))^{-1}$$

(The inverse of transport along $p$ is transport along $p^{-1}$, which is also the inverse function.)

**Transport in constant families:**
If $P(x) = C$ for all $x$ (constant family), then $\mathsf{transport}^P(p) = \mathsf{id}_C$ for all paths $p$. (A constant fibration has trivial transport.)

**Transport and path types:**
If $P(x) = (a = x)$ for fixed $a$, then $\mathsf{transport}^P(p) : (a = x_1) \to (a = x_2)$ for $p : x_1 = x_2$. This is pre-composition with $p^{-1}$: $q \mapsto q \cdot p$.

## The Action on Paths: ap

**Definition.** Given a function $f : A \to B$ and a path $p : a =_A b$, there is a path:

$$\mathsf{ap}_f(p) : f(a) =_B f(b)$$

**Construction via J:** Apply J with:
- Motive: $C(b', p') = f(a) =_B f(b')$ — for each $b'$ and path $p' : a = b'$, a path from $f(a)$ to $f(b')$
- Base case: $d = \mathsf{refl}_{f(a)} : f(a) = f(a)$ — the reflexivity path at $f(a)$

J gives: $\mathsf{J}(C, \mathsf{refl}_{f(a)}) : \prod_{b:A} \prod_{p:a=b} f(a) = f(b)$

Define $\mathsf{ap}_f(p) = \mathsf{J}(C, \mathsf{refl}_{f(a)}, b, p)$.

**Computation:** $\mathsf{ap}_f(\mathsf{refl}_a) = \mathsf{refl}_{f(a)}$.

## What ap Does

$\mathsf{ap}_f$ (also written $f_*$ or $f_\#$ in the literature) says that functions are *continuous*: they map paths to paths. If $a$ and $b$ are identified in $A$, then $f(a)$ and $f(b)$ are identified in $B$.

This is the type-theoretic version of the statement "every function between types is continuous" — a consequence of the topological interpretation of types. There are no discontinuous functions in MLTT (all definable functions are continuous in the appropriate sense).

**Example.** Let $f = \mathsf{succ} : \mathbb{N} \to \mathbb{N}$. If $p : m = n$, then $\mathsf{ap}_\mathsf{succ}(p) : \mathsf{succ}(m) = \mathsf{succ}(n)$. Applying $\mathsf{succ}$ to both sides of an equation gives a new equation.

**Example.** Let $f = \pi_1 : A \times B \to A$. If $p : (a_1, b_1) = (a_2, b_2)$, then $\mathsf{ap}_{\pi_1}(p) : a_1 = a_2$.

## Properties of ap

**ap preserves reflexivity:**
$$\mathsf{ap}_f(\mathsf{refl}_a) = \mathsf{refl}_{f(a)}$$

**ap respects concatenation:**
$$\mathsf{ap}_f(p \cdot q) = \mathsf{ap}_f(p) \cdot \mathsf{ap}_f(q)$$

**ap respects inversion:**
$$\mathsf{ap}_f(p^{-1}) = (\mathsf{ap}_f(p))^{-1}$$

**ap respects composition:**
$$\mathsf{ap}_g(\mathsf{ap}_f(p)) = \mathsf{ap}_{g \circ f}(p)$$

**ap on identity:**
$$\mathsf{ap}_{\mathsf{id}_A}(p) = p$$

These properties say that $\mathsf{ap}$ makes every function a *functor between groupoids*:
- It preserves the identity (reflexivity)
- It preserves composition (transitivity / concatenation)
- It respects inverses

In categorical language: every function $f : A \to B$ between types induces a functor $\mathsf{ap}_f$ from the fundamental groupoid of $A$ to the fundamental groupoid of $B$.

## ap for Dependent Functions: apd

For dependent functions $f : \prod_{x:A} B(x)$, the story is slightly more complex. If $f(a) : B(a)$ and $f(b) : B(b)$, these are in *different types* when $a \neq b$. You can't directly form a path between them without transport.

The *dependent action on paths* $\mathsf{apd}$ handles this:

$$\mathsf{apd}_f(p) : \mathsf{transport}^B(p, f(a)) =_{B(b)} f(b)$$

For $p : a = b$ and $f : \prod_{x:A} B(x)$, $\mathsf{apd}_f(p)$ says that $f(b)$ equals the transport of $f(a)$ along $p$.

**Construction via J:** Apply J with motive $C(b', p') = \mathsf{transport}^B(p', f(a)) = f(b')$ and base case $\mathsf{refl}_{f(a)}$.

When $B$ is a constant family (non-dependent), $\mathsf{transport}^B(p, f(a)) = f(a)$ (transport in constant families is identity), so $\mathsf{apd}_f(p) : f(a) = f(b)$ — this recovers $\mathsf{ap}_f(p)$.

## The Path-Over Perspective

$\mathsf{apd}_f(p)$ is a *path over $p$*: it's a path in the total space $\sum_{x:A} B(x)$ that lies over the path $p : a = b$ in the base.

Geometrically: if $f$ is a section of a fibration $B \to A$ (a function that assigns a point in each fiber), then $\mathsf{apd}_f$ lifts paths in the base to paths in the total space that respect the section. This is exactly the *lifting property* of fibrations in topology.

The path-over perspective is essential for HITs (Higher Inductive Types), where the elimination principle for path constructors produces paths-over-paths in a systematic way.

## Transport and Identity Proofs: The Groupoid Structure

Here's a way to see the relationship between transport and ap clearly.

If $P(x) = (a =_A x)$ (paths starting from a fixed $a$), then transport along $q : x_1 = x_2$ gives:

$$\mathsf{transport}^{P}(q) : (a = x_1) \to (a = x_2)$$

This function sends $r : a = x_1$ to $r \cdot q : a = x_2$ (concatenate $r$ with $q$). So transport in path spaces is concatenation on the right.

Similarly, if $P(x) = (x =_A c)$, transport along $q : x_1 = x_2$ gives:

$$\mathsf{transport}^{P}(q) : (x_1 = c) \to (x_2 = c)$$

This sends $r : x_1 = c$ to $q^{-1} \cdot r : x_2 = c$ (invert $q$ and concatenate on the left). So transport in path spaces (with the path endpoint varying on the left) is concatenation with the inverse on the left.

These computations, derived from J, show that transport is a concrete operation in terms of path concatenation when applied to path type families.

## Summary

| Operation | Type | Meaning |
|---|---|---|
| $\mathsf{transport}^P(p)$ | $P(a) \to P(b)$ for $p : a = b$ | Move along a path in a fibration |
| $\mathsf{ap}_f(p)$ | $f(a) = f(b)$ for $p : a = b$ | Functions map paths to paths |
| $\mathsf{apd}_f(p)$ | $\mathsf{transport}^B(p, f(a)) = f(b)$ | Dependent functions send paths to paths-over-paths |

All three are derived from J alone, using only the type theory of MLTT. The geometric picture — transport as parallel transport, ap as functoriality, apd as the lifting property — is the beginning of HoTT's synthetic approach to topology.

In the next sections, we'll see how homotopies (pointwise equalities between functions) fit into this picture, and why function extensionality (funext: homotopic functions are equal) is not provable in basic MLTT but becomes a theorem in cubical type theory and a consequence of Univalence in HoTT.
