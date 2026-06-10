# Connectedness

## The Intuition

Before definitions, consider what it should mean for a topological space to be "in one piece." The real line $\mathbb{R}$ is in one piece. The union of two disjoint open intervals $(0,1) \cup (2,3)$ is not. A figure-eight is in one piece — you can reach any point from any other point by traveling along the curve. The set of rational numbers $\mathbb{Q}$ is surprisingly not in one piece, topologically speaking, even though it looks like a connected subset of the real line.

The formalization of "in one piece" turns out to be more subtle than it first appears, and this subtlety is itself mathematically important. There are actually two natural formalization — connectedness and path-connectedness — and they differ. The gap between them is occupied by pathological but mathematically illuminating examples.

## Connected Spaces

**Definition.** A topological space $X$ is *connected* if it cannot be written as the union of two disjoint non-empty open sets. Equivalently, the only *clopen* (simultaneously open and closed) subsets of $X$ are $\emptyset$ and $X$ itself.

Why this definition? The idea is that if $X = U \cup V$ with $U$ and $V$ open, disjoint, and non-empty, then $U$ and $V$ are "separated" — you cannot travel continuously from $U$ to $V$ without jumping. The space is in two disconnected pieces.

**Examples.**
- $\mathbb{R}$ is connected. (If $\mathbb{R} = U \cup V$ with $U, V$ disjoint, open, non-empty, pick $u \in U$ and $v \in V$, and consider the infimum of $\{t : t > u \text{ and } t \notin U\}$. This leads to a contradiction via the completeness of $\mathbb{R}$.)
- $(0,1) \cup (2,3)$ is not connected: the two intervals form a separation.
- $\mathbb{Q}$ is not connected: $\mathbb{Q} = (\mathbb{Q} \cap (-\infty, \sqrt{2})) \cup (\mathbb{Q} \cap (\sqrt{2}, \infty))$ is a separation into two open sets.
- Every discrete space with more than one point is disconnected: singletons are clopen.

**Theorem.** The continuous image of a connected space is connected.

*Proof.* If $f : X \to Y$ is continuous and $X$ is connected, suppose $f(X) = U \cup V$ with $U, V$ open and disjoint in $f(X)$. Then $X = f^{-1}(U) \cup f^{-1}(V)$, a separation of $X$ into disjoint open sets. Since $X$ is connected, one of these must be empty, so one of $U, V$ is empty.

**Corollary (Intermediate Value Theorem).** If $f : [a, b] \to \mathbb{R}$ is continuous and $f(a) < c < f(b)$, then there exists $x \in (a,b)$ with $f(x) = c$.

*Proof.* The interval $[a,b]$ is connected (it is an interval in $\mathbb{R}$). Its image $f([a,b])$ is connected. A connected subset of $\mathbb{R}$ is an interval (this requires proof: see exercises). An interval containing $f(a)$ and $f(b)$ contains every value between them.

## Path-Connected Spaces

**Definition.** A topological space $X$ is *path-connected* if for every two points $x, y \in X$ there exists a *path* from $x$ to $y$: a continuous function $\gamma : [0,1] \to X$ with $\gamma(0) = x$ and $\gamma(1) = y$.

Path-connectivity is the "explicit" version of connectivity: it asks not just that the space cannot be separated, but that there is a positive witness — an actual path — connecting any two points.

**Theorem.** Every path-connected space is connected.

*Proof.* Suppose $X$ is path-connected and $X = U \cup V$ with $U, V$ open and disjoint. Suppose both are non-empty; pick $u \in U$ and $v \in V$. By path-connectivity, there is a path $\gamma : [0,1] \to X$ with $\gamma(0) = u$ and $\gamma(1) = v$. Then $[0,1] = \gamma^{-1}(U) \cup \gamma^{-1}(V)$ is a separation of $[0,1]$ into two disjoint open sets. But $[0,1]$ is connected — contradiction.

The converse fails. There exist connected spaces that are not path-connected. The canonical example is the topologist's sine curve.

## The Topologist's Sine Curve

Define the following subset of $\mathbb{R}^2$:
$$S = \left\{(x, \sin(1/x)) : 0 < x \leq 1\right\} \cup \left(\{0\} \times [-1, 1]\right)$$

This is the graph of $\sin(1/x)$ on $(0,1]$, together with the entire vertical segment $\{0\} \times [-1,1]$ that the graph approaches as $x \to 0$.

**$S$ is connected.** The graph $\Gamma = \{(x, \sin(1/x)) : 0 < x \leq 1\}$ is the continuous image of the connected interval $(0,1]$, hence connected. The closure of a connected set is connected (if $\overline{\Gamma} = U \cup V$ is a separation, then $\Gamma \subseteq U$ or $\Gamma \subseteq V$ by connectedness; both $U$ and $V$ are closed in $\overline{\Gamma}$, and the closure of $\Gamma$ in $\overline{\Gamma}$ is all of $\overline{\Gamma}$, so neither can miss all of $\Gamma$). The closure of $\Gamma$ in $\mathbb{R}^2$ is $S$, so $S$ is connected.

**$S$ is not path-connected.** Suppose for contradiction that there is a path $\gamma : [0,1] \to S$ with $\gamma(0) = (1, \sin 1)$ and $\gamma(1) = (0, 0)$. Let $\pi_1 : \mathbb{R}^2 \to \mathbb{R}$ be the projection to the first coordinate. Then $\pi_1 \circ \gamma : [0,1] \to [0,1]$ is continuous with $(\pi_1 \circ \gamma)(0) = 1$ and $(\pi_1 \circ \gamma)(1) = 0$. Let $t_0 = \sup\{t : (\pi_1 \circ \gamma)(t) > 0\}$. Then $(\pi_1 \circ \gamma)(t_0) = 0$ by continuity (the supremum is achieved), so $\gamma(t_0) = (0, y)$ for some $y \in [-1,1]$.

For $t < t_0$, we have $\gamma(t) \in \Gamma$, so $\gamma(t) = (x(t), \sin(1/x(t)))$ with $x(t) > 0$. As $t \to t_0^-$, $x(t) \to 0$, so $\sin(1/x(t))$ oscillates between $-1$ and $1$ without converging. But $\gamma(t) \to \gamma(t_0) = (0,y)$ by continuity of $\gamma$, which requires $\sin(1/x(t)) \to y$ — a contradiction.

This example shows that path-connectedness is genuinely stronger than connectedness, and that the difference matters.

## Why Path-Connectedness Is More Useful for Homotopy Theory

Despite being the stronger condition, path-connectedness is the more useful notion for homotopy theory. Here is why:

1. **The fundamental group requires paths.** The fundamental group $\pi_1(X, x_0)$ is defined in terms of loops — paths that begin and end at $x_0$. If $X$ is not path-connected, $\pi_1$ sees only the path-component of $x_0$ and misses the rest.

2. **Homotopy equivalences preserve path-connectedness.** If $X \simeq Y$ (homotopy equivalence), then $X$ is path-connected if and only if $Y$ is. Connectedness alone is not a homotopy invariant in the most useful sense.

3. **$\pi_0$ classifies path-components.** The set of path-components of $X$, denoted $\pi_0(X)$, is a homotopy invariant and the "0th homotopy group" of $X$. In HoTT, $\pi_0(X) = \|X\|_0$ is the 0-truncation of the type $X$. The path-components of a topological space correspond to the elements of the 0-truncated type.

4. **Connected components and path-components can differ.** The topologist's sine curve is connected (one connected component) but has two path-components: the graph and the vertical segment. In homotopy theory, these two pieces are genuinely separate — there is no path between them.

## Components

**Definition.** The *connected component* of $x \in X$ is the largest connected subspace of $X$ containing $x$: the union of all connected subspaces containing $x$. The *path-component* of $x$ is the set of all points reachable from $x$ by a path.

The connected components of $X$ partition $X$ into disjoint closed sets. The path-components of $X$ partition $X$ into disjoint sets (not necessarily closed or open). For "locally path-connected" spaces — those where every point has a neighborhood base of path-connected sets — the path-components are open, and connected components coincide with path-components.

Most spaces arising in practice are locally path-connected: manifolds, CW complexes, polyhedra, algebraic varieties with their classical topology. For these spaces, the distinction between connectedness and path-connectedness vanishes. The distinction only matters for pathological examples, but those examples are important for understanding what the definitions really say.

## Local Connectedness

A space is *locally connected* if every point has a neighborhood base of connected sets. Similarly for local path-connectedness. These local conditions are independent of the global ones.

The rational numbers $\mathbb{Q}$ are not connected but are locally connected (every point has arbitrarily small connected neighborhoods — wait, no, they're not: every neighborhood contains irrationals and rationals, and the rationals are not connected). Actually $\mathbb{Q}$ is not locally connected either. The real line $\mathbb{R}$ is locally connected (every point has connected intervals as neighborhoods). The Sorgenfrey line is connected (as a subset of $\mathbb{R}$) but not locally connected.

For homotopy theory, the key condition is local path-connectedness: it ensures that path-components are open and closed, that the fundamental group is well-defined (as a group, not just a set), and that covering space theory works as expected.

## The HoTT Connection

In HoTT:
- A type $A$ is *connected* (in the homotopy-theoretic sense) if $\|A\|_0$ is contractible — the 0-truncation has a single element, meaning all points are "homotopically connected."
- A type is *path-connected* in the classical sense if any two points $a, b : A$ satisfy $\|a = b\|$ — they are connected by some path.
- The type $\pi_0(A) = \|A\|_0$ is the type of path-components.

The intermediate value theorem has a synthetic analog in HoTT that is more subtle: the classical IVT requires real numbers and connectivity of $[0,1]$, and in HoTT the real numbers must be constructed (as Cauchy reals or Dedekind reals) and their connectedness proved. But the structure of the proof is the same: continuous maps preserve connectivity, and the HoTT proof mirrors the classical one almost point for point.
