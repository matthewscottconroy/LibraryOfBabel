# Section 13.2: Topological Spaces

---

## Section Introduction

A **topological space** is the most general setting in which continuity makes sense. The definition abstracts away the metric: a topology $\tau$ on a set $X$ is a collection of subsets (called **open sets**) satisfying: (1) $\emptyset$ and $X$ are open; (2) arbitrary unions of open sets are open; (3) finite intersections of open sets are open. Continuity is then defined as in metric spaces: $f: X\to Y$ is continuous iff the preimage of every open set is open.

The generality of topological spaces is both their strength and their weakness. Every metric space is a topological space (take the metric-ball topology), but not every topological space comes from a metric. Non-metrizable topological spaces arise naturally: the Zariski topology in algebraic geometry, the topology on spaces of distributions, the weak topology on infinite-dimensional Banach spaces. For most purposes in analysis and physics, however, metrizable spaces or manifolds (locally metrizable) suffice.

The **separation axioms** — Hausdorff ($T_2$): any two distinct points have disjoint neighborhoods; regular ($T_3$): a point and a closed set not containing it have disjoint neighborhoods; normal ($T_4$): two disjoint closed sets have disjoint neighborhoods — measure how well the topology separates points and sets. Manifolds (the spaces of GR) are Hausdorff and second-countable (have a countable basis of open sets), which guarantees many useful properties.

**Compactness** in topological spaces is defined by open covers: $X$ is compact if every open cover has a finite subcover. For metric spaces, this coincides with sequential compactness (every sequence has a convergent subsequence), but in general topological spaces the notions differ. **Connectedness** — a space is connected if it cannot be split into two disjoint nonempty open sets — is a purely topological concept, as is the related notion of **path-connectedness**.

---

## Subsections

- [13.2.1: Topologies, Open and Closed Sets](13.2.1-topology.md)
- [13.2.2: Continuity and Homeomorphisms](13.2.2-continuity.md)
- [13.2.3: Separation Axioms](13.2.3-separation.md)
- [13.2.4: Compactness](13.2.4-compactness.md)
- [13.2.5: Connectedness and Path-Connectedness](13.2.5-connectedness.md)
