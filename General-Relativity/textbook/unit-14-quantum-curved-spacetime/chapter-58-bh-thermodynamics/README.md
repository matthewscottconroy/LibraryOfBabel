# Chapter 58: Black Hole Thermodynamics

---

## Chapter Introduction

In 1973, Bardeen, Carter, and Hawking proved four classical theorems about black holes that are formally identical to the four laws of thermodynamics. At the time, the analogy seemed to be merely mathematical — a coincidence of form. One year later, Hawking's derivation of thermal radiation from black holes transformed the analogy into physical identity.

The four laws of black hole thermodynamics:

**Zeroth law**: The surface gravity $\kappa$ is constant over the horizon of a stationary black hole.

*Thermodynamic analogue*: Temperature $T$ is constant throughout a system in thermal equilibrium.

**First law**: For a stationary black hole, perturbations of the mass, angular momentum, and charge satisfy:
$$\delta M = \frac{\kappa}{8\pi G/c^4}\delta A + \Omega_H \delta J + \Phi_H \delta Q$$

where $A$ is the horizon area, $\Omega_H$ is the angular velocity of the horizon, $J$ is the angular momentum, $\Phi_H$ is the electromagnetic potential at the horizon, and $Q$ is the charge.

*Thermodynamic analogue*: $\delta E = T\delta S - p\delta V + \mu\delta N$. The identification is $T \leftrightarrow \kappa c/(8\pi G)$, $S \leftrightarrow k_B A c^3/(4G\hbar)$.

**Second law** (area theorem, Hawking 1971): In classical GR, assuming the null energy condition, the total horizon area never decreases:
$$\frac{dA}{dt} \geq 0$$

*Thermodynamic analogue*: $\delta S \geq 0$ (entropy never decreases). The generalized second law (Bekenstein 1972): the total entropy $S_{\rm total} = S_{\rm matter} + k_B A/(4\ell_P^2)$ never decreases even when Hawking radiation is included.

**Third law**: The surface gravity $\kappa$ cannot be reduced to zero by a finite sequence of physical processes.

*Thermodynamic analogue*: Absolute zero temperature ($T = 0$) is unattainable.

What makes these not merely analogies but physical equalities: once Hawking showed $T = \hbar\kappa/(2\pi k_B c)$, the first law gives the entropy directly:
$$S_{\rm BH} = \frac{k_B c^3 A}{4G\hbar} = \frac{k_B A}{4\ell_P^2}$$

This is the **Bekenstein-Hawking entropy**. It is a genuinely thermodynamic entropy — associated with the information that fell into the black hole and is hidden behind the horizon.

---

## Black Hole Entropy and the Area

For a Schwarzschild black hole ($M$, $J = Q = 0$): horizon area $A = 4\pi r_s^2 = 16\pi G^2 M^2/c^4$, giving:
$$S_{\rm BH} = \frac{4\pi k_B G M^2}{\hbar c} = \frac{k_B M^2}{8\pi M_{\rm Pl}^2}$$

where $M_{\rm Pl} = \sqrt{\hbar c/G} \approx 2.18\times 10^{-8}$ kg is the Planck mass.

For a solar-mass black hole: $S_{\rm BH} \approx k_B \times 10^{77}$. For comparison, the entropy of the Sun is $S_\odot \approx k_B\times 10^{58}$. The black hole has $10^{19}$ times more entropy — and takes up only $(3 \text{ km})^3$ vs. the Sun's $(10^6 \text{ km})^3$.

For a $10^9 M_\odot$ SMBH (like M87*): $S_{\rm BH} \approx k_B\times 10^{95}$. Penrose has argued that the low entropy of the initial Big Bang — necessary for the arrow of time — would require $|\Omega_k| \ll 10^{-123}$ if we imagine the entire observable universe as a classical black hole, requiring extraordinary fine-tuning.

---

## Deriving the Entropy: Euclidean Path Integral

The cleanest derivation of $S_{\rm BH}$ comes from the **Euclidean path integral** approach (Gibbons and Hawking 1977).

The partition function of a gravitational system at inverse temperature $\beta = 1/(k_B T)$ is:
$$Z(\beta) = \int \mathcal{D}g_{\mu\nu}\,e^{-I_E[g]/\hbar}$$

where $I_E$ is the Euclidean action. For the Schwarzschild black hole, the Euclidean metric (obtained by $t \to -i\tau$) is:
$$ds_E^2 = \left(1 - \frac{r_s}{r}\right)c^2 d\tau^2 + \left(1 - \frac{r_s}{r}\right)^{-1}dr^2 + r^2 d\Omega^2$$

Near the horizon, this looks like a cone in polar coordinates. For the metric to be smooth (no conical singularity), the Euclidean time must be periodic with period:
$$\beta = \frac{4\pi r_s}{c} = \frac{8\pi G M}{c^3}$$

This gives temperature $T = \hbar c^3/(8\pi G M k_B)$ — the Hawking temperature — as a purely geometric constraint.

The on-shell Euclidean action:
$$I_E = -\frac{c^4}{16\pi G\hbar}\int R\sqrt{g_E}\,d^4x + I_{\rm boundary} = \frac{\beta M c^2}{2}$$

From $Z = e^{-I_E/\hbar}$, the free energy $F = -k_B T\ln Z = I_E k_B T/\hbar = Mc^2/2$. The entropy:
$$S = -\frac{\partial F}{\partial T} = k_B\frac{\partial}{\partial T}(T\ln Z) = k_B\frac{\partial}{\partial T}\left(-\frac{I_E}{\hbar}\cdot k_B T\right)$$

Working through the computation with the correct normalization:
$$S_{\rm BH} = \frac{k_B A}{4\ell_P^2}$$

---

## The Holographic Principle

The Bekenstein-Hawking entropy scales as area, not volume. For ordinary matter, entropy is extensive: $S \propto V$ (volume). For black holes: $S \propto A = V^{2/3}$ (area). This suggests that the maximum entropy of any region of space is bounded by its boundary area:
$$S \leq \frac{k_B c^3 A}{4G\hbar} = \frac{k_B A}{4\ell_P^2}$$

This is the **Bekenstein bound** or the **holographic entropy bound** (Bousso 1999). The interpretation: the number of degrees of freedom in a region of spacetime is proportional to the area of its boundary, measured in Planck units. The information needed to describe a region is encoded on its boundary.

This leads to the **holographic principle** (t'Hooft 1993, Susskind 1995): there exists a description of quantum gravity where the fundamental degrees of freedom live on a lower-dimensional boundary of space. This is realized concretely in the **AdS/CFT correspondence** (Maldacena 1997).

---

## AdS/CFT and Black Hole Thermodynamics

The Anti-de Sitter/Conformal Field Theory (AdS/CFT) correspondence (Maldacena 1997) states:

**Type IIB string theory on $\text{AdS}_5\times S^5$ $\equiv$ $\mathcal{N} = 4$ super-Yang-Mills theory on $\mathbb{R}^{3,1}$**

The correspondence maps:
- Black holes in AdS $\leftrightarrow$ thermal states in the CFT
- Hawking temperature $\leftrightarrow$ CFT temperature
- Black hole entropy $\leftrightarrow$ thermal entropy of the CFT
- Gravitational dynamics in the bulk $\leftrightarrow$ RG flow in the boundary theory

Most importantly: the CFT is a unitary quantum theory. If the correspondence is exact, then black hole evaporation must be unitary — the information must be preserved. AdS/CFT provides a framework for computing the Page curve (using the "island formula" derived from quantum extremal surfaces) and suggests that the information paradox is resolved in favor of unitarity.

However, AdS/CFT applies to asymptotically anti-de Sitter spacetimes (negative cosmological constant), not to our asymptotically flat or de Sitter universe. The extension of holography to de Sitter and flat spacetimes remains an active research area.

---

## Important Concepts

**Surface gravity**: For a Killing horizon with Killing vector $\xi^\mu$ becoming null, $\xi^\mu\nabla_\mu\xi^\nu = \kappa\xi^\nu$. Physical meaning: the force per unit mass needed at infinity to hold a unit mass stationary at the horizon. For Schwarzschild: $\kappa = c^4/(4GM)$.

**First law for Kerr-Newman**: $\delta M = (\kappa/8\pi)\delta A + \Omega_H\delta J + \Phi_H\delta Q$ (in units $G = c = \hbar = k_B = 1$). This is a first-order relation between changes in the macroscopic parameters.

**Extremal black holes**: When $|a| \to M$ (or $|Q| \to M$ in appropriate units), $\kappa \to 0$ and $T_H \to 0$. Extremal black holes are "cold" — they have zero temperature. The third law says they cannot be reached by a finite process.

**Thermodynamic stability**: A Schwarzschild black hole has negative heat capacity: $C = dM/dT_H = -8\pi k_B G M^2/(\hbar c) < 0$. As the black hole loses energy (radiates), its temperature increases and it radiates faster — a thermal runaway. Schwarzschild black holes are thermodynamically unstable in flat spacetime. In AdS, large black holes have positive heat capacity and are stable.

---

## Exercises

**58.1.** *Computing black hole entropy.*

(a) Compute $S_{\rm BH}$ for a $10 M_\odot$ Schwarzschild black hole. Express as $S/k_B$.

(b) Compare to the entropy of: (i) a cubic meter of air at STP, (ii) the Sun, (iii) a neutron star.

(c) If 3 solar masses were radiated as gravitational waves in GW150914 (producing a $62 M_\odot$ remnant from $65 M_\odot$ initial), compute the change in black hole entropy $\Delta S_{\rm BH}$.

(d) The second law requires $\Delta S_{\rm BH} \geq 0$ (classically). Compute $\Delta A = A_f - A_i$ and verify the area theorem.

---

**58.2.** *The first law of black hole mechanics.*

For a Kerr black hole ($M$, $J = aM$, $Q = 0$):
$$A = \frac{8\pi G M r_+}{c^2}, \quad r_+ = \frac{GM}{c^2} + \sqrt{\left(\frac{GM}{c^2}\right)^2 - a^2}, \quad \Omega_H = \frac{ac}{2Mr_+}$$

(a) Compute $\partial A/\partial M$ at fixed $J$ and $\partial A/\partial J$ at fixed $M$.

(b) Verify the first law $\delta M = (\kappa/8\pi G)\delta A + \Omega_H\delta J$ (in $c = 1$ units) using your results.

(c) For an extremal Kerr black hole ($a = GM/c^2$): compute $\kappa$, $T_H$, $S_{\rm BH}$, $\Omega_H$.

---

**58.3.** *The Penrose process and the second law.*

In the Penrose process, a particle falls into the ergosphere of a Kerr black hole, splits into two, and one piece escapes with more energy than the infalling particle — extracting rotational energy from the black hole.

(a) The maximum energy extraction in the Penrose process is $\delta M_{\rm extr} = \Omega_H\delta J_{\rm absorbed}$. Show this is consistent with the first law if $\delta A = 0$ (irreversible processes have $\delta A > 0$).

(b) The maximum efficiency of energy extraction is $\eta = 1 - r_+/M$ (in natural units). For an extremal Kerr black hole: what is $\eta$? Compare to nuclear fusion efficiency ($\sim 0.7\%$) and matter-antimatter annihilation ($100\%$).

(c) Superradiance: waves scattered by a Kerr black hole with $m\Omega_H > \omega$ (where $m$ is the azimuthal mode number and $\omega$ the frequency) are amplified. Show this is the classical analogue of stimulated Hawking emission. What does the second law require about the area change during superradiant scattering?

---

**Thought Experiment T58.1.** *What does black hole entropy mean?*

Ordinary thermodynamic entropy counts the number of microstates: $S = k_B\ln\Omega$. Black hole entropy $S_{\rm BH} = k_B A/(4\ell_P^2)$ is also entropy — but what are the microstates?

In string theory, for extremal charged black holes, the entropy can be computed by counting BPS states (Strominger-Vafa 1996) and agrees with $k_B A/(4\ell_P^2)$ to the leading order. In loop quantum gravity, entropy is computed from counting spin foam states on the horizon. In AdS/CFT, it is the entropy of the dual CFT thermal state.

Does any of these microstate pictures feel more physically natural? What would it mean to "directly observe" a black hole microstate? Is there an operational definition of black hole entropy that could in principle be measured?

**Thought Experiment T58.2.** *The black hole as a quantum computer.*

Susskind and Hayden (2007) argued that a black hole is the "fastest scrambler" in nature: it takes infalling information and distributes it across all its degrees of freedom in the shortest possible time ($\sim GM/c^3$). The Hayden-Preskill calculation showed that an observer who collected all early Hawking radiation and decoded it could in principle recover the quantum state of an object thrown into the black hole after only a few additional bits were emitted.

Consider: if a black hole scrambles information faster than any other system, what does this tell us about the computational complexity of its internal dynamics? Is there a "holographic" computer inside every black hole, with a clock ticking at the Planck frequency?
