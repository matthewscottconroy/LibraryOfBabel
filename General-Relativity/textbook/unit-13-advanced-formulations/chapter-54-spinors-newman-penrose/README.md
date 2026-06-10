# Chapter 54: Spinors and the Newman-Penrose Formalism

---

## Chapter Introduction

The tensor formalism we have developed throughout this textbook — metric, curvature, covariant derivatives — is powerful and general. But it has limitations. It cannot directly describe fermions: particles with half-integer spin (electrons, neutrinos, quarks) whose quantum states change sign under a full $2\pi$ rotation. For these, the natural mathematical objects are **spinors** — objects that transform under the spin representation of the Lorentz group rather than the tensor representation.

In GR, the natural spinor formalism is the **Newman-Penrose (NP) formalism** — a tetrad-based approach using null tetrads (a frame of four null vectors at each point) and representing all geometric quantities as complex scalars called **spin coefficients** and **Weyl scalars**. The NP formalism is the natural language for:
- Gravitational radiation (the Weyl scalars $\Psi_0, \ldots, \Psi_4$ directly encode the radiation content)
- Exact solutions with symmetry (Petrov classification is most natural in NP)
- The Kerr metric and rotating black holes
- The Dirac equation in curved spacetime
- Perturbation theory for black holes (Teukolsky equation)

This chapter introduces the spinor concept, develops the Newman-Penrose formalism, and shows how it illuminates the structure of gravitational radiation and black hole perturbations.

---

## Spinors: The Double Cover of the Lorentz Group

The Lorentz group $SO^+(1,3)$ has a double cover: the group $SL(2,\mathbb{C})$ of $2\times 2$ complex matrices with determinant $1$. The covering map $\pi: SL(2,\mathbb{C})\to SO^+(1,3)$ has kernel $\{+I, -I\}$ — two elements in $SL(2,\mathbb{C})$ map to each Lorentz transformation.

A **2-component Weyl spinor** $\kappa^A$ (or $\kappa_A$, with $A = 0, 1$) transforms under the fundamental representation of $SL(2,\mathbb{C})$:
$$\kappa'^A = M^A_{\ B}\kappa^B, \quad M\in SL(2,\mathbb{C})$$

This is the spin-$1/2$ representation. Under a full $2\pi$ spatial rotation, $M\to -M$, so $\kappa^A\to -\kappa^A$ — the famous sign change of spinors.

**The spinor metric**: The antisymmetric $2\times 2$ matrix $\varepsilon_{AB} = \varepsilon^{AB} = \begin{pmatrix}0 & 1 \\ -1 & 0\end{pmatrix}$ plays the role of the metric for spinors: $\kappa_A = \varepsilon_{AB}\kappa^B$ raises/lowers spinor indices.

**Connection to 4-vectors**: Any 4-vector $v^\mu$ can be mapped to a $2\times 2$ Hermitian matrix:
$$v^{AB'} = v^\mu\sigma_\mu^{\ AB'} = \frac{1}{\sqrt{2}}\begin{pmatrix}v^0+v^3 & v^1-iv^2 \\ v^1+iv^2 & v^0-v^3\end{pmatrix}$$

where $\sigma^\mu = (I, \boldsymbol{\sigma})$ are the Pauli matrices extended. The spacetime interval:
$$\eta_{\mu\nu}v^\mu v^\nu = -\det(v^{AB'})$$

This is the van der Waerden isomorphism: 4-vectors $\leftrightarrow$ Hermitian $2\times 2$ matrices.

---

## Spinor Algebra in GR

On a curved spacetime manifold, spinors require a **spin structure**: a choice of local frames (tetrads) and a consistent way to patch them together. Not all manifolds admit spin structures (they must have vanishing second Stiefel-Whitney class), but the physically relevant spacetimes do.

The **spin covariant derivative** $\nabla_{AA'}$ acts on spinors. For a spinor $\kappa^B$:
$$\nabla_{AA'}\kappa^B = \partial_{AA'}\kappa^B + \Gamma^B_{\ CAA'}\kappa^C$$

where $\Gamma^B_{\ CAA'}$ are the spin connection coefficients.

The **Weyl spinor** $\Psi_{ABCD}$ is the spinor equivalent of the Weyl tensor — totally symmetric in its four spinor indices, with 5 complex independent components:
$$\Psi_{ABCD} = C_{\mu\nu\rho\sigma}\sigma^{\mu}_{\ AA'}\sigma^{\nu}_{\ BB'}\sigma^{\rho}_{\ CC'}\sigma^{\sigma}_{\ DD'}$$

(with the appropriate normalization). Similarly, the Ricci spinor $\Phi_{ABA'B'}$ encodes the Ricci tensor.

**Petrov types in spinor language**: The Weyl spinor is the totally symmetric product of four principal spinors $\alpha^A, \beta^A, \gamma^A, \delta^A$:
$$\Psi_{ABCD} = \alpha_{(A}\beta_B\gamma_C\delta_{D)}$$

The Petrov type depends on coincidences:
- Type I: all four distinct
- Type II: two coincide $(\alpha = \beta)$
- Type III: three coincide $(\alpha = \beta = \gamma)$
- Type N: all four coincide $(\alpha = \beta = \gamma = \delta)$
- Type D: two pairs coincide $(\alpha = \beta, \gamma = \delta)$
- Type O: $\Psi_{ABCD} = 0$ (conformally flat)

Schwarzschild and Kerr are Petrov type D; gravitational waves far from source are type N.

---

## Null Tetrads

The Newman-Penrose formalism is built on a **null tetrad** at each point: four null vectors $(\ell^\mu, n^\mu, m^\mu, \bar{m}^\mu)$ where:
- $\ell^\mu, n^\mu$ are real null vectors: $\ell^\mu\ell_\mu = n^\mu n_\mu = 0$
- $m^\mu, \bar{m}^\mu$ are complex conjugate null vectors: $m^\mu m_\mu = 0$
- Cross products: $\ell^\mu n_\mu = -1$, $m^\mu\bar{m}_\mu = +1$, all others zero

The metric is decomposed as:
$$g^{\mu\nu} = -\ell^\mu n^\nu - n^\mu\ell^\nu + m^\mu\bar{m}^\nu + \bar{m}^\mu m^\nu$$

**Physical interpretation**: 
- $\ell^\mu$: principal null direction (for Schwarzschild/Kerr: outgoing radial null geodesic)
- $n^\mu$: second principal null direction (ingoing radial null geodesic)
- $m^\mu, \bar{m}^\mu$: complex directions spanning the 2-sphere

---

## Spin Coefficients and Weyl Scalars

The NP formalism replaces the 40 Christoffel symbols with 12 complex **spin coefficients** $\kappa, \sigma, \rho, \tau, \pi, \nu, \mu, \lambda, \varepsilon, \gamma, \alpha, \beta$ encoding the optical properties of the null congruences.

Key spin coefficients:
- $\rho$: complex divergence of $\ell^\mu$ (expansion + twist)
- $\sigma$: shear of $\ell^\mu$
- $\kappa$: geodesic deviation of $\ell^\mu$ (vanishes iff $\ell$ is geodesic)

The **Weyl scalars** $\Psi_0, \Psi_1, \Psi_2, \Psi_3, \Psi_4$ are the 5 complex components of the Weyl spinor projected onto the null tetrad:
$$\Psi_0 = C_{\mu\nu\rho\sigma}\ell^\mu m^\nu\ell^\rho m^\sigma$$
$$\Psi_1 = C_{\mu\nu\rho\sigma}\ell^\mu n^\nu\ell^\rho m^\sigma$$
$$\Psi_2 = C_{\mu\nu\rho\sigma}\ell^\mu m^\nu\bar{m}^\rho n^\sigma$$
$$\Psi_3 = C_{\mu\nu\rho\sigma}\ell^\mu n^\nu\bar{m}^\rho n^\sigma$$
$$\Psi_4 = C_{\mu\nu\rho\sigma}n^\mu\bar{m}^\nu n^\rho\bar{m}^\sigma$$

**Physical interpretation** (for asymptotically flat spacetime at large $r$):
- $\Psi_0, \Psi_1$: ingoing gravitational radiation (components fall as $r^{-5}$ or faster)
- $\Psi_2$: Coulomb-like part of the gravitational field (fall as $r^{-3}$)
- $\Psi_3, \Psi_4$: outgoing gravitational radiation ($r^{-1}$)

The **Peeling theorem** (Sachs 1961): In an asymptotically flat spacetime, as $r\to\infty$:
$$\Psi_k = \Psi_k^{(5-k)}/r^{5-k} + O(r^{-6+k})$$

Outgoing gravitational radiation is encoded in $\Psi_4^{(1)}/(r)$ — the leading-order Weyl scalar.

**For Kerr**: $\Psi_2 \neq 0$ (it encodes the mass and angular momentum), $\Psi_0 = \Psi_1 = \Psi_3 = \Psi_4 = 0$ — consistent with Petrov type D.

---

## The Teukolsky Equation

The most powerful application of the NP formalism: perturbations of the Kerr metric. 

In the 1970s, Teukolsky found that perturbations of a Kerr black hole in NP variables decouple completely. The master equation for a spin-$s$ perturbation:
$$\left[\frac{(r^2+a^2)^2}{\Delta} - a^2\sin^2\theta\right]\frac{\partial^2\Psi}{\partial t^2} + \frac{4Mar}{\Delta}\frac{\partial^2\Psi}{\partial t\partial\phi} + \left[\frac{a^2}{\Delta} - \frac{1}{\sin^2\theta}\right]\frac{\partial^2\Psi}{\partial\phi^2} - \Delta^{-s}\frac{\partial}{\partial r}\left(\Delta^{s+1}\frac{\partial\Psi}{\partial r}\right) - \frac{1}{\sin\theta}\frac{\partial}{\partial\theta}\left(\sin\theta\frac{\partial\Psi}{\partial\theta}\right) - 2s\left[\frac{a(r-M)}{\Delta} + \frac{i\cos\theta}{\sin^2\theta}\right]\frac{\partial\Psi}{\partial\phi} - 2s\left[\frac{M(r^2-a^2)}{\Delta} - r - ia\cos\theta\right]\frac{\partial\Psi}{\partial t} + (s^2\cot^2\theta - s)\Psi = 0$$

where $s = 0$ (scalar), $\pm 1/2$ (Dirac), $\pm 1$ (Maxwell), $\pm 2$ (gravitational waves).

For $s = -2$: $\Psi = \Psi_4$ (outgoing gravitational radiation). The equation separates with mode functions $e^{-i\omega t}e^{im\phi}S_{lm}(\theta)R_{lm}(r)$, where $S_{lm}$ are spheroidal harmonics.

**Quasinormal modes** (QNMs): The resonance frequencies $\omega_{lmn}$ of the Teukolsky equation with purely outgoing boundary conditions. For gravitational wave events, the post-merger ringdown phase is described by QNMs — the "ringing" of the final Kerr black hole as it settles down. Measuring QNMs tests the Kerr nature of the remnant (no-hair theorem tests).

For the dominant $l = m = 2$ mode of a Schwarzschild black hole:
$$\omega_{220} \approx \frac{c^3}{GM}\left(0.3737 - 0.0890i\right)$$

The imaginary part gives the damping time $\tau = M/(\text{Im}[\omega_{220}]c^3/G)$.

---

## The Dirac Equation in Curved Spacetime

The curved-spacetime Dirac equation for a spin-1/2 particle of mass $m$:
$$(\gamma^a e_a^{\ \mu}\nabla_\mu + mc/\hbar)\psi = 0$$

where:
- $e_a^{\ \mu}$ is the tetrad (vierbein): $g_{\mu\nu} = \eta_{ab}e^a_{\ \mu}e^b_{\ \nu}$
- $\gamma^a$ are the flat-space Dirac matrices: $\{\gamma^a, \gamma^b\} = 2\eta^{ab}$
- $\nabla_\mu\psi = \partial_\mu\psi + \frac{1}{4}\omega_\mu^{\ ab}\gamma_a\gamma_b\psi$ with $\omega_\mu^{\ ab}$ the spin connection

In Schwarzschild background: the Dirac equation separates (Chandrasekhar 1976). The solution involves spinor harmonics (half-integer angular momentum) and behaves as expected at the horizon and infinity.

The Dirac equation in curved spacetime is used for:
- Calculating pair creation near black holes (Hawking radiation for fermions)
- Stability analysis of black holes against fermionic perturbations
- Cosmological fermion dynamics (neutrino propagation in FLRW)

---

## Important Concepts

- **Spinor**: Object transforming under $SL(2,\mathbb{C})$ (double cover of Lorentz group); changes sign under $2\pi$ rotation
- **Weyl spinor**: 2-component undotted/dotted spinors; fundamental representation of $SL(2,\mathbb{C})$
- **Van der Waerden isomorphism**: 4-vectors $\leftrightarrow$ Hermitian $2\times 2$ matrices; $v^\mu\to v^{AA'}$
- **Null tetrad**: $(\ell, n, m, \bar{m})$; metric decomposed as $g^{\mu\nu} = -\ell^\mu n^\nu - n^\mu\ell^\nu + m^\mu\bar{m}^\nu + \bar{m}^\mu m^\nu$
- **Spin coefficients**: 12 complex scalars replacing Christoffel symbols in NP formalism
- **Weyl scalars** $\Psi_0\ldots\Psi_4$: Projections of Weyl tensor on null tetrad; $\Psi_4$ = outgoing GW
- **Peeling theorem**: Weyl scalars peel off as $r^{-k}$; radiation in $\Psi_4 \sim r^{-1}$
- **Teukolsky equation**: Master equation for spin-$s$ perturbations of Kerr; decoupled in NP variables
- **Quasinormal modes**: Complex resonance frequencies of black hole perturbations; measured in GW ringdown
- **Dirac equation in curved spacetime**: Requires tetrads and spin connection; used for Hawking radiation, stability

---

## Important Figures

**Ezra Newman** (1929–2021) and **Roger Penrose** (1931–): Developed the NP formalism (1962); made gravitational wave analysis tractable.

**Rainer Sachs** (1932–): Proved the peeling theorem; foundational work on gravitational radiation.

**Subrahmanyan Chandrasekhar** (1910–1995): Applied NP formalism to Dirac equation in Schwarzschild; wrote *The Mathematical Theory of Black Holes* (1983); Nobel Prize 1983.

**Saul Teukolsky** (1947–): Derived the Teukolsky equation (1973) for perturbations of Kerr; still used for GW waveform calculations.

---

## Further Reading

**Primary Sources**
- Newman, E. & Penrose, R. (1962). "An Approach to Gravitational Radiation by a Method of Spin Coefficients." *J. Math. Phys.*, 3, 566.
- Teukolsky, S.A. (1973). "Perturbations of a Rotating Black Hole." *ApJ*, 185, 635.

**Textbooks**
- Chandrasekhar, S. (1983). *The Mathematical Theory of Black Holes*. Oxford. — Comprehensive NP treatment of black hole perturbations.
- Penrose, R. & Rindler, W. (1984). *Spinors and Space-Time*. Cambridge (2 vols.). — The definitive reference for spinors in GR.
- Wald, R.M. (1984). *General Relativity*. Chicago. — Appendix on spinors.

---

## Exercises

**54.1.** *Spinor basics.*

(a) A 2-component Weyl spinor $\kappa^A = \begin{pmatrix}1 \\ 0\end{pmatrix}$ transforms under the $SL(2,\mathbb{C})$ element $M = \begin{pmatrix}e^{i\theta/2} & 0 \\ 0 & e^{-i\theta/2}\end{pmatrix}$ (rotation by $\theta$ about $z$-axis). What is $\kappa^A$ after a full $2\pi$ rotation? After $4\pi$?

(b) The spinor $\kappa^A\mu_A = \varepsilon_{AB}\kappa^A\mu^B$ is a Lorentz scalar. Verify this is antisymmetric: $\kappa^A\mu_A = -\mu^A\kappa_A$.

(c) The null vector $\ell^\mu = \bar\kappa^{A'}\kappa^A\sigma^\mu_{\ AA'}$ (flagpole construction). Show $\ell^\mu\ell_\mu = 0$ and $\ell^0 > 0$ (future-pointing) if $\kappa^A \neq 0$.

---

**54.2.** *Weyl scalars for Schwarzschild.*

For Schwarzschild with the null tetrad $\ell = \partial_v + \frac{1}{2}(1-r_s/r)\partial_r$ (ingoing Eddington-Finkelstein), $n = -\partial_r$, $m = r^{-1}(1/\sqrt{2})(\partial_\theta + i\csc\theta\partial_\phi)$:

(a) Verify $\ell\cdot n = -1$ and $m\cdot\bar{m} = 1$.

(b) Compute $\Psi_2 = C_{\mu\nu\rho\sigma}\ell^\mu m^\nu\bar{m}^\rho n^\sigma$. Show $\Psi_2 = -M/(r^3)$ (in units $G = c = 1$). What does this represent physically?

(c) Verify $\Psi_0 = \Psi_1 = \Psi_3 = \Psi_4 = 0$ — consistent with Petrov type D.

---

**54.3.** *Quasinormal modes and ringdown.*

The dominant QNM of a Schwarzschild black hole ($l = m = 2$) has $\omega_{220} \approx (0.3737 - 0.0890i)\,c^3/(GM)$.

(a) For the GW150914 remnant ($M_f \approx 63M_\odot$), compute the QNM frequency in Hz and the damping time in ms.

(b) Compare to the observed GW150914 post-merger signal (peak frequency $\sim 250$ Hz, ring-down time $\sim 10$ ms). Are these consistent?

(c) In a Kerr black hole, the QNM frequencies depend on spin $a/M$. If the spin is $a/M = 0.68$ (consistent with GW150914), by what percent does $\text{Re}[\omega_{220}]$ change from the Schwarzschild value? (Look up Kerr QNM tables or the fitting formula.)

---

**Thought Experiment T54.1.** *Why do fermions exist?*

Spinors arise because the Lorentz group has a double cover — $SL(2,\mathbb{C})$ rather than $SO^+(1,3)$. Mathematically, this is just a fact about Lie groups. But physically, it means nature can make objects that are "square roots of vectors" — half-integer spin particles.

Why should nature use the double cover? Could there be a universe with only integer-spin particles (bosons, no fermions)? 

In fact, the CPT theorem and the spin-statistics theorem tell us: in any Lorentz-invariant quantum field theory, half-integer spin particles must be fermions (antisymmetric under exchange). The Pauli exclusion principle — which is responsible for the stability of matter, the periodic table, and all of chemistry — is a consequence of Lorentz symmetry plus quantum mechanics.

Could GR have been formulated with only bosonic matter? What would such a universe look like? Is there a deep reason that the Lorentz group's double cover must be used, or is it an observational fact about our universe?
