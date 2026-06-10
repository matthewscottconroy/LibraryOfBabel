# Chapter 63: Approaches to Quantum Gravity

---

## Chapter Introduction

The two pillars of twentieth-century physics — general relativity and quantum mechanics — are mutually incompatible. GR describes a smooth, continuous, dynamical spacetime. Quantum mechanics requires discrete outcomes, superpositions, and a fixed background. Combining them into a single consistent theory is one of the central unsolved problems in physics.

The obstacles are both conceptual and technical. Conceptually: in GR, spacetime geometry is dynamical; in quantum mechanics, spacetime is a fixed background. In GR, there is no preferred time; in quantum mechanics, time is an external parameter that labels the evolution of the wave function. In GR, the theory is non-renormalizable by naive power counting — quantum corrections grow without bound at the Planck scale.

At the Planck scale ($\ell_P = \sqrt{\hbar G/c^3} \approx 10^{-35}$ m, $m_P = \sqrt{\hbar c/G} \approx 10^{19}$ GeV), these incompatibilities become unavoidable. Spacetime fluctuations would be of order the curvature scale itself; the notion of a smooth metric breaks down; the singularity theorems suggest breakdown of classical GR.

Two leading approaches have emerged:

**String theory**: Replaces point particles with 1-dimensional strings, whose vibrations give rise to all particles including the graviton. Requires 10 spacetime dimensions (superstring theory) or 11 (M-theory). Contains GR as a low-energy limit. UV-finite. The AdS/CFT correspondence provides a non-perturbative definition of quantum gravity in Anti-de Sitter backgrounds.

**Loop quantum gravity (LQG)**: Directly quantizes the geometry of GR without a background spacetime. Space is quantized in discrete units at the Planck scale. Spin networks provide a basis for the quantum geometry of space; spin foams provide the covariant path integral. Background-independent, non-perturbative.

Both approaches remain incomplete and have not yet made confirmed, testable predictions beyond classical GR. This chapter surveys their main features, achievements, and open problems.

---

## The Problem of Quantum Gravity: Why It's Hard

**Non-renormalizability**: Perturbative quantum gravity — expanding the metric as $g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}$ and quantizing $h_{\mu\nu}$ as a spin-2 field — works at low energies. But loop corrections generate an infinite tower of new terms in the Lagrangian at each loop order:
$$\mathcal{L} = \frac{M_{\rm Pl}^2}{2}R + c_1 R^2 + c_2 R_{\mu\nu}R^{\mu\nu} + \frac{c_3}{M_{\rm Pl}^2}R^3 + \cdots$$

Each coefficient $c_i$ requires a new measurement to fix — the theory has infinitely many free parameters and loses all predictive power above the Planck scale. This is the non-renormalizability problem.

**Background independence**: GR is diffeomorphism-invariant — there is no preferred coordinate system or background metric. Standard QFT quantizes fields on a fixed background. A quantum theory of gravity must be background-independent: the geometry itself is a quantum operator, not a background.

**The problem of time**: In the canonical quantization of GR (ADM), the Hamiltonian is a constraint $\mathcal{H} = 0$ (rather than a generator of time evolution). Physical states are annihilated by $\hat{\mathcal{H}}$, giving the Wheeler-DeWitt equation $\hat{\mathcal{H}}|\Psi\rangle = 0$ — a time-independent Schrödinger equation with no explicit time variable. How does time and dynamics emerge from a timeless equation?

---

## String Theory

String theory begins with the observation that elementary particles — if they were 1-dimensional strings rather than 0-dimensional points — would automatically give rise to gravity. The modes of a closed string include a spin-2 massless excitation (the graviton) as well as spin-1 and spin-0 modes.

**Key features**:
- **Superstring theory**: 5 consistent superstring theories exist in 10 dimensions (Type I, IIA, IIB, Heterotic SO(32), Heterotic $E_8\times E_8$)
- **M-theory**: 11-dimensional theory that unifies the 5 superstring theories as different limits
- **Compactification**: Extra 6 (or 7) spatial dimensions are compactified on a small manifold (Calabi-Yau space), giving rise to 4D physics
- **UV finiteness**: String amplitudes are UV finite — the non-renormalizability problem of perturbative gravity is resolved
- **Landscape**: There are $\sim 10^{500}$ distinct Calabi-Yau compactifications, each giving a different 4D effective theory — the "string landscape"
- **The Standard Model embedding**: String theory contains GR plus the Standard Model (in principle) but finding the correct compactification that gives exactly our universe is an open problem

**AdS/CFT correspondence** (Maldacena 1997): The most concrete realization of string theory. Type IIB string theory on $\text{AdS}_5\times S^5$ (5D Anti-de Sitter space times a 5-sphere) is exactly equivalent to $\mathcal{N} = 4$ super-Yang-Mills theory in 4D (a conformal field theory on the boundary of AdS). This is a duality: strong coupling in the CFT corresponds to weak coupling (semiclassical) gravity in AdS, and vice versa.

**Implications of AdS/CFT**:
- Black hole thermodynamics in AdS maps to thermal states in the CFT (unitary theory) → information is preserved
- The Page curve for Hawking radiation can be computed in the dual CFT
- Holographic entanglement entropy: the Ryu-Takayanagi formula $S = A_{\rm RT}/(4G\hbar)$ expresses entanglement entropy in the CFT as the area of a minimal surface in the bulk AdS
- Strongly coupled fluids (quark-gluon plasma at RHIC) can be modeled using holographic gravity

---

## Loop Quantum Gravity

LQG directly quantizes general relativity without a background metric. The starting point is the reformulation of GR in terms of the Ashtekar variables (1986): instead of the metric $g_{ij}$ and extrinsic curvature $K_{ij}$, use the Ashtekar-Barbero connection $A_a^i$ and the densitized triad $E_i^a$ (related to the spatial metric by $q^{ab} = E_i^a E_i^b/\det(E)$).

In these variables, GR looks like a gauge theory (Yang-Mills theory with gauge group $SU(2)$). The Wilson loops of $A_a^i$ — holonomies along closed loops — are gauge-invariant observables.

**Spin networks**: The kinematic Hilbert space $\mathcal{H}_{\rm kin}$ is spanned by **spin network states** $|s\rangle$ — graphs embedded in 3-space, with edges labeled by $SU(2)$ representations (half-integers $j_e = 0, 1/2, 1, \ldots$) and nodes labeled by intertwiners (invariant tensors). These are the quantum states of geometry.

**Area operator**: The area of a surface $\Sigma$ is quantized:
$$\hat{A}[\Sigma]|s\rangle = 8\pi\gamma\ell_P^2\sum_{e\cap\Sigma}\sqrt{j_e(j_e+1)}|s\rangle$$

where $\gamma$ is the Barbero-Immirzi parameter ($\gamma \approx 0.274$ from BH entropy calculations) and the sum is over edges $e$ of the spin network that pierce $\Sigma$. The minimum non-zero eigenvalue is $4\pi\sqrt{3}\gamma\ell_P^2 \approx 10^{-69}$ m² — an area eigenvalue near the Planck scale.

**Spin foams**: The covariant path integral formulation of LQG. 4D spacetime is described by a "spin foam" — a 2-complex with faces, edges, and vertices colored by $SU(2)$ representations. The transition amplitude from one spin network state to another is:
$$\langle s_f|U|s_i\rangle = \sum_{\rm spin foams} \prod_{\rm faces}d_{j_f}\prod_{\rm vertices} A_v$$

where $d_j = 2j+1$ and $A_v$ is the vertex amplitude (the EPRL or FK model). This is analogous to Feynman path integrals in QFT.

**Black hole entropy in LQG**: Counting spin network states that pierce the horizon gives:
$$S_{\rm BH} = \frac{k_B A}{4\ell_P^2}$$

with $\gamma$ chosen to match the Bekenstein-Hawking entropy. This provides a microstate interpretation of black hole entropy in LQG.

**Loop quantum cosmology**: Applying LQG techniques to homogeneous cosmology, the Big Bang singularity is replaced by a "quantum bounce" at Planck density $\rho_{\rm crit} \approx 0.41\rho_{\rm Pl}$.

---

## Other Approaches

**Causal dynamical triangulations (CDT)**: Path integral over spacetime geometries by summing over triangulated 4-manifolds with Lorentzian signature. Numerical simulations show emergence of a 4D de Sitter-like spacetime at large scales.

**Asymptotic safety**: GR may be non-perturbatively renormalizable — a non-Gaussian UV fixed point of the renormalization group may exist, making GR UV complete without new degrees of freedom.

**Causal set theory**: Spacetime is fundamentally discrete, with events forming a causal set (partially ordered set) whose continuum limit is GR. The number of elements in a region equals its volume in Planck units.

**Emergent gravity**: Perhaps gravity is not fundamental but emergent — like thermodynamics from statistical mechanics. Verlinde (2011) proposed that gravity is an entropic force arising from information changes, related to Jacobson's (1995) derivation of Einstein's equations from thermodynamics of local Rindler horizons.

---

## Testable Predictions?

All approaches to quantum gravity make predictions at the Planck scale, which is $10^{15}$ times the reach of the LHC. Direct experimental tests are currently impossible. However:

**Lorentz invariance violation**: Some quantum gravity models predict small violations of Lorentz invariance at Planck energies. GRB observations (photons from gamma-ray bursts at cosmological distances) constrain $\delta v/c < 6\times 10^{-20}$ for 100 GeV photons (Fermi LAT).

**Quantum gravity in the CMB**: Primordial gravitational waves from inflation (if detected via B-mode polarization) would probe Planck-scale physics during inflation. The tensor-to-scalar ratio $r$ constrains inflationary models.

**Black hole shadow and ringdown**: Very precisely measured ringdown frequencies could in principle show deviations from Kerr if the black hole has quantum corrections to its near-horizon geometry ("fuzzball," "gravastar," etc.).

**Gravitational wave echoes**: Some quantum gravity models predict "echoes" — reflected near-horizon modes that would appear as late-time signals in gravitational wave detectors. No confirmed detection.

**Analogue gravity**: Hawking radiation has been observed in analogue systems (sonic black holes in BECs, Steinhauer 2016). While not direct evidence for quantum gravity in spacetime, it confirms the underlying mechanism.

---

## Important Figures

**John Archibald Wheeler (1911–2008)**: Coined "black hole," "wormhole," "it from bit," and introduced the Wheeler-DeWitt equation. Developed many of the conceptual foundations of quantum gravity and the idea that information is fundamental.

**Roger Penrose (born 1931)**: Developed twistors — a reformulation of GR and quantum mechanics using complex null lines in spacetime, designed to unify the two theories.

**Abhay Ashtekar (born 1949)**: Reformulated GR in terms of connection variables, making quantization tractable. Founded the Ashtekar variables and (with Lewandowski and Rovelli) the loop quantization program.

**Carlo Rovelli (born 1956) and Lee Smolin (born 1955)**: Co-developed loop quantum gravity from the Ashtekar variables, deriving spin networks and the discrete spectra of area and volume operators.

**Juan Maldacena (born 1968)**: Discovered the AdS/CFT correspondence — the most concrete and powerful result in quantum gravity. Also made foundational contributions to the information paradox and the entanglement structure of gravity.

**Edward Witten (born 1951)**: Discovered M-theory unifying the five superstring theories, made foundational contributions to topological field theories, the Jones polynomial, mirror symmetry, and many other areas connecting physics and mathematics. Fields Medal 1990.

---

## Exercises

**63.1.** *Planck units and quantum gravity scale.*

(a) From $c$, $\hbar$, $G$, construct the Planck length $\ell_P$, Planck mass $m_P$, and Planck time $t_P$ by dimensional analysis.

(b) Compute $\ell_P$, $m_P$, $t_P$ in SI units. Compare $m_P$ to the proton mass and $\ell_P$ to the proton radius.

(c) The Schwarzschild radius of a Planck-mass black hole: $r_s = 2Gm_P/c^2$. Compare to $\ell_P$. What does this tell you about why a quantum theory of gravity is needed at the Planck scale?

(d) An accelerating proton at the LHC reaches $7$ TeV. How many orders of magnitude below the Planck energy is this? What center-of-mass energy (in electron-volts) would be needed to probe Planck-scale physics directly?

---

**63.2.** *Discrete area spectrum in LQG.*

(a) In LQG, the area eigenvalues are $A_j = 8\pi\gamma\ell_P^2\sqrt{j(j+1)}$ for $j = 0, 1/2, 1, 3/2, \ldots$. Compute the first five non-zero eigenvalues (in Planck units $\ell_P = 1$) for $\gamma = 0.274$.

(b) The horizon area of a $1 M_\odot$ Schwarzschild black hole is $A = 16\pi G^2 M^2/c^4$. How many Planck area quanta are needed? Is this number consistent with $S_{\rm BH} = k_B A/(4\ell_P^2)$?

(c) If space is discrete at the Planck scale, what happens to the notion of a smooth spacetime manifold at intermediate scales? How does a discrete quantum geometry reduce to the smooth Riemannian geometry of GR?

---

**Thought Experiment T63.1.** *Is quantum gravity observable?*

The Planck scale is $10^{19}$ GeV — $10^{15}$ times the LHC energy. At first sight, quantum gravity seems permanently inaccessible.

But consider: (1) cosmological observations probe the inflationary epoch, possibly near the Planck energy; (2) black hole physics tests the near-Planck regime; (3) Lorentz invariance tests at cosmological baselines probe very small violations; (4) precision tests of the equivalence principle might detect quantum corrections; (5) analogue systems can simulate Hawking radiation.

Which of these is most likely to provide observational evidence for quantum gravity in the next 30 years? What would constitute "proof" of a specific quantum gravity theory (as opposed to just a test of semiclassical gravity)?

**Thought Experiment T63.2.** *What is spacetime made of?*

Wheeler proposed "it from bit" — that physical reality is fundamentally informational. Verlinde proposed gravity is entropic — an emergent force from information content. Maldacena's AdS/CFT shows that bulk spacetime geometry emerges from entanglement in the boundary theory.

If spacetime is emergent rather than fundamental, what are the fundamental degrees of freedom? Qubits? Strings? Causal sets? Spin networks? Is there a sense in which different quantum gravity theories are "the same" in different variables (like wave vs. matrix mechanics in QM)?

What experiment could distinguish between spacetime being fundamental (LQG view) and emergent (holographic view)?
