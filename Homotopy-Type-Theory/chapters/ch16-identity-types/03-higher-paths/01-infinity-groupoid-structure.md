# 3.1 Higher Paths and the ∞-Groupoid Structure

## Paths All the Way Up

We've established that $a =_A b$ is a type. But what kind of type? If it's a "plain" type, it has its own elements and its own identity type.

Given $p, q : a =_A b$, the identity type $p =_{a=_Ab} q$ is the type of *2-paths* or *homotopies between paths*. An element $H : p = q$ is a proof that $p$ and $q$ are the same path, witnessed by a specific homotopy.

And $p =_{a=_Ab} q$ is itself a type, with its own identity type $H =_{p=q} K$ — the type of *3-paths* or *homotopies between homotopies*.

This tower never terminates (in general). This is the *∞-groupoid structure* of a type.

## The Tower of Identity Types

For a type $A$ with $a, b : A$ and $p, q : a = b$ and $H, K : p = q$:

**Level 0:** $A$ (the type itself) — objects
**Level 1:** $a =_A b$ — 1-paths (morphisms)  
**Level 2:** $p =_{a=b} q$ — 2-paths (homotopies between morphisms)
**Level 3:** $H =_{p=q} K$ — 3-paths (coherences between homotopies)
**Level $n$:** $n$-fold iterated identity types

Each level has groupoid structure (concatenation, inversion, groupoid laws), all derived from J. The structure at each level is compatible with the structure at adjacent levels.

**Formally.** For each $n \geq 0$, define the $n$th iterated loop space:
$$\Omega^0 A = A$$
$$\Omega^{n+1} A = \Omega(\Omega^n A) = \underbrace{\mathsf{refl}_{...} =_{...} \mathsf{refl}_{...}}_{\text{$n$ levels deep}}$$

Then $\pi_n(A, a) = \|\Omega^n A\|_0$ (the set of connected components of the $n$-fold loop space).

## Level 1: The Fundamental Groupoid

At level 1, the structure is:
- Objects: terms $a : A$
- Morphisms: paths $p : a = b$
- Composition: $p \cdot q : a = c$ (concatenation)
- Identity: $\mathsf{refl}_a : a = a$
- Inverse: $p^{-1} : b = a$

All with the groupoid laws from Section 2. This makes $A$ into a *groupoid* (in a weak, propositional sense).

## Level 2: 2-Cells and Coherences

At level 2, each identity type $a =_A b$ is itself a groupoid. The 2-paths $H : p = q$ can be:
- Concatenated (vertically): $H \cdot_v K : p = r$ for $H : p = q$ and $K : q = r$
- Concatenated (horizontally): using the groupoid structure in a different dimension

There are two distinct operations on 2-paths:
1. **Vertical composition:** Compose $H$ and $K$ as paths in the type $a = b$
2. **Horizontal composition (whiskering):** Given $H : p = q$ (paths in $a = b$) and a path $r : b = c$, form $H \star r : p \cdot r = q \cdot r$ (the homotopy "slid" by $r$)

These two operations satisfy the *interchange law*: vertical and horizontal composition commute in a suitable sense.

## The Eckmann-Hilton Argument in Detail

The interchange law between vertical and horizontal composition has a remarkable consequence at the level of 2-loops (2-paths from $\mathsf{refl}_a$ to $\mathsf{refl}_a$).

**Setup.** Fix $a : A$ and consider 2-paths $\alpha, \beta : \mathsf{refl}_a = \mathsf{refl}_a$. These are elements of $\Omega^2 A$.

There are two operations:
- $\alpha \cdot \beta$: vertical composition (concatenate as paths in the loop space)
- $\alpha \star \beta$: horizontal composition (compose the underlying paths and the 2-cells)

Both operations have unit $\mathsf{refl}_{\mathsf{refl}_a}$ and satisfy the interchange law:
$$(\alpha \cdot \beta) \star (\gamma \cdot \delta) = (\alpha \star \gamma) \cdot (\beta \star \delta)$$

**The Eckmann-Hilton argument.** From the interchange law:

1. Set $\gamma = \mathsf{refl}$ and $\delta = \beta$: $(\alpha \cdot \beta) \star (\mathsf{refl} \cdot \beta) = (\alpha \star \mathsf{refl}) \cdot (\beta \star \beta)$

Hmm, this isn't quite right. Let me do it properly.

Set $\alpha = \delta = \mathsf{refl}_{\mathsf{refl}_a}$:
$$(\mathsf{refl} \cdot \gamma) \star (\beta \cdot \mathsf{refl}) = (\mathsf{refl} \star \beta) \cdot (\gamma \star \mathsf{refl})$$

The left side: $\gamma \star \beta$ (using unit laws).
The right side: $\beta \cdot \gamma$ (using that whiskering by $\mathsf{refl}$ is identity).

So $\gamma \star \beta = \beta \cdot \gamma$.

Similarly, setting $\beta = \delta = \mathsf{refl}$:
$$(\alpha \cdot \mathsf{refl}) \star (\mathsf{refl} \cdot \gamma) = (\alpha \star \mathsf{refl}) \cdot (\mathsf{refl} \star \gamma)$$
So $\alpha \star \gamma = \alpha \cdot \gamma$.

Combining: $\alpha \star \gamma = \alpha \cdot \gamma = \gamma \star \alpha = \gamma \cdot \alpha$. So $\alpha \cdot \gamma = \gamma \cdot \alpha$.

**Conclusion.** The operation on $\Omega^2 A = (\mathsf{refl}_a = \mathsf{refl}_a)$ is commutative!

**Theorem 3.1 (Eckmann-Hilton).** For any type $A$ with $a : A$, the composition on $\pi_2(A, a) = \|\Omega^2 A\|_0$ is commutative. More generally, $\pi_n(A, a)$ is abelian for all $n \geq 2$.

This is a purely type-theoretic proof of a classical theorem in algebraic topology: higher homotopy groups are abelian.

## When Does the Tower Stabilize?

The tower of identity types continues indefinitely, but for many types, it stabilizes at some level:

**Propositions (h-level -1):** $a = b$ is contractible or empty. No interesting paths.

**Sets (h-level 0):** $a = b$ is a proposition (at most one element). No interesting 2-paths — the groupoid is discrete.

**1-Types (Groupoids, h-level 1):** $p = q$ is a proposition. No interesting 2-paths.

**$n$-Types:** The tower stabilizes at level $n$: $n$-fold identity types are contractible or empty.

**Infinite types:** Like $S^1$, $S^2$, the universe — the tower never truly stabilizes (though individual homotopy groups may vanish above a certain dimension).

## Path Induction at Higher Levels

The J rule applies at every level: to prove something about all 2-paths $H : p = q$, it suffices to prove it for $\mathsf{refl}_p : p = p$. And similarly for higher paths.

This is path induction applied iteratively. At each level, we have:
- Objects: the identity type from the previous level
- Reflexivity: the reflexivity of the previous level's paths
- J rule: path induction on the current level

So the entire ∞-groupoid structure is derived from a single axiom (J) applied iteratively. This is remarkable: the simple rule "to prove something for all paths, prove it for reflexivity" generates all of higher homotopy theory when applied in an iterated type-theoretic setting.

## Higher Paths in Practice

In practice, higher paths appear in several important contexts:

**Coherences in algebra.** For a monoid structure on a type $A$ (a type with $m : A \to A \to A$ and $e : A$ and laws), the laws hold propositionally (as paths). The coherences between the laws are 2-paths. The coherences between coherences are 3-paths. etc.

**The pentagon identity.** For the associativity 2-path $\mathsf{assoc}(p,q,r)$ and the Mac Lane pentagon (a coherence between five uses of associativity), the pentagon identity is a 3-path (an equality between two specific 2-paths).

**Naturality squares.** For a natural transformation $\alpha : F \Rightarrow G$ and a path $p : a = b$ in the base, the naturality condition $\mathsf{ap}_G(p) \cdot \alpha_b = \alpha_a \cdot \mathsf{ap}_F(p)$ is a 2-path (an equation between 1-paths).

**The three-fold loop space.** The 3-fold loop space $\Omega^3 A = \mathsf{refl}_{\mathsf{refl}_a} = \mathsf{refl}_{\mathsf{refl}_a}$ is the home of 3-paths. By the Eckmann-Hilton argument applied again, operations here are not just abelian but also "commutative in the extra dimension."

## The ∞-Groupoid Structure

Putting it all together: every type $A$ in HoTT has the structure of a *weak ∞-groupoid*:
- At level 0: objects (terms)
- At level 1: morphisms (paths)
- At level 2: 2-morphisms (2-paths), all invertible
- At level $n$: $n$-morphisms ($n$-paths), all invertible
- All composition operations defined via J
- All laws holding propositionally (as paths at the next level)
- All laws coherent with higher laws (also propositionally)

The "weak" refers to the fact that laws hold propositionally, not definitionally. This is the right notion: strict ∞-groupoids are too rigid (as we saw in Chapter 12), and weak ∞-groupoids capture all homotopy types.

## Summary

| Level | Structure | Key operations |
|---|---|---|
| Level 0 | Objects $a : A$ | — |
| Level 1 | Paths $p : a = b$ | Concatenation, inversion |
| Level 2 | 2-paths $H : p = q$ | Vertical and horizontal composition |
| Level $n$ | $n$-paths | $n$-dim composition, all invertible |
| All levels | Weak ∞-groupoid | Groupoid laws, Eckmann-Hilton |

The ∞-groupoid structure of types is the mathematical content of the homotopy hypothesis applied to type theory. Types are not just sets or categories — they're spaces, with all the rich homotopy structure that entails. And all of this structure is derived from the single axiom of path induction (J).
