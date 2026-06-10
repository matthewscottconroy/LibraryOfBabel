# Chapter 57: Hawking Radiation

---

## Chapter Introduction

In 1974, Stephen Hawking made one of the most surprising theoretical discoveries of the twentieth century: black holes are not entirely black. They emit thermal radiation at a temperature inversely proportional to their mass:
$$T_H = \frac{\hbar c^3}{8\pi G M k_B} \approx \frac{6.17\times 10^{-8}\text{ K}}{M/M_\odot}$$

For a stellar-mass black hole ($M = 10 M_\odot$): $T_H \approx 6\times 10^{-9}$ K — far below the CMB temperature of 2.73 K, completely undetectable. For a primordial black hole of $M = 10^{11}$ kg (the mass of a small asteroid): $T_H \approx 10^{11}$ K — explosive evaporation.

Hawking arrived at this result by computing quantum field theory in the Schwarzschild background spacetime. The key: the definition of "vacuum" (no particles) depends on the choice of time coordinate, and the asymptotic observers at infinity (who use Schwarzschild time) disagree with infalling observers about the particle content of the vacuum. The mismatch is precisely a Planck distribution at temperature $T_H$.

The Hawking effect has profound implications:
1. **Black holes evaporate**: By Stefan-Boltzmann, the power radiated is $P \propto T^4 A \propto T^4 M^2 \propto M^{-2}$. The black hole loses mass, which raises $T_H$, which raises the power, which accelerates mass loss — a runaway. The lifetime of a black hole of mass $M$: $\tau_{\rm ev} = 5120\pi G^2 M^3/(\hbar c^4) \approx 2\times 10^{71}(M/M_\odot)^3$ years.

2. **The information paradox**: Hawking radiation is precisely thermal — it carries no information about what fell into the black hole. When the black hole evaporates completely, the pure quantum state of the infalling matter seems to have been converted to a mixed thermal state. This violates unitarity — the fundamental principle of quantum mechanics. The resolution of the black hole information paradox is one of the central unsolved problems in theoretical physics.

3. **The holographic principle**: Bekenstein-Hawking entropy $S = k_B A/(4\ell_P^2)$ scales as area, not volume. This is wildly different from ordinary thermodynamics, where entropy scales with volume. It suggests that the information content of a region of space is encoded on its boundary — a principle called holography, realized concretely in the AdS/CFT correspondence.

---

## Setting Up the Calculation

Hawking's calculation uses the **Bogoliubov transformation** — the standard QFT technique for relating different quantizations of the same field. We work with a massless scalar field $\phi$ in Schwarzschild spacetime.

**Two natural bases.** There are two natural complete sets of mode solutions to $\Box\phi = 0$ in Schwarzschild:

1. **"In" modes** $\{u^{\rm in}_\omega, \bar{u}^{\rm in}_\omega\}$: positive frequency with respect to an affine parameter on the past null infinity $\mathscr{I}^-$ (or past horizon $\mathcal{H}^-$). These are the natural modes for an observer who prepared the quantum state in the far past, before the black hole formed.

2. **"Out" modes** $\{u^{\rm out}_\omega, \bar{u}^{\rm out}_\omega\}$: positive frequency with respect to the Killing vector $\partial_t$ for asymptotic observers at future null infinity $\mathscr{I}^+$.

The "in" vacuum $|0_{\rm in}\rangle$ is the state prepared by observers in the far past (before the black hole collapsed). The question is: what does this state look like to observers at $\mathscr{I}^+$, who use the "out" modes?

**Bogoliubov transformation.** The two mode bases are related by:
$$u^{\rm in}_\omega = \int_0^\infty\left(\alpha_{\omega\omega'}u^{\rm out}_{\omega'} + \beta_{\omega\omega'}\bar{u}^{\rm out}_{\omega'}\right)d\omega'$$

The Bogoliubov coefficients $\alpha_{\omega\omega'}$ and $\beta_{\omega\omega'}$ relate the two quantizations. The expected number of "out" particles in the in-vacuum is:
$$\langle 0_{\rm in}|\hat{N}_\omega^{\rm out}|0_{\rm in}\rangle = \int_0^\infty|\beta_{\omega\omega'}|^2 d\omega'$$

---

## The Hawking Calculation

The key to Hawking's calculation is tracing the mode functions backward in time through the gravitational collapse. A wavepacket at late retarded time $u$ on $\mathscr{I}^+$ can be traced back: through the exterior Schwarzschild spacetime, through the collapsing matter, to $\mathscr{I}^-$ (before the black hole formed).

The transformation between Schwarzschild time $t$ and the advanced time $v$ of an infalling ray creates a non-trivial phase relationship. For late-time modes at $\mathscr{I}^+$, the mapping from future to past null infinity involves the **Kruskal relation** between the retarded and advanced null coordinates near the horizon:
$$U \sim -e^{-u\kappa}$$

where $\kappa = c^4/(4GM)$ is the surface gravity and $U$ is the Kruskal null coordinate. This exponential relation is the source of the Hawking effect.

**Computing $\beta_{\omega\omega'}$**: Substituting the mode functions and computing the inner products:
$$|\beta_{\omega\omega'}|^2 \sim \frac{1}{e^{2\pi\omega/\kappa} - 1}\times|\alpha_{\omega\omega'}|^2$$

(This is a simplified version; the actual calculation involves the saddle-point approximation for the mode-tracing integral.)

The spectrum at $\mathscr{I}^+$:
$$\langle \hat{N}_\omega\rangle = \frac{\Gamma(\omega)}{e^{2\pi\omega/\kappa} - 1}$$

where $\Gamma(\omega)$ is the **greybody factor** — the transmission probability for a mode of frequency $\omega$ to escape from the black hole (Schwarzschild potential barrier). This is exactly a **Planck spectrum** at temperature:
$$T_H = \frac{\hbar\kappa}{2\pi k_B c} = \frac{\hbar c^3}{8\pi G M k_B}$$

where $\kappa = c^4/(4GM)$ is the surface gravity of the Schwarzschild black hole.

---

## The Surface Gravity and Planck Spectrum

The **surface gravity** $\kappa$ of a stationary black hole is defined by the relation:
$$\xi^\mu\nabla_\mu\xi^\nu = \kappa\xi^\nu$$

evaluated on the horizon, where $\xi^\mu$ is the Killing vector that becomes null on the horizon. For Schwarzschild: $\xi^\mu = (\partial_t)^\mu$, and $\kappa = c^4/(4GM)$.

The Hawking temperature:
$$T_H = \frac{\hbar\kappa}{2\pi k_B c}$$

For Kerr-Newman (rotating, charged) black holes:
$$\kappa = \frac{c^4}{G}\frac{\sqrt{M^2 - Q^2/G - a^2}}{2M r_+ - Q^2/G}$$

where $r_+ = GM/c^2 + \sqrt{G^2 M^2/c^4 - G Q^2/(c^4) - a^2/c^2}$ is the outer horizon and $a = J/(Mc)$ is the specific angular momentum. The surface gravity vanishes for extremal black holes ($M^2 = Q^2/G + a^2$), which have $T_H = 0$ — consistent with the third law of black hole thermodynamics.

**Greybody factors.** A photon created near the horizon must climb out of the Schwarzschild gravitational potential. The transmission probability $\Gamma(\omega)$ deviates from 1, modifying the Planck spectrum. For $\omega M \gg 1$ (high frequencies): $\Gamma \to 1$. For $\omega M \ll 1$ (low frequencies, below the "photon sphere" barrier): $\Gamma \to 0$. The Hawking spectrum is not a perfect blackbody but is the Planck spectrum multiplied by $\Gamma(\omega)$.

---

## Black Hole Evaporation

By Stefan-Boltzmann, the power radiated by a body at temperature $T$ with area $A$ is $P = \sigma_{\rm SB}T^4 A$. For a black hole with horizon area $A = 16\pi G^2 M^2/c^4$ and temperature $T_H$:
$$P = \frac{\hbar c^6}{15360\pi G^2 M^2}$$

(including all particle species; the exact coefficient depends on the particle content). The mass-loss rate:
$$\frac{dM}{dt} = -\frac{P}{c^2} = -\frac{\hbar c^4}{15360\pi G^2 M^2}$$

Integrating:
$$M(t)^3 = M_0^3 - \frac{\hbar c^4}{5120\pi G^2}t$$

The **evaporation time**:
$$\tau_{\rm ev} = \frac{5120\pi G^2}{\hbar c^4}M_0^3 \approx 2.1\times 10^{67}\text{ yr}\left(\frac{M}{M_\odot}\right)^3$$

For a $10 M_\odot$ black hole: $\tau_{\rm ev} \approx 2\times 10^{70}$ yr — vastly longer than the current age of the universe ($1.4\times 10^{10}$ yr). Hawking radiation from stellar black holes is entirely negligible.

For a primordial black hole with initial mass $M_0 \sim 5\times 10^{14}$ g:
$$\tau_{\rm ev} \approx 13.8\text{ Gyr}$$

Primordial black holes of this mass would be evaporating now, producing a distinctive burst of gamma rays. Searches for such bursts place upper bounds on the primordial black hole abundance.

**Final stages**: As $M\to 0$, $T_H\to\infty$ and the power $P \propto M^{-2}\to\infty$. The final evaporation is explosive — Planck-scale physics is required and the calculation breaks down. The final fate of the evaporating black hole is not known.

---

## The Information Paradox

The most profound consequence of Hawking radiation: it appears to be purely thermal, carrying no information about the quantum state of matter that formed or fell into the black hole. If the black hole evaporates completely, the initial pure quantum state $|\Psi_{\rm initial}\rangle$ is mapped to a mixed thermal density matrix $\rho_{\rm final} = \sum_n p_n|n\rangle\langle n|$. This violates **unitary evolution** — one of the foundational principles of quantum mechanics.

This is the **black hole information paradox** (Hawking 1975). Possible resolutions:

**Information is encoded in correlations**: The Hawking radiation is not exactly thermal; subtle quantum correlations encode the information, only becoming apparent at late times. This is the viewpoint of most quantum information theorists and string theorists. But how? The state is highly entangled and the information is scrambled — recovering it requires a complete quantum-gravitational calculation.

**Black hole remnants**: The evaporation stops at the Planck scale, leaving a stable remnant of Planck mass that stores all the information. But remnants would have to store unbounded amounts of information in a Planck-scale object, creating other problems.

**Information is lost**: Hawking himself originally believed information was genuinely lost, requiring a modification of quantum mechanics. He later changed his mind (2004), conceding a bet with Preskill and acknowledging that black holes preserve information.

**Page time and Page curve**: Page (1993) showed that in a unitary theory, the entanglement entropy of the Hawking radiation first increases to a maximum ("Page time"), then decreases back to zero — tracing a characteristic curve. The information is released at late times. Recent work using the "island formula" from quantum extremal surfaces (Penington 2019, Almheiri-Engelhardt-Marolf-Maxfield 2019) has computed this Page curve in simplified models, suggesting unitarity is preserved, but the mechanism in 4D GR remains unclear.

**The firewall paradox**: Almheiri, Marolf, Polchinski, and Sully (2012) argued that if late Hawking radiation is purified by early radiation (unitarity), and the early and late radiation are entangled (quantum mechanics), then the equivalence principle must fail at the horizon — infalling observers would hit a "firewall" of high-energy radiation rather than passing smoothly through the horizon. This either breaks GR (equivalence principle fails) or breaks quantum mechanics (unitarity fails). The firewall paradox remains unresolved.

---

## Important Figures

**Stephen Hawking (1942–2018)**: Derived Hawking radiation in 1974, connecting black holes to thermodynamics and quantum mechanics in a single result. Also proved the area theorem (classical), the Penrose-Hawking singularity theorems, and wrote the foundational papers on the information paradox. His popular book *A Brief History of Time* (1988) brought cosmology to a general audience.

**Jacob Bekenstein (1947–2015)**: Proposed that black holes have entropy proportional to their horizon area before Hawking's calculation confirmed it (1972). Bekenstein was a graduate student at the time; Wheeler (his advisor) had challenged him on the thermodynamic implications of black holes. Also developed the "generalized second law" $\delta(S_{\rm matter} + S_{\rm BH}) \geq 0$.

**William Unruh (born 1945)**: Derived the Unruh effect and the interpretation of Hawking radiation as a consequence of the observer-dependence of the vacuum (1976). Also showed that the Hawking effect is connected to the thermal Green's function in the Euclidean Schwarzschild geometry.

**Gary Gibbons (born 1946) and Stephen Hawking (1942–2018)**: Derived the Hawking effect using the Euclidean path integral (1977), providing an elegant and coordinate-independent derivation.

---

## Exercises

**57.1.** *Hawking temperature and surface gravity.*

(a) For Schwarzschild, verify that the surface gravity is $\kappa = c^4/(4GM)$ using $\xi^\mu\nabla_\mu\xi^\nu = \kappa\xi^\nu$ where $\xi^\mu = (\partial_t)^\mu$.

(b) Compute the Hawking temperature for: (i) a $1 M_\odot$ black hole, (ii) a $10^6 M_\odot$ SMBH, (iii) a primordial black hole of $M = 10^{12}$ kg.

(c) For which masses does $T_H$ exceed the CMB temperature $T_{\rm CMB} = 2.73$ K? Compute the critical mass.

---

**57.2.** *Evaporation time and lifetime.*

(a) Compute the evaporation time $\tau_{\rm ev} = 5120\pi G^2 M^3/(\hbar c^4)$ for: (i) a $10 M_\odot$ BH, (ii) a $10^6 M_\odot$ BH, (iii) a primordial BH of mass $M = 5\times 10^{14}$ g.

(b) Solve for the initial mass of a black hole that would be evaporating today (completing its evaporation in $t_0 = 13.8$ Gyr). This is the "critical mass" for primordial black hole evaporation.

(c) As the black hole evaporates, its temperature increases. Show that the luminosity scales as $L \propto M^{-2}$ and the evaporation is self-accelerating. Sketch $M(t)$ from $M_0$ to 0.

---

**57.3.** *The information paradox and the Page curve.*

(a) Define the **entanglement entropy** of the Hawking radiation: at early times (small fraction of mass evaporated), why does the entanglement entropy of the radiation increase?

(b) At late times (most of the mass has evaporated), if the process is unitary, the entanglement entropy must decrease. Draw the schematic "Page curve" $S_{\rm rad}(t)$, marking the Page time $t_{\rm Page} \approx \tau_{\rm ev}/2$.

(c) The "island formula" for the entanglement entropy of Hawking radiation in 2D dilaton gravity is:
$$S_{\rm rad} = \min\left\{S_{\rm standard},\ S_{\rm island}\right\}$$
Without deriving this, explain qualitatively why the "island" — a region behind the horizon that must be included in the radiation's entropy — would reduce the entropy at late times and recover the Page curve.

---

**Thought Experiment T57.1.** *The information paradox: physics or mathematics?*

The information paradox arises from combining four principles: (1) Hawking radiation is thermal (QFT in curved spacetime), (2) unitarity of quantum mechanics, (3) equivalence principle (smooth horizon crossing), (4) effective field theory below the Planck scale.

Any resolution must abandon at least one of these. Which would you prefer to give up, and why? Consider:
- Abandoning unitarity modifies quantum mechanics at a fundamental level
- Abandoning the equivalence principle (firewall) means GR fails at horizons
- Abandoning EFT validity means Planck-scale physics affects macroscopic horizons ("fuzzball" solutions in string theory)
- Arguing Hawking radiation is not exactly thermal means non-perturbative corrections are important even far from the Planck scale

Is there an observational or experimental consequence of any of these choices?

**Thought Experiment T57.2.** *What would Hawking radiation "look like"?*

For a stellar-mass black hole, the Hawking temperature is $\sim 10^{-8}$ K — far below the CMB. Is Hawking radiation in principle detectable, or only computable?

Consider: (1) primordial black holes near the end of their lifetime, (2) analogue gravity experiments (sonic black holes in Bose-Einstein condensates, Weinfurtner et al. 2011), (3) laser-induced Schwinger pair production (analogous mechanism).

What evidence would count as "detecting" Hawking radiation? Is there a meaningful distinction between detecting the effect in a laboratory analogue and detecting it from an actual black hole?
