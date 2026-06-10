# 3.2.2 — Einstein $A$ and $B$ Coefficients

## Three Radiative Processes

In 1917, Einstein identified three distinct ways in which a two-level system can exchange energy with the electromagnetic field [1]. These are:

**1. Stimulated absorption**: An atom in state $|1\rangle$ absorbs a photon of energy $\hbar\omega_0$ and transitions to $|2\rangle$. The rate is proportional to the radiation density $\rho(\omega_0)$ at the transition frequency:
$$W_\text{abs} = B_{12}\rho(\omega_0) N_1$$
where $B_{12}$ is the *Einstein B coefficient for absorption* (in SI units: m³/(J·s²)) and $N_1$ is the population density in state 1.

**2. Stimulated emission**: An atom in state $|2\rangle$ is stimulated by an incoming photon to emit an identical photon into the same mode, transitioning to $|1\rangle$. The emitted photon is *coherent* with the incoming one — same frequency, direction, phase, and polarization. Rate:
$$W_\text{stim} = B_{21}\rho(\omega_0) N_2$$

**3. Spontaneous emission**: An atom in state $|2\rangle$ spontaneously emits a photon into a random mode and decays to $|1\rangle$. This occurs even in the absence of any incident radiation — it is driven by vacuum fluctuations of the electromagnetic field (which cannot be eliminated quantum mechanically). Rate:
$$W_\text{spon} = A_{21} N_2$$
where $A_{21}$ is the *Einstein A coefficient* (units: s⁻¹).

## The Einstein Relations

Einstein derived relations between $A_{21}$, $B_{12}$, and $B_{21}$ by requiring that at thermal equilibrium, the system follows the Boltzmann distribution and the radiation follows the Planck blackbody spectrum. In equilibrium, the rate of population increase in state 2 equals the rate of decrease:

$$B_{12}\rho(\omega_0)N_1 = (B_{21}\rho(\omega_0) + A_{21})N_2$$

The Boltzmann distribution: $N_2/N_1 = e^{-\hbar\omega_0/(k_B T)}$.

The Planck radiation density (energy per unit angular frequency per unit volume):

$$\rho(\omega) = \frac{\hbar\omega^3}{\pi^2 c^3} \cdot \frac{1}{e^{\hbar\omega/(k_BT)} - 1}$$

Substituting both conditions into the rate balance equation and requiring consistency for all temperatures:

$$\boxed{B_{12} = B_{21}, \qquad A_{21} = \frac{\hbar\omega_0^3}{\pi^2 c^3} B_{21}}$$

These are the **Einstein relations**. Their consequences:

1. $B_{12} = B_{21}$: absorption and stimulated emission have equal cross-sections. If you have equal populations in states 1 and 2, stimulated emission exactly cancels absorption — the medium is transparent (zero net gain or loss).

2. $A_{21} = (\hbar\omega_0^3/\pi^2 c^3) B_{21}$: spontaneous emission rate is proportional to stimulated emission rate, with a factor of $\hbar\omega_0^3/\pi^2 c^3$ — the density of electromagnetic modes at frequency $\omega_0$ per unit volume (the photon density of states). Spontaneous emission is stimulated emission induced by the zero-point fluctuations of all the vacuum modes.

**The spontaneous emission lifetime**: $\tau_\text{sp} = 1/A_{21}$ is the characteristic time for the excited state to decay spontaneously. For optical transitions in atoms at visible wavelengths: $\tau_\text{sp} \sim 1$–100 ns. For semiconductor quantum dots at telecom wavelengths: $\tau_\text{sp} \sim 1$–10 ns. For free-space optical transitions (no cavity), $A_{21}$ is fixed by $B_{21}$ via the Einstein relation.

## The $B$ Coefficient from Quantum Mechanics

Quantum mechanics gives the $B$ coefficient in terms of the transition dipole moment:

$$B_{21} = \frac{\pi|\mathbf{d}_{12}|^2}{3\varepsilon_0\hbar^2}$$

Combining with the Einstein relation:

$$A_{21} = \frac{\omega_0^3 |\mathbf{d}_{12}|^2}{3\pi\varepsilon_0\hbar c^3}$$

The $A$ coefficient (spontaneous emission rate) scales as:
- $\propto |\mathbf{d}_{12}|^2$: strong dipole transitions decay faster.
- $\propto \omega_0^3$: higher-frequency transitions (UV) have much faster spontaneous emission than IR transitions.

For a two-level atom at infrared wavelengths ($\lambda = 1550$ nm, $\omega_0 = 1.22 \times 10^{15}$ rad/s) with $|\mathbf{d}_{12}| = ea_0$:

$$A_{21} \approx \frac{(1.22 \times 10^{15})^3 \times (1.6 \times 10^{-19} \times 5.3 \times 10^{-11})^2}{3\pi \times 8.85 \times 10^{-12} \times (10^{-34}) \times (3 \times 10^8)^3} \approx 6 \times 10^6 \text{ s}^{-1}$$

Lifetime $\tau_\text{sp} \approx 170$ ns — reasonable for a strong optical transition at telecom wavelengths.

## The Gain Coefficient

Consider a medium with density $N$ two-level systems per volume, with $N_2$ in the excited state and $N_1$ in the ground state. The net stimulated rate per unit volume:

$$\text{Net stimulated emission rate} = B_{21}\rho(\omega_0)(N_2 - N_1) = B_{21}\rho(\omega_0)\Delta N$$

where $\Delta N = N_2 - N_1$ is the *population inversion density*. If $\Delta N > 0$ (more atoms excited than ground): net stimulated emission — *gain*. If $\Delta N < 0$ (more in ground state): net absorption — *loss*.

The gain coefficient $g$ (intensity exponential growth rate, in m⁻¹) is:

$$g(\omega) = \sigma(\omega) \Delta N$$

where $\sigma(\omega)$ is the *stimulated emission cross section* (m²):

$$\sigma(\omega) = \frac{B_{21}\hbar\omega}{c} \cdot L(\omega) = \frac{|\mathbf{d}_{12}|^2 \omega}{3\varepsilon_0\hbar c} \cdot L(\omega)$$

and $L(\omega)$ is the normalized Lorentzian lineshape of the transition ($\int L(\omega)d\omega = 1$). The intensity grows as $I(z) = I_0 e^{gz}$ (or decays as $e^{-|g|z}$ for absorption).

## Significance for Photonic Computing: The Optical Amplifier

The optical amplifier — an erbium-doped fiber amplifier (EDFA) or semiconductor optical amplifier (SOA) — is based on stimulated emission from a population-inverted medium. Without optical amplification, signal losses in waveguides and fibers would limit the scale of photonic computing to tiny systems. With amplification:

- EDFAs operate at 1550 nm (C-band), providing 20–40 dB gain over small bandwidths. They are the reason that transoceanic fiber communication is possible — signals are boosted periodically to overcome fiber losses.
- SOAs are compact, integrate on chip, and provide gain over broader bandwidths but with higher noise.
- Erbium-doped waveguide amplifiers (EDWAs) can be integrated on silicon photonic chips, providing on-chip gain to compensate insertion losses.

The physics of the optical amplifier is Einstein's three processes, applied to erbium ions in a silica host (for EDFAs) or to electrons in a semiconductor heterostructure (for SOAs).

## Summary

- Three processes: spontaneous emission ($A_{21}$), stimulated emission ($B_{21}$), stimulated absorption ($B_{12}$).
- Einstein relations: $B_{12} = B_{21}$; $A_{21} = (\hbar\omega_0^3/\pi^2c^3)B_{21}$.
- Spontaneous emission is stimulated by vacuum fluctuations — it is stimulated emission into vacuum modes.
- Gain coefficient $g = \sigma\Delta N$; gain requires population inversion $\Delta N > 0$.
- Optical amplifiers (EDFA, SOA) are essential for practical-scale photonic systems.

---

*References*

[1] Einstein, A. (1917). Zur Quantentheorie der Strahlung. *Physikalische Zeitschrift*, 18, 121–128. [The paper introducing the A and B coefficients and the concept of stimulated emission — laying the theoretical foundation for the laser, 43 years before its invention.]
