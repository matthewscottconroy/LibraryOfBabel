# Chapter 24: Curvature

---

## Chapter Introduction

Curvature is the central concept of differential geometry and the mathematical heart of general relativity. A space is flat if parallel transport around any closed loop returns a vector to its original orientation. Curvature measures the failure of this — the rotation acquired by a vector after parallel transport around an infinitesimal loop.

This rotation is encoded in the **Riemann curvature tensor** $R^\rho_{\ \sigma\mu\nu}$: a rank-4 tensor built from second derivatives of the metric (equivalently, first derivatives of the Christoffel symbols). Its contractions — the **Ricci tensor** $R_{\mu\nu}$ and the **Ricci scalar** $R$ — appear directly in the Einstein equations.

The **Einstein tensor** $G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$ is the unique symmetric, divergence-free combination of curvature that appears on the left-hand side of the Einstein equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$. The divergence-free condition $\nabla_\mu G^{\mu\nu} = 0$ — the contracted Bianchi identity — is what guarantees conservation of energy-momentum ($\nabla_\mu T^{\mu\nu} = 0$).

Understanding the Riemann tensor — its definition, symmetries, geometric meaning, and contractions — is the prerequisite for everything in GR from this point forward.

---

## Chapter Contents

- **Section 24.1**: The Riemann curvature tensor; definition via the commutator of covariant derivatives; geometric interpretation (holonomy); symmetries; the Bianchi identity; the geodesic deviation equation

- **Section 24.2**: The Ricci tensor; the Ricci scalar; the Einstein tensor; the contracted Bianchi identity; Einstein's equations in vacuum and with sources; the cosmological constant

---

## Curvature and Gravity

The profound insight of general relativity is that gravity is not a force — it is curvature of spacetime. A freely falling particle follows a geodesic (Section 23.2.4). Two nearby geodesics that start parallel will converge or diverge — not because of a force, but because the geodesics are the "straightest possible paths" in a curved spacetime.

The equation governing the relative acceleration of nearby geodesics — the **geodesic deviation equation** — involves the Riemann tensor directly: $D^2\xi^\mu/d\tau^2 = -R^\mu_{\ \nu\rho\sigma}u^\nu\xi^\rho u^\sigma$. The gravitational tidal force is the Riemann tensor.

The Einstein equations specify which curvature is consistent with the matter content: $G_{\mu\nu} = 8\pi G T_{\mu\nu}$. Given the matter distribution $T_{\mu\nu}$, the Einstein equations determine the metric $g_{\mu\nu}$ (up to gauge/coordinate freedom), and hence the curvature — and hence the motion of all particles and light rays.

The Einstein equations are ten coupled, nonlinear, second-order PDEs for the ten independent components of $g_{\mu\nu}$. Their nonlinearity reflects the fact that gravitational energy itself contributes to the curvature (gravity gravitates). This self-coupling is what makes GR qualitatively different from Newtonian gravity and electromagnetism.
