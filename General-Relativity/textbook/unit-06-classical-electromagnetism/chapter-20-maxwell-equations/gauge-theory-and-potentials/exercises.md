# Chapter 18: Exercises

---

## Section 18.1 — Gauge Invariance and the Aharonov-Bohm Effect

**18.1.1.** *Gauge transformations and physical observables.*

Consider the electromagnetic potentials $(\phi, \mathbf{A})$ and a gauge transformation $\mathbf{A}\to\mathbf{A}' = \mathbf{A} + \nabla\chi$, $\phi\to\phi' = \phi - \partial\chi/\partial t$.

(a) Show explicitly that the physical fields $\mathbf{E} = -\nabla\phi - \partial\mathbf{A}/\partial t$ and $\mathbf{B} = \nabla\times\mathbf{A}$ are unchanged by the transformation.

(b) Show that Maxwell's equations are gauge-invariant (i.e., if $(\phi,\mathbf{A})$ satisfies them, so does $(\phi',\mathbf{A}')$).

(c) A particle of charge $q$ in a potential $A^\mu$ has the Lagrangian $L = \frac{1}{2}mv^2 + q\mathbf{v}\cdot\mathbf{A} - q\phi$. Under a gauge transformation, the Lagrangian changes by $L\to L + q\frac{d\chi}{dt}$. Show that adding a total time derivative to the Lagrangian does not change the Euler-Lagrange equations of motion.

(d) The canonical momentum is $\mathbf{p}_{\rm can} = m\mathbf{v} + q\mathbf{A}$. Is the canonical momentum gauge-invariant? What about the mechanical momentum $\mathbf{p}_{\rm mech} = m\mathbf{v}$? Which is a physical observable?

---

**18.1.2.** *The Aharonov-Bohm effect.*

A long solenoid of radius $R$ carries flux $\Phi_B = \pi R^2 B_0$. Outside the solenoid ($r > R$), $\mathbf{B} = 0$ but the vector potential is $\mathbf{A} = \frac{\Phi_B}{2\pi r}\hat{\phi}$ (in cylindrical coordinates).

(a) Verify that $\mathbf{B} = \nabla\times\mathbf{A} = 0$ for $r > R$.

(b) An electron travels from the source to a screen, with the solenoid between the two paths (one above, one below the solenoid). The phase accumulated by the electron wavefunction along a path $C$ is:
$$\Delta\phi = \frac{e}{\hbar}\oint_C \mathbf{A}\cdot d\mathbf{l}$$
Compute the phase difference between the two paths (encircling the solenoid in opposite senses). Show it equals $e\Phi_B/\hbar$.

(c) When $e\Phi_B/\hbar = n\pi$ for integer $n$, what happens to the interference pattern? When the flux is increased continuously from 0 to $h/e$ (one flux quantum), how does the interference pattern shift?

(d) The Aharonov-Bohm effect is a topological effect — the phase depends on the topology of the path (whether it encircles the solenoid) rather than local physics (the field is zero everywhere the electron travels). Explain why this makes the vector potential $\mathbf{A}$ more "physical" than a mere mathematical convenience.

---

**18.1.3.** *Dirac monopoles and charge quantization.*

Paul Dirac (1931) showed that if a magnetic monopole of strength $g$ exists, the vector potential outside it has a string singularity. For the Dirac condition to be consistent, the product of any electric charge $q$ with the monopole strength $g$ must satisfy:
$$\frac{qg}{4\pi} = n\frac{\hbar c}{e} \cdot \frac{e}{2} = \frac{n\hbar c}{2}$$
i.e., $qg = n\hbar c/2$ for integer $n$.

(a) The wave function of an electron orbiting a monopole acquires a phase $e\Phi/\hbar$ where $\Phi = 4\pi g$ is the total flux. For the wavefunction to be single-valued after a $2\pi$ rotation around the monopole, what condition must the phase satisfy?

(b) If even one magnetic monopole exists somewhere in the universe, then all electric charges must be integer multiples of $e/n$ for some $n$. Explain why this is remarkable — electric charge quantization is otherwise unexplained in classical electromagnetism.

(c) Dirac strings are invisible: they produce no physically observable effects (no forces, no scattering). Why? The key is that the vector potential along the string can be removed by a gauge transformation — but the transformation is singular (ill-defined) on the string itself. What does this tell you about gauge theory and topology?

(d) In grand unified theories (GUTs), magnetic monopoles arise as topological solitons — finite-energy solutions of the field equations that are topologically nontrivial. The 't Hooft-Polyakov monopole (1974) automatically satisfies the Dirac condition. What does this suggest about the origin of charge quantization?

---

## Section 18.2 — The 4-Potential and Covariant Formulation

**18.2.1.** *Lorenz gauge and electromagnetic waves.*

In Lorenz gauge $\partial_\mu A^\mu = \partial_\nu A^\nu = 0$, Maxwell's equations reduce to $\Box A^\mu = \mu_0 J^\mu$.

(a) Show that Lorenz gauge is a valid gauge choice — i.e., starting from any potential $A^\mu$, a gauge transformation $A^\mu \to A^\mu + \partial^\mu\chi$ with $\Box\chi = -\partial_\mu A^\mu$ brings us to Lorenz gauge.

(b) In Lorenz gauge with no sources ($J^\mu = 0$), the wave equation $\Box A^\mu = 0$ has plane wave solutions $A^\mu = \varepsilon^\mu e^{ik_\mu x^\mu}$ where $k^\mu k_\mu = 0$ (null 4-wavevector). How many polarization states does the 4-potential have? How many are physical (transverse) photon polarizations?

(c) Residual gauge freedom: after imposing Lorenz gauge, there is still the freedom $A^\mu\to A^\mu + \partial^\mu\chi$ with $\Box\chi = 0$. Show that this freedom can be used to impose additional conditions. For a plane wave, it can be used to set $A^0 = 0$ and $\mathbf{k}\cdot\mathbf{A} = 0$ (transverse gauge). How many physical degrees of freedom remain?

(d) Massless spin-1 fields (photons) have 2 physical degrees of freedom. Massive spin-1 fields (W bosons, Z bosons) have 3. Why the difference? What happens to the "extra" degree of freedom when the mass goes to zero?

---

**18.2.2.** *The electromagnetic action and symmetry.*

The action for the electromagnetic field is:
$$S[A] = \int\left(-\frac{1}{4\mu_0}F_{\mu\nu}F^{\mu\nu} + A_\mu J^\mu\right)\sqrt{-g}\,d^4x$$

(a) Vary $S$ with respect to $A_\nu$ to derive the Euler-Lagrange equations. Show they are $\partial_\mu F^{\mu\nu} = \mu_0 J^\nu$ (Maxwell's inhomogeneous equations). The homogeneous equations $\partial_{[\rho}F_{\mu\nu]} = 0$ are automatic since $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$.

(b) The action is invariant under the global phase transformation $\psi\to e^{i\alpha}\psi$ of the charged matter field (where $\psi$ is any charged field). By Noether's theorem, what conserved current does this symmetry generate?

(c) Promoting the global symmetry to a local symmetry $\psi\to e^{i\alpha(x)}\psi$ requires introducing the gauge field $A_\mu$ with the covariant derivative $D_\mu = \partial_\mu - iqA_\mu/\hbar$. Show that $D_\mu\psi \to e^{i\alpha}D_\mu\psi$ under local gauge transformations if $A_\mu\to A_\mu + \hbar\partial_\mu\alpha/q$.

(d) In Yang-Mills theory, the gauge group is non-Abelian (e.g., SU(2) for weak interactions, SU(3) for strong). The field strength becomes $F^a_{\mu\nu} = \partial_\mu A^a_\nu - \partial_\nu A^a_\mu + gf^{abc}A^b_\mu A^c_\nu$ where $f^{abc}$ are structure constants. The extra term makes the field strength nonlinear in $A$. What is the analog in GR? (Hint: compare to the Riemann tensor formula.)

---

**18.2.3.** *General relativity as gauge theory.*

The analogy between GR and gauge theory can be made precise:

| Electromagnetism | GR |
|---|---|
| Gauge group: U(1) | Local Lorentz group: SO(3,1) |
| Gauge field: $A_\mu$ | Spin connection: $\omega^\mu_{\ ab}$ |
| Field strength: $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$ | Curvature: $R^\mu_{\ \nu ab}$ |
| Covariant derivative: $D_\mu = \partial_\mu - iqA_\mu$ | $\nabla_\mu = \partial_\mu + \omega_\mu$ |
| Bianchi identity: $dF = 0$ | Bianchi identity: $dR + \omega\wedge R = R\wedge\omega$ |

(a) In the vielbein formalism, the metric is $g_{\mu\nu} = \eta_{ab}e^a_\mu e^b_\nu$ where $e^a_\mu$ are the vielbein (tetrad) fields. The connection 1-form $\omega^a_{\ b} = \omega^a_{\ b\mu}dx^\mu$ is the gauge field for local Lorentz transformations. Under a local Lorentz transformation $\Lambda^a_{\ b}(x)$: $e^a_\mu\to\Lambda^a_{\ b}e^b_\mu$ and $\omega^a_{\ b\mu}\to\Lambda^a_{\ c}\omega^c_{\ d\mu}(\Lambda^{-1})^d_{\ b} + \Lambda^a_{\ c}\partial_\mu(\Lambda^{-1})^c_{\ b}$. Compare this to the transformation of $A_\mu\to A_\mu + \partial_\mu\chi$.

(b) The Cartan structure equations are $T^a = de^a + \omega^a_{\ b}\wedge e^b$ (torsion) and $R^a_{\ b} = d\omega^a_{\ b} + \omega^a_{\ c}\wedge\omega^c_{\ b}$ (curvature). Compare to $F = dA$ in electromagnetism. What is the extra $\omega\wedge\omega$ term, and why does it not appear in EM?

(c) The Einstein-Hilbert action in terms of vielbeins is $S = \frac{1}{16\pi G}\int\epsilon_{abcd}R^{ab}\wedge e^c\wedge e^d$. Compare to the Yang-Mills action $S = -\frac{1}{4g^2}\int\text{tr}(F\wedge\star F)$. What is the key difference that makes GR non-renormalizable in perturbation theory?

---

## Thought Experiments

**T18.1.** *What is the physical content of the electromagnetic potential?*

In classical electromagnetism, the potential $(\phi, \mathbf{A})$ is considered a mathematical convenience — only the fields $\mathbf{E}$ and $\mathbf{B}$ are "real." The Aharonov-Bohm effect (1959) challenged this view: a charged particle can detect the presence of a solenoid through a region where $\mathbf{E} = \mathbf{B} = 0$. The potential $\mathbf{A}$ is nonzero there.

There are two ways to interpret this:
1. The potential is real — it has physical effects even where fields vanish.
2. The physical quantity is the *holonomy* $\exp(ie\oint A_\mu dx^\mu/\hbar)$ — a gauge-invariant quantity that encodes the topology of the field configuration.

These interpretations give the same predictions but different pictures. In interpretation (2), the Aharonov-Bohm phase is a global property of the field configuration, not a local one. This is the modern understanding: gauge theories are fundamentally about holonomies. Apply this to GR: what is the "holonomy" of the gravitational field, and what physical effect does it correspond to?

---

**T18.2.** *Gauge invariance and the principle of equivalence.*

The electromagnetic principle of minimal coupling says: in curved spacetime, replace $\partial_\mu\to\nabla_\mu = \partial_\mu + \Gamma$ and $A_\mu$ sources add $-iqA_\mu/\hbar$. Together: $D_\mu = \nabla_\mu - iqA_\mu/\hbar$. This is forced by the requirement that both gravity (through $\Gamma$) and electromagnetism (through $A_\mu$) act on the same footing — both are connections on a fiber bundle.

Now consider: the equivalence principle says gravity can be "gauged away" locally (by going to a freely-falling frame). Can the electromagnetic force be "gauged away" locally? If not, what is the essential difference between gravity and electromagnetism at the level of gauge theory?

---

## Laboratory Exercise: Direct Observation of Gauge Invariance

**L18.1.** *The Aharonov-Bohm effect with electron microscopy.*

The Aharonov-Bohm effect has been directly observed in electron microscopes using magnetized iron whiskers (Tonomura et al., 1986, *Physical Review Letters*, 56, 792–795). The experiment is a landmark: it demonstrated the reality of the vector potential in quantum mechanics.

**Conceptual setup:** A field-emission electron gun sends a coherent electron beam toward a biprism. The biprism splits the beam into two paths passing on either side of a magnetized iron toroid (a donut-shaped magnet). The magnetic field is entirely confined inside the toroid; outside, $\mathbf{B} = 0$. An electron interferogram forms on the detector screen.

**Task 1:** Read the abstract and figure captions of Tonomura et al. (1986). The observed interference shift was $\Delta x = e\Phi_B/(h \cdot$ fringe-spacing), where $\Phi_B$ is the magnetic flux through the toroid. What flux quantum corresponds to one full fringe shift?

**Task 2:** The magnetic flux quantum is $\Phi_0 = h/e \approx 4.14\times 10^{-15}$ Wb. An iron toroid of cross-sectional area $A = 10^{-14}$ m$^2$ with $B = 0.1$ T encloses flux $\Phi_B = 10^{-15}$ Wb $\approx 0.24\Phi_0$. Calculate the expected fringe shift.

**Task 3:** The Tonomura experiment also used a superconducting ring, where flux is *quantized* in units of $\Phi_0 = h/(2e)$ (Cooper pairs have charge $2e$). Why does this give flux quantization in units half as large as the single-electron case?

