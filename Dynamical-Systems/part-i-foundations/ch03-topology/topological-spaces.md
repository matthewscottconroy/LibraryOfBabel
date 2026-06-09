# 3.1 Topological Spaces

## 3.1.1 Definitions

In Chapter 1, we defined open sets using balls — a set is open if every point inside it has a ball around it that stays inside. The topological perspective goes further: we can start with the open sets themselves as the primitive notion, and build everything else from them. This frees us from the metric and lets us study spaces where no sensible notion of "distance" exists.

**Definition 3.1.1.** A *topological space* is a pair $(X, \tau)$ where $X$ is a set and $\tau \subseteq 2^X$ (the *topology*, whose elements are *open sets*) satisfies:
1. $\emptyset, X \in \tau$
2. Arbitrary unions of elements of $\tau$ are in $\tau$
3. Finite intersections of elements of $\tau$ are in $\tau$

A set $F$ is *closed* if $X \setminus F$ is open.

These three axioms are exactly the properties we proved for metric space open sets in Chapter 1. The topological perspective says: let's take these properties as the definition, and drop the requirement that the open sets come from a metric. The result is a much more general framework.

The range of topologies is vast:

**Examples 3.1.2.**
- The *discrete topology*: every subset is open. The *indiscrete topology*: only $\emptyset$ and $X$ are open.
- Any metric $(X, d)$ induces a topology: $U$ is open iff for every $x \in U$, some ball $B(x,r) \subseteq U$. Every metric space is a topological space, but not conversely.
- The *Zariski topology* on $\mathbb{R}^n$: closed sets are zero sets of polynomials. This is important in algebraic geometry but produces non-Hausdorff spaces — distinct points can fail to have disjoint neighborhoods.

The properties that distinguish "nice" topological spaces from pathological ones:

**Definition 3.1.3.** A topological space is:
- *Hausdorff (T₂)*: distinct points have disjoint open neighborhoods.
- *Second countable*: there is a countable base for the topology.
- *Separable*: there is a countable dense subset.

A *Polish space* is a separable completely metrizable topological space. Most spaces in dynamics are Polish — they have a compatible metric that makes them complete and separable, even if the topology isn't defined by that metric. Polish spaces are the natural setting for both ergodic theory and descriptive set theory.

## 3.1.2 Continuous Maps

In topology, the fundamental notion is continuity — maps that preserve the topological structure:

**Definition 3.1.4.** $f: X \to Y$ is *continuous* if $f^{-1}(U)$ is open in $X$ for every open $U \subseteq Y$. A *homeomorphism* is a continuous bijection with continuous inverse.

This is the coordinate-free version of continuity: preimages of open sets are open. For metric spaces, this is equivalent to the $\varepsilon$-$\delta$ definition. Two topological spaces are *homeomorphic* if there is a homeomorphism between them — they are "the same" as topological spaces.

**Definition 3.1.5.** A *basis* for a topology $\tau$ is a collection $\mathcal{B} \subseteq \tau$ such that every open set is a union of elements of $\mathcal{B}$.

A basis is a "generating set" for the topology. For metric spaces, the open balls form a basis. Second countability says there's a countable basis.

The topological framework sets the stage for the more structured notions that follow: homotopy, fundamental groups, and smooth manifolds. The key point to take from this section is that topology is about *which maps are continuous* — and continuity is determined by the topology, not the geometry.
