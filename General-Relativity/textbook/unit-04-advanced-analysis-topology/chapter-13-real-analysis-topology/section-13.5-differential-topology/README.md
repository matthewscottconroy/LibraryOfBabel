# Section 13.5: Differential Topology

---

## Section Introduction

**Differential topology** studies smooth manifolds and smooth maps between them — topology in the setting of differentiable structure. It occupies the boundary between topology and differential geometry: it uses the tools of calculus (derivatives, vector fields, differential forms) to prove topological results, and it connects the local structure (tangent spaces, smooth maps) to the global topology (handles, surgeries, cobordisms).

The central object is the **smooth manifold** — a topological space that locally looks like $\mathbb{R}^n$ (it is locally homeomorphic to $\mathbb{R}^n$) with the additional structure of a maximal smooth atlas (a consistent collection of coordinate charts with smooth transition functions). The archetypal examples are spheres $S^n$, tori $T^n$, projective spaces $\mathbb{RP}^n$ and $\mathbb{CP}^n$, and Lie groups. Spacetime in GR is a smooth 4-manifold with a Lorentzian metric.

A **smooth map** $f: M\to N$ between manifolds is differentiable in each coordinate chart. The **rank** of $f$ at $p$ is the rank of the Jacobian. If $f$ has full rank everywhere, it is an **immersion** (if rank = dim $M$) or a **submersion** (if rank = dim $N$). The **regular value theorem** (or preimage theorem) is fundamental: if $q$ is a regular value of $f: M\to N$ (all points in $f^{-1}(q)$ are regular), then $f^{-1}(q)$ is a smooth submanifold of $M$ of dimension $\dim M - \dim N$.

**Morse theory** connects the topology of a manifold to the critical points of smooth functions on it. A smooth function $f: M\to\mathbb{R}$ is a **Morse function** if all its critical points are nondegenerate (the Hessian is nonsingular). The Morse lemma gives local coordinates near each critical point, and the **Morse inequalities** bound the Betti numbers (topological invariants) by the numbers of critical points of each index. Morse theory is a bridge from analysis (smooth functions) to topology (homology groups) — the Morse-Smale-Witten complex is the precursor to Floer homology and the mathematical structure of topological field theories.

---

## Subsections

- [13.5.1: Smooth Manifolds and Atlases](13.5.1-manifolds.md)
- [13.5.2: Smooth Maps and Diffeomorphisms](13.5.2-smooth-maps.md)
- [13.5.3: Submersions, Immersions, and Embeddings](13.5.3-submersions.md)
- [13.5.4: Morse Theory](13.5.4-morse.md)
- [13.5.5: Cobordism and Surgery](13.5.5-cobordism.md)
