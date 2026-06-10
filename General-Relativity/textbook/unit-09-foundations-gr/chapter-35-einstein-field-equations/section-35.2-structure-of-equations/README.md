# Section 35.2: Structure of the Einstein Field Equations

---

## Ten Equations, Four Constraints, Six Degrees of Freedom

The Einstein field equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}$ are ten equations (since $G_{\mu\nu}$ is symmetric: $G_{00}, G_{01}, G_{02}, G_{03}, G_{11}, G_{12}, G_{13}, G_{22}, G_{23}, G_{33}$). They determine the ten components of the symmetric metric tensor $g_{\mu\nu}$.

But there is a subtlety. The contracted Bianchi identity says $\nabla_\mu G^{\mu\nu} = 0$ identically — for *any* metric, not just solutions of the field equations. This means four of the ten equations are automatically satisfied (they are constraints) and cannot be used to evolve the metric independently. The ten equations have only six independent pieces.

Furthermore, the four-fold freedom of diffeomorphism invariance (general covariance) means that the metric has only $10 - 4 = 6$ physical components — four of the ten can be set to any value by a choice of coordinates. Of these six physical components, four are determined by initial conditions and four constraints reduce the initial data further. In the end, the gravitational field has **two physical degrees of freedom** — corresponding to the two polarization states of gravitational waves.

This counting is analogous to electromagnetism: the vector potential $A_\mu$ has four components, gauge invariance removes one, and Lorenz gauge plus the wave equation leaves two physical polarizations.

---

## Initial Value Formulation

The Einstein equations can be split into constraints and evolution equations, analogous to Maxwell's equations (which split into two constraint equations $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$, $\nabla\cdot\mathbf{B} = 0$ and two evolution equations $\partial_t\mathbf{E}$ and $\partial_t\mathbf{B}$).

In the **ADM (Arnowitt-Deser-Misner) formalism**, spacetime is foliated by spacelike hypersurfaces $\Sigma_t$ (equal-time slices). The metric is written:
$$ds^2 = -\alpha^2 dt^2 + \gamma_{ij}(dx^i + \beta^i dt)(dx^j + \beta^j dt)$$
where:
- $\alpha$ is the **lapse function** (measures how proper time relates to coordinate time)
- $\beta^i$ is the **shift vector** (measures how spatial coordinates shift between slices)
- $\gamma_{ij}$ is the **spatial metric** on each hypersurface

The Einstein equations split into:
- **Hamiltonian constraint:** ${}^{(3)}R + K^2 - K_{ij}K^{ij} = 16\pi G\rho$ (one equation)
- **Momentum constraints:** $\nabla_j(K^{ij} - \gamma^{ij}K) = 8\pi G J^i$ (three equations)
- **Evolution equations:** $\partial_t\gamma_{ij}$ and $\partial_t K_{ij}$ (twelve equations, but six physical degrees of freedom)

Here $K_{ij}$ is the extrinsic curvature of the spacelike slice (how it bends in the ambient spacetime), ${}^{(3)}R$ is its intrinsic 3D Ricci scalar, $\rho = T_{\mu\nu}n^\mu n^\nu$ is the energy density, and $J^i = -T_{\mu\nu}n^\mu\gamma^{i\nu}$ is the momentum density ($n^\mu$ is the unit normal to the slice).

The four constraint equations must be satisfied on the initial slice; the evolution equations then guarantee they are satisfied on all subsequent slices (the Cauchy problem for GR).

This formalism is the foundation of numerical relativity — the computation of GR solutions on a computer. The binary black hole merger waveforms detected by LIGO are computed using numerical GR codes based on the ADM formalism or variants of it.

---

## Linearized Gravity

The simplest solutions arise when the metric is a small perturbation of flat Minkowski spacetime:
$$g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}, \quad |h_{\mu\nu}| \ll 1$$

In Lorenz gauge $\partial^\mu\bar{h}_{\mu\nu} = 0$ (where $\bar{h}_{\mu\nu} = h_{\mu\nu} - \frac{1}{2}\eta_{\mu\nu}h$ is the trace-reversed perturbation), the field equations linearize to:
$$\Box\bar{h}_{\mu\nu} = -\frac{16\pi G}{c^4}T_{\mu\nu}$$

This is exactly analogous to Maxwell's equations in Lorenz gauge: $\Box A^\mu = \mu_0 J^\mu$. The gravitational perturbation $\bar{h}_{\mu\nu}$ plays the role of the electromagnetic 4-potential $A^\mu$, and $T_{\mu\nu}/c^4$ plays the role of the 4-current $J^\mu/c$.

The retarded solution is:
$$\bar{h}_{\mu\nu}(t, \mathbf{x}) = \frac{4G}{c^4}\int\frac{T_{\mu\nu}(t - |\mathbf{x}-\mathbf{x}'|/c, \mathbf{x}')}{|\mathbf{x}-\mathbf{x}'|}d^3x'$$

This is the gravitational analog of the retarded electromagnetic potential. For a slow-moving source at large distance, the leading term gives the Newtonian potential; the next term gives gravitational waves (the quadrupole formula).

**Propagation speed:** The wave equation $\Box\bar{h}_{\mu\nu} = 0$ propagates at speed $c$. Gravitational waves travel at the speed of light. Confirmed by GW170817 (2017): the gravitational wave signal from a neutron star merger arrived 1.74 seconds before the gamma-ray burst — consistent with both traveling at $c$ within $3\times 10^{-15}c$ over 1.3 billion light-years.

---

## The Role of Symmetry in Finding Solutions

The full nonlinear Einstein equations are intractable in general. Exact analytic solutions exist only when the metric has sufficient symmetry to reduce the PDEs to ODEs or to algebraic equations.

**Spherical symmetry:** Birkhoff's theorem (1923) states that the unique spherically symmetric, asymptotically flat vacuum solution of the Einstein equations is the Schwarzschild metric:
$$ds^2 = -\left(1 - \frac{2GM}{rc^2}\right)c^2dt^2 + \left(1 - \frac{2GM}{rc^2}\right)^{-1}dr^2 + r^2d\Omega^2$$
Even a pulsating spherical star has a Schwarzschild exterior metric — there is no monopole gravitational radiation.

**Axial symmetry + stationarity:** The Kerr metric (1963) is the unique solution for a rotating black hole. It took 47 years after Schwarzschild to find it, despite the symmetry.

**Homogeneity + isotropy:** The Friedmann-Lemaître-Robertson-Walker (FLRW) metric describes a homogeneous, isotropic universe:
$$ds^2 = -c^2dt^2 + a(t)^2\left(\frac{dr^2}{1-kr^2} + r^2d\Omega^2\right)$$
The Einstein equations reduce to the **Friedmann equations** for the scale factor $a(t)$.

---

## Counting Solutions and the Landscape

The Einstein equations do not have a unique solution — they have many. Given boundary conditions, they have a unique solution (Cauchy problem, Choquet-Bruhat 1952), but different boundary conditions give different spacetimes.

The space of solutions includes:
- Flat Minkowski space ($T_{\mu\nu} = 0$, $\Lambda = 0$)
- Anti-de Sitter space (AdS, $\Lambda < 0$)
- de Sitter space (dS, $\Lambda > 0$)
- Schwarzschild black holes (any mass $M > 0$)
- Kerr black holes (any mass $M$ and angular momentum $J$ with $|J| \leq GM^2/c$)
- Cosmological solutions (FLRW with various matter contents)
- Gravitational wave spacetimes (pp-waves, plane waves)
- And many others: Reissner-Nordström, Kerr-Newman, Taub-NUT, Kasner, etc.

This richness is a feature, not a bug. The Einstein equations describe a tremendous variety of phenomena with a single compact set of equations.

---

## Why Not Other Theories?

The uniqueness provided by Lovelock's theorem is a powerful argument for GR, but it assumes 4 dimensions and second-order equations. Several alternatives have been proposed:

**Brans-Dicke theory:** A scalar field $\phi$ (the reciprocal of the gravitational "constant") couples to the Ricci scalar. Reduces to GR for large coupling parameter $\omega_{BD}$. Solar system tests require $\omega_{BD} > 40,000$.

**$f(R)$ gravity:** Replaces $R$ with $f(R)$ in the action, giving fourth-order equations. Can mimic dark energy. Some versions are equivalent to a scalar-tensor theory.

**Gauss-Bonnet gravity:** In $n \geq 5$ dimensions, adds the Gauss-Bonnet term $R^2 - 4R_{\mu\nu}R^{\mu\nu} + R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma}$, which gives second-order equations. Topological (total divergence) in $n = 4$, so adds nothing in 4D.

**Massive gravity:** Gives the graviton a mass $m_g > 0$. Changes the long-range behavior. The Vainshtein mechanism recovers GR predictions on small scales. Solar system tests and the detection of gravitational waves at the speed of light constrain $m_g < 7\times 10^{-23}$ eV/$c^2$.

All alternatives are more complex than GR and require additional parameters. In the absence of observational evidence for departures from GR, Occam's razor strongly favors the Einstein equations.

