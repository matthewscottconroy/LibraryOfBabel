# 5.1 Connected and Path-Connected Spaces

## What Is Connectedness?

Informally, a space is connected if it's "in one piece." The real line is connected; two separate points are not. A circle is connected; removing a point leaves an arc that's still connected, but removing two points disconnects it.

Making this precise requires care. The naive idea — "you can travel from any point to any other" — works for nice spaces but needs refinement for pathological ones.

## Topological Connectedness

**Definition.** A metric space $X$ is *connected* if it cannot be written as the union of two non-empty, disjoint open sets.

Equivalently: the only subsets of $X$ that are both open and closed (clopen) are $\emptyset$ and $X$ itself.

**Examples:**
- $\mathbb{R}$ is connected.
- Any interval $(a, b)$, $[a, b]$, $(a, b]$, $[a, b)$ is connected.
- $\mathbb{R} \setminus \{0\}$ is disconnected: it's the union of $(-\infty, 0)$ and $(0, \infty)$.
- $\mathbb{Q}$ is totally disconnected: the only connected subsets are single points.
- $\mathbb{R}^n$ is connected for all $n \geq 1$.

**Theorem.** The connected subsets of $\mathbb{R}$ are precisely the intervals (including single points, $\mathbb{R}$ itself, and the empty set).

*Proof.* Suppose $A \subseteq \mathbb{R}$ is connected and contains points $a < b$. Suppose $c \in (a, b)$ but $c \notin A$. Then $A = (A \cap (-\infty, c)) \cup (A \cap (c, \infty))$ — two non-empty open sets (in the subspace topology on $A$) that partition $A$, contradicting connectedness. So $A$ contains every point between any two of its points — this is exactly the interval property. Conversely, intervals are connected (proof by the same argument: if $I = U \cup V$ with $U, V$ open and disjoint in $I$, take $a \in U$ and $b \in V$ and define $c = \sup\{U \cap [a, b]\}$; checking $c \in U$ and $c \in V$ both lead to contradictions). $\square$

## Properties of Connected Spaces

**Theorem.** The continuous image of a connected space is connected.

*Proof.* Suppose $f : X \to Y$ is continuous and $X$ is connected. Suppose $f(X) = V_1 \cup V_2$ with $V_1, V_2$ open, disjoint, and both non-empty (intersecting $f(X)$). Then $X = f^{-1}(V_1) \cup f^{-1}(V_2)$: both preimages are open (by continuity), disjoint, and non-empty. This contradicts the connectedness of $X$. $\square$

**Corollary (Intermediate Value Theorem).** If $f : X \to \mathbb{R}$ is continuous, $X$ is connected, and $f(a) < c < f(b)$ for some $a, b \in X$, then there exists $x \in X$ with $f(x) = c$.

*Proof.* $f(X)$ is a connected subset of $\mathbb{R}$, hence an interval. If $f(a) < c < f(b)$, then $c \in f(X)$. $\square$

This is the general form of the Intermediate Value Theorem: it says a continuous function on a connected space can't "jump over" a value.

**Theorem.** Products of connected spaces are connected.

*Proof.* For two spaces $X$ and $Y$: fix $x_0 \in X, y_0 \in Y$. For each $x \in X$, the "horizontal slice" $X \times \{y_0\}$ is homeomorphic to $X$ (connected), and for each $y \in Y$, the "vertical slice" $\{x\} \times Y$ is homeomorphic to $Y$ (connected). Any two slices share the point $(x, y_0)$, so their union is connected. The union of all slices is $X \times Y$. $\square$

## Path Connectedness

**Definition.** A *path* in $X$ from $x$ to $y$ is a continuous function $\gamma : [0, 1] \to X$ with $\gamma(0) = x$ and $\gamma(1) = y$.

**Definition.** $X$ is *path-connected* if for every $x, y \in X$, there is a path from $x$ to $y$.

**Theorem.** Path-connected $\Rightarrow$ connected.

*Proof.* Suppose $X$ is path-connected but not connected. Write $X = U \cup V$ with $U, V$ non-empty, open, disjoint. Take $x \in U$ and $y \in V$, and let $\gamma : [0, 1] \to X$ be a path from $x$ to $y$. Then $[0, 1] = \gamma^{-1}(U) \cup \gamma^{-1}(V)$: two non-empty open sets (preimages of open sets under the continuous $\gamma$) that are disjoint and cover $[0, 1]$. But $[0, 1]$ is connected — contradiction. $\square$

The converse fails: there exist connected spaces that are not path-connected.

**Example: The Topologist's Sine Curve.** Let:
$$X = \{(x, \sin(1/x)) : x > 0\} \cup (\{0\} \times [-1, 1])$$

This is the closure of the graph of $\sin(1/x)$ on $(0, \infty)$, together with the vertical segment $\{0\} \times [-1, 1]$.

$X$ is connected (it's the closure of a connected set). But there is no path from any point $(0, y)$ on the segment to any point on the oscillating part: the oscillations become infinitely dense as $x \to 0^+$, and no continuous path can cross from the segment to the graph. So $X$ is connected but not path-connected.

## Local Connectedness and Local Path-Connectedness

**Definition.** A space is *locally connected* if every point has a neighborhood base of connected sets: every neighborhood of $x$ contains a connected neighborhood.

**Definition.** A space is *locally path-connected* if every point has a neighborhood base of path-connected sets.

For locally path-connected spaces, connected $\Leftrightarrow$ path-connected. This is why, for nice spaces (manifolds, CW complexes), the two notions coincide.

**Theorem.** A connected open subset of $\mathbb{R}^n$ is path-connected.

This is because $\mathbb{R}^n$ is locally path-connected (every point has path-connected ball neighborhoods), so open connected subsets are path-connected.

## Connected Components

**Definition.** The *connected component* of a point $x \in X$ is the largest connected subset of $X$ containing $x$.

Connected components are always closed (the closure of a connected set is connected). They partition $X$ into maximal connected pieces.

**Definition.** The *path component* of $x$ is the set of points reachable from $x$ by paths.

For path components: $x \sim y$ iff there's a path from $x$ to $y$ is an equivalence relation, and path components are the equivalence classes.

In algebraic topology, we write $\pi_0(X)$ for the set of path components of $X$. This is the "zeroth homotopy group" (it's a set, not always a group). Two spaces with different $\pi_0$ are not homotopy equivalent. We'll study $\pi_0$ and higher homotopy groups $\pi_n$ extensively in later chapters.

## Connectedness and the IVT in Practice

The Intermediate Value Theorem is one of the most practically useful results in mathematics. Let's see some applications.

**Application 1: Root-finding.** If $f : [a, b] \to \mathbb{R}$ is continuous and $f(a) < 0 < f(b)$, then $f$ has a root in $(a, b)$. This is the basis of the bisection method for numerically finding roots.

**Application 2: Fixed points in 1D.** If $f : [0, 1] \to [0, 1]$ is continuous, then $f$ has a fixed point. *Proof:* Let $g(x) = f(x) - x$. Then $g(0) = f(0) \geq 0$ and $g(1) = f(1) - 1 \leq 0$. By IVT, $g$ has a root.

**Application 3: Topological obstructions.** The IVT shows that there is no continuous surjection from an interval to two disjoint points: a continuous image of a connected space is connected. More generally, "topological obstructions" are properties (like connectedness) that prevent the existence of certain continuous maps.

This principle — that topology constrains what maps can exist — is fundamental to algebraic topology and, ultimately, to HoTT.

## Paths as the Bridge to Homotopy Theory

The concept of a path as a continuous map $\gamma : [0, 1] \to X$ is the entry point into homotopy theory.

**Key constructions with paths:**
- *Constant path at $x$*: $c_x(t) = x$ for all $t$.
- *Concatenation*: if $\gamma : [0,1] \to X$ is a path from $x$ to $y$ and $\delta : [0,1] \to X$ is a path from $y$ to $z$, then the concatenated path goes first along $\gamma$ at double speed, then along $\delta$ at double speed.
- *Reversal*: if $\gamma$ is a path from $x$ to $y$, then $\bar\gamma(t) = \gamma(1-t)$ is a path from $y$ to $x$.

These operations make "paths up to reparametrization" into a groupoid structure — exactly the structure we'll see in the identity type in HoTT. Identity proofs form a groupoid, and the groupoid operations correspond precisely to constant paths, concatenation, and reversal.

Two paths from $x$ to $y$ are *homotopic* (with endpoints fixed) if there's a continuous deformation of one into the other. The homotopy classes of loops based at $x$ form the *fundamental group* $\pi_1(X, x)$. This is the foundation of algebraic topology and the motivating example for identity types in HoTT.

The next section on the real numbers pulls these threads together and connects the analytic and algebraic structures.
