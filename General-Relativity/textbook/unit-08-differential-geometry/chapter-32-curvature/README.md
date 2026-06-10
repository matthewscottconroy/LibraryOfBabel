# Chapter 32: Curvature

---

## Chapter Introduction

Curvature is the central concept of Riemannian and Lorentzian geometry. It measures the deviation of a space from being flat — the inability to choose coordinates in which the metric is constant everywhere. In GR, curvature is the mathematical expression of gravity: the Einstein equations relate the curvature of spacetime to the energy-momentum content.

The Riemann curvature tensor $R^\rho_{\ \sigma\mu\nu}$ completely encodes the local geometry. From it, all other curvature quantities are derived: the Ricci tensor $R_{\mu\nu}$, the Ricci scalar $R$, the Weyl tensor $C_{\mu\nu\rho\sigma}$, and the Einstein tensor $G_{\mu\nu}$.

Curvature has an intrinsic geometric meaning: if you parallel-transport a vector around a small loop, it returns rotated by an angle proportional to the curvature enclosed. This is the holonomy interpretation. The Gauss-Bonnet theorem and its generalizations connect curvature to topology — the integral of curvature over a manifold is a topological invariant.

---

## The Riemann Curvature Tensor

Recall from Chapter 31: the Riemann tensor measures the failure of commutativity of covariant derivatives:
$$[\nabla_\mu, \nabla_\nu]V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma$$

In terms of Christoffel symbols:
$$R^\rho_{\ \sigma\mu\nu} = \partial_\mu\Gamma^\rho_{\nu\sigma} - \partial_\nu\Gamma^\rho_{\mu\sigma} + \Gamma^\rho_{\mu\lambda}\Gamma^\lambda_{\nu\sigma} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\mu\sigma}$$

The fully covariant form $R_{\rho\sigma\mu\nu} = g_{\rho\lambda}R^\lambda_{\ \sigma\mu\nu}$ has the symmetries:
$$R_{\rho\sigma\mu\nu} = -R_{\sigma\rho\mu\nu} = -R_{\rho\sigma\nu\mu} = R_{\mu\nu\rho\sigma}$$

and the algebraic Bianchi identity $R_{\rho[\sigma\mu\nu]} = 0$.

**Physical meaning**: Consider a small closed loop in the $x^\mu x^\nu$-plane with sides $\delta a^\mu$ and $\delta b^\nu$. The change in a vector $V^\rho$ after parallel transport around this loop is:
$$\delta V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma\delta a^\mu\delta b^\nu$$

This is the tidal distortion of a vector — geometrically, it measures the "intrinsic curvature" visible to inhabitants of the manifold without reference to any embedding.

---

## Ricci Tensor and Scalar

The **Ricci tensor** is the trace of the Riemann tensor:
$$R_{\mu\nu} = R^\rho_{\ \mu\rho\nu} = g^{\rho\sigma}R_{\rho\mu\sigma\nu}$$

The Ricci tensor is symmetric: $R_{\mu\nu} = R_{\nu\mu}$.

The **Ricci scalar** (scalar curvature) is the trace of the Ricci tensor:
$$R = g^{\mu\nu}R_{\mu\nu} = R^\mu_{\ \mu}$$

Physical significance: $R_{\mu\nu}$ measures the focusing of geodesics (via the Raychaudhuri equation). It is sourced by matter: in GR, $R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R + \Lambda g_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$.

---

## The Weyl Tensor

The **Weyl tensor** $C_{\mu\nu\rho\sigma}$ is the trace-free part of Riemann:
$$R_{\mu\nu\rho\sigma} = C_{\mu\nu\rho\sigma} + \frac{2}{n-2}(g_{\mu[\rho}R_{\sigma]\nu} - g_{\nu[\rho}R_{\sigma]\mu}) - \frac{2}{(n-1)(n-2)}Rg_{\mu[\rho}g_{\sigma]\nu}$$

Properties:
- $C_{\mu\nu\rho\sigma}$ is trace-free: $g^{\mu\rho}C_{\mu\nu\rho\sigma} = 0$
- $C_{\mu\nu\rho\sigma} = 0$ iff the metric is conformally flat ($g_{\mu\nu} = \Omega^2(x)\eta_{\mu\nu}$ for some $\Omega$)
- In 4D: Weyl has 10 independent components; Ricci has 10; together they give the 20 components of Riemann

**Petrov classification**: In 4D Lorentzian geometry, the Weyl tensor is classified by its algebraic type (Petrov I, II, D, III, N, O) based on the structure of its principal null directions. The Schwarzschild metric has Petrov type D (double degenerate) — the two principal null directions are the ingoing and outgoing radial null geodesics.

**Physical significance**: Weyl represents the "free gravitational field" — curvature that exists in vacuum ($R_{\mu\nu} = 0$) but can still produce tidal distortions and gravitational waves. Ricci is "sourced" curvature — directly related to matter. In gravitational waves, $R_{\mu\nu} = 0$ but $C_{\mu\nu\rho\sigma} \neq 0$.

---

## Sectional Curvature

For a 2D subspace $\Pi\subset T_p M$ spanned by orthonormal vectors $e_1, e_2$:
$$K(\Pi) = R_{1212} = R(e_1, e_2, e_1, e_2)$$

is the **sectional curvature** of $\Pi$. On a 2D manifold, $K$ is the Gaussian curvature.

**Spaces of constant curvature**: A manifold with $K = \text{const}$ everywhere is a **space form**:
- $K > 0$: sphere $S^n$ (with appropriate normalization)
- $K = 0$: flat Euclidean space $\mathbb{R}^n$
- $K < 0$: hyperbolic space $H^n$

These three cases correspond to the FLRW cosmological models with $k = +1, 0, -1$.

---

## The Gauss-Bonnet Theorem

For a compact oriented 2-manifold $M$ with Gaussian curvature $K$:
$$\int_M K\,dA + \int_{\partial M}\kappa_g\,ds = 2\pi\chi(M)$$

where $\kappa_g$ is the geodesic curvature of the boundary and $\chi(M)$ is the **Euler characteristic** (a topological invariant).

For closed surfaces: $\chi(S^2) = 2$, $\chi(T^2) = 0$, $\chi(\Sigma_g) = 2 - 2g$ (genus-$g$ surface).

This is a profound result: the integral of the local curvature equals a global topological number. It is the prototype for the Chern-Gauss-Bonnet theorem in higher dimensions and the Atiyah-Singer index theorem.

**Example**: For a sphere of radius $R$: $K = 1/R^2$, area $= 4\pi R^2$, so $\int K\,dA = 4\pi = 2\pi\chi(S^2) = 4\pi$. ✓

**Consequence**: The sum of angles in a triangle on a sphere exceeds $\pi$ by an amount equal to the solid angle of the triangle. For a triangle on a negatively curved surface, the sum is less than $\pi$.

---

## Sectional Curvature and GR

In 4D GR, the sectional curvatures along spacelike 2-planes determine the tidal forces experienced by freely falling observers. For a geodesic observer with 4-velocity $u^\mu$, the tidal acceleration of a nearby geodesic with displacement $\xi^\mu$ is:
$$\frac{D^2\xi^\mu}{d\tau^2} = -R^\mu_{\ \nu\rho\sigma}u^\nu\xi^\rho u^\sigma$$

The **electric part** of the Weyl tensor $E_{\mu\nu} = C_{\mu\rho\nu\sigma}u^\rho u^\sigma$ gives the tidal stretching and compression (the "stretching tensor"). The **magnetic part** $B_{\mu\nu} = \frac{1}{2}\varepsilon_{\mu\rho\lambda\sigma}C^{\lambda\sigma}_{\ \ \nu\tau}u^\rho u^\tau$ encodes frame-dragging and gravitomagnetic effects.

---

## Curvature Invariants

**Kretschner scalar**: $K_{\rm scal} = R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma}$ — a scalar formed from the full Riemann tensor. For Schwarzschild: $K_{\rm scal} = 48G^2M^2/(c^4 r^6)$ — diverges at $r = 0$ (true singularity) but finite at $r = r_s$ (confirming $r = r_s$ is not a curvature singularity).

**Chern-Pontryagin density**: $\tilde{R}R = R_{\mu\nu\rho\sigma}\epsilon^{\rho\sigma\lambda\kappa}R^{\mu\nu}_{\ \ \lambda\kappa}$ — the gravitational analogue of the QCD theta term. Its integral is a topological invariant (Pontryagin number).

**Euler density**: $E_4 = R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma} - 4R_{\mu\nu}R^{\mu\nu} + R^2$ — integrates to give the Euler characteristic in 4D (Chern-Gauss-Bonnet theorem).

---

## Exercises

**32.1.** *Curvature of $S^2$.*

For the 2-sphere with metric $g = d\theta^2 + \sin^2\theta\,d\phi^2$:

(a) Compute the Riemann tensor components $R^\theta_{\ \phi\theta\phi}$ and $R^\phi_{\ \theta\phi\theta}$ from the Christoffel symbols.

(b) Show the Ricci tensor is $R_{\theta\theta} = 1$, $R_{\phi\phi} = \sin^2\theta$.

(c) Compute the Ricci scalar $R$.

(d) Verify that $K = R/2 = 1$ (Gaussian curvature of unit sphere).

---

**32.2.** *Riemann tensor in the weak-field limit.*

The Schwarzschild metric at large $r$ (weak field): $g_{00} = -(1 - 2\Phi/c^2)$, $g_{ij} = (1 + 2\Phi/c^2)\delta_{ij}$ where $\Phi = -GM/r$.

(a) Compute $R_{0i0j}$ to first order in $\Phi/c^2$ (the tidal tensor).

(b) Show that $R_{0i0j} = \partial_i\partial_j\Phi/c^2 = \frac{GM}{c^2 r^3}(\delta_{ij} - 3n_i n_j)$ where $n_i = x_i/r$.

(c) The tidal acceleration of two particles separated by $\xi^i$ is $\delta a^i = -R^i_{\ 0j0}c^2\xi^j$. Compare to the Newtonian tidal force $F^i_{\rm tidal} = -\partial_i\partial_j\Phi\,\xi^j m$.

---

**32.3.** *Gauss-Bonnet for a surface of revolution.*

A surface of revolution $\mathbf{r}(u, \phi) = (f(u)\cos\phi, f(u)\sin\phi, h(u))$ has Gaussian curvature:
$$K = -\frac{f''}{f\sqrt{1+f'^2+h'^2}\cdot\sqrt{1+f'^2+h'^2}}$$

(a) For a cylinder ($f = R = \text{const}$, $h = u$): compute $K$. Verify $\int K\,dA = 0$ over a finite cylinder (open boundary).

(b) For a torus with radii $R > r$: $f = R + r\cos u$, $h = r\sin u$. Compute $K$ and verify $\int K\,dA = 0$ (consistent with $\chi(T^2) = 0$).

---

**Thought Experiment T32.1.** *Is spacetime intrinsically curved or flat?*

The Riemann tensor measures intrinsic curvature — curvature detectable by inhabitants of the manifold without reference to an embedding. On the surface of a sphere, an ant can detect the curvature by measuring triangles, parallel-transporting vectors, or measuring the circumference of a circle relative to its radius.

The equivalence principle says spacetime is locally flat (like a flat tangent plane at a point). But globally, spacetime is curved by matter.

Can you design an experiment that an observer *inside* spacetime could perform to measure the Riemann tensor — without access to any external reference? What is the minimum apparatus required? This is related to the actual experiments done with GRACE satellites (measuring Earth's gravitational field gradient) and with the LISA pathfinder mission.
