# 9.7 Axiom A and Structural Stability

Smale's Axiom A gives a framework for organizing the global structure of a large class of chaotic systems.

**Definition 9.7.1 (Smale's Axiom A).** A diffeomorphism $f: M \to M$ satisfies *Axiom A* if:
1. The nonwandering set $\Omega(f)$ is hyperbolic
2. Periodic points are dense in $\Omega(f)$

Recall from Chapter 6 that the nonwandering set $\Omega(f)$ consists of points near which orbits keep returning. Axiom A says this set — where all the interesting long-term behavior happens — is hyperbolic and has dense periodic points. This is exactly the structure we saw in the horseshoe: a hyperbolic invariant Cantor set with dense periodic orbits.

**Theorem 9.7.2 (Smale's Spectral Decomposition).** For an Axiom A diffeomorphism, $\Omega(f) = \Lambda_1 \cup \cdots \cup \Lambda_k$ where each $\Lambda_i$ is a closed, $f$-invariant, topologically transitive set (a *basic set*). The basic sets are ordered: there is no cycle among them.

What this is really saying: the nonwandering set breaks into finitely many irreducible pieces, each of which is topologically transitive (has a dense orbit). These pieces are ordered — you can define a partial order where $\Lambda_i \geq \Lambda_j$ if there's an orbit from near $\Lambda_i$ to near $\Lambda_j$. The "no cycle" condition means this partial order has no loops. This is a precise decomposition of the global attractor structure.

---

## Structural Stability

**Definition 9.7.3.** $f$ is *structurally stable* if every $g$ sufficiently $C^1$-close to $f$ is topologically conjugate to $f$.

**Theorem 9.7.4 (Robbin-Robinson).** Axiom A + strong transversality (stable and unstable manifolds intersect transversally) implies structural stability.

**Theorem 9.7.5 (Mañé).** Structural stability implies Axiom A + strong transversality. (So Axiom A + ST $\Leftrightarrow$ structural stability.)

The Robbin-Robinson theorem (1971-1976) gave a sufficient condition for structural stability. Mañé's theorem (1988) showed the condition is also necessary — completing the characterization that structural stability is equivalent to Axiom A plus the transversality condition. This resolved a question that had been open for two decades.

The strong transversality condition says: whenever a stable manifold and an unstable manifold of periodic points intersect, they do so transversally (their tangent spaces span the ambient space). This prevents the kind of tangential homoclinic intersections (Newhouse phenomena) that can generate infinitely many coexisting sinks.

The structural stability of Anosov diffeomorphisms follows from Theorem 9.7.4 as a special case: Anosov diffeomorphisms are Axiom A (the nonwandering set is the entire manifold, which is hyperbolic), and the transversality condition is automatically satisfied (stable and unstable manifolds of the whole manifold intersect transversally by the Anosov property). So any perturbation of an Anosov diffeomorphism is topologically conjugate to the original.
