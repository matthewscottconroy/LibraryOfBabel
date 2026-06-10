# 3.1 Covering Spaces and Their Classification

## The Idea of a Cover

A covering space $\tilde{X} \to X$ is like a "multi-layered" version of $X$ — you're spreading $X$ out into multiple copies (sheets) that sit above it.

The simplest example: the real line $\mathbb{R}$ covering the circle $S^1$ via $p(t) = e^{2\pi i t}$. As $t$ goes from $0$ to $1$, we traverse the circle once. As $t$ continues from $1$ to $2$, we traverse the circle again. The real line is like an infinite "unwinding" of the circle into a staircase.

Each point of $S^1$ has exactly one pre-image in each "sheet" $[n, n+1]$ for $n \in \mathbb{Z}$. There are $\mathbb{Z}$-many sheets. The fiber $p^{-1}(e^{0}) = \mathbb{Z}$ is a discrete set of points.

**Definition 3.1 (Covering Space).** A continuous surjection $p : \tilde{X} \to X$ is a *covering map* if every point $x \in X$ has an open neighborhood $U$ that is *evenly covered*: $p^{-1}(U) = \bigsqcup_\alpha V_\alpha$ (a disjoint union of open sets) where each $p|_{V_\alpha} : V_\alpha \to U$ is a homeomorphism.

The pair $(\tilde{X}, p)$ is a *covering space* of $X$. The open sets $V_\alpha$ are called *sheets* over $U$.

## Examples

**$\mathbb{R} \to S^1$.** The universal cover. Fiber $p^{-1}(x) = \mathbb{Z}$ for all $x \in S^1$. Infinitely many sheets.

**$S^1 \to S^1$ (degree $n$).** The map $z \mapsto z^n$ (in complex notation). This is an $n$-sheeted cover. The fiber $p^{-1}(1) = \{e^{2\pi i k/n} : k = 0,\ldots,n-1\}$ has $n$ elements.

**$S^n \to \mathbb{RP}^n$ (double cover).** The quotient map that identifies antipodal points. The fiber has 2 elements. For $n \geq 2$, $S^n$ is simply connected, making this the universal cover of $\mathbb{RP}^n$.

**Trivial covers.** $X \times F \to X$ (where $F$ is discrete) is always a covering. These are the "trivial" covering spaces.

## Path Lifting

The most important property of covering spaces: you can always lift paths (and homotopies) uniquely.

**Theorem 3.2 (Unique Path Lifting).** Let $p : \tilde{X} \to X$ be a covering map, $\gamma : [0,1] \to X$ a path, and $\tilde{x}_0 \in p^{-1}(\gamma(0))$. There is a unique continuous lift $\tilde\gamma : [0,1] \to \tilde{X}$ with:
- $\tilde\gamma(0) = \tilde{x}_0$
- $p \circ \tilde\gamma = \gamma$

*Proof sketch.* The path $\gamma$ is compact, so cover $[0,1]$ by finitely many intervals $[t_{i-1}, t_i]$ such that $\gamma([t_{i-1}, t_i])$ is contained in an evenly covered neighborhood. Over each interval, the lift is determined uniquely by the choice of sheet (and sheets are determined by continuity). $\square$

**Theorem 3.3 (Homotopy Lifting).** If $H : [0,1] \times [0,1] \to X$ is a homotopy (rel endpoints) between paths $\gamma_0$ and $\gamma_1$, and $\tilde\gamma_0$ is a lift of $\gamma_0$, then $H$ lifts uniquely to a homotopy $\tilde{H}$ with $\tilde{H}(-,0) = \tilde\gamma_0$. Moreover, $\tilde\gamma_1 = \tilde{H}(-,1)$ is a lift of $\gamma_1$, and the lift $\tilde{H}$ is a homotopy from $\tilde\gamma_0$ to $\tilde\gamma_1$.

**Corollary 3.4.** Path-homotopic paths have the same endpoint when lifted. So there's a well-defined *monodromy action*:
$$\phi : \pi_1(X, x_0) \to \mathsf{Sym}(p^{-1}(x_0))$$
$$\phi([\gamma])(\tilde{x}_0) = \tilde\gamma(1)$$
where $\tilde\gamma$ is the lift of $\gamma$ starting at $\tilde{x}_0$.

## The Monodromy Action and $\pi_1(S^1)$

For the covering $p : \mathbb{R} \to S^1$ with basepoint $1 \in S^1$ and fiber $p^{-1}(1) = \mathbb{Z}$:

The fundamental group $\pi_1(S^1, 1)$ acts on $\mathbb{Z}$ by the monodromy. The loop $\omega$ (winding once counterclockwise) lifts to the path from $n$ to $n+1$ in $\mathbb{R}$ (it moves the starting point forward by one). So $\omega$ acts as $n \mapsto n + 1$ — the translation by $1$.

This gives a homomorphism $\pi_1(S^1, 1) \to \mathsf{Aut}(\mathbb{Z}) = \mathbb{Z}$ (where the automorphisms of $\mathbb{Z}$ as a set with the action are the translations). The loop $\omega^n$ maps to $+n$, and the inverse $\omega^{-1}$ maps to $-1$. So the monodromy action is an isomorphism $\pi_1(S^1) \cong \mathbb{Z}$.

## The Classification Theorem

The fundamental theorem of covering spaces establishes a beautiful correspondence between topology and algebra.

**Setup:** Let $X$ be path-connected, locally path-connected, and semi-locally simply connected. These are mild conditions satisfied by all CW complexes, manifolds, and most spaces you'll encounter.

**Theorem 3.5 (Classification of Covering Spaces).** There is a bijective correspondence:
$$\left\{ \begin{array}{c} \text{Connected covering spaces} \\ (\tilde{X}, p) \text{ of } X \text{ (up to isomorphism over } X\text{)} \end{array} \right\} \longleftrightarrow \left\{ \begin{array}{c} \text{Conjugacy classes of} \\ \text{subgroups } H \leq \pi_1(X, x_0) \end{array} \right\}$$

The correspondence:
- Given $(\tilde{X}, p)$ and a basepoint $\tilde{x}_0 \in p^{-1}(x_0)$: the subgroup is $H = p_*(\pi_1(\tilde{X}, \tilde{x}_0)) \leq \pi_1(X, x_0)$.
- Given a subgroup $H \leq \pi_1(X, x_0)$: the covering space has fiber $\pi_1(X, x_0)/H$ (left cosets), and the total space is constructed explicitly.

**Special cases:**
- $H = \pi_1(X, x_0)$ (the whole group): the trivial 1-sheeted cover $X$ itself
- $H = \{e\}$ (trivial subgroup): the universal cover $\tilde{X}$ (simply connected)
- $H$ normal in $\pi_1(X, x_0)$: the covering is a *regular (Galois) cover*, and $\pi_1(X)/H$ acts freely and transitively on the fiber (as deck transformations)

**Number of sheets:** The covering corresponding to $H$ has $|\pi_1(X)/H|$ sheets (the index of $H$). For the universal cover, $H = \{e\}$, so the number of sheets = $|\pi_1(X)|$.

## Covering Spaces of the Circle

$\pi_1(S^1) = \mathbb{Z}$. Subgroups of $\mathbb{Z}$: they are $n\mathbb{Z}$ for $n \geq 0$ (and $\mathbb{Z}$ itself for $n = 1$).

- $n = 0$: $H = \{0\}$ → universal cover $\mathbb{R}$ (infinitely many sheets)
- $n = 1$: $H = \mathbb{Z}$ → trivial cover $S^1$ itself (1 sheet)
- $n \geq 2$: $H = n\mathbb{Z}$ → the $n$-sheeted cover $S^1 \xrightarrow{z^n} S^1$

Every subgroup of $\mathbb{Z}$ is of the form $n\mathbb{Z}$, and these are all distinct. So the covering spaces of $S^1$ are exactly: $\mathbb{R}$ and $S^1 \xrightarrow{z^n} S^1$ for $n \geq 1$.

This is a complete classification!

## Deck Transformations

A *deck transformation* (or *covering transformation*) of $(\tilde{X}, p)$ is a homeomorphism $\phi : \tilde{X} \to \tilde{X}$ with $p \circ \phi = p$. Deck transformations permute the fibers.

**Theorem 3.6.** The group of deck transformations $\mathsf{Deck}(\tilde{X}/X)$ is isomorphic to $\pi_1(X, x_0)/\overline{H}$ where $\overline{H}$ is the normal core of $H$ (the largest normal subgroup of $\pi_1(X)$ contained in $H$).

For the universal cover ($H = \{e\}$): $\mathsf{Deck}(\tilde{X}/X) \cong \pi_1(X, x_0)$. The fundamental group acts on the universal cover by deck transformations.

**Example.** For $\mathbb{R} \to S^1$: deck transformations are integer translations $t \mapsto t + n$ for $n \in \mathbb{Z}$. The group is $\mathbb{Z} \cong \pi_1(S^1)$. ✓

## Covering Spaces in HoTT

In HoTT, covering spaces are modeled by type families over connected types. Specifically, a covering space of a connected type $B$ corresponds to a type family $F : B \to \mathsf{Type}$ where all fibers are sets.

The classification theorem in HoTT:

**Theorem 3.7 (HoTT Classification).** Covering spaces (set-fibrations) over a connected type $B$ with basepoint $b_0 : B$ correspond to actions of $\pi_1(B, b_0)$ on sets.

The correspondence:
- A type family $F : B \to \mathsf{Type}$ (all fibers sets) ↔ an action of $\pi_1(B, b_0)$ on the fiber $F(b_0)$
- The action is given by transport: $p : b_0 = b_0$ acts on $F(b_0)$ by $\mathsf{transport}^F(p, -)$

For the circle $S^1$ (with $\pi_1(S^1) = \mathbb{Z}$): covering spaces of $S^1$ correspond to $\mathbb{Z}$-actions on sets, i.e., sets with a bijection. This matches the classical classification: the $n$-sheeted cover corresponds to $\mathbb{Z}$ acting on $\mathbb{Z}/n$ by translation; the universal cover corresponds to $\mathbb{Z}$ acting on $\mathbb{Z}$ by translation.

## Summary

| Subgroup $H \leq \pi_1(X)$ | Covering space | Properties |
|---|---|---|
| $H = \pi_1(X)$ | $X$ itself | 1-sheeted, trivial |
| $H$ of index $n$ | $n$-sheeted cover | |
| $H$ normal, index $n$ | Regular $n$-sheeted cover | Deck group $= \pi_1(X)/H$ |
| $H = \{e\}$ | Universal cover $\tilde{X}$ | Simply connected |

Covering spaces are both a computational tool (for computing $\pi_1$) and a conceptual framework (subgroups of $\pi_1$ classify geometric structures over $X$). In HoTT, they correspond to type families with set-valued fibers, and the classification is proved constructively using the universal property of the circle HIT.
