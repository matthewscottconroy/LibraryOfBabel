# 5.1 Homotopies and Function Extensionality

## Pointwise Equality of Functions

In mathematics, two functions $f, g : A \to B$ are equal if and only if $f(x) = g(x)$ for all $x$. This seems obvious — what else would it mean for two functions to be equal? But in MLTT, this is not provable from J alone, and understanding why reveals something deep about the structure of type theory.

**Definition.** A *homotopy* from $f$ to $g$ (both of type $A \to B$) is a term:

$$H : \prod_{x:A} f(x) =_B g(x)$$

A homotopy says: for every input $x$, the outputs $f(x)$ and $g(x)$ are equal. This is *pointwise equality*.

**Example.** Let $f = \lambda n. n + 0$ and $g = \mathsf{id}_{\mathbb{N}} = \lambda n. n$. The proof that $n + 0 = n$ for all $n$ (by induction, as we computed in Section 5.1 of Chapter 8) is a homotopy $H : f \sim g$.

**Notation.** We write $f \sim g$ for the type of homotopies from $f$ to $g$:
$$f \sim g = \prod_{x:A} f(x) = g(x)$$

## Function Extensionality

**Axiom (Function Extensionality, funext).** Homotopic functions are equal:

$$\mathsf{funext} : (f \sim g) \to (f = g)$$

for all $f, g : \prod_{x:A} B(x)$.

This says: if you have a homotopy (a proof that $f$ and $g$ agree pointwise), you get an actual identity proof $f = g$.

**Is funext provable from J?** No. In the *set-theoretic model* of MLTT (where types are interpreted as sets and equality is set equality), funext holds — but it's not a theorem in the type theory itself. There are models of MLTT where funext fails.

**Why funext fails without extra axioms:** To prove $f = g$ in the identity type $\prod_{x:A} B(x)$, you'd need to use J on the identity type of the function type. But J only lets you induct over paths *after* you have one — it doesn't help you create a path from pointwise data. There's a fundamental difference between "a path in $A \to B$" (an element of the function type's identity type) and "a function from $A$ to paths in $B$" (a homotopy).

## Funext as a Consequence of Univalence

In HoTT, function extensionality is *not* an axiom — it's a *theorem* that follows from the Univalence Axiom (Chapter 11).

The rough argument: Univalence says that paths in the universe $\mathsf{Type}$ correspond to equivalences of types. If $f \sim g$ (pointwise equal), then $f$ and $g$ determine the same function from a homotopy-theoretic perspective. The path space of $\prod_{x:A} B(x)$ in the universe, computed using Univalence, is exactly the type of homotopies.

More precisely: HoTT's Univalence Axiom, combined with function extensionality for *equivalences* (which does follow from Univalence), implies function extensionality for all functions.

## Funext in Cubical Type Theory

In cubical type theory (Chapter 24), function extensionality holds *without any additional axiom*. The reason: in cubical type theory, the identity type $f = g$ is not defined by J from first principles — instead, it's defined using *cubical interval* structure, and a path between functions is literally a family of functions parameterized by the interval $[0, 1]$.

Under this definition, a homotopy $H : f \sim g$ is immediately a path $\lambda i. \lambda x. H(x)_i$: at each interval point $i$, you get the function that sends $x$ to the $i$-th point of the path $H(x)$. When $i = 0$, you get $f$; when $i = 1$, you get $g$. So the homotopy is literally a path in the function type.

This is why cubical type theory has computational advantages for HoTT: many axioms (Univalence, funext) become theorems with explicit computational rules.

## Homotopy as a Type

The type of homotopies $f \sim g = \prod_{x:A} f(x) = g(x)$ is itself a type in MLTT. This means:
- Homotopies can themselves have homotopies (second-level homotopies, or homotopies between homotopies)
- You can form $\Sigma$ types of homotopies
- You can prove theorems about homotopies by induction

**Example: Homotopy composition.** If $H : f \sim g$ and $K : g \sim h$, then $H \cdot K : f \sim h$ defined by $(H \cdot K)(x) = H(x) \cdot K(x)$ (concatenate paths pointwise).

**Example: Whiskering.** Given $H : f \sim g$ (functions $A \to B$) and $k : B \to C$, the *right-whiskered homotopy* is $k \circ H : k \circ f \sim k \circ g$, defined by $(k \circ H)(x) = \mathsf{ap}_k(H(x))$.

These operations on homotopies mirror operations on paths, reflecting the categorical structure of dependent type theory.

## Quasi-Inverses and Equivalences

Function extensionality matters most when studying equivalences.

**Definition.** A function $f : A \to B$ has a *quasi-inverse* if there exists $g : B \to A$ with homotopies $H : f \circ g \sim \mathsf{id}_B$ and $K : g \circ f \sim \mathsf{id}_A$.

The type:
$$\mathsf{qinv}(f) = \sum_{g : B \to A} (f \circ g \sim \mathsf{id}_B) \times (g \circ f \sim \mathsf{id}_A)$$

This is the type of quasi-inverse data for $f$. An element of $\mathsf{qinv}(f)$ is a proof that $f$ is an isomorphism (in the type-theoretic sense).

**Problem with quasi-inverses:** $\mathsf{qinv}(f)$ can have multiple elements even when $f$ is an equivalence. For example, if $f = \mathsf{id}_A$, the quasi-inverse data is a choice of $g, H, K$, and multiple such choices may exist (differing by non-trivial homotopies).

**Solution: Equivalences ($\simeq$).** The correct notion of isomorphism in HoTT is the type of *equivalences*:

$$A \simeq B = \sum_{f : A \to B} \mathsf{isEquiv}(f)$$

where $\mathsf{isEquiv}(f)$ is a *contractible* type whenever $f$ is an isomorphism. Several equivalent definitions work:
- **Half-adjoint equivalences:** $g$ with $H, K$ as above, plus a coherence $\prod_{a:A} \mathsf{ap}_f(K(a)) = H(f(a))$
- **Bi-invertible maps:** $g_1$ with $f \circ g_1 \sim \mathsf{id}_B$ and $g_2$ with $g_2 \circ f \sim \mathsf{id}_A$

With any of these definitions, $\mathsf{isEquiv}(f)$ is a mere proposition (h-proposition): any two proofs it's an equivalence are equal. This is what makes $\simeq$ the right notion for Univalence.

## Funext and Propositional Uniqueness for Π

With function extensionality, we get:

**Theorem.** The $\eta$-rule for Π types holds *propositionally*: for $f : \prod_{x:A} B(x)$,
$$f = \lambda x. f\, x$$

*Proof:* The homotopy $H(x) = \mathsf{refl}_{f(x)} : f(x) = (\lambda x. f\, x)(x)$ is trivial. By funext, $f = \lambda x. f\, x$.

Without funext, this might only hold definitionally (as part of the $\eta$-rule in the type theory), not propositionally. Funext upgrades it.

## Dependent Function Extensionality

The same issue arises for dependent functions. If $f, g : \prod_{x:A} B(x)$ and $H : \prod_{x:A} f(x) = g(x)$ (a homotopy), then funext gives $f = g$.

For dependent funext, we need $\mathsf{apd}$ rather than $\mathsf{ap}$: if $p : f = g$, then for each $x$, $\mathsf{apd}_p(x) : f(x) = g(x)$ (over $\mathsf{refl}$... actually this needs more care with the dependent case).

The precise statement: the function
$$\mathsf{happly} : (f = g) \to (f \sim g)$$
defined by $\mathsf{happly}(p)(x) = \mathsf{ap}_{\mathsf{ev}_x}(p)$ (where $\mathsf{ev}_x(h) = h(x)$ is the evaluation-at-$x$ function) is an equivalence.

Function extensionality asserts that $\mathsf{happly}$ is an equivalence: its inverse is $\mathsf{funext}$. This is the precise form of funext in HoTT: not just that funext exists, but that it's inverse to happly, making funext and happly into an equivalence.

## Why Funext Matters

Function extensionality is pervasive in mathematics. Without it, you can't:
- Prove that two functions defined by the same recursive equations are equal (without an explicit identity proof from J)
- Identify the type $\mathbb{N} \to \mathbb{N}$ with "sequences of natural numbers" (where two sequences are equal iff they agree everywhere)
- State the universal property of function types (the hom-set adjunction in category theory)
- Prove that homotopy is an equivalence relation (the transitivity step requires concatenating function types)

With funext (as an axiom or theorem from Univalence):
- Any two functions that agree pointwise are propositionally equal
- The $\eta$-rule holds propositionally
- Functional programming and mathematical reasoning about functions are fully aligned

## Homotopies Between Dependent Functions

For dependent functions $f, g : \prod_{x:A} B(x)$, a homotopy is:
$$H : \prod_{x:A} f(x) =_{B(x)} g(x)$$

For each $x$, $H(x)$ is a path from $f(x)$ to $g(x)$ in the fiber $B(x)$. This is a *fiberwise* homotopy.

The homotopy $H$ is a term of type $f \sim g$, which is a Π type (as always). Funext says this Π type is equivalent to $f = g$ in the function type.

## Naturality of Homotopies

Homotopies interact nicely with other operations. One key property:

**Naturality square.** If $H : f \sim g$ (functions $A \to B$) and $p : a_1 = a_2$ (a path in $A$), then the following *commutes* (as a path between paths):

$$H(a_1) \cdot \mathsf{ap}_g(p) = \mathsf{ap}_f(p) \cdot H(a_2)$$

This is a homotopy coherence condition: the square formed by the homotopy on the two objects and $\mathsf{ap}$ on the two paths commutes (up to a 2-path).

In categorical language: a homotopy $H$ is a *natural transformation* between functors $f$ and $g$ (viewed as functors between fundamental groupoids). Naturality says the obvious square commutes.

This naturality condition is used extensively in HoTT proofs, especially when dealing with loop spaces and higher homotopy groups.

## Summary

| Concept | Type/Definition | Status in MLTT |
|---|---|---|
| Homotopy $f \sim g$ | $\prod_{x:A} f(x) = g(x)$ | Definable (Π type) |
| Function extensionality | $(f \sim g) \to (f = g)$ | Not provable from J alone |
| funext in HoTT | Theorem from Univalence | Theorem (Chap. 11) |
| funext in cubical TT | Computational rule | Theorem with computation |
| Equivalence $A \simeq B$ | $\sum_{f:A\to B} \mathsf{isEquiv}(f)$ | Definable |
| happly | $(f = g) \to (f \sim g)$ | Derivable from ap |
| funext = (happly)⁻¹ | Equivalence | HoTT axiom/theorem |

Function extensionality is one of the places where basic MLTT is genuinely incomplete from the perspective of mathematical practice. HoTT fills this gap via Univalence, making funext a theorem rather than an axiom — one of the key benefits of the univalent foundations program.
