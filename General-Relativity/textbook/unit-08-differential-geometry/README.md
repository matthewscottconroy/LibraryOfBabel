# Unit VIII: Differential Geometry

---

## Unit Introduction

Differential geometry is the mathematics of curved spaces. It is the language in which GR is written.

The central objects are **smooth manifolds** — spaces that locally look like ℝⁿ but may be globally curved or topologically non-trivial. The sphere S², the torus T², and spacetime are all manifolds. A manifold is defined by its **atlas** — a collection of coordinate charts with smooth transition maps. On a manifold, we can do calculus: smooth functions, tangent vectors, differential forms, and tensor fields are all well-defined, coordinate-independently.

The **metric tensor** gᵤᵥ is a smooth assignment of a non-degenerate inner product to each tangent space. It defines distances, angles, volumes, and the notion of a "straight line" (geodesic). Given a metric, there is a unique symmetric, metric-compatible connection — the **Levi-Civita connection** — that defines parallel transport and the covariant derivative.

The **Riemann curvature tensor** Rᵅ_{βγδ} measures the failure of parallel transport to be path-independent. On a flat space (ℝⁿ with the Euclidean metric), R = 0. A curved space has R ≠ 0; the curvature encodes how nearby geodesics diverge or converge. Einstein's insight was that gravity is not a force but the curvature of spacetime: the Einstein equations Gᵤᵥ = 8πTᵤᵥ relate the Einstein tensor (built from R) to the stress-energy tensor (the source of curvature).

This unit develops, in full rigor:
- Smooth manifolds, tangent spaces, differential forms
- Vector fields, tensor fields, Lie derivatives
- Connections, parallel transport, covariant derivatives
- Geodesics, the geodesic equation, the exponential map
- Curvature: the Riemann tensor, Ricci tensor, Ricci scalar
- The Bianchi identities and the Einstein tensor

These are the immediate prerequisites for GR.

---

## Chapters in This Unit

- [Chapter 23: Connections and Covariant Derivatives](chapter-23-connections/README.md)
- [Chapter 24: Curvature](chapter-24-curvature/README.md)
- [Chapter 27: Smooth Manifolds](chapter-27-smooth-manifolds/README.md)
- [Chapter 28: Tangent and Cotangent Spaces](chapter-28-tangent-cotangent-spaces/README.md)
- [Chapter 29: Tensor Fields and Tensor Algebra](chapter-29-tensors/README.md)
- [Chapter 30: Differential Forms and Integration](chapter-30-differential-forms/README.md)
- [Chapter 31: Connections and Covariant Derivatives](chapter-31-connections-covariant-derivatives/README.md)
- [Chapter 32: Curvature](chapter-32-curvature/README.md)
- [Chapter 33: Geodesics](chapter-33-geodesics/README.md)
