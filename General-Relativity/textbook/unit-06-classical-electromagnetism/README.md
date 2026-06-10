# Unit VI: Classical Electromagnetism

---

## Unit Introduction

Of all the classical field theories, electromagnetism is the one that most clearly announced the future. Maxwell's equations, written down in 1865, already contained special relativity — hidden in plain sight. The speed of light emerged as a derived quantity, $c = 1/\sqrt{\mu_0\varepsilon_0}$, fixed by the properties of the electromagnetic vacuum and independent of any reference frame. When Einstein confronted this fact as a young man, he found that the only consistent resolution was to abandon absolute simultaneity. The field theory of charges and currents turned out to be, all along, a relativistic theory.

This unit develops classical electromagnetism from first principles to its relativistic formulation, with deliberate attention to the mathematical structures that will reappear in general relativity. The path follows history but is not slavishly historical: we move efficiently from Coulomb's law and the Biot-Savart law through Maxwell's synthesis, arriving at radiation theory — the physics of accelerating charges, light, and waves. Throughout, we emphasize the geometric content: electric and magnetic fields as components of a single antisymmetric tensor, gauge freedom as a redundancy of description, and the action principle as the organizing framework.

The deep structural lesson of electromagnetism is the reality and autonomy of the field. A charge moved here produces a disturbance that propagates outward at speed $c$, carrying energy and momentum independently of the original source. The field is not a convenient bookkeeping device for forces between charges — it is a physical entity in its own right, capable of existing without any charges at all (as free electromagnetic waves). This shift in perspective, from particles to fields, is what makes Maxwell's theory the prototype for all of modern physics: quantum field theory, the Standard Model, and general relativity itself all inherit the field concept from electromagnetism.

The unit also introduces the concept of gauge invariance in its simplest and most concrete form. The choice of scalar potential $\phi$ and vector potential $\mathbf{A}$ is not unique; different gauges describe the same physics. This redundancy is not a bug but a feature: it reflects a deep symmetry of the theory, $U(1)$ gauge invariance, which is the simplest case of the local gauge symmetries that govern all fundamental interactions. When you learn to change gauge freely in electrodynamics — choosing Coulomb gauge for one problem, Lorenz gauge for another — you are practicing the same conceptual move that physicists make in Yang-Mills theories and in GR (diffeomorphism invariance is the gauge symmetry of gravity).

The final chapter on radiation connects to the broader theme of the textbook. Gravitational waves — one of the central phenomena of general relativity — are, in many ways, analogous to electromagnetic waves. Both arise from accelerating sources, both carry energy at speed $c$, and both are described by multipole expansions (with the monopole and dipole being absent for gravitational radiation due to conservation laws). The Larmor formula for electric dipole radiation has a precise gravitational analogue in the quadrupole formula. Learning radiation physics in the EM context builds exactly the intuition needed for gravitational wave emission in GR.

---

## Unit Contents

### [Chapter 18: Electrostatics](chapter-18-electrostatics/README.md)

The static electric field: Coulomb's law, Gauss's law in integral and differential form, the scalar potential, Poisson's equation, boundary conditions, and multipole expansions. We develop the mathematics of solving Laplace's equation in spherical coordinates (Legendre polynomials, spherical harmonics) and establish the uniqueness theorems that underpin all electrostatic calculations.

### [Chapter 19: Magnetostatics](chapter-19-magnetostatics/README.md)

The static magnetic field: the Lorentz force, the Biot-Savart law, Ampère's law, the vector potential, magnetic multipoles, and the origin of magnetic forces in relativity. We show how magnetism is, in a precise sense, a relativistic correction to electrostatics — a theme that deepens when we later write the unified electromagnetic field tensor.

### [Chapter 20: Maxwell's Equations](chapter-20-maxwell-equations/README.md)

Maxwell's completion of the theory: the displacement current, the full set of four Maxwell equations, their wave equation content, the speed of light, and the energy-momentum of the electromagnetic field (Poynting vector, Maxwell stress tensor). We also develop gauge invariance and the two canonical gauges: Coulomb gauge and Lorenz gauge.

### [Chapter 21: Radiation](chapter-21-radiation/README.md)

Radiation from accelerating charges: retarded potentials, the Liénard-Wiechert potentials, the Larmor formula, electric dipole radiation, magnetic dipole and electric quadrupole radiation. We discuss Thomson scattering, synchrotron radiation, and the connection to gravitational wave emission — previewing the quadrupole formula that governs gravitational radiation from binary systems.

---

## Mathematical Prerequisites

This unit assumes facility with:
- Vector calculus: gradient, divergence, curl, Stokes's theorem, divergence theorem
- Ordinary and partial differential equations (Units III–IV)
- Complex numbers and Fourier analysis
- Basic linear algebra

The unit develops spherical harmonics, Green's functions for the Laplacian and wave operator, and the multipole expansion — tools that recur throughout the rest of the textbook.

---

## Connections to General Relativity

Classical electromagnetism is not merely preparatory material for GR — it is a direct conceptual ancestor:

| Electromagnetism | General Relativity |
|---|---|
| $U(1)$ gauge invariance: $A_\mu\to A_\mu + \partial_\mu\lambda$ | Diffeomorphism invariance: $g_{\mu\nu}\to\phi^*g_{\mu\nu}$ |
| Field tensor $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$ | Riemann tensor built from Christoffel symbols |
| Maxwell equations: $\partial_\nu F^{\mu\nu} = \mu_0 J^\mu$ | Einstein equations: $G_{\mu\nu} = (8\pi G/c^4)T_{\mu\nu}$ |
| Electromagnetic wave equation | Gravitational wave equation (linearized GR) |
| Larmor formula: $P \propto q^2 a^2$ | Quadrupole formula: $P \propto G\dddot{Q}^2/c^5$ |
| Charge conservation: $\partial_\mu J^\mu = 0$ | Energy-momentum conservation: $\nabla_\mu T^{\mu\nu} = 0$ |

The Reissner-Nordström solution (Unit X) is the exact GR solution for a charged, spherically symmetric mass — the Einstein-Maxwell equations, combining the field equations of both theories. The electromagnetic field tensor $F_{\mu\nu}$ will appear throughout the GR portion of the textbook as the canonical example of a tensor field on curved spacetime.

---

## Important Figures

- **Charles-Augustin de Coulomb** (1736–1806): Torsion balance measurements; inverse-square law of electrostatics
- **Hans Christian Ørsted** (1777–1851): Discovery of the magnetic effect of electric current (1820)
- **André-Marie Ampère** (1775–1836): Mathematical law of magnetostatics; concept of the solenoid
- **Michael Faraday** (1791–1867): Electromagnetic induction; field lines as physical reality; Faraday cage
- **James Clerk Maxwell** (1831–1879): Displacement current; four unified equations; prediction of electromagnetic waves and their identification with light
- **Heinrich Hertz** (1857–1894): Experimental demonstration of electromagnetic waves (1887–1888)
- **Hendrik Lorentz** (1853–1928): Electron theory; Lorentz force; Lorenz-Lorentz relation; near-miss on special relativity

---

## Unit Theme: The Reality of Fields

The conceptual arc of this unit is the progressive establishment of the electromagnetic field as a real, physical entity. We begin with action-at-a-distance (Coulomb's law): charges acting on each other across space with no mechanism described. We end with radiation: energy and momentum propagating through empty space at the speed of light, carrying information about the past motions of charges that may no longer exist.

Faraday intuited this reality of fields before the mathematics existed to express it. Maxwell gave it precise form. Hertz demonstrated it experimentally. And Einstein, in 1905, showed that the field concept requires a new geometry of space and time.

When you compute the Poynting vector for a propagating electromagnetic wave and find that energy flows through a region of space containing no charges — that the field carries energy — you have understood why fields are real. This lesson carries directly into general relativity, where the gravitational field (the metric) is the central dynamical variable, and where gravitational waves — ripples in the geometry of spacetime — carry energy across billions of light-years to be detected by LIGO.
