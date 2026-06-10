# Chapter 23: Connections and Covariant Derivatives

---

## Chapter Introduction

The fundamental challenge of calculus on curved manifolds is differentiation. On flat $\mathbb{R}^n$, we can differentiate a vector field $V^\mu(x)$ by taking $\partial_\nu V^\mu$ — the partial derivative of each component. On a curved manifold, this fails: the components $V^\mu$ depend on the choice of coordinates, and $\partial_\nu V^\mu$ does not transform as a tensor under coordinate changes.

The solution is the **covariant derivative** $\nabla_\nu V^\mu$: a derivative operation that transforms as a tensor and reduces to $\partial_\nu V^\mu$ in flat space. To define it, we need a **connection** — additional structure that specifies how to "transport" vectors between nearby tangent spaces (which are distinct vector spaces at each point of the manifold).

On a Riemannian manifold (or pseudo-Riemannian, for GR), there is a unique connection that is compatible with the metric ($\nabla_\rho g_{\mu\nu} = 0$) and torsion-free ($\Gamma^\rho_{\mu\nu} = \Gamma^\rho_{\nu\mu}$): the **Levi-Civita connection**. This is the connection used in GR.

The Christoffel symbols $\Gamma^\rho_{\mu\nu}$ are the components of the Levi-Civita connection in a coordinate basis. They encode the "bending" of coordinates — in Cartesian coordinates on flat space, they vanish; on curved surfaces or in curvilinear coordinates, they are nonzero. The geodesic equation (the GR replacement for Newton's first law) is written in terms of Christoffel symbols.

---

## Chapter Contents

- **Section 23.1**: The covariant derivative; parallel transport; the torsion-free, metric-compatible (Levi-Civita) connection; geometric interpretation

- **Section 23.2**: The Christoffel symbols; explicit formula; computation on specific metrics (sphere, Schwarzschild); the geodesic equation

---

## What is a Connection?

Imagine you stand at the equator of a sphere and hold a vector pointing north. You walk east 90°, then north to the pole, then south back to where you started — but along a different meridian. When you return, the vector you've been "parallel transporting" now points in a different direction than when you left. This is **holonomy** — a consequence of the curvature of the sphere.

To define parallel transport, you need to specify what "keeping the vector constant along a path" means on a curved surface. This is precisely what a **connection** does: it provides a rule for parallel transporting vectors along curves.

The Levi-Civita connection is the canonical choice on a Riemannian manifold: it preserves the metric (so angles and lengths don't change under parallel transport) and has no torsion (so parallel transport "without twisting"). These two conditions uniquely determine the connection, which is then given by the Christoffel symbols.

The holonomy of the connection (the rotation acquired by a vector after parallel transport around a closed loop) is measured by the **curvature** — the Riemann tensor (Chapter 24). This is exactly the sense in which tidal forces (the GR notion of gravity) are curvature.
