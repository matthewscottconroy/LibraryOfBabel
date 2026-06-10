# Chapter 26: Relativistic Electrodynamics

---

## Chapter Introduction

Electromagnetism was the first relativistic theory — it just didn't know it. Maxwell's equations, written in 1865, are Lorentz-covariant. They do not need to be "corrected" for special relativity because they were already correct. The reason Lorentz and Poincaré discovered the Lorentz group was precisely because they were studying Maxwell's equations and asking what transformations leave them invariant.

The covariant formulation of electrodynamics — expressing Maxwell's equations as tensor equations — is one of the great achievements of mathematical physics. It unifies the electric and magnetic fields into a single antisymmetric tensor $F^{\mu\nu}$. The four Maxwell equations reduce to two tensor equations. The Lorentz force becomes a single 4-vector equation. The transformation of electric and magnetic fields under boosts — a set of six equations — becomes a single formula for how $F^{\mu\nu}$ transforms.

This chapter is also, conceptually, the bridge to GR. The electromagnetic field strength $F^{\mu\nu}$ plays the same role in EM as the Riemann tensor $R^{\mu\nu\rho\sigma}$ in gravity — it is the gauge-invariant field strength of a connection. The gauge potential $A^\mu$ plays the role of the metric (or Christoffel symbols). The covariant formulation of EM is a template for understanding GR.

---

## The Electromagnetic Field Tensor

In non-relativistic notation, the electric field $\mathbf{E}$ and magnetic field $\mathbf{B}$ are separate 3-vectors. In the covariant formulation, they are components of a single antisymmetric $(0,2)$-tensor:

$$F_{\mu\nu} = \begin{pmatrix}0 & -E_x/c & -E_y/c & -E_z/c \\ E_x/c & 0 & -B_z & B_y \\ E_y/c & B_z & 0 & -B_x \\ E_z/c & -B_y & B_x & 0\end{pmatrix}$$

with indices in the order $(ct, x, y, z) = (0, 1, 2, 3)$. Explicitly:
$$F_{0i} = -E_i/c, \quad F_{ij} = -\varepsilon_{ijk}B^k$$

The antisymmetry $F_{\mu\nu} = -F_{\nu\mu}$ means $F$ has $4\times 3/2 = 6$ independent components — exactly $\mathbf{E}$ (3) + $\mathbf{B}$ (3).

The contravariant form (raise both indices with $\eta^{\mu\rho}\eta^{\nu\sigma}$):
$$F^{\mu\nu} = \eta^{\mu\rho}\eta^{\nu\sigma}F_{\rho\sigma}$$

gives $F^{0i} = +E^i/c$, $F^{ij} = -\varepsilon^{ijk}B_k$.

---

## Maxwell's Equations in Covariant Form

All four Maxwell equations reduce to two covariant equations.

**The inhomogeneous Maxwell equations** (Gauss's law and Ampère's law):
$$\partial_\nu F^{\mu\nu} = \frac{4\pi}{c}J^\mu$$

or in SI: $\partial_\nu F^{\mu\nu} = \mu_0 J^\mu$.

The 4-current is $J^\mu = (c\rho, \mathbf{J})$ where $\rho$ is the charge density and $\mathbf{J}$ is the current density.

- $\mu = 0$: $\partial_i F^{0i} = \mu_0 c\rho$ gives $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$ (Gauss)
- $\mu = i$: $\partial_0 F^{i0} + \partial_j F^{ij} = \mu_0 J^i$ gives $\nabla\times\mathbf{B} - \partial_t\mathbf{E}/c^2 = \mu_0\mathbf{J}$ (Ampère)

**The homogeneous Maxwell equations** (Gauss's law for magnetism and Faraday's law):
$$\partial_{[\lambda}F_{\mu\nu]} = 0 \quad\Leftrightarrow\quad \partial_\lambda F_{\mu\nu} + \partial_\mu F_{\nu\lambda} + \partial_\nu F_{\lambda\mu} = 0$$

- $(\lambda,\mu,\nu) = (0,1,2)$: gives $\partial_t B_z + (\nabla\times\mathbf{E})_z = 0$ (Faraday)
- $(\lambda,\mu,\nu) = (1,2,3)$: gives $\nabla\cdot\mathbf{B} = 0$

Alternatively, writing $F = dA$ where $A = A_\mu dx^\mu$ is the gauge potential:
$$F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$$

then $\partial_{[\lambda}F_{\mu\nu]} = 0$ is automatic ($d^2 = 0$). The homogeneous equations are a Bianchi identity.

**Gauge freedom**: $A_\mu \to A_\mu + \partial_\mu\lambda$ leaves $F_{\mu\nu}$ unchanged (since $\partial_\mu\partial_\nu\lambda = \partial_\nu\partial_\mu\lambda$). In Lorenz gauge: $\partial_\mu A^\mu = 0$, the inhomogeneous equations become:
$$\Box A^\mu = -\mu_0 J^\mu$$

where $\Box = \partial_\mu\partial^\mu = -\partial_t^2/c^2 + \nabla^2$ is the d'Alembertian. This is the wave equation for the electromagnetic potential — with sources.

---

## The Electromagnetic Dual

The **dual field strength tensor** $\tilde{F}^{\mu\nu} = \frac{1}{2}\varepsilon^{\mu\nu\rho\sigma}F_{\rho\sigma}$ exchanges $\mathbf{E}$ and $\mathbf{B}$:
$$\tilde{F}^{0i} = -B^i, \quad \tilde{F}^{ij} = \varepsilon^{ijk}E_k/c$$

In terms of the dual, the homogeneous Maxwell equations are:
$$\partial_\nu\tilde{F}^{\mu\nu} = 0$$

This has the same form as the inhomogeneous equations with $J^\mu = 0$. If magnetic monopoles existed with magnetic charge density $\rho_m$ and current $\mathbf{J}_m$, this would become $\partial_\nu\tilde{F}^{\mu\nu} = \mu_0 J_m^\mu$ — a beautifully symmetric pair of equations.

**Two Lorentz invariants** can be formed from $F_{\mu\nu}$:
$$F_{\mu\nu}F^{\mu\nu} = 2(B^2 - E^2/c^2), \quad F_{\mu\nu}\tilde{F}^{\mu\nu} = -4\mathbf{E}\cdot\mathbf{B}/c$$

These are genuine scalars — the same in all frames. If $\mathbf{E}\cdot\mathbf{B} = 0$ in one frame, it is zero in all frames.

---

## Transformation of $\mathbf{E}$ and $\mathbf{B}$

Under a boost with velocity $v$ along $x$, the field tensor transforms as $F'^{\mu\nu} = \Lambda^\mu_{\ \rho}\Lambda^\nu_{\ \sigma}F^{\rho\sigma}$. Working out the components:

$$E'_x = E_x, \quad E'_y = \gamma(E_y - vB_z), \quad E'_z = \gamma(E_z + vB_y)$$
$$B'_x = B_x, \quad B'_y = \gamma(B_y + vE_z/c^2), \quad B'_z = \gamma(B_z - vE_y/c^2)$$

**Electric and magnetic fields are frame-dependent**: What one observer sees as a pure magnetic field, another (moving) observer sees as a combination of electric and magnetic fields.

**Example**: A wire carrying current $I$ creates a magnetic field $B = \mu_0 I/(2\pi r)$ in the lab frame. In the frame of the moving charges (the electrons), the wire is Lorentz-contracted differently for positive and negative charges (one is at rest, one is moving). This creates a net charge density and thus an *electric* field. The magnetic force on a moving test charge (in the lab frame) is equivalent to the electric force on a stationary test charge (in the electron frame). Magnetism is literally a relativistic effect of electricity.

This argument — that magnetism is a consequence of special relativity applied to electrostatics — is one of the most striking demonstrations of the physical content of SR.

---

## The Covariant Lorentz Force

The equation of motion for a charged particle:
$$\frac{dp^\mu}{d\tau} = qF^{\mu}_{\ \nu}u^\nu$$

where $q$ is the charge, $u^\nu = \gamma(c, \mathbf{v})$ is the 4-velocity, and $F^{\mu}_{\ \nu} = \eta^{\mu\rho}F_{\rho\nu}$.

The spatial components give:
$$\frac{d\mathbf{p}}{dt} = q(\mathbf{E} + \mathbf{v}\times\mathbf{B})$$

This is the Lorentz force law. The time component:
$$\frac{dE}{dt} = q\mathbf{v}\cdot\mathbf{E}$$

(Magnetic fields do no work — consistent with the cross product structure.)

The covariant form $f^\mu = qF^{\mu}_{\ \nu}u^\nu$ is manifestly a 4-vector equation: it holds in all frames. The non-covariant form is just the spatial projection in a particular frame.

---

## Energy-Momentum of the Electromagnetic Field

The electromagnetic field carries energy and momentum. The **energy-momentum tensor** of the EM field:
$$T^{\mu\nu}_{\rm EM} = \frac{1}{\mu_0}\left(F^{\mu\rho}F^\nu_{\ \rho} - \frac{1}{4}\eta^{\mu\nu}F_{\rho\sigma}F^{\rho\sigma}\right)$$

Components:
- $T^{00} = \frac{1}{2}\left(\varepsilon_0 E^2 + B^2/\mu_0\right) = u_{\rm EM}$ (energy density)
- $T^{0i} = (\mathbf{E}\times\mathbf{B}/\mu_0)^i/c = S^i/c$ where $\mathbf{S}$ is the Poynting vector (energy flux)
- $T^{ij}$: Maxwell stress tensor (momentum flux)

The conservation law $\partial_\nu T^{\mu\nu} = -F^{\mu}_{\ \nu}J^\nu$ (the right-hand side is the force density on charges). When combined with the particle stress-energy tensor, total energy-momentum is conserved: $\partial_\nu(T^{\mu\nu}_{\rm EM} + T^{\mu\nu}_{\rm matter}) = 0$.

---

## Electromagnetic Waves

In vacuum ($J^\mu = 0$) with Lorenz gauge ($\partial_\mu A^\mu = 0$):
$$\Box A^\mu = 0$$

Plane wave solutions: $A^\mu = \epsilon^\mu e^{ik_\nu x^\nu}$ where $k^\mu = (\omega/c, \mathbf{k})$ is the 4-wavevector. The wave equation requires $k^\mu k_\mu = 0$ — photons are massless, and light waves travel at $c$.

**Polarization**: The residual gauge freedom in Lorenz gauge ($A^\mu \to A^\mu + \partial^\mu\chi$ with $\Box\chi = 0$) allows us to set $A^0 = 0$ and $\mathbf{k}\cdot\mathbf{A} = 0$ (radiation gauge). Only two polarization states remain — the two transverse modes. This is why photons have two polarizations.

**Radiation from accelerating charges**: The Liénard-Wiechert potentials give the field of a moving charge:
$$\phi = \frac{q}{4\pi\varepsilon_0}\frac{1}{R(1-\hat{R}\cdot\boldsymbol{\beta})}, \quad \mathbf{A} = \frac{\mathbf{v}}{c^2}\phi$$

evaluated at the retarded time. The radiation fields (falling as $1/r$) are present only when $d\boldsymbol{\beta}/dt \neq 0$ — accelerating charges radiate. The radiated power:
$$P = \frac{q^2}{6\pi\varepsilon_0 c}\gamma^6\left(\dot{v}^2 - \left|\frac{\mathbf{v}\times\dot{\mathbf{v}}}{c}\right|^2\right)$$

For non-relativistic motion: $P = q^2\dot{v}^2/(6\pi\varepsilon_0 c^3)$ (Larmor formula).

---

## The Connection to GR

The parallel between electromagnetism and gravity is deep:

| Electromagnetism | Gravity (GR) |
|---|---|
| Gauge potential $A_\mu$ | Metric perturbation $h_{\mu\nu}$ |
| Field strength $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$ | Riemann tensor $R^\rho_{\ \sigma\mu\nu}[\Gamma]$ |
| Lorenz gauge $\partial^\mu A_\mu = 0$ | Lorenz gauge $\partial^\mu\bar{h}_{\mu\nu} = 0$ |
| Wave equation $\Box A^\mu = -\mu_0 J^\mu$ | $\Box\bar{h}_{\mu\nu} = -16\pi G T_{\mu\nu}/c^4$ |
| 2 transverse polarizations | 2 transverse polarizations (TT gauge) |
| Photon (spin-1, massless) | Graviton (spin-2, massless) |
| U(1) gauge invariance | Diffeomorphism invariance |
| Source: 4-current $J^\mu$ | Source: stress-energy tensor $T_{\mu\nu}$ |

The covariant formulation of EM is not just a technical convenience — it reveals the structure that GR generalizes.

---

## Important Concepts

- **Field strength tensor** $F_{\mu\nu}$: Antisymmetric $(0,2)$-tensor encoding $\mathbf{E}$ and $\mathbf{B}$
- **4-potential** $A^\mu$: $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$; gauge freedom $A\to A + \partial\lambda$
- **Maxwell equations**: $\partial_\nu F^{\mu\nu} = \mu_0 J^\mu$ (inhomogeneous) and $\partial_{[\lambda}F_{\mu\nu]} = 0$ (homogeneous = Bianchi identity)
- **Lorentz force (covariant)**: $dp^\mu/d\tau = qF^\mu_{\ \nu}u^\nu$
- **EM invariants**: $F_{\mu\nu}F^{\mu\nu} = 2(B^2 - E^2/c^2)$ and $F_{\mu\nu}\tilde{F}^{\mu\nu} = -4\mathbf{E}\cdot\mathbf{B}/c$
- **Field transformation**: $\mathbf{E}$ and $\mathbf{B}$ mix under boosts; magnetism is relativistic electricity
- **EM stress-energy tensor** $T^{\mu\nu}_{\rm EM}$: Encodes EM energy density, Poynting vector, Maxwell stress
- **Radiation gauge**: Two physical polarizations; photon is spin-1
- **Liénard-Wiechert**: Fields of moving charges; radiation from acceleration; Larmor formula

---

## Important Figures

**James Clerk Maxwell** (1831–1879): Unified electricity, magnetism, and optics; the equations are Lorentz-covariant 36 years before Lorentz.

**Hendrik Lorentz** (1853–1928): Worked out the electron theory and the transformations that leave Maxwell's equations invariant; Nobel Prize 1902.

**Emil Wiechert** (1861–1928) and **Alfred-Marie Liénard** (1869–1958): Independently derived the potentials for moving charges (1898–1900).

**Hermann Minkowski** (1864–1909): Formulated the covariant tensor form of Maxwell's equations in 1907.

**Paul Dirac** (1902–1984): Extended to quantum electrodynamics; developed the Dirac equation combining SR and quantum mechanics.

---

## Further Reading

**Primary Sources**
- Maxwell, J.C. (1865). "A Dynamical Theory of the Electromagnetic Field." *Phil. Trans. Royal Society*, 155, 459.
- Minkowski, H. (1908). "Die Grundgleichungen für die elektromagnetischen Vorgänge in bewegten Körpern." *Nachrichten der Ges. der Wiss. zu Göttingen*, 53–111.

**Textbooks**
- Jackson, J.D. (1999). *Classical Electrodynamics* (3rd ed.). Wiley. — Chapters 11–12; the definitive reference for covariant EM.
- Griffiths, D.J. (2017). *Introduction to Electrodynamics* (4th ed.). Cambridge. — Chapter 12 on electrodynamics and relativity.
- Landau, L.D. & Lifshitz, E.M. (1975). *Classical Theory of Fields* (4th ed.). Pergamon. — Elegant, concise, covariant throughout.

---

## Exercises

**26.1.** *Maxwell's equations in component form.*

Starting from $\partial_\nu F^{\mu\nu} = \mu_0 J^\mu$:

(a) Show that $\mu = 0$ gives Gauss's law $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$.

(b) Show that $\mu = 1$ gives the $x$-component of Ampère's law $(\nabla\times\mathbf{B})_x = \mu_0 J_x + \mu_0\varepsilon_0\partial_t E_x$.

(c) Verify charge conservation from the equations: show $\partial_\mu J^\mu = 0$ follows from Maxwell's equations and the identity $\partial_\mu\partial_\nu F^{\mu\nu} = 0$.

---

**26.2.** *Magnetism as relativistic electricity.*

A long wire carries current $I$ along the $x$-axis. The wire has linear charge density $\lambda_+$ (positive ions, at rest in lab frame) and $\lambda_- = -\lambda_+$ (electrons, moving at drift velocity $v_d$ in the $-x$ direction). In the lab frame, the wire is neutral.

(a) A test charge $q$ moves parallel to the wire at velocity $v$ (same direction as electron drift). Compute the magnetic force on $q$ in the lab frame.

(b) Transform to the frame of the test charge. In this frame, the wire is moving at $-v$. Use the Lorentz transformation of $E$ and $B$ to find the fields at the test charge location.

(c) Show that the force computed in part (b) agrees with part (a) when transformed back, demonstrating that the magnetic force is a relativistic consequence of the electric force.

---

**26.3.** *Electromagnetic invariants.*

(a) Compute $F_{\mu\nu}F^{\mu\nu}$ and $F_{\mu\nu}\tilde{F}^{\mu\nu}$ in terms of $\mathbf{E}$ and $\mathbf{B}$.

(b) Show: if $\mathbf{E}\perp\mathbf{B}$ and $|\mathbf{E}| = c|\mathbf{B}|$ in some frame, then these conditions hold in all frames.

(c) A plane electromagnetic wave has $\mathbf{E}\perp\mathbf{B}$ and $|\mathbf{E}| = c|\mathbf{B}|$. By the invariant argument, what frame-independent statement can you make about the wave?

(d) Is there a frame in which a pure magnetic field (no electric field) looks like a pure electric field? Under what condition on the invariants?

---

**Thought Experiment T26.1.** *Is gravity a gauge theory?*

Electromagnetism is a U(1) gauge theory: the physics is invariant under $A_\mu\to A_\mu + \partial_\mu\lambda$. The field strength $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$ is gauge-invariant. The physical observable is $F_{\mu\nu}$, not $A_\mu$.

GR has a similar structure: the physics is invariant under diffeomorphisms $x^\mu\to x'^\mu(x)$ (coordinate changes). The metric $g_{\mu\nu}$ transforms non-trivially; the Riemann tensor is the "gauge-invariant field strength."

But there is a key difference: in EM, the gauge group U(1) acts on an internal space (phase of the wave function). In GR, the "gauge group" (diffeomorphisms) acts on spacetime itself. Is GR a gauge theory in the same sense as EM? 

The question is not purely semantic: attempts to quantize gravity using gauge theory methods (Yang-Mills quantization) lead to non-renormalizability. Why? What is fundamentally different about the gravitational "gauge group"?
