# Chapter 61: Numerical Relativity

---

## Chapter Introduction

The exact solutions of GR — Schwarzschild, Kerr, FLRW — are beautiful, but they describe idealized situations: perfect spherical symmetry, perfect rotation, perfect homogeneity. The real universe is messier. Two black holes in a binary system, spiraling toward merger, emit gravitational waves with a complex, evolving waveform. A massive star collapsing to form a neutron star or black hole, driven by neutrino pressure and magnetic fields, is emphatically non-symmetric. The collision of two neutron stars involves nuclear physics, magnetohydrodynamics, and strong-field GR all at once.

To understand these phenomena, we need to solve Einstein's equations numerically. This is **numerical relativity** — the computational branch of GR. It is technically and conceptually demanding: the Einstein equations are a system of coupled nonlinear PDEs with both elliptic and hyperbolic character, on a domain whose topology can change (black holes can merge, horizons can form). Formulating them in a way that is both mathematically well-posed and computationally stable took decades of effort.

The key breakthrough came in 2005, when Frans Pretorius solved the binary black hole merger problem for the first time — enabling the calculation of the gravitational waveforms used in GW event detection. Numerical relativity is now a mature field, essential for the science of gravitational wave astronomy.

---

## The $3+1$ Decomposition (ADM)

To solve Einstein's equations as an initial value problem, we decompose spacetime into a family of spacelike slices $\Sigma_t$. The ADM (Arnowitt-Deser-Misner) decomposition:
$$ds^2 = -(\alpha^2 - \beta_i\beta^i)c^2dt^2 + 2\beta_i dx^i\,c\,dt + \gamma_{ij}dx^idx^j$$

where:
- $\gamma_{ij}$: induced 3-metric on $\Sigma_t$ (the spatial metric)
- $\alpha$: lapse function (how much coordinate time elapses per unit proper time of normal observers)
- $\beta^i$: shift vector (how much coordinates "shift" between slices)

**Physical interpretation**: $\alpha$ controls the "speed of time" — choosing $\alpha = 1$ (geodesic slicing) means normal observers follow geodesics. $\beta^i$ controls how coordinates move between slices — choosing $\beta^i = 0$ means coordinates are "locked" to normal observers.

**Extrinsic curvature** $K_{ij}$: Measures how $\Sigma_t$ is embedded in spacetime:
$$K_{ij} = -\frac{1}{2\alpha}\left(\partial_t\gamma_{ij} - \mathcal{L}_\beta\gamma_{ij}\right)$$

(the rate of change of the spatial metric, adjusted for the shift).

---

## Constraint and Evolution Equations

The Einstein equations split into constraints (holding on each slice) and evolution equations (determining how the geometry changes in time).

**Hamiltonian constraint**:
$${}^{(3)}R + K^2 - K_{ij}K^{ij} = \frac{16\pi G}{c^4}\rho_{\rm ADM}$$

**Momentum constraints** (3 equations):
$${}^{(3)}\nabla_j(K^{ij} - \gamma^{ij}K) = \frac{8\pi G}{c^4}j^i_{\rm ADM}$$

where ${}^{(3)}R$ is the Ricci scalar of $\gamma_{ij}$, $K = \gamma^{ij}K_{ij}$, and $\rho_{\rm ADM}$, $j^i_{\rm ADM}$ are the energy and momentum densities measured by normal observers.

**Evolution equations** (12 equations — 6 for $\gamma_{ij}$, 6 for $K_{ij}$):
$$\partial_t\gamma_{ij} = -2\alpha K_{ij} + \mathcal{L}_\beta\gamma_{ij}$$

$$\partial_t K_{ij} = -{}^{(3)}\nabla_i\nabla_j\alpha + \alpha\left({}^{(3)}R_{ij} + KK_{ij} - 2K_{ik}K^k_{\ j} - \frac{8\pi G}{c^4}\left(S_{ij} - \frac{1}{2}\gamma_{ij}(S - \rho_{\rm ADM})\right)\right) + \mathcal{L}_\beta K_{ij}$$

**Gauge freedom**: The lapse $\alpha$ and shift $\beta^i$ are freely specifiable — they represent coordinate freedom (which time slices to choose and how to label points within them). The choice of gauge profoundly affects the stability and accuracy of numerical evolutions.

**DOF counting**: $\gamma_{ij}$ has 6 components + $K_{ij}$ has 6 = 12 total; 4 constraints (1+3); 4 gauge freedoms (lapse + shift); $12 - 4 - 4 = 4$ physical degrees of freedom = 2 polarizations × 2 (position + momentum).

---

## The Well-Posedness Problem and BSSN

The ADM equations as written are **weakly hyperbolic** — they are not well-posed as an initial value problem. Small perturbations can grow at any rate. This makes them numerically unstable.

**The BSSN formulation** (Baumgarte-Shapiro-Shibata-Nakamura, 1995–1999): Rewrite the system by:
1. Conformally decompose: $\gamma_{ij} = e^{4\phi}\tilde\gamma_{ij}$ with $\det\tilde\gamma_{ij} = 1$
2. Trace-separate extrinsic curvature: $K_{ij} = e^{4\phi}\tilde A_{ij} + \frac{1}{3}\gamma_{ij}K$ with $\tilde\gamma^{ij}\tilde A_{ij} = 0$
3. Introduce conformal connection functions: $\tilde\Gamma^i = -\partial_j\tilde\gamma^{ij}$ as auxiliary variables

The BSSN system is **strongly hyperbolic** — well-posed, and numerically stable for moderate-duration evolutions. It became the workhorse of numerical relativity in the late 1990s and early 2000s.

---

## The Moving Puncture Method

The central technical challenge in binary black hole simulations: how to handle the singularities?

**Early approach (excision)**: Cut out the interior of each black hole (since it's causally disconnected), imposing boundary conditions on the excision surface inside the apparent horizon. Technically demanding but successful for single black holes.

**Moving puncture method** (Campanelli et al. 2005; Baker et al. 2005, simultaneously with Pretorius 2005): 
- Represent the black hole interior as a "puncture" — a single point in the numerical grid
- Use the gauge conditions: "1+log" slicing $\partial_t\alpha = -2\alpha K$ and "Gamma-driver" shift $\partial_{tt}\beta^i = \ldots$
- These gauge conditions cause the coordinates to "collapse" inside the horizon, keeping all physical information in the exterior

The remarkable fact: with these gauge conditions, the numerical grid can actually *move* the puncture. The coordinates adjust to follow the black holes as they orbit each other. The interior is effectively "sucked away" to a small region that never needs to be accurately resolved.

**Why it works**: The gauge conditions create a large "trumpet" geometry near each black hole — the slice stretches inside the horizon, effectively removing the singularity from the computational domain.

---

## The 2005 Breakthrough

Frans Pretorius (2005) and, independently, the Campanelli-Lousto-Zlochower and Baker-Centrella-Choi-Koppitz-van Meter groups (2005) performed the first successful binary black hole merger simulations. Key results:
- Gravitational waveform from inspiral through merger to ringdown
- Recoil ("kick") velocity of the remnant due to asymmetric GW emission
- Final spin of the merged black hole

The waveforms from numerical relativity are now the "gold standard" for GW data analysis. The LIGO detection of GW150914 (2015) used NR waveforms from the SXS (Simulating eXtreme Spacetimes) and Georgia Tech catalogs to confirm the merger parameters.

**Current capabilities** (2024):
- Binary black holes: hundreds of orbits before merger, mass ratios up to $\sim 1:100$, arbitrary spins
- Binary neutron stars: with full magnetohydrodynamics, microphysical equations of state, neutrino transport
- Core-collapse supernovae: 3D, with neutrino radiative transfer
- Neutron star – black hole mergers: electromagnetic counterpart predictions

---

## Gravitational Waveforms from NR

The gravitational waveform from a binary black hole merger has three phases:

**Inspiral**: The two black holes orbit, emitting GWs (post-Newtonian calculation provides the waveform; NR needed for last $\sim 10$ orbits). The frequency and amplitude increase (chirp). Waveform accurately described by post-Newtonian theory until $r \sim 10$ $GM/c^2$.

**Merger**: The horizons touch and merge into a single apparent horizon. NR is essential here. Duration $\sim GM/c^3$ (a few ms for stellar-mass black holes, hours for supermassive BHs). Waveform reaches peak amplitude.

**Ringdown**: The remnant Kerr black hole radiates as it settles down. Waveform is a sum of quasinormal modes (exponentially damped sinusoids). Dominant mode: $l = m = 2$ QNM frequency and damping time.

The waveform accuracy required for GW detection: $\lesssim 10^{-2}$ radian phase error over hundreds of orbits. Achieving this with NR alone requires $\sim 100$ orbits × computational cost; in practice, NR waveforms are matched to PN/EOB (effective-one-body) approximations to build hybrid templates.

---

## Numerical Neutron Stars and GRMHD

Simulating neutron stars requires coupling GR to:
- **MHD**: Magnetohydrodynamics in curved spacetime (GRMHD); the magnetic field is evolved alongside the metric
- **Equation of state**: Relating pressure to density at nuclear densities (poorly constrained; tidal deformability from GW170817 provides constraints)
- **Neutrino transport**: Crucial for the supernova explosion mechanism and r-process nucleosynthesis

**Key results from NR+GRMHD**:
- GW170817 tidal deformability: NR predicts the GW waveform modification from tidal coupling; comparing to observation constrains $\Lambda\tilde \sim 300$–$800$ at 90% confidence
- Neutron star radius: $R_{\rm NS} = 11$–$13$ km inferred from tidal deformability
- Post-merger oscillation frequencies: $\sim 2$–$4$ kHz (potentially detectable with next-generation detectors)
- Kilonova (r-process nucleosynthesis): NR predicts ejecta mass and velocity from NS-NS merger; AT2017gfo confirmed

---

## Important Concepts

- **$3+1$ decomposition (ADM)**: Spacetime foliated by $\Sigma_t$; metric decomposed as $\alpha$, $\beta^i$, $\gamma_{ij}$
- **Lapse and shift**: Gauge functions controlling time slicing and coordinate labeling; not determined by physics
- **Constraint equations**: Hamiltonian + momentum constraints; must be satisfied on each slice
- **Evolution equations**: Determine $\partial_t\gamma_{ij}$, $\partial_t K_{ij}$; propagate constraints
- **Weak hyperbolicity**: ADM equations not well-posed; numerical instabilities
- **BSSN formulation**: Conformal decomposition + auxiliary variables; strongly hyperbolic; stable
- **Moving puncture method**: Handle BH singularities via gauge; puncture "moves" on the grid
- **2005 breakthrough**: First BBH merger simulations; Pretorius, Campanelli et al., Baker et al.
- **Waveform accuracy**: NR matched to PN/EOB for long inspirals; $\lesssim 10^{-2}$ phase error needed
- **GRMHD**: Coupled GR + MHD for neutron stars; essential for multimessenger predictions

---

## Important Figures

**Frans Pretorius** (1973–): First successful binary black hole merger simulation (2005); also co-developed the generalized harmonic formulation of GR.

**Manuela Campanelli**, **Carlos Lousto**, **Yosef Zlochower**: Moving puncture breakthrough (2005); gravitational recoil of merged black holes; SXS-related work.

**Saul Teukolsky** (1947–): Pioneered the formulation of perturbation theory (Teukolsky equation, 1973); SXS collaboration; key contributor to GW template construction.

**Thomas Baumgarte** (1967–) and **Stuart Shapiro** (1947–): Developed the BSS (now BSSN) formulation (1999); textbook on numerical relativity.

**Masaru Shibata** (1966–): Developed the BSSN formulation independently; leading pioneer in neutron star and GRMHD simulations.

---

## Further Reading

**Primary Sources**
- Pretorius, F. (2005). "Evolution of Binary Black-Hole Spacetimes." *Phys. Rev. Lett.*, 95, 121101.
- Campanelli, M. et al. (2005). "Accurate Evolutions of Orbiting Black-Hole Binaries Without Excision." *Phys. Rev. Lett.*, 96, 111101.
- Arnowitt, R., Deser, S., & Misner, C.W. (1959). "Dynamical Structure and Definition of Energy in General Relativity." *Phys. Rev.*, 116, 1322.

**Textbooks**
- Baumgarte, T.W. & Shapiro, S.L. (2010). *Numerical Relativity: Solving Einstein's Equations on the Computer*. Cambridge. — The standard graduate reference.
- Alcubierre, M. (2008). *Introduction to 3+1 Numerical Relativity*. Oxford. — Detailed and pedagogical.
- Shibata, M. (2015). *Numerical Relativity*. World Scientific. — Emphasizes neutron stars and GRMHD.

---

## Exercises

**61.1.** *The $3+1$ split.*

(a) The 4D metric $g_{\mu\nu}$ in ADM form can be written as the $4\times 4$ matrix $\begin{pmatrix}-\alpha^2+\beta_i\beta^i & \beta_i \\ \beta_j & \gamma_{ij}\end{pmatrix}$. Compute the inverse metric $g^{\mu\nu}$ in terms of $\alpha, \beta^i, \gamma^{ij}$.

(b) The unit normal to $\Sigma_t$: $n_\mu = (-\alpha, 0, 0, 0)$ (in the adapted coordinates). Verify $n^\mu n_\mu = -1$ and $n^\mu\partial_i = 0$.

(c) The projection tensor $\gamma_{\mu\nu} = g_{\mu\nu} + n_\mu n_\nu$ projects onto $\Sigma_t$. Show that $\gamma_{\mu\nu}n^\mu = 0$.

---

**61.2.** *Constraint equations.*

(a) For a flat initial data ($\gamma_{ij} = \delta_{ij}$, $K_{ij} = 0$): verify that the Hamiltonian and momentum constraints require $\rho_{\rm ADM} = j^i_{\rm ADM} = 0$. This is just Minkowski spacetime — consistent.

(b) For a Schwarzschild black hole in isotropic coordinates ($\gamma_{ij} = \psi^4\delta_{ij}$ with $\psi = 1 + M/(2r)$, $K_{ij} = 0$): show that the Hamiltonian constraint ${}^{(3)}R = 0$ is satisfied if $\nabla^2\psi = 0$ (flat-space Laplacian). Verify $\nabla^2\psi = 0$ away from the puncture.

(c) This is the "Brill-Lindquist" initial data for a single Schwarzschild black hole. How would you generalize this to two black holes at positions $\mathbf{r}_1$, $\mathbf{r}_2$?

---

**61.3.** *Binary black hole waveform.*

A binary black hole with $m_1 = m_2 = 30M_\odot$ merges at luminosity distance $D = 500$ Mpc.

(a) The merger frequency: $f_{\rm merger} \approx c^3/(6\sqrt{6}\pi G M_{\rm total}) \approx 160$ Hz. Compute this for the given masses.

(b) The peak gravitational wave strain: $h \sim G\mu\omega^2 R^2/(Dc^4)$ where $\mu = m_1m_2/(m_1+m_2)$ and $R$ is the orbital separation at merger ($R \sim 6G M_{\rm total}/c^2$). Estimate $h$ and compare to GW150914's $h \sim 10^{-21}$.

(c) The ringdown frequency is the dominant QNM: $f_{\rm QNM} = 0.3737 c^3/(2\pi G M_f)$ with $M_f \approx 0.95(m_1+m_2)$ (accounting for GW losses). Compute $f_{\rm QNM}$ and the ringdown time $\tau = M_f G/(0.0890c^3)$.

---

**Thought Experiment T61.1.** *Numerical experiments as theoretical physics.*

Numerical relativity produces data — numbers, waveforms, spacetime evolutions. In what sense is this "theoretical physics"?

One view: NR is like experimental physics, but with a computer instead of a detector. The universe being simulated is perfectly known (the initial data, the equations), but the output is complex and not analytically tractable.

Another view: NR is a form of exact theory — it solves Einstein's equations without approximation (subject to discretization error). An analytic result and an NR result, if they agree, constitute a complete verification of the theory.

But NR also *discovers* things: the gravitational recoil of a merged black hole was not known analytically before NR computed it. The post-merger oscillation frequencies of neutron stars were revealed by NR before any detection. Is discovering an equation's behavior the same as deriving it?

What is the epistemological status of numerical results in physics? How do you falsify a numerical relativity prediction? How does NR interact with analytic GR — do they constrain each other, or are they independent?
