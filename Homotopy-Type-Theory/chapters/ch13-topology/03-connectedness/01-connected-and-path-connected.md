# 3.1 Connectedness and Path-Connectedness

## The Intuitive Idea

A connected space is one that's "all in one piece" — it can't be split into two separate parts. The precise definition formalizes this:

**Definition 3.1 (Connected Space).** A topological space $X$ is *connected* if it cannot be written as a disjoint union of two non-empty open sets. Equivalently: if $X = U \cup V$ with $U, V$ open and $U \cap V = \emptyset$, then $U = \emptyset$ or $V = \emptyset$.

Another equivalent formulation: $X$ is connected iff the only clopen (simultaneously open and closed) subsets of $X$ are $\emptyset$ and $X$ itself.

**Examples of connected spaces:**
- $\mathbb{R}$ (or any interval): can't split the real line into two disjoint open sets
- $\mathbb{R}^n$ for any $n$: similarly connected
- $S^n$ (the $n$-sphere): connected for $n \geq 1$
- Any convex subset of $\mathbb{R}^n$

**Examples of disconnected spaces:**
- $\{0, 1\}$ with the discrete topology: the sets $\{0\}$ and $\{1\}$ are both open (and closed)
- $\mathbb{Q}$ (the rationals): for any irrational $r$, $\{x \in \mathbb{Q} : x < r\}$ and $\{x \in \mathbb{Q} : x > r\}$ are disjoint open sets covering $\mathbb{Q}$
- The integers $\mathbb{Z}$ with the discrete topology

## Connected Components

**Definition 3.2.** A *connected component* of $X$ is a maximal connected subset of $X$ — a connected subset $C \subseteq X$ such that if $C \subseteq D$ and $D$ is connected, then $D = C$.

**Theorem 3.3.** Every point $x \in X$ is contained in a unique connected component. The connected components partition $X$ into pairwise disjoint closed subsets.

*Proof sketch.* The component containing $x$ is the union of all connected subsets containing $x$. A union of connected sets that share a point is connected. Components are closed because the closure of a connected set is connected. $\square$

**The path-components** are defined analogously, using path-connectivity (see below). They may differ from connected components.

## Image Theorem and Applications

**Theorem 3.4.** The continuous image of a connected space is connected.

*Proof.* Let $f : X \to Y$ be continuous with $X$ connected. Suppose $f(X) = U \cup V$ with $U, V$ open in $f(X)$ (with the subspace topology), $U \cap V = \emptyset$, $U, V \neq \emptyset$. Then $f^{-1}(U)$ and $f^{-1}(V)$ are open in $X$, partition $X$, and are non-empty. This contradicts $X$ connected. $\square$

**Corollary 3.5 (Intermediate Value Theorem).** If $f : X \to \mathbb{R}$ is continuous and $X$ is connected, then $f(X)$ is an interval. In particular, if $f(a) < c < f(b)$ for $a, b \in X$, then $f(x) = c$ for some $x \in X$.

This is the topological formulation of IVT. The proof: $f(X)$ is a connected subset of $\mathbb{R}$, and connected subsets of $\mathbb{R}$ are exactly the intervals (including single points and the empty set).

## Path-Connectedness

Path-connectedness is a stronger notion than connectedness.

**Definition 3.6 (Path-Connected).** A space $X$ is *path-connected* if for any two points $x, y \in X$, there exists a continuous path $\gamma : [0,1] \to X$ with $\gamma(0) = x$ and $\gamma(1) = y$.

Path-connected implies connected (proof: if $X = U \cup V$ with $U, V$ open, non-empty, and disjoint, and $x \in U, y \in V$, then any path $\gamma$ from $x$ to $y$ gives $\gamma^{-1}(U)$ and $\gamma^{-1}(V)$ as a partition of $[0,1]$ into open non-empty sets, contradicting $[0,1]$ connected).

But connected does not imply path-connected!

**Example 3.7 (Topologist's Sine Curve).** The space:
$$S = \{(x, \sin(1/x)) \mid 0 < x \leq 1\} \cup \{(0, y) \mid -1 \leq y \leq 1\} \subseteq \mathbb{R}^2$$

is connected (the closure of the graph of $\sin(1/x)$) but not path-connected: there's no path from any point $(x_0, \sin(1/x_0))$ (for $x_0 > 0$) to $(0, 0)$. The "oscillation" near $x = 0$ is too extreme.

**Theorem 3.8.** The continuous image of a path-connected space is path-connected.

*Proof.* Given $f : X \to Y$ continuous with $X$ path-connected, and $y_1, y_2 \in f(X)$: pick $x_1, x_2 \in X$ with $f(x_i) = y_i$. Take a path $\gamma : [0,1] \to X$ from $x_1$ to $x_2$. Then $f \circ \gamma : [0,1] \to Y$ is a path from $y_1$ to $y_2$. $\square$

## Path Components and $\pi_0$

**Definition 3.9 (Path Components).** The *path components* of $X$ are the equivalence classes under the relation $x \sim y \iff$ there is a path from $x$ to $y$.

**Theorem 3.10.** Path-connectivity is an equivalence relation (reflexivity: constant paths; symmetry: reverse paths; transitivity: concatenate paths).

The set of path components of $X$ is denoted $\pi_0(X)$ — the *0th homotopy set*. For a topological space, $\pi_0(X)$ is just a set (the components). In HoTT, $\pi_0(A)$ is the *0-truncation* $\|A\|_0$ of the type $A$.

This is the first instance of a general pattern: the homotopy groups $\pi_n(X)$ measure the higher connectivity of $X$, and they correspond to the higher identity types in HoTT.

## Local Connectedness

A space can be connected globally but "disconnected locally":

**Definition 3.11.** A space $X$ is *locally connected* if every point has a neighborhood basis of connected open sets. $X$ is *locally path-connected* if every point has a neighborhood basis of path-connected open sets.

**Theorem 3.12.** If $X$ is locally path-connected and connected, then $X$ is path-connected.

This theorem is why many familiar spaces (manifolds, CW complexes) are path-connected: they're locally contractible (hence locally path-connected) and connected.

## Connectedness and HoTT

In HoTT, connectedness corresponds to *truncation* conditions:

**$(-1)$-connected (non-empty).** A type $A$ is $(-1)$-connected iff $\|A\|$ is non-empty (there merely exists a term of $A$). In topology: $\pi_0(X) \neq \emptyset$.

**$0$-connected (connected).** A type $A$ is $0$-connected iff $\pi_0(A) = \|A\|_0$ is contractible — i.e., $\|A\|_{-1}$ is inhabited and $\pi_0(A)$ is a singleton. In topology: $X$ has exactly one path component.

**$1$-connected (simply connected).** A type $A$ is $1$-connected iff $A$ is $0$-connected and $\pi_1(A) = 0$. In topology: connected with trivial fundamental group.

**$n$-connected.** A type $A$ is $n$-connected iff it is $(n-1)$-connected and $\pi_n(A) = 0$ (for all basepoints). In topology: connected up to and including dimension $n$.

So the HoTT notion of connectivity is precisely the homotopy-theoretic notion: a space/type is $n$-connected iff its first non-trivial homotopy occurs at dimension $n+1$ or higher.

The *identity type* $a = b$ being non-empty (there exists a path from $a$ to $b$) corresponds to $a$ and $b$ being in the same connected component. So the identity type in HoTT is asking: "are these two points in the same connected component?" But it's more than just asking — it actually *is* the set of paths, carrying all the higher homotopy information.

## Compact Spaces and Connectedness

Some useful interactions:

**Theorem 3.13.** A quotient of a connected space is connected.

**Theorem 3.14.** If $X$ and $Y$ are connected, so is $X \times Y$.

*Proof.* Fix $(x_0, y_0) \in X \times Y$. For any $(x, y)$, there's a path from $(x_0, y_0)$ to $(x_0, y)$ (via a path in $\{x_0\} \times Y$, which is connected since $Y$ is) and from $(x_0, y)$ to $(x, y)$ (via a path in $X \times \{y\}$). Concatenating gives a path from $(x_0, y_0)$ to $(x, y)$. $\square$

**Corollary.** $\mathbb{R}^n$ and $S^n$ are connected (since $S^n$ is the one-point compactification or quotient of connected spaces).

## Summary

| Property | Definition | HoTT analog |
|---|---|---|
| Connected | No non-trivial clopen partition | $\|A\|_0$ contractible (0-connected) |
| Path-connected | Any two points joined by a path | All pairs $(a,b)$ have $\|a = b\|$ inhabited |
| Simply connected | Connected + $\pi_1 = 0$ | 1-connected |
| $n$-connected | $\pi_k = 0$ for $k \leq n$ | $n$-truncation maps are equivalences |

Connectedness is the topological precursor to HoTT's notion of connectivity. The homotopy groups $\pi_n(X)$ measure how "connected" a space is at each dimension, and these correspond exactly to the iterative identity type structure of a HoTT type.
