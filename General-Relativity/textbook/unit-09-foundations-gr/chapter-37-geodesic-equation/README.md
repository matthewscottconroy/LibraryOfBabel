# Chapter 37: The Geodesic Equation

---

## Chapter Introduction

What is the "law of motion" in general relativity?

In Newtonian mechanics, it is Newton's second law: $\mathbf{F} = m\mathbf{a}$. In special relativity, it is the covariant extension: $f^\mu = m du^\mu/d\tau$. Both require a force. Remove the force, and the particle moves in a straight line.

In GR, the law of motion for a freely falling particle — one subject to no forces except gravity — is the **geodesic equation:**
$$\frac{d^2 x^\mu}{d\tau^2} + \Gamma^\mu_{\nu\rho}\frac{dx^\nu}{d\tau}\frac{dx^\rho}{d\tau} = 0$$

This says: in the absence of non-gravitational forces, a particle follows a geodesic — the curved-spacetime generalization of a straight line. The Christoffel symbols $\Gamma^\mu_{\nu\rho}$, built from the metric, encode the "bending" of coordinates due to the curvature of spacetime.

The geodesic equation is remarkable in what it does *not* contain: the mass of the particle. Every particle, regardless of its mass or composition, follows the same geodesic in the same spacetime. This is the equivalence principle, expressed geometrically: the trajectory of a freely falling body depends only on the geometry, not on the body's properties.

**What is a geodesic?** Geometrically, a geodesic is a curve that parallel-transports its own tangent vector: $Du^\mu/d\tau = u^\nu\nabla_\nu u^\mu = 0$. Alternatively, it is the curve of extremal (for timelike geodesics, maximal) proper time between two events. Among all worldlines connecting two events, the geodesic — the freely falling worldline — is the one with the longest proper time. This is the principle of maximal aging.

**Light** follows null geodesics: $ds^2 = 0$ along the path, and the same geodesic equation holds (with an affine parameter replacing proper time). The bending of light by gravity — confirmed by Eddington in 1919 — is the statement that photon paths in curved spacetime are null geodesics.

This chapter develops the geodesic equation, its derivation from the variational principle, the role of Killing vectors in identifying conserved quantities, and the geodesic hypothesis (that the equations of motion for matter follow from the field equations, not from an independent postulate). We also work out the Newtonian limit in detail: how the geodesic equation reduces to Newton's second law with the gravitational force.

---

## Chapter Sections

- [Section 37.1: Geodesics as the Law of Motion](section-37.1-geodesics-as-law-of-motion/README.md)
- [Section 37.2: The Newtonian Limit and Post-Newtonian Expansion](section-37.2-newtonian-limit/README.md)
