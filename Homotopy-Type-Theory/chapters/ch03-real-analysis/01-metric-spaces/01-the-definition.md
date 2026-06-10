# 1.1 The Definition of a Metric Space

## Starting with Distance

The central objects of real analysis — sequences, limits, continuity, convergence — all depend on a prior notion of *distance*. When we say a sequence $(x_n)$ converges to a limit $L$, we mean the distances $d(x_n, L)$ go to zero. When we say a function $f$ is continuous at $p$, we mean: inputs close to $p$ produce outputs close to $f(p)$. Distance is the foundation.

In elementary calculus, distance means the Euclidean distance on $\mathbb{R}$ or $\mathbb{R}^n$. But the same ideas apply to many other situations:
- Sequences of functions (where "distance" between $f$ and $g$ might be the maximum of $|f(x) - g(x)|$)
- Strings of symbols (where "distance" might count the number of character changes needed to go from one string to another)
- Probability distributions (where "distance" measures how different two distributions are)
- Graphs (where "distance" counts the length of the shortest path between two nodes)

What do all these have in common? They're all measuring some notion of "how far apart" two things are, and they all satisfy a small set of natural properties. The *metric space* axioms capture exactly these properties.

## The Axioms

**Definition.** A *metric space* is a pair $(X, d)$ where $X$ is a set and $d : X \times X \to \mathbb{R}$ is a function satisfying, for all $x, y, z \in X$:

1. **Non-negativity:** $d(x, y) \geq 0$
2. **Identity of indiscernibles:** $d(x, y) = 0 \iff x = y$
3. **Symmetry:** $d(x, y) = d(y, x)$
4. **Triangle inequality:** $d(x, z) \leq d(x, y) + d(y, z)$

The function $d$ is called a *metric* or *distance function*.

Let's think about why each axiom is there.

**Non-negativity** says distances are never negative. This is almost definitional — we wouldn't call a negative number a "distance."

**Identity of indiscernibles** captures two things: first, the distance from any point to itself is zero ($d(x, x) = 0$); second, if the distance between two points is zero, they must be the same point ($d(x, y) = 0 \Rightarrow x = y$). The first part is obvious. The second is more subtle: it says the metric can *distinguish* distinct points. A function $d$ with $d(x, y) = 0$ for all $x, y$ would satisfy non-negativity but fail identity of indiscernibles.

**Symmetry** says the distance from $x$ to $y$ equals the distance from $y$ to $x$. This holds for Euclidean distance and for most natural distance functions. It can fail in applications (e.g., "information distance" in asymmetric coding), giving rise to *quasi-metrics*. For our purposes, symmetry always holds.

**The triangle inequality** is the deepest axiom. It says: going from $x$ to $z$ directly is at most as far as going via an intermediate point $y$. In $\mathbb{R}^2$, this is literally the statement that the length of one side of a triangle is at most the sum of the lengths of the other two sides — which is why it's called the triangle inequality. It captures the idea that taking a detour can't shorten a path.

The triangle inequality does a lot of work in proofs. Many of the key estimates in analysis ultimately reduce to applying it cleverly.

## Immediate Consequences

From the axioms, we can derive a few useful properties.

**Proposition.** For any $x, y$ in a metric space: $d(x, y) \geq 0$, with equality iff $x = y$. (Already in the axioms.)

**Proposition (Reverse triangle inequality).** $|d(x, z) - d(y, z)| \leq d(x, y)$.

*Proof.* From the triangle inequality: $d(x, z) \leq d(x, y) + d(y, z)$, so $d(x, z) - d(y, z) \leq d(x, y)$. By symmetry (swapping $x$ and $y$): $d(y, z) - d(x, z) \leq d(y, x) = d(x, y)$. Together: $|d(x, z) - d(y, z)| \leq d(x, y)$. $\square$

This says: distances can't change too rapidly. If $x$ and $y$ are close, then $d(x, z)$ and $d(y, z)$ are close for any $z$. In other words, the distance function $d(\cdot, z) : X \to \mathbb{R}$ is continuous (we'll formalize this later).

## Standard Examples

It's important to have a rich supply of examples, both to test your intuition and to see how widely the theory applies.

**Example 1: Euclidean space $\mathbb{R}^n$.** The *Euclidean metric* on $\mathbb{R}^n$ is:
$$d_2(x, y) = \sqrt{\sum_{i=1}^n (x_i - y_i)^2}$$

This is the familiar straight-line distance. For $n = 1$, it's $d(x, y) = |x - y|$. For $n = 2$, it's the Pythagorean formula. The triangle inequality is equivalent to the Cauchy-Schwarz inequality.

**Example 2: The taxicab metric on $\mathbb{R}^n$.** Also called the $\ell^1$ metric:
$$d_1(x, y) = \sum_{i=1}^n |x_i - y_i|$$

In $\mathbb{R}^2$, this measures the distance you'd travel on a grid (like city blocks). The unit "ball" under this metric is a diamond shape, not a circle.

**Example 3: The sup metric on $\mathbb{R}^n$.** Also called the $\ell^\infty$ metric:
$$d_\infty(x, y) = \max_{1 \leq i \leq n} |x_i - y_i|$$

The unit "ball" under this metric is a square (in $\mathbb{R}^2$), not a circle. Despite looking different from the Euclidean metric, this metric induces the same topology on $\mathbb{R}^n$ — meaning the same sequences converge, the same functions are continuous, and the same sets are open.

**Example 4: The discrete metric.** For any set $X$, define:
$$d(x, y) = \begin{cases} 0 & \text{if } x = y \\ 1 & \text{if } x \neq y \end{cases}$$

This is a metric. It makes every point "distance 1 from every other point." In this metric, the only convergent sequences are eventually constant. Every subset is both open and closed. The discrete metric is important as a degenerate case and for counterexamples.

**Example 5: Function spaces.** Let $C([0, 1])$ denote the set of continuous real-valued functions on $[0, 1]$. Define:
$$d_\infty(f, g) = \sup_{x \in [0,1]} |f(x) - g(x)|$$

This is the *uniform metric* or *supremum metric*. Convergence in this metric is exactly *uniform convergence* of functions — a stronger and more useful notion than pointwise convergence. This metric space is complete (a fact that requires proof and is important in analysis).

**Example 6: The Hamming distance.** For binary strings of length $n$, the Hamming distance $d(u, v)$ counts the number of positions where $u$ and $v$ differ. This is a metric on $\{0, 1\}^n$, used extensively in coding theory and combinatorics.

## Isometries and Metric Equivalences

Two metric spaces $(X, d_X)$ and $(Y, d_Y)$ can be related in different ways.

**Definition.** An *isometry* is a bijection $f : X \to Y$ satisfying $d_Y(f(x), f(x')) = d_X(x, x')$ for all $x, x' \in X$.

Isometries are the "sameness" notion for metric spaces that preserves distance exactly. Two metric spaces that are isometric are indistinguishable as metric spaces.

A weaker notion is *Lipschitz equivalence*: two metrics $d$ and $d'$ on the same set $X$ are Lipschitz equivalent if there exist constants $c_1, c_2 > 0$ with $c_1 d(x,y) \leq d'(x,y) \leq c_2 d(x,y)$ for all $x, y$. The metrics $d_1$, $d_2$, $d_\infty$ on $\mathbb{R}^n$ are all Lipschitz equivalent (though not isometric). Lipschitz equivalent metrics have the same convergent sequences, continuous functions, and open sets.

## Subspaces

If $(X, d)$ is a metric space and $A \subseteq X$, then the restriction $d|_{A \times A}$ makes $(A, d|_{A \times A})$ a metric space. We call this the *metric subspace* structure. For example, $[0, 1]$ with the Euclidean metric is a metric subspace of $\mathbb{R}$.

## Why Not Just Use $\mathbb{R}$?

A reasonable question: why develop all this general theory? Can't we just work in $\mathbb{R}$ (or $\mathbb{R}^n$) and be done with it?

The answer is that many important applications require more general spaces:

1. **Function spaces** like $C([0, 1])$ are infinite-dimensional and cannot be embedded in $\mathbb{R}^n$ for any finite $n$. Understanding their topology is central to functional analysis and PDEs.

2. **Completions** of metric spaces are a fundamental construction (we'll see this shortly). To complete $\mathbb{Q}$, you need to build $\mathbb{R}$ — you can't stay inside $\mathbb{Q}$.

3. **Topological generality.** The key properties — connectedness, compactness, continuity — are really properties of the *topology* (the open sets), not of the particular distance function. Working at the level of metric spaces makes the relationship between the topology and the distance visible.

4. **Abstraction reveals structure.** When you prove a theorem about arbitrary metric spaces, you can see exactly which axioms are needed. This makes it clear what properties are essential and what can be varied. It also means your theorem applies to all the examples at once, without re-proving it for each case.

The general metric space framework is the right setting for analysis. In the next section, we'll use the metric to define open balls, open sets, and the topology they generate.
