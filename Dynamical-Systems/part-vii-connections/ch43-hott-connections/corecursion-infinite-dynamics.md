# 43.2 Corecursion and Infinite Dynamical Systems

## 43.2.1 Coinductive Types

How do you represent an infinite dynamical orbit in type theory? The answer is coinductive types, defined by their destructors (projections) rather than by constructors. The canonical example is the type of streams — infinite sequences.

**Definition 43.2.1.** A *coinductive type* (or *codata type*) is defined by its *destructors* (projections) rather than its *constructors*. The canonical example is the type of streams:

```haskell
-- Codata (coinductive) definition:
Stream A := { head : A, tail : Stream A }
```

A stream of $A$s is an element with a head (of type $A$) and a tail (another stream of $A$s).

The key difference from inductive types: an inductive type is defined by how you build it (constructors); a coinductive type is defined by how you observe it (destructors). A stream is not defined by "how you make a stream" but by "what you can do with a stream" — extract the head, and recursively process the tail.

**Theorem 43.2.2 (Bisimulation = Equality for Streams).** Two streams $s, t: \text{Stream}(A)$ are *bisimilar* if there is a relation $R$ with $(s, t) \in R$ and whenever $(s', t') \in R$: $\text{head}(s') = \text{head}(t')$ and $(\text{tail}(s'), \text{tail}(t')) \in R$.

In type theory with coinduction, bisimulation implies equality: bisimilar streams are provably equal. This is the *coinduction principle*.

**Connection to Dynamical Systems:** A stream $s: \text{Stream}(A)$ is the orbit of a point under a dynamics. The head is the current state, the tail is the future orbit. Bisimulation is the stream-version of topological conjugacy: two orbits are "the same dynamics" if they are bisimilar.

## 43.2.2 Corecursion as Orbit Generation

**Definition 43.2.3.** A *corecursive* definition of a stream uses a *coalgebra* map $f: B \to A \times B$ (giving the next state and current output):

```
unfold : (B → A × B) → B → Stream A
unfold f b = let (a, b') = f(b) in a :: unfold f b'
```

**Example 43.2.4 (Doubling Map Stream).** The doubling map $T: [0,1] \to [0,1]$, $T(x) = 2x \pmod 1$, generates the binary expansion of $x$:

```
doublingBits : [0,1] → Stream {0,1}
doublingBits x = unfold (λx. (⌊2x⌋, 2x mod 1)) x
```

This corecursive definition computes the orbit of $x$ under the doubling map — the symbolic coding with respect to the partition $\{[0,1/2), [1/2,1)\}$.

The symbolic orbit is a stream: each step, you extract the current bit (which half of $[0,1]$ you're in) and update the state (apply the doubling map). The corecursive `unfold` captures this exactly. And bisimulation of streams corresponds to topological conjugacy: two starting points $x, y$ produce the same stream iff they have the same symbolic orbit iff they are in the same orbit equivalence class for the doubling map.

This gives a type-theoretic formulation of symbolic dynamics: a symbolic coding is a coalgebra map from phase space to the alphabet, and bisimulation is topological conjugacy.
