# Section 24.1: The Riemann Curvature Tensor

---

## Section Introduction

The Riemann curvature tensor $R^\rho_{\ \sigma\mu\nu}$ is the fundamental measure of curvature in general relativity. It answers the question: when we parallel transport a vector around an infinitesimal closed loop, by how much does it rotate?

In flat spacetime, the answer is zero — covariant derivatives commute. In curved spacetime, $[\nabla_\mu, \nabla_\nu]V^\rho \neq 0$, and the commutator is proportional to the Riemann tensor: $[\nabla_\mu, \nabla_\nu]V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma$. This commutator is the definition of the Riemann tensor.

The Riemann tensor has 20 independent components in 4D (reduced from 256 = 4⁴ by symmetries). Its contraction gives the Ricci tensor (10 components), and further contraction gives the Ricci scalar (1 component). These appear in the Einstein equations.

---

## 24.1.1 Definition via Commutator of Covariant Derivatives

**Motivation**: In flat space, partial derivatives commute: $\partial_\mu\partial_\nu f = \partial_\nu\partial_\mu f$. But covariant derivatives of vectors need not commute on a curved manifold. The failure to commute is the curvature.

**Definition**: For a vector field $V^\rho$:

$$[\nabla_\mu, \nabla_\nu]V^\rho \equiv \nabla_\mu\nabla_\nu V^\rho - \nabla_\nu\nabla_\mu V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma$$

This defines the **Riemann tensor** $R^\rho_{\ \sigma\mu\nu}$.

**Explicit formula**: Computing $\nabla_\mu\nabla_\nu V^\rho - \nabla_\nu\nabla_\mu V^\rho$ using the definition of the covariant derivative:

$$R^\rho_{\ \sigma\mu\nu} = \partial_\mu\Gamma^\rho_{\nu\sigma} - \partial_\nu\Gamma^\rho_{\mu\sigma} + \Gamma^\rho_{\mu\lambda}\Gamma^\lambda_{\nu\sigma} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\mu\sigma}$$

*Derivation*:
$$\nabla_\mu(\nabla_\nu V^\rho) = \partial_\mu(\nabla_\nu V^\rho) + \Gamma^\rho_{\mu\lambda}(\nabla_\nu V^\lambda) - \Gamma^\lambda_{\mu\nu}(\nabla_\lambda V^\rho)$$
$$= \partial_\mu(\partial_\nu V^\rho + \Gamma^\rho_{\nu\sigma}V^\sigma) + \Gamma^\rho_{\mu\lambda}(\partial_\nu V^\lambda + \Gamma^\lambda_{\nu\sigma}V^\sigma) - \Gamma^\lambda_{\mu\nu}\nabla_\lambda V^\rho$$

Antisymmetrizing in $[\mu, \nu]$ (the $\partial_\mu\partial_\nu V^\rho$ and $\Gamma^\lambda_{\mu\nu}\nabla_\lambda V^\rho$ terms cancel since we're using a torsion-free connection):

$$[\nabla_\mu, \nabla_\nu]V^\rho = \left(\partial_\mu\Gamma^\rho_{\nu\sigma} - \partial_\nu\Gamma^\rho_{\mu\sigma} + \Gamma^\rho_{\mu\lambda}\Gamma^\lambda_{\nu\sigma} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\mu\sigma}\right)V^\sigma = R^\rho_{\ \sigma\mu\nu}V^\sigma \qquad \square$$

**For general tensors**: The curvature appears as a commutator term for each index:
$$[\nabla_\mu, \nabla_\nu]T^{\rho\sigma} = R^\rho_{\ \lambda\mu\nu}T^{\lambda\sigma} + R^\sigma_{\ \lambda\mu\nu}T^{\rho\lambda}$$

---

## 24.1.2 Symmetries of the Riemann Tensor

The fully covariant Riemann tensor $R_{\rho\sigma\mu\nu} = g_{\rho\lambda}R^\lambda_{\ \sigma\mu\nu}$ has the following symmetries:

1. **Antisymmetry in last two indices**: $R_{\rho\sigma\mu\nu} = -R_{\rho\sigma\nu\mu}$

2. **Antisymmetry in first two indices**: $R_{\rho\sigma\mu\nu} = -R_{\sigma\rho\mu\nu}$

3. **Symmetry under pair exchange**: $R_{\rho\sigma\mu\nu} = R_{\mu\nu\rho\sigma}$

4. **First Bianchi identity** (algebraic): $R_{\rho[\sigma\mu\nu]} = 0$, i.e., $R_{\rho\sigma\mu\nu} + R_{\rho\mu\nu\sigma} + R_{\rho\nu\sigma\mu} = 0$

These symmetries reduce the number of independent components:
- In $n$ dimensions: $\frac{n^2(n^2-1)}{12}$ independent components
- In 4D: $\frac{16\cdot15}{12} = 20$ independent components

The 20 components decompose as: 10 in the Weyl tensor $C_{\rho\sigma\mu\nu}$ (trace-free part) + 10 in the Ricci tensor $R_{\mu\nu}$ (the traced part).

**Second Bianchi identity** (differential):

$$\nabla_{[\lambda}R_{\rho\sigma]\mu\nu} = 0 \quad \Leftrightarrow \quad \nabla_\lambda R_{\rho\sigma\mu\nu} + \nabla_\rho R_{\sigma\lambda\mu\nu} + \nabla_\sigma R_{\lambda\rho\mu\nu} = 0$$

The contracted version gives the contracted Bianchi identity $\nabla_\mu G^{\mu\nu} = 0$ (Section 24.2.4).

---

## 24.1.3 Geometric Interpretation: Holonomy

**Parallel transport around an infinitesimal loop**: Consider a small parallelogram with sides $\delta a^\mu$ and $\delta b^\nu$. Parallel transport a vector $V^\rho$ around this loop. The change in $V^\rho$ is:

$$\delta V^\rho = -R^\rho_{\ \sigma\mu\nu}V^\sigma\delta a^\mu\delta b^\nu$$

This is the **holonomy** of the Levi-Civita connection around the infinitesimal loop. The Riemann tensor measures the curvature per unit area: $\delta V^\rho / (\delta A) = -R^\rho_{\ \sigma\mu\nu}V^\sigma (\delta a^\mu\delta b^\nu/\delta A)$.

**Physical picture**: On a sphere, parallel transport a vector around a triangle with three right-angle corners (total angle $3\pi/2$, vs. $\pi$ for a flat triangle). The vector rotates by $\pi/2$ (the "excess angle" = area × Gaussian curvature for the unit sphere). This is the Gauss-Bonnet theorem in action.

**Flat spacetime**: $R^\rho_{\ \sigma\mu\nu} = 0$ iff $\Gamma = 0$ in some global coordinate system iff parallel transport has trivial holonomy around all contractible loops. (Non-contractible loops can have non-trivial holonomy even in flat space — this is related to the topology, not the curvature.)

---

## 24.1.4 The Geodesic Deviation Equation

The relative acceleration of two nearby geodesics (the "tidal acceleration") is governed by the **geodesic deviation equation**:

$$\frac{D^2\xi^\mu}{d\tau^2} = -R^\mu_{\ \nu\rho\sigma}u^\nu\xi^\rho u^\sigma$$

where $\xi^\mu$ is the separation vector between the two geodesics, $u^\mu = dx^\mu/d\tau$ is the 4-velocity, and $D/d\tau = u^\mu\nabla_\mu$ is the covariant derivative along the geodesic.

*Derivation sketch*: Consider a 2-parameter family of geodesics $x^\mu(\tau, s)$, with $u^\mu = \partial x^\mu/\partial\tau$ (velocity along geodesics) and $\xi^\mu = \partial x^\mu/\partial s$ (separation vector). Using the geodesic equation and the definition of the Riemann tensor (via the commutator of covariant derivatives), one arrives at the geodesic deviation equation.

**Newtonian limit**: In the weak-field, slow-motion limit, $R^i_{\ 0j0} = -\partial^2\Phi/\partial x^i\partial x^j$ (tidal tensor), and the geodesic deviation equation reduces to:

$$\ddot{\xi}^i = -R^i_{\ 0j0}\xi^j = \frac{\partial^2\Phi}{\partial x^i\partial x^j}\xi^j$$

This is exactly the Newtonian tidal force equation (Section 14.3.5). **Tidal forces are components of the Riemann tensor.** This is the most direct physical meaning of $R_{\mu\nu\rho\sigma}$.

**Gravitational waves**: A gravitational wave is a propagating perturbation of the metric — and hence of the Riemann tensor. The geodesic deviation equation gives the response of a LIGO interferometer: the two test masses at the ends of the arms are on nearby geodesics, and the passing gravitational wave (a time-varying $R_{\mu\nu\rho\sigma}$) accelerates them relative to each other, changing the arm length.

---

## 24.1.5 The Weyl Tensor

The Riemann tensor decomposes as:

$$R_{\rho\sigma\mu\nu} = C_{\rho\sigma\mu\nu} + \frac{2}{n-2}\left(g_{\rho[\mu}R_{\nu]\sigma} - g_{\sigma[\mu}R_{\nu]\rho}\right) - \frac{2}{(n-1)(n-2)}Rg_{\rho[\mu}g_{\nu]\sigma}$$

where $C_{\rho\sigma\mu\nu}$ is the **Weyl tensor** (trace-free: $C^\rho_{\ \sigma\rho\nu} = 0$). In 4D:

$$R_{\rho\sigma\mu\nu} = C_{\rho\sigma\mu\nu} + \frac{1}{1}(g_{\rho[\mu}R_{\nu]\sigma} - g_{\sigma[\mu}R_{\nu]\rho}) - \frac{1}{6}Rg_{\rho[\mu}g_{\nu]\sigma}$$

**Physical meaning**:
- The **Ricci tensor** $R_{\mu\nu}$ encodes the "volume" curvature — it is zero in vacuum (Einstein's equations in vacuum: $R_{\mu\nu} = 0$). The Ricci tensor describes tidal forces that focus/defocus geodesics.
- The **Weyl tensor** $C_{\rho\sigma\mu\nu}$ encodes the "shape" curvature — it can be nonzero in vacuum (e.g., in the Schwarzschild and Kerr solutions). The Weyl tensor describes tidal forces that distort (stretch and compress) a sphere of freely falling particles without changing its volume.

**Gravitational waves are Weyl curvature**: A gravitational wave in vacuum has $R_{\mu\nu} = 0$ (it satisfies the vacuum Einstein equations) but $C_{\rho\sigma\mu\nu} \neq 0$ (there is tidal distortion). The gravitational wave signal is purely Weyl curvature.

**Conformal flatness**: A spacetime is conformally flat ($g_{\mu\nu} = \Omega^2 \eta_{\mu\nu}$ for some scalar $\Omega$) iff $C_{\rho\sigma\mu\nu} = 0$. FLRW spacetimes (homogeneous, isotropic cosmology) are conformally flat — there is no Weyl curvature, which reflects the fact that the universe is homogeneous and there are no preferred directions for tidal distortion.

---

## References

- Riemann, B. (1854). "Über die Hypothesen, welche der Geometrie zu Grunde liegen." [First defines the curvature tensor (in terms of sectional curvatures) and proves the Gauss-Bonnet theorem generalization. The curvature tensor now bearing his name is derived from these ideas.]
- Bianchi, L. (1902). "Sui simboli a quattro indici e sulla curvatura di Riemann." *Rendiconti della Reale Accademia dei Lincei*, 11, 3–7. [The Bianchi identity $\nabla_{[\lambda}R_{\rho\sigma]\mu\nu} = 0$ — the differential Bianchi identity for the Riemann tensor. Its contraction gives $\nabla_\mu G^{\mu\nu} = 0$, which is what guarantees $\nabla_\mu T^{\mu\nu} = 0$ in GR.]
- Pirani, F.A.E. (1957). "Invariant formulation of gravitational radiation theory." *Physical Review*, 105, 1089–1099. [The physical interpretation of the Riemann tensor via the geodesic deviation equation; tidal forces as curvature; the Weyl tensor and gravitational radiation.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [Chapter 11 on the Riemann tensor and its symmetries; Chapter 13 on geodesic deviation; §18.1 on the Weyl tensor.]
