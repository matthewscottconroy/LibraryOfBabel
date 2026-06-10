# Section 12.5: Conformal Mappings

---

## Section Introduction

A **conformal map** is an angle-preserving map between domains in the complex plane. Since holomorphic functions with nonzero derivative preserve angles at every point (the derivative acts locally as a rotation and scaling), every analytic function with nonzero derivative is conformal. Conformal maps transform geometric shapes while preserving local angles — they stretch and compress but never shear.

The **Riemann mapping theorem** is the central existence theorem: every simply connected open proper subset of $\mathbb{C}$ is conformally equivalent to the unit disk $\mathbb{D}$. Equivalently, there exists a bijective holomorphic map from any such domain to $\mathbb{D}$. This theorem is remarkable in that it guarantees the existence of the map but says almost nothing about how to find it. Specific conformal maps — Möbius transformations, the Joukowski map, the Schwarz-Christoffel formula — are tools for actually constructing the maps for domains with special shapes.

**Möbius transformations** (fractional linear transformations) $f(z) = (az+b)/(cz+d)$ with $ad-bc\neq 0$ are the automorphisms of the Riemann sphere $\hat{\mathbb{C}} = \mathbb{C}\cup\{\infty\}$. They map circles and lines to circles and lines (with lines being circles through $\infty$). The group of Möbius transformations is $PSL(2,\mathbb{C}) = SL(2,\mathbb{C})/\{\pm I\}$ — the same group that appears in spinor theory and in the symmetry group of de Sitter spacetime's conformal boundary.

In physics, conformal mappings appear in two-dimensional electrostatics (the potential of a line charge is a harmonic function, and conformal maps between domains transform solutions of Laplace's equation into solutions), in fluid mechanics (flow past obstacles), and fundamentally in **conformal field theory** (CFT). The conformal group in 2D is infinite-dimensional (all holomorphic functions are conformal), which is why 2D CFT is especially powerful. In GR, **conformal compactification** (Chapter 53) uses conformal maps to bring infinity to a finite location, producing the Penrose diagrams that visualize the global causal structure of spacetimes.

---

## Subsections

- [12.5.1: Conformal Maps and Angle Preservation](12.5.1-definition.md)
- [12.5.2: The Riemann Mapping Theorem](12.5.2-riemann.md)
- [12.5.3: Möbius Transformations](12.5.3-mobius.md)
- [12.5.4: The Schwarz-Christoffel Formula](12.5.4-schwarz-christoffel.md)
- [12.5.5: Applications to Physics and Penrose Diagrams](12.5.5-applications.md)
