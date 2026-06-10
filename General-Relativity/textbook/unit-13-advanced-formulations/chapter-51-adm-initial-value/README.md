# Chapter 51: The ADM Formalism and the Initial Value Problem

---

## Chapter Introduction

Einstein's equations $G_{\mu\nu} = 8\pi G T_{\mu\nu}/c^4$ are ten coupled nonlinear PDEs for the ten components of the metric. But how many of these are dynamical equations versus constraints? Which combinations of metric components evolve freely, and which are fixed by the constraint equations at each moment? What does it mean to specify initial data for GR?

These questions are answered by the **ADM formalism** (Arnowitt, Deser, and Misner, 1962) and the associated $3+1$ decomposition of spacetime. The key idea: foliate spacetime into a one-parameter family of spatial hypersurfaces $\Sigma_t$ of constant time $t$. The spacetime metric is decomposed into:
- The **3-metric** $\gamma_{ij}$ on each spatial slice (6 components)
- The **lapse function** $\alpha$ (how much proper time elapses per coordinate time)
- The **shift vector** $\beta^i$ (how the spatial coordinates shift between slices)

The full spacetime metric is:
$$ds^2 = -\alpha^2 c^2 dt^2 + \gamma_{ij}(dx^i + \beta^i dt)(dx^j + \beta^j dt)$$

This decomposition separates GR's constraints from its true dynamics. The lapse $\alpha$ and shift $\beta^i$ are **gauge variables** — they encode coordinate freedom and carry no physical information. The physical degrees of freedom are encoded in $\gamma_{ij}$ and its conjugate momentum $K_{ij}$ (the extrinsic curvature — how the spatial slice curves in the surrounding spacetime).

The Einstein equations in the 3+1 form split into:
- **Constraint equations**: 4 equations ($G^{0\mu} = 8\pi G T^{0\mu}/c^4$) that must hold on each spatial slice; they do not involve time derivatives of $\alpha$ or $\beta^i$ — they constrain the initial data
- **Evolution equations**: 6 equations ($G^{ij} = 8\pi G T^{ij}/c^4$) that evolve $\gamma_{ij}$ and $K_{ij}$ forward in time

This is the foundation of **numerical relativity**: choose initial data satisfying the constraints, choose a gauge (lapse and shift), and evolve the spatial metric forward in time using the evolution equations. The merger and ringdown phases of GW150914 were computed using exactly this approach — the BSSN (Baumgarte-Shapiro-Shibata-Nakamura) formulation and the moving puncture method, which evolved the binary black hole spacetime through dozens of orbits to coalescence.

---

## The 3+1 Decomposition

**Foliating spacetime.** Choose a time function $t(x^\mu)$ with everywhere spacelike level surfaces $\Sigma_t = \{x : t(x) = \text{const}\}$. The unit normal to $\Sigma_t$ is $n^\mu = -\alpha\nabla^\mu t$, with $n_\mu n^\mu = -1$.

**The ADM metric.** The most general metric compatible with the foliation:
$$g_{\mu\nu} = \begin{pmatrix} -\alpha^2 + \beta_k\beta^k & \beta_j\\ \beta_i & \gamma_{ij}\end{pmatrix}$$

**Inverse metric:**
$$g^{\mu\nu} = \begin{pmatrix} -1/\alpha^2 & \beta^j/\alpha^2\\ \beta^i/\alpha^2 & \gamma^{ij} - \beta^i\beta^j/\alpha^2\end{pmatrix}$$

**The spatial metric** $\gamma_{ij} = g_{ij}$ is the induced metric on $\Sigma_t$, which is also a projection operator $\gamma^{\mu\nu} = g^{\mu\nu} + n^\mu n^\nu/c^2$ (projecting perpendicular to $n^\mu$ gives the spatial metric).

**Extrinsic curvature.** The extrinsic curvature $K_{ij}$ measures how the spatial slice bends in the surrounding spacetime:
$$K_{ij} = -\frac{1}{2\alpha}(\partial_t\gamma_{ij} - \mathcal{L}_\beta\gamma_{ij}) = -\nabla_{(i}n_{j)}$$

where $\mathcal{L}_\beta$ is the Lie derivative along the shift vector. $K_{ij}$ is the conjugate momentum to $\gamma_{ij}$ in the Hamiltonian sense.

---

## The Constraint Equations

Projecting the Einstein equations along and perpendicular to the normal $n^\mu$ gives:

**Hamiltonian constraint** (one scalar equation):
$${}^{(3)}R + K^2 - K_{ij}K^{ij} = \frac{16\pi G}{c^4}\rho_{\rm ADM}$$

where ${}^{(3)}R$ is the Ricci scalar of $\gamma_{ij}$, $K = \gamma^{ij}K_{ij}$ is the trace, and $\rho_{\rm ADM} = T_{\mu\nu}n^\mu n^\nu$ is the energy density as measured by normal observers.

**Momentum constraints** (three vector equations):
$$D_j K^j_{\ i} - D_i K = \frac{8\pi G}{c^4}j_i$$

where $D_i$ is the covariant derivative compatible with $\gamma_{ij}$ and $j_i = -T_{\mu\nu}n^\mu\gamma^\nu_{\ i}$ is the momentum density.

These four equations contain no time derivatives of $\alpha$ or $\beta^i$. They are **elliptic PDEs** for the initial data $(\gamma_{ij}, K_{ij})$ — they must be satisfied on each slice, and particularly as conditions on the initial slice.

**Free data vs. constrained data.** In 3+1, the physical initial data is $(\gamma_{ij}, K_{ij})$ — 12 functions on $\Sigma_0$. But 4 constraint equations reduce the physical degrees of freedom to $12 - 4 = 8$ per spatial point. Since gauge freedom uses 4 more (choice of $\alpha$ and $\beta^i$), the true physical degrees of freedom are $8 - 4 = 4$ per spatial point — 2 for each of the 2 gravitational wave polarizations and their conjugate momenta.

---

## The Evolution Equations

The evolution equations follow from $G^{ij} = 8\pi G T^{ij}/c^4$:

**Metric evolution**:
$$\partial_t\gamma_{ij} = -2\alpha K_{ij} + D_i\beta_j + D_j\beta_i$$

**Extrinsic curvature evolution**:
$$\partial_t K_{ij} = -D_i D_j\alpha + \alpha\left({}^{(3)}R_{ij} + KK_{ij} - 2K_{ik}K^k_{\ j}\right)$$
$$+ 4\pi G\alpha\left[\gamma_{ij}(\text{tr}T - \rho_{\rm ADM}c^2) - 2S_{ij}\right] + \mathcal{L}_\beta K_{ij}$$

where $S_{ij} = T_{\mu\nu}\gamma^\mu_{\ i}\gamma^\nu_{\ j}$ is the spatial stress tensor.

These are **hyperbolic PDEs** — they evolve the initial data forward in time. Given $(\gamma_{ij}, K_{ij})$ satisfying the constraints on $\Sigma_0$ and a choice of $\alpha$, $\beta^i$, they determine the entire spacetime.

---

## Numerical Relativity and Binary Black Holes

The ADM equations as written are numerically unstable — constraint violations grow without bound. The community developed stable reformulations:

**BSSN formulation (1995–1999)**: Baumgarte, Shapiro, Shibata, and Nakamura reformulate by introducing conformal variables:
$$\gamma_{ij} = e^{4\phi}\tilde{\gamma}_{ij}, \quad K_{ij} = e^{4\phi}(\tilde{A}_{ij} + \frac{1}{3}\tilde{\gamma}_{ij}K)$$

where $\tilde{\gamma}_{ij}$ is the conformal metric with $\det(\tilde{\gamma}) = 1$, $\tilde{A}_{ij}$ is the traceless extrinsic curvature, and $\phi$ is the conformal factor. Additional auxiliary variables $\tilde{\Gamma}^i = \tilde{\gamma}^{jk}\tilde{\Gamma}^i_{jk}$ are evolved. The result is a strongly hyperbolic system with stable numerical properties.

**Puncture method (2005–2006)**: Pretorius, and independently Baker et al. and Campanelli et al., achieved the first stable long-term binary black hole evolutions. The "moving puncture" approach treats the black hole singularities as punctures in the spatial slice (coordinate singularities), moving them through the grid rather than removing them. The "1+log" slicing condition for $\alpha$ and the "Gamma-driver" condition for $\beta^i$ evolved from the initial gauge choice and proved remarkably robust.

**GW150914 validation**: The merger waveform for GW150914 was compared to $\sim 10^5$ numerical relativity waveforms from the Surrogate Gravitational Waveform Model (NR-surrogate, Blackman et al. 2017). The event's parameters were recovered by fitting to this bank.

---

## The ADM Mass and Conserved Charges

In asymptotically flat spacetimes, one can define global conserved charges from the boundary terms of the ADM Hamiltonian.

**ADM mass:**
$$M_{\rm ADM} = \frac{c^2}{16\pi G}\lim_{r\to\infty}\oint_{S^2}\left(\partial_j\gamma_{ij} - \partial_i\gamma_{jj}\right)dS^i$$

This integral over a sphere at spatial infinity measures the total energy-momentum content of the spacetime, including gravitational binding energy.

**ADM linear momentum** $P_i$ and angular momentum $J_i$ are similarly defined from higher-order falloff of the metric.

These charges are conserved (time-independent) in vacuum. When gravitational waves are present, energy is radiated to null infinity — the **Bondi mass** $M_{\rm Bondi}(u)$ (a function of retarded time $u$) decreases as waves escape. The Bondi mass loss formula $\dot{M}_{\rm Bondi} = -c^3/(16\pi G)\oint|N_{AB}|^2 d\Omega$ (where $N_{AB}$ is the Bondi news tensor) gives the gravitational wave luminosity.

---

## The Problem of Time in Quantum Gravity

The Hamiltonian formulation of GR has a profound consequence for quantum gravity: the Hamiltonian is a **constraint** ($\mathcal{H} = 0$), not a generator of dynamics. In the Dirac quantization scheme, physical states $|\Psi\rangle$ must satisfy:
$$\hat{\mathcal{H}}|\Psi\rangle = 0 \quad \text{(Wheeler-DeWitt equation)}$$

But this equation has no explicit time derivative — there is no external time variable. The universe is its own clock; time must emerge from the correlations between different degrees of freedom within the wavefunction of the universe. This is the "**problem of time**" in quantum gravity — one of the deepest unsolved conceptual problems at the intersection of GR and quantum mechanics.

---

## Exercises

**51.1.** *ADM decomposition for Schwarzschild.*

(a) Write the Schwarzschild metric in ADM form using isotropic coordinates $\tilde{r}$ where $r = \tilde{r}(1 + M/2\tilde{r})^2$. Identify $\alpha$, $\beta^i$, and $\gamma_{ij}$.

(b) Show that the extrinsic curvature $K_{ij} = 0$ (the $t = \text{const}$ slices of Schwarzschild are maximal slices with $K = 0$).

(c) Verify the Hamiltonian constraint ${}^{(3)}R = 0$ (vacuum) on these slices.

---

**51.2.** *Initial data for a binary black hole.*

(a) For a time-symmetric initial data set ($K_{ij} = 0$), the constraint equations reduce to ${}^{(3)}R = 0$ (vacuum). This is the Laplace equation for the conformal factor $\phi$ (with $\gamma_{ij} = e^{4\phi}\eta_{ij}$).

(b) The Brill-Lindquist initial data for two black holes at positions $\mathbf{r}_1$, $\mathbf{r}_2$ with masses $m_1$, $m_2$ is $\psi = 1 + m_1/(2r_1) + m_2/(2r_2)$ (superposition of two isotropic Schwarzschild metrics). Show this satisfies $\nabla^2\psi = 0$.

(c) What is the ADM mass of this initial data? Does it equal $m_1 + m_2$? If not, explain the discrepancy physically.

---

**Thought Experiment T51.1.** *Gauge choices and physical observables in NR.*

In numerical relativity, the gauge choice (lapse and shift conditions) determines how the coordinates evolve. Different gauge choices can make the same physical spacetime look very different numerically.

The "singularity-avoiding" slicing (1+log lapse) keeps the lapse function small near singularities, preventing the coordinate system from crashing into the singularity. The "Gamma-driver" shift condition moves the coordinate singularities ("punctures") through the grid.

If two NR codes use different gauge choices but identical initial data, they will produce different-looking numerical grids. How do you extract physical observables (like the gravitational waveform at infinity) that are gauge-independent? What is the relationship between gauge freedom in GR and gauge freedom in quantum field theory?
