# 3.2 Compactness and Connectedness

We met compactness in Chapter 1 in the metric space setting. Here we generalize to arbitrary topological spaces, and add connectedness — the property that captures whether a space is "in one piece."

**Definition 3.2.1.** A topological space $X$ is:
- *Compact*: every open cover has a finite subcover.
- *Connected*: the only clopen (simultaneously open and closed) sets are $\emptyset$ and $X$.
- *Path-connected*: for every $x, y \in X$ there exists a continuous $\gamma: [0,1] \to X$ with $\gamma(0) = x$, $\gamma(1) = y$.
- *Locally connected* / *locally path-connected*: every point has a neighborhood base of connected / path-connected sets.

Connectedness says you can't split $X$ into two disjoint open pieces. Path-connectedness is stronger: any two points can be joined by a continuous path. Path-connected implies connected, but not conversely (the "topologist's sine curve" $\{(x, \sin(1/x)) : x > 0\} \cup \{0\} \times [-1,1]$ is connected but not path-connected).

The most important compactness theorem for general topological spaces:

**Theorem 3.2.2 (Tychonoff's Theorem).** An arbitrary product of compact spaces is compact (in the product topology).

This requires the Axiom of Choice in full generality. For countable products, a simpler argument using diagonal sequences suffices. Tychonoff's theorem is one of the most used results in functional analysis — the closed unit ball of a dual Banach space is compact in the weak-* topology, by Tychonoff.

The image of a compact space under a continuous map is compact:

**Theorem 3.2.3.** Let $f: X \to Y$ be continuous with $X$ compact. Then $f(X)$ is compact. If additionally $Y$ is Hausdorff and $f$ is injective, then $f$ is an embedding (homeomorphism onto its image).

This is the topological version of the fact we proved in Chapter 1 for metric spaces. The Hausdorff condition is essential for the second statement — in non-Hausdorff spaces, a continuous bijection from a compact space need not have a continuous inverse.

One space deserves special attention because it appears constantly in symbolic dynamics:

**Key Example 3.2.4 (The Cantor Set).** The Cantor set $C = \{0,1\}^{\mathbb{N}}$ (binary sequences) with the product topology is a compact metrizable space. It is:
- *Totally disconnected*: no connected subset has more than one point.
- *Perfect*: closed with no isolated points.
- *Homeomorphic* to any compact metrizable totally disconnected perfect space — this is Brouwer's theorem.

The Cantor set is the prototypical compact invariant set in chaotic dynamics. It appears as the attractor of the tent map, the Julia set of certain complex maps, and the limit sets of Axiom A diffeomorphisms. The fact that all such sets are homeomorphic (as long as they're compact, metrizable, totally disconnected, and perfect) is a remarkable rigidity result.

Compactness and connectedness together constrain what dynamical behaviors are possible. Compact invariant sets support invariant measures (by Prokhorov). Connected phase spaces prevent certain types of bifurcations. The topology of the phase space is not just a container for the dynamics — it shapes the dynamics profoundly.
