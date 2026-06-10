# Chapter 24: Important Concepts

---

**Riemann Curvature Tensor**
$R^\rho_{\ \sigma\mu\nu} = \partial_\mu\Gamma^\rho_{\nu\sigma} - \partial_\nu\Gamma^\rho_{\mu\sigma} + \Gamma^\rho_{\mu\lambda}\Gamma^\lambda_{\nu\sigma} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\mu\sigma}$: the fundamental measure of curvature. Defined by $[\nabla_\mu, \nabla_\nu]V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma$ (commutator of covariant derivatives). Has 20 independent components in 4D. Zero iff the manifold is flat (locally). The Riemann tensor is the exact GR analog of the electromagnetic field strength $F_{\mu\nu}$.

**Symmetries of the Riemann Tensor**
Antisymmetry in first pair: $R_{\rho\sigma\mu\nu} = -R_{\sigma\rho\mu\nu}$. Antisymmetry in second pair: $R_{\rho\sigma\mu\nu} = -R_{\rho\sigma\nu\mu}$. Pair symmetry: $R_{\rho\sigma\mu\nu} = R_{\mu\nu\rho\sigma}$. Algebraic Bianchi: $R_{\rho[\sigma\mu\nu]} = 0$. Differential Bianchi: $\nabla_{[\lambda}R_{\rho\sigma]\mu\nu} = 0$. These reduce 256 components to 20 independent ones in 4D.

**Riemann Tensor as Tidal Force**
The geodesic deviation equation $D^2\xi^\mu/d\tau^2 = -R^\mu_{\ \nu\rho\sigma}u^\nu\xi^\rho u^\sigma$ shows that the Riemann tensor is the tidal force tensor. In the Newtonian limit: $R^i_{\ 0j0} = \partial^2\Phi/\partial x^i\partial x^j$ (the Hessian of the Newtonian potential). Gravitational tidal forces are curvature. LIGO detects gravitational waves by measuring the time-varying $R_{\mu\nu\rho\sigma}$ via the geodesic deviation of its mirrors.

**Holonomy**
The rotation acquired by a vector after parallel transport around a closed loop. Measures curvature per unit area: $\delta V^\rho = -R^\rho_{\ \sigma\mu\nu}V^\sigma \delta a^\mu \delta b^\nu$. Non-trivial holonomy (in contractible loops) indicates curvature. Non-trivial holonomy in non-contractible loops can occur even in flat space (topology). The Aharonov-Bohm effect is electromagnetic holonomy; the geodesic deviation due to gravitational waves is Riemannian holonomy.

**Ricci Tensor**
$R_{\mu\nu} = R^\rho_{\ \mu\rho\nu}$: the trace of the Riemann tensor. Symmetric (10 independent components in 4D). Geometric meaning: measures volume change in geodesic deviation (Raychaudhuri equation). In vacuum: $R_{\mu\nu} = 0$ (vacuum Einstein equations). The Ricci tensor is directly constrained by the matter distribution through the Einstein equations.

**Ricci Scalar**
$R = g^{\mu\nu}R_{\mu\nu}$: the trace of the Ricci tensor (1 scalar). Measures the deviation of geodesic ball volume from flat-space prediction. Positive $R$: volume smaller than flat (focusing). Negative $R$: volume larger (defocusing). Appears in the Einstein-Hilbert action $S = \int R\sqrt{-g}d^4x/(16\pi G)$ — it is the "Lagrangian" for gravity.

**Weyl Tensor**
$C_{\rho\sigma\mu\nu}$: the trace-free part of the Riemann tensor (10 independent components in 4D). Measures "shape" curvature (tidal distortion without volume change). Can be nonzero in vacuum (Schwarzschild, Kerr, gravitational waves). Conformally invariant. Zero iff the metric is conformally flat. FLRW spacetimes (homogeneous cosmology) have $C_{\rho\sigma\mu\nu} = 0$ (conformally flat). Gravitational waves are pure Weyl curvature.

**Bianchi Identity (Differential)**
$\nabla_{[\lambda}R_{\rho\sigma]\mu\nu} = 0$: a differential identity for the Riemann tensor (the analog of $d(dA) = 0$ for the Maxwell field $F = dA$). Contracting twice: $\nabla_\mu G^{\mu\nu} = 0$ — the Einstein tensor is divergence-free. This is the GR analog of $\partial_\nu(\partial_\mu F^{\mu\nu}) = 0$ (which implies charge conservation in electromagnetism).

**Einstein Tensor**
$G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$: the unique symmetric, divergence-free, rank-2 tensor built from the metric and its first two derivatives (Lovelock 1971). Satisfies $\nabla_\mu G^{\mu\nu} = 0$ identically. The left-hand side of the Einstein equations. Its trace: $G = g^{\mu\nu}G_{\mu\nu} = -R$ (in 4D). The 10 components split into 4 constraints (Bianchi) + 6 evolution equations.

**Einstein Field Equations**
$G_{\mu\nu} = 8\pi G T_{\mu\nu}$ (with $c = 1$): spacetime curvature = stress-energy of matter. Ten equations for ten metric components (minus gauge freedom). Uniquely determine the metric given the stress-energy (up to boundary conditions and gauge). Imply $\nabla_\mu T^{\mu\nu} = 0$ (energy-momentum conservation). Reduce to Newton's $\nabla^2\Phi = 4\pi G\rho$ in the weak-field slow-motion limit.

**Cosmological Constant**
$G_{\mu\nu} + \Lambda g_{\mu\nu} = 8\pi G T_{\mu\nu}$: the generalized Einstein equations including $\Lambda$. Consistent with $\nabla_\mu G^{\mu\nu} = 0$ since $\nabla_\mu g^{\mu\nu} = 0$. Physically: a constant energy density $\rho_\Lambda = \Lambda/(8\pi G)$ and pressure $p_\Lambda = -\rho_\Lambda$ (equation of state $w = -1$). Drives accelerated expansion if $\Lambda > 0$. Observed value $\Lambda \sim 10^{-52}$ m$^{-2}$; quantum field theory predicts $\sim 10^{71}$ m$^{-2}$ — the cosmological constant problem.

**Raychaudhuri Equation**
$d\theta/d\tau = -\frac{1}{n-1}\theta^2 - \sigma_{\mu\nu}\sigma^{\mu\nu} + \omega_{\mu\nu}\omega^{\mu\nu} - R_{\mu\nu}u^\mu u^\nu$: governs the expansion $\theta$ (rate of change of volume of a geodesic congruence). The term $-R_{\mu\nu}u^\mu u^\nu$ is the focusing due to matter (via the Einstein equations and energy conditions). Under the strong energy condition ($R_{\mu\nu}u^\mu u^\nu \geq 0$), geodesics focus — this is the key input to the Penrose-Hawking singularity theorems.

**Vacuum Einstein Equations**
$R_{\mu\nu} = 0$: Einstein's equations in the absence of matter sources. The Schwarzschild and Kerr metrics satisfy these equations (except at the singularity). Nonzero Weyl tensor is allowed. 10 equations, but 4 are Bianchi identities, leaving 6 independent vacuum equations. The vacuum Schwarzschild solution is uniquely determined by spherical symmetry and asymptotic flatness (Birkhoff's theorem).
