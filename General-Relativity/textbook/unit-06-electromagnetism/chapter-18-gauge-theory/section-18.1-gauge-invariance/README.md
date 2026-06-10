# Section 18.1: Gauge Invariance and the Aharonov-Bohm Effect

---

## Section Introduction

What is gauge invariance? On the surface: a mathematical redundancy in the description of electromagnetic fields using potentials. The same physical situation ($\mathbf{E}$, $\mathbf{B}$) can be described by infinitely many different potentials $(\phi, \mathbf{A})$, all related by gauge transformations. This seems like a nuisance — we have more variables than we need.

But this view misses something deep. In quantum mechanics, the Aharonov-Bohm effect (1959) shows that the potential $A_\mu$ affects measurable interference fringes even in a region where $\mathbf{E} = \mathbf{B} = 0$. The potential cannot be purely a bookkeeping artifact — it has physical content not contained in the field strengths. The physical content is topological: it is the holonomy of the connection $A_\mu$ around non-contractible loops.

This is the modern understanding: $A_\mu$ is a connection on a U(1) principal bundle over spacetime. The gauge freedom is the action of the structure group (phase rotations at each point). The field strength $F_{\mu\nu}$ is the curvature of this connection. Physical observables are gauge-invariant quantities — but not all gauge-invariant information is captured by the local curvature $F_{\mu\nu}$ alone. The Aharonov-Bohm effect is the obstruction.

The parallel to GR: the Christoffel symbols $\Gamma^\rho_{\mu\nu}$ are a connection on the frame bundle (not a tensor, not gauge-invariant under coordinate changes). The Riemann tensor $R^\rho_{\ \sigma\mu\nu}$ is the curvature of this connection (a tensor, gauge-invariant). The "gauge freedom" of GR is diffeomorphism invariance — coordinate changes act on $\Gamma$ as a gauge transformation acts on $A_\mu$.

---

## 18.1.1 The Scalar and Vector Potentials

**Introducing potentials**: Since $\nabla\cdot\mathbf{B} = 0$, we can write $\mathbf{B} = \nabla\times\mathbf{A}$ for some vector field $\mathbf{A}$ (the **vector potential**). This is locally guaranteed by the Poincaré lemma for the closed 2-form $\mathbf{B}$.

Substituting into Faraday's law: $\nabla\times\mathbf{E} = -\partial_t(\nabla\times\mathbf{A}) = -\nabla\times(\partial_t\mathbf{A})$, so $\nabla\times(\mathbf{E} + \partial_t\mathbf{A}) = 0$. Since this curl vanishes, we can write $\mathbf{E} + \partial_t\mathbf{A} = -\nabla\phi$ for some scalar $\phi$ (the **scalar potential**). Therefore:

$$\mathbf{E} = -\nabla\phi - \partial_t\mathbf{A}, \qquad \mathbf{B} = \nabla\times\mathbf{A}$$

These automatically satisfy the two "source-free" Maxwell equations ($\nabla\cdot\mathbf{B} = 0$ and $\nabla\times\mathbf{E} + \partial_t\mathbf{B} = 0$).

**Gauge transformation**: For any smooth function $\chi(\mathbf{r}, t)$:

$$\mathbf{A} \to \mathbf{A}' = \mathbf{A} + \nabla\chi, \qquad \phi \to \phi' = \phi - \partial_t\chi$$

Verify: $\mathbf{B}' = \nabla\times\mathbf{A}' = \nabla\times\mathbf{A} + \nabla\times\nabla\chi = \mathbf{B}$ (curl of a gradient vanishes). $\mathbf{E}' = -\nabla\phi' - \partial_t\mathbf{A}' = -\nabla\phi + \nabla\partial_t\chi - \partial_t\mathbf{A} - \partial_t\nabla\chi = \mathbf{E}$. ✓

The physical fields $\mathbf{E}$ and $\mathbf{B}$ are gauge-invariant; the potentials $\phi$, $\mathbf{A}$ are not.

---

## 18.1.2 Gauge Fixing

To avoid redundancy, one imposes a **gauge condition** — an additional constraint that partially or completely fixes the gauge freedom.

**Coulomb gauge (radiation gauge)**: $\nabla\cdot\mathbf{A} = 0$. Advantage: the Gauss law becomes $\nabla^2\phi = -\rho/\varepsilon_0$ (Poisson equation), so $\phi$ is determined instantaneously by the charge distribution (no retardation). The vector potential $\mathbf{A}$ carries the radiation degrees of freedom. Disadvantage: not Lorentz covariant.

**Lorenz gauge**: $\partial_\mu A^\mu = 0$ (in covariant notation), i.e., $\nabla\cdot\mathbf{A} + (1/c^2)\partial_t\phi = 0$. Advantage: Lorentz covariant. In this gauge, the remaining two Maxwell equations become $\Box A^\mu = \mu_0 J^\mu$ (wave equation for each component). Disadvantage: doesn't completely fix the gauge — residual gauge freedom $\chi$ with $\Box\chi = 0$ remains.

**Axial gauge**: $A^3 = 0$ (or $A^0 = 0$). Used in certain quantization schemes.

**Temporal gauge**: $A^0 = 0$ (i.e., $\phi = 0$). The analog of the lapse function in GR's ADM formalism.

*Note on "Lorenz" vs. "Lorentz"*: The condition is due to Ludvig Lorenz (Danish, 1829–1891), not Hendrik Lorentz (Dutch, 1853–1928). A common confusion.

---

## 18.1.3 The Aharonov-Bohm Effect

**Setup**: A long solenoid (effectively infinite) carries current and has a magnetic field $\mathbf{B}$ inside but $\mathbf{B} = 0$ outside. An electron beam is split, with one beam passing on each side of the solenoid, then recombined. The interference pattern is observed.

**Classical prediction**: Since $\mathbf{B} = 0$ along both electron paths, there is no classical force. The interference pattern should be the same whether the solenoid is on or off.

**Quantum-mechanical prediction**: The electron wave function acquires a phase from the vector potential:

$$\psi \to \psi \exp\left(i\frac{e}{\hbar}\int_{\rm path} \mathbf{A}\cdot d\boldsymbol{\ell}\right)$$

The phase difference between the two paths is:

$$\Delta\phi = \frac{e}{\hbar}\oint \mathbf{A}\cdot d\boldsymbol{\ell} = \frac{e}{\hbar}\int_S \mathbf{B}\cdot d\mathbf{A} = \frac{e\Phi_B}{\hbar}$$

where $\Phi_B$ is the magnetic flux through the solenoid. The interference fringes shift by $\Delta\phi$ — **even though the electrons never enter the solenoid and never experience the field $\mathbf{B}$**.

*Experimental confirmation*: Chambers (1960) [following Aharonov and Bohm's 1959 prediction] confirmed the effect using electron microscopy. The phase shift $e\Phi_B/\hbar$ modulo $2\pi$ produces observable fringe shifts. This is the cleanest demonstration that the potential $A_\mu$ (or more precisely, its line integral — a gauge-invariant quantity) is physically meaningful.

**The key**: The quantity $\exp(i(e/\hbar)\oint A_\mu\,dx^\mu)$ — the **holonomy** of the connection $A_\mu$ — is gauge-invariant (the gauge transformation $\chi$ contributes a boundary term to the line integral, which vanishes for a closed loop). The local field strength $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$ vanishes outside the solenoid, but the holonomy around the solenoid can be nonzero. **The potential contains topological information not encoded in the local curvature**.

This is precisely the situation in GR: a spacetime can be flat everywhere ($R^\rho_{\ \sigma\mu\nu} = 0$) but topologically non-trivial (e.g., a flat torus), with non-trivial holonomy (the holonomy of the Levi-Civita connection around a non-contractible loop need not be the identity).

---

## 18.1.4 Dirac Quantization Condition

If magnetic monopoles exist (Dirac, 1931): a particle of magnetic charge $g$ has $\mathbf{B} = g\hat{\mathbf{r}}/(4\pi r^2)$ (a Coulomb magnetic field). The vector potential $\mathbf{A}$ cannot be defined globally (the Faraday tensor $F$ has nonzero integral over $S^2$, i.e., $[F] \neq 0 \in H^2(S^2) \cong \mathbb{Z}$). The potential can only be defined on patches, with transition functions.

For an electron (charge $e$) orbiting a magnetic monopole (charge $g$): the single-valuedness of the electron's wave function requires the Aharonov-Bohm phase around any loop enclosing the monopole to be a multiple of $2\pi$:

$$\frac{eg}{\hbar c} = \frac{n}{2}, \qquad n \in \mathbb{Z}$$

This is the **Dirac quantization condition**. It implies: if even one magnetic monopole exists anywhere in the universe, then all electric charges must be multiples of $\hbar c/(2g)$. This would explain the observed quantization of electric charge — one of the deepest unanswered questions in physics.

GR analogy: in GR, a spacetime with a non-trivial $\pi_1$ (fundamental group) can have holonomies. The topology of spacetime constrains what field configurations are allowed — a direct topological analog of the Dirac quantization condition.

---

## References

- Aharonov, Y. and Bohm, D. (1959). "Significance of electromagnetic potentials in the quantum theory." *Physical Review*, 115, 485–491. [The prediction of the Aharonov-Bohm effect: the vector potential affects quantum interference even in field-free regions. One of the most important conceptual papers in quantum mechanics.]
- Chambers, R.G. (1960). "Shift of an electron interference pattern by enclosed magnetic flux." *Physical Review Letters*, 5, 3–5. [The first experimental confirmation of the Aharonov-Bohm effect, using electron microscopy.]
- Dirac, P.A.M. (1931). "Quantised singularities in the electromagnetic field." *Proceedings of the Royal Society A*, 133, 60–72. [Introduces magnetic monopoles and the Dirac quantization condition. If monopoles exist, electric charge must be quantized.]
- Wu, T.T. and Yang, C.N. (1975). "Concept of nonintegrable phase factors and global formulation of gauge fields." *Physical Review D*, 12, 3845–3857. [The modern understanding: gauge fields are connections on fiber bundles; the Aharonov-Bohm effect is holonomy; electromagnetism is a U(1) principal bundle.]
- Nakahara, M. (2003). *Geometry, Topology and Physics*, 2nd ed. Institute of Physics Publishing. [Chapter 10: fiber bundles and gauge theories. The cleanest mathematical treatment of gauge theories as connections on principal bundles.]
