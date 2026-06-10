# Chapter 35: Important Concepts

---

**Einstein Field Equations**
$G_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$ (or $G_{\mu\nu} = 8\pi G T_{\mu\nu}$ in natural units): ten symmetric coupled nonlinear second-order PDEs relating the Einstein tensor (curvature of spacetime) to the stress-energy tensor (matter and energy). The most fundamental equations of GR.

**Einstein Tensor**
$G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$: the unique symmetric divergence-free ($\nabla_\mu G^{\mu\nu} = 0$) rank-2 tensor built from the metric and its first two derivatives (Lovelock 1971). Satisfies the contracted Bianchi identity identically.

**Lovelock's Theorem**
In 4 spacetime dimensions, the only symmetric, divergence-free, rank-2 tensor built from $g_{\mu\nu}$ and its first two derivatives is $\alpha G_{\mu\nu} + \Lambda g_{\mu\nu}$. This uniqueness theorem explains why the Einstein equations are the "correct" theory of gravity — they are the only second-order metric theory consistent with energy-momentum conservation.

**Einstein-Hilbert Action**
$S = \frac{c^4}{16\pi G}\int R\sqrt{-g}\,d^4x + S_{\rm matter}$: the action whose variation with respect to $g^{\mu\nu}$ gives the Einstein field equations. The Ricci scalar $R$ is the simplest covariant Lagrangian density for gravity. Hilbert (1915) derived the field equations from this action five days before Einstein presented them to the Prussian Academy.

**Cosmological Constant**
$\Lambda$ added as $G_{\mu\nu} + \Lambda g_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$: consistent with the Bianchi identity since $\nabla_\mu g^{\mu\nu} = 0$. Introduced by Einstein (1917) for a static universe, called his "greatest blunder" after Hubble's discovery of expansion, revived by the 1998 supernova observations. Current value $\Lambda \approx 1.1\times 10^{-52}$ m$^{-2}$. The cosmological constant problem: why is $\Lambda$ so small ($10^{-123}$ times the QFT prediction)?

**Linearized Gravity**
$g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}$, $|h_{\mu\nu}| \ll 1$: the perturbative expansion of GR around flat spacetime. In Lorenz gauge $\partial^\mu\bar{h}_{\mu\nu} = 0$, the field equations become $\Box\bar{h}_{\mu\nu} = -16\pi G T_{\mu\nu}/c^4$ — formally identical to Maxwell's equations. Gravitational waves are propagating solutions of this equation with speed $c$.

**Gravitational Degrees of Freedom**
The 10 metric components minus 4 gauge (diffeomorphism) freedom = 6 physical components. Minus 4 constraint equations = 2 dynamical degrees of freedom — the two polarizations (+, ×) of gravitational waves. Analogous to the 2 photon polarizations in electromagnetism.

**ADM Formalism (3+1 Split)**
Decomposition of the metric into spatial metric $\gamma_{ij}$, lapse $\alpha$, and shift $\beta^i$ on spacelike hypersurfaces. The Einstein equations split into 4 constraints (Hamiltonian + momentum) and 12 evolution equations (for $\gamma_{ij}$ and extrinsic curvature $K_{ij}$). Foundation of numerical relativity.

**Birkhoff's Theorem**
The unique spherically symmetric, asymptotically flat vacuum solution of the Einstein equations is the Schwarzschild metric. Even a pulsating spherical shell has a Schwarzschild exterior. Corollary: no monopole gravitational radiation.

**Choquet-Bruhat Theorem**
The Einstein equations with appropriate matter have a well-posed Cauchy problem: given initial data $(g_{ij}, K_{ij}, T_{\mu\nu}|_\Sigma)$ satisfying the constraints on a spacelike slice $\Sigma$, there exists a unique maximal globally hyperbolic development. GR is a well-posed initial value problem.

**Post-Newtonian Expansion**
Systematic expansion in $v/c \sim (GM/rc^2)^{1/2}$. At 0PN: Newtonian gravity. At 1PN: first relativistic corrections, gravitomagnetic effects. At 2.5PN: radiation damping (gravitational wave emission, orbital inspiral). Templates for LIGO go to 3.5PN in phase.

**Gravitoelectromagnetism**
In the weak-field limit, the linearized Einstein equations take the form of Maxwell's equations with $\mathbf{g}$ (gravitoelectric) and $\mathbf{H}$ (gravitomagnetic) fields. The gravitomagnetic force is 4 times stronger than the EM analogy would suggest (signature of spin-2 graviton vs. spin-1 photon). Gives Lense-Thirring precession and geodetic precession.

