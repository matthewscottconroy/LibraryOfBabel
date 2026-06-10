# Chapter 55: Quantum Field Theory in Curved Spacetime

---

## Chapter Introduction

Quantum field theory (QFT) and general relativity are the two pillars of modern physics — and they barely speak to each other. A full quantum theory of gravity remains one of the great unsolved problems of physics. But there is an intermediate regime: we can study quantum fields propagating on a classical curved spacetime background, ignoring the backreaction of the quantum fields on the metric. This is **quantum field theory in curved spacetime (QFTCS)**, and it yields some of the deepest and most surprising results in theoretical physics.

The central prediction of QFTCS — Hawking radiation — is a genuine discovery: black holes are not black. They emit thermal radiation at a temperature set by their surface gravity. This result, derived independently by Hawking (1974) and by others using different methods, profoundly changes our picture of black holes and introduces deep puzzles that remain unresolved.

QFTCS also clarifies what it means to define "particles" — a concept that turns out to be observer-dependent in curved spacetime. The vacuum state of one observer is a thermal state for another (the Unruh effect). And the boundary between classical and quantum, background and field, becomes philosophically loaded in a way it never is in flat spacetime.

This chapter develops the formalism of QFTCS, focusing on the free scalar field as the simplest example. The key concepts — mode expansion, Bogoliubov transformations, particle creation, renormalization of the stress tensor — appear already at this level and carry over to more realistic fields.

---

## Classical Scalar Field in Curved Spacetime

The **action** for a free real scalar field $\phi$ with mass $m$ and coupling $\xi$ to curvature:
$$S = -\frac{1}{2}\int\left(g^{\mu\nu}\nabla_\mu\phi\nabla_\nu\phi + m^2\phi^2 + \xi R\phi^2\right)\sqrt{-g}\,d^4x$$

The equation of motion (Klein-Gordon equation in curved spacetime):
$$(\Box - m^2 - \xi R)\phi = 0$$

where $\Box = g^{\mu\nu}\nabla_\mu\nabla_\nu$ is the curved-spacetime d'Alembertian.

**Coupling choices**:
- $\xi = 0$: minimal coupling — no direct coupling to curvature
- $\xi = 1/6$ (in 4D): conformal coupling — the equation is conformally invariant for $m = 0$
- The trace anomaly (quantum effect) distinguishes these cases

**Inner product**: On a Cauchy surface $\Sigma$ with unit normal $n^\mu$:
$$(\phi_1, \phi_2) = -i\int_\Sigma\phi_1\overset{\leftrightarrow}{\nabla}_\mu\phi_2^*\,n^\mu\sqrt{h}\,d^3x$$

This is the conserved Klein-Gordon inner product: $\partial_t(\phi_1,\phi_2) = 0$ when both satisfy the equation of motion.

---

## Mode Expansions and Quantization

**In flat spacetime**: The field is expanded in plane waves:
$$\hat\phi = \int\frac{d^3k}{(2\pi)^3}\frac{1}{\sqrt{2\omega_k}}\left(\hat{a}_{\mathbf{k}}e^{ik\cdot x} + \hat{a}^\dagger_{\mathbf{k}}e^{-ik\cdot x}\right)$$

with $\omega_k = \sqrt{k^2 + m^2}$. The operators satisfy $[\hat{a}_{\mathbf{k}}, \hat{a}^\dagger_{\mathbf{k}'}] = (2\pi)^3\delta^3(\mathbf{k}-\mathbf{k}')$.

**In curved spacetime**: Choose a complete set of normalized mode functions $\{u_i, u_i^*\}$ satisfying $(\Box - m^2 - \xi R)u_i = 0$ and $(u_i, u_j) = \delta_{ij}$, $(u_i^*, u_j^*) = -\delta_{ij}$, $(u_i, u_j^*) = 0$. Expand:
$$\hat\phi = \sum_i\left(\hat{a}_i u_i + \hat{a}^\dagger_i u_i^*\right)$$

The vacuum state $|0\rangle$ is defined by $\hat{a}_i|0\rangle = 0$ for all $i$.

**The problem**: In curved spacetime, there is no preferred choice of mode functions. Different choices give different decompositions and different vacuum states. If observers in different regions choose different mode functions, they will disagree on what constitutes "no particles."

---

## Bogoliubov Transformations

Suppose two observers choose different complete sets: $\{u_i\}$ (observer A's modes) and $\{v_j\}$ (observer B's modes). Since both are complete, one can expand:
$$v_j = \sum_i(\alpha_{ji}u_i + \beta_{ji}u_i^*)$$

The $\alpha_{ji}$ and $\beta_{ji}$ are **Bogoliubov coefficients**. The unitarity condition requires:
$$\sum_k(\alpha_{ik}\alpha^*_{jk} - \beta_{ik}\beta^*_{jk}) = \delta_{ij}, \quad \sum_k(\alpha_{ik}\beta_{jk} - \beta_{ik}\alpha_{jk}) = 0$$

The creation/annihilation operators of the two observers are related by:
$$\hat{b}_j = \sum_i(\alpha_{ji}^*\hat{a}_i - \beta_{ji}^*\hat{a}^\dagger_i)$$

**Key result**: If $\beta_{ji}\neq 0$, then A's vacuum $|0_A\rangle$ (with $\hat{a}_i|0_A\rangle = 0$) is *not* B's vacuum. In A's vacuum, B observes particles:
$$\langle 0_A|\hat{N}_j^{(B)}|0_A\rangle = \langle 0_A|\hat{b}^\dagger_j\hat{b}_j|0_A\rangle = \sum_i|\beta_{ji}|^2$$

This is the mechanism for particle creation in curved spacetime: if $\beta \neq 0$, the curved background creates particles from the vacuum.

---

## Particle Creation in an Expanding Universe

The clearest example of particle creation from expansion: a scalar field in a spatially flat FLRW universe:
$$ds^2 = -c^2dt^2 + a(t)^2d\mathbf{x}^2 = a(\eta)^2(-c^2d\eta^2 + d\mathbf{x}^2)$$

(using conformal time $\eta$). For a conformally coupled massless field ($\xi = 1/6$, $m = 0$): $\Box\phi = 0$ becomes the flat-space equation $\partial_\eta^2\varphi - \nabla^2\varphi = 0$ where $\varphi = a\phi$. No particle creation!

For a massive field or minimal coupling: the mode equation becomes:
$$\varphi_k'' + \omega_k(\eta)^2\varphi_k = 0, \quad \omega_k^2 = k^2 + a^2(m^2 + \xi_{eff}R)$$

where $\varphi_k'' \equiv d^2\varphi_k/d\eta^2$ and $\xi_{\rm eff}R$ includes curvature terms. The time-varying frequency causes mixing of positive and negative frequencies — Bogoliubov coefficients $\beta\neq 0$ — and hence particle creation.

**During inflation**: Quantum fluctuations in $\phi$ are stretched to super-Hubble scales by inflation. These fluctuations later re-enter the horizon and seed cosmic structure — the origin of the CMB power spectrum is quantum particle creation in de Sitter space.

The **power spectrum** of quantum fluctuations created during inflation:
$$\mathcal{P}_\phi(k) = \left(\frac{H}{2\pi}\right)^2$$

(for a massless minimally coupled field in de Sitter). This was computed by Parker (1969) and applied to inflation by Starobinsky, Guth-Pi, Bardeen-Steinhardt-Turner, and Hawking in 1982–1983.

---

## The Stress-Energy Tensor and Renormalization

The classical stress-energy tensor of the scalar field $T_{\mu\nu}[\phi]$ involves products of fields at the same spacetime point — which, when quantized, become operator products $\hat\phi(x)^2$, $\nabla_\mu\hat\phi\,\nabla_\nu\hat\phi$, etc. These are divergent — the field fluctuates at all scales.

**Renormalization**: In flat spacetime, the vacuum energy is subtracted: $\langle T_{\mu\nu}\rangle_{\rm ren} = \langle T_{\mu\nu}\rangle - \langle 0|T_{\mu\nu}|0\rangle_{\rm flat}$.

In curved spacetime, the subtraction is more subtle — one subtracts the **Hadamard parametrix**: the short-distance singular behavior of the two-point function in a general curved spacetime. The result is finite but contains curvature-dependent ambiguities.

**The trace anomaly** (conformal anomaly): For a conformally coupled massless field, classical theory gives $T^\mu_{\ \mu} = 0$. But quantum mechanically:
$$\langle T^\mu_{\ \mu}\rangle_{\rm ren} = \frac{1}{2880\pi^2}\left(aR_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma} - bR_{\mu\nu}R^{\mu\nu} + cR^2 + d\Box R\right)$$

with specific numerical coefficients depending on the field type. This **Weyl anomaly** is not zero — even a conformally invariant classical action has a conformally non-invariant quantum stress tensor. The Weyl anomaly plays a crucial role in:
- Black hole entropy (Wald's Noether charge formula)
- AdS/CFT: the CFT stress tensor trace equals the holographic Weyl anomaly
- Cosmology: trace anomaly may drive inflation (Starobinsky model)

**Semiclassical Einstein equations**:
$$G_{\mu\nu} + \Lambda g_{\mu\nu} = \frac{8\pi G}{c^4}\langle T_{\mu\nu}\rangle_{\rm ren}$$

This treats the metric classically and the matter quantum mechanically. It breaks down at Planck scale but is valid for sub-Planckian field energies.

---

## The Unruh-DeWitt Detector

How do we operationally define "particles" in curved spacetime? The **Unruh-DeWitt detector** model: a localized two-level system with energy gap $E$ coupled to the field along its worldline. The transition rate from the ground state to the excited state is:
$$\Gamma_{0\to E} = g^2\int_{-\infty}^\infty d\tau\,e^{-iE\tau/\hbar}G^+(x(\tau), x(0))$$

where $G^+(x,x') = \langle 0|\hat\phi(x)\hat\phi(x')|0\rangle$ is the Wightman function evaluated along the worldline.

For an **inertial detector** in Minkowski vacuum: $G^+$ has no thermal character; $\Gamma_{0\to E} = 0$ for $E > 0$ — the detector registers no particles.

For a **uniformly accelerating detector** (acceleration $a$) in Minkowski vacuum: the Wightman function becomes thermal:
$$G^+(x(\tau), x(0)) \propto \frac{1}{e^{2\pi E/(\hbar a/c)} - 1}$$

The detector sees a thermal bath at the **Unruh temperature**:
$$T_U = \frac{\hbar a}{2\pi ck_B}$$

For a detector in the vacuum state of a black hole spacetime with surface gravity $\kappa$: the same calculation gives thermal radiation at the Hawking temperature. The Unruh effect is the local (flat spacetime) version of Hawking radiation.

---

## Important Concepts

- **QFTCS**: Quantum fields on a fixed classical curved background; semiclassical approximation
- **Mode expansion**: Field in terms of positive/negative frequency solutions; vacuum defined by $\hat{a}_i|0\rangle = 0$
- **Observer-dependence of particles**: No unique vacuum in curved spacetime; different observers see different particle numbers
- **Bogoliubov transformation**: Relates mode functions of different observers; $\beta\neq 0$ implies particle creation
- **Particle creation**: $\langle 0_A|\hat{N}^{(B)}|0_A\rangle = \sum_i|\beta_{ji}|^2$ — vacuum of A contains particles for B
- **Inflationary perturbations**: Quantum fluctuations stretched to super-Hubble scales; Bogoliubov mechanism; origin of CMB spectrum
- **Hadamard renormalization**: Subtraction of short-distance singular behavior to define finite $\langle T_{\mu\nu}\rangle$
- **Trace anomaly**: Quantum breaking of classical conformal symmetry; $\langle T^\mu_{\ \mu}\rangle_{\rm ren}\neq 0$ even for conformally invariant fields
- **Unruh-DeWitt detector**: Operational definition of particle content via detector response rates
- **Semiclassical Einstein equations**: $G_{\mu\nu} = 8\pi G\langle T_{\mu\nu}\rangle_{\rm ren}/c^4$; valid below Planck scale

---

## Important Figures

**Leonard Parker** (1938–): Pioneered particle creation by expanding universes (1969); the calculation underlying inflationary perturbation theory.

**Robert Wald** (1947–): Systematic axiomatic treatment of QFTCS; proved that Hadamard states are the physically correct class; developed algebraic QFTCS.

**William Unruh** (1945–): Discovered the Unruh effect (1976); operational definition of particles via detectors; also discovered Hawking radiation for acoustic black holes.

**Bryce DeWitt** (1923–2004): Pioneered quantum gravity and QFTCS; introduced the DeWitt-Schwinger proper-time expansion for renormalization.

**Stephen Hawking** (1942–2018): Derived black hole radiation (1974–1975) using Bogoliubov techniques; posed the information paradox.

---

## Further Reading

**Primary Sources**
- Parker, L. (1969). "Quantized Fields and Particle Creation in Expanding Universes." *Phys. Rev.*, 183, 1057.
- Unruh, W.G. (1976). "Notes on Black-Hole Evaporation." *Phys. Rev. D*, 14, 870.
- Hawking, S.W. (1975). "Particle Creation by Black Holes." *Comm. Math. Phys.*, 43, 199.
- DeWitt, B.S. (1975). "Quantum Field Theory in Curved Spacetime." *Physics Reports*, 19, 295.

**Textbooks**
- Birrell, N.D. & Davies, P.C.W. (1982). *Quantum Fields in Curved Space*. Cambridge. — The standard reference; comprehensive.
- Wald, R.M. (1994). *Quantum Field Theory in Curved Spacetime and Black Hole Thermodynamics*. University of Chicago Press. — Rigorous axiomatic approach.
- Parker, L. & Toms, D. (2009). *Quantum Field Theory in Curved Spacetime*. Cambridge. — Modern treatment including applications.

---

## Exercises

**55.1.** *Bogoliubov transformation.*

(a) A Bogoliubov transformation $\hat{b} = \alpha\hat{a} + \beta\hat{a}^\dagger$ with $|\alpha|^2 - |\beta|^2 = 1$ maps one vacuum to another. Show that in the $\hat{a}$-vacuum, the mean $\hat{b}$-particle number is $|\beta|^2$.

(b) The squeezed vacuum state $|0_b\rangle = \hat{S}(\xi)|0_a\rangle$ where $\hat{S}(\xi) = \exp(\xi^* \hat{a}^2/2 - \xi(\hat{a}^\dagger)^2/2)$ is the squeezing operator. Show that $\hat{b}|0_b\rangle = 0$ with $\hat{b} = \cosh|\xi|\hat{a} - (ξ/|ξ|)\sinh|ξ|\hat{a}^\dagger$.

(c) Particle creation in cosmology: a mode of the scalar field starts in the vacuum before inflation and evolves through the de Sitter phase. Estimate $|\beta_k|^2$ for modes with $k\ll aH$ (super-Hubble) using the fact that $\omega_k^2 \to 0$ and the WKB approximation breaks down.

---

**55.2.** *Trace anomaly.*

For a conformally coupled ($\xi = 1/6$) massless scalar in 4D:

(a) The classical action is conformally invariant: show $T^\mu_{\ \mu} = 0$ classically using the equations of motion.

(b) The quantum trace anomaly is $\langle T^\mu_{\ \mu}\rangle = \frac{1}{2880\pi^2}(R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma} - R_{\mu\nu}R^{\mu\nu} + \Box R/6)$ (Wald). For a Schwarzschild black hole with $R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma} = 48M^2/r^6$ and $R_{\mu\nu} = 0$: compute $\langle T^\mu_{\ \mu}\rangle$ at $r = 2GM/c^2$.

(c) The trace anomaly is related to the Hawking temperature: $\langle T\rangle_{\rm anomaly} \propto T_H^4$ (Stefan-Boltzmann). Verify the dimensional scaling.

---

**55.3.** *Particle creation in cosmology.*

For a massless minimally coupled scalar in de Sitter space $a(\eta) = -1/(H\eta)$ ($\eta < 0$):

(a) The mode equation is $\varphi_k'' + (k^2 - 2/\eta^2)\varphi_k = 0$ (Bunch-Davies). The solution regular in the past ($\eta\to -\infty$): $\varphi_k = \frac{1}{\sqrt{2k}}(1 - \frac{i}{k\eta})e^{-ik\eta}$. Verify this satisfies the mode equation.

(b) In the future $k\eta\to 0$: $\varphi_k\to i/(k\eta\sqrt{2k}) = iH/(k^{3/2}\sqrt{2})a^{-1}$. Show the power spectrum $\mathcal{P}_\phi(k) = k^3|\varphi_k|^2/(2\pi^2) = H^2/(4\pi^2)$ (scale-invariant).

(c) This scale-invariant spectrum is the origin of the CMB power spectrum. Why does scale invariance imply $n_s = 1$ (Harrison-Zel'dovich), and what small deviation from de Sitter introduces $n_s < 1$?

---

**Thought Experiment T55.1.** *The meaning of particles.*

The concept of "particle" seems fundamental — we detect particles in accelerators, we count photons in photodetectors. But QFTCS reveals that "particle" is an observer-dependent concept. The Minkowski vacuum (no particles) is a thermal state for an accelerating observer (Unruh effect). The vacuum near a black hole is a thermal state at infinity (Hawking radiation).

Does this mean particles are "not real"? Or does it mean "real" depends on the observer?

In non-relativistic quantum mechanics, the particle number is absolute: either there are $N$ electrons or there aren't. In QFT, particle number is not conserved (virtual particles, pair creation), and in curved spacetime, it's not even observer-independent.

What is the observer-independent, physically meaningful content of QFTCS? (Hint: the stress-energy tensor $T_{\mu\nu}$ is observer-independent after renormalization, even if the particle number is not.) Is there a formulation of QFTCS that avoids the particle concept entirely?
