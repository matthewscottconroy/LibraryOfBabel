# 3.3.2 — Second-Harmonic Generation

## The Physical Process

Second-harmonic generation (SHG) is the conversion of a pump field at frequency $\omega$ into a signal field at frequency $2\omega$. It is the simplest $\chi^{(2)}$ process. In materials science terms: two pump photons are absorbed and one second-harmonic photon is emitted. Energy conservation: $\hbar\omega + \hbar\omega = \hbar(2\omega)$ ✓.

The physical mechanism: the pump field $E_\omega e^{-i\omega t} + \text{c.c.}$ drives the nonlinear polarization $P^{(2)} = \varepsilon_0\chi^{(2)}E^2$, which contains a term at frequency $2\omega$:

$$P^{(2)}(2\omega) = \varepsilon_0\chi^{(2)} E_\omega^2$$

This oscillating polarization at $2\omega$ radiates like an antenna — emitting a field at $2\omega$. For this process to be efficient, the emitted $2\omega$ field from different parts of the medium must add constructively — requiring phase matching.

## Coupled Wave Equations

Let the pump field be $E_1(z) e^{i(k_1 z - \omega t)}$ (at frequency $\omega$) and the generated second-harmonic field be $E_2(z) e^{i(k_2 z - 2\omega t)}$ (at $2\omega$). Substituting into Maxwell's equations with the nonlinear polarization:

$$\frac{dE_1}{dz} = -i\frac{\omega d_\text{eff}}{n_1 c} E_2 E_1^* e^{-i\Delta kz}$$

$$\frac{dE_2}{dz} = -i\frac{2\omega d_\text{eff}}{n_2 c} E_1^2 e^{i\Delta kz}$$

where $\Delta k = k_2 - 2k_1 = 2\omega n_2/c - 2\omega n_1/c = (2\omega/c)(n_2 - n_1)$ is the *phase mismatch*, and $d_\text{eff} = \chi^{(2)}/2$ is the effective nonlinear coefficient.

## Solution Without Phase Matching ($\Delta k \neq 0$)

In the *undepleted pump* approximation ($|E_1|$ approximately constant — valid for low conversion efficiency), the second-harmonic amplitude after propagation length $L$:

$$E_2(L) \approx -i\frac{2\omega d_\text{eff}}{n_2 c} E_1^2 \int_0^L e^{i\Delta kz} dz = -i\frac{2\omega d_\text{eff}}{n_2 c} E_1^2 L \, \text{sinc}\left(\frac{\Delta kL}{2}\right) e^{i\Delta kL/2}$$

The second-harmonic power:

$$P_{2\omega} \propto |E_2|^2 \propto L^2 d_\text{eff}^2 P_\omega^2 \, \text{sinc}^2\left(\frac{\Delta kL}{2}\right)$$

**Observations**:
- $P_{2\omega} \propto P_\omega^2$: the SHG power grows as the square of the pump power (nonlinear response).
- $P_{2\omega} \propto L^2$ when $\Delta k = 0$ (phase-matched): power grows quadratically with length.
- $P_{2\omega} \propto \text{sinc}^2(\Delta kL/2)$: for $\Delta k \neq 0$, the conversion efficiency oscillates with period $2\pi/\Delta k$ (the *coherence length* $L_c = \pi/\Delta k$). After one coherence length, the second harmonic and the polarization are $\pi$ out of phase and the signal starts to flow back to the fundamental — no net conversion.

**The coherence length** is the key parameter: $L_c = \lambda/(4|n_{2\omega} - n_\omega|)$. For LiNbO₃ without phase matching at 1064 nm (Nd:YAG pump): $n_{532} - n_{1064} \approx 0.04$, giving $L_c = 1064/(4 \times 4 \times 10^{-2}) \approx 7$ μm. Without phase matching, conversion efficiency is negligible even for $L = 1$ mm: $P_{2\omega}/P_\omega \sim (L/L_c)^2 \cdot [...]/ \sim 10^{-4}$. This is why phase matching is essential for practical SHG.

## Spontaneous Parametric Downconversion (SPDC)

The reverse of SHG — one pump photon at $2\omega$ splits into two photons at $\omega_1 + \omega_2 = 2\omega$ (with phase matching determining the specific frequencies and directions) — is *spontaneous parametric downconversion (SPDC)*. SPDC is a quantum process that generates pairs of photons that are *entangled* in time, frequency (energy-time entanglement), and polarization (type-II SPDC in birefringent crystals).

SPDC is the primary source of entangled photon pairs for quantum photonic computing and quantum communication (Unit VII). The same $\chi^{(2)}$ nonlinearity responsible for SHG, running in reverse, produces entangled photon pairs. Efficient SPDC in LiNbO₃, KTP, BBO, and thin-film LiNbO₃ waveguides (which offer mode confinement and phase matching simultaneously) is an active area of quantum photonics research.

## Applications in Photonic Computing

**Wavelength conversion**: SHG and its reverse (optical parametric amplification) enable converting signals between wavelength bands. For example: convert a 1550 nm signal to 775 nm (SHG), process at 775 nm, convert back (optical parametric amplification or SPDC). This extends the operational wavelength range beyond what any single laser or detector can cover.

**Entangled photon generation for quantum computing**: SPDC in integrated waveguides generates on-chip entangled photon pairs. Thin-film LiNbO₃ waveguides have achieved pair generation rates of $> 10^9$ pairs/s/mW/GHz [1] with high purity — approaching the requirements for practical quantum photonic processors.

**Optical frequency combs**: SHG combined with other $\chi^{(2)}$ processes (sum frequency generation) is used in *f-to-2f interferometry* for stabilizing optical frequency combs. Frequency combs are the basis of the most precise optical clocks and are used in some photonic computing proposals for generating multiple phase-locked optical carriers.

## Summary

- SHG: pump at $\omega$ → second harmonic at $2\omega$ via $\chi^{(2)}$.
- Power: $P_{2\omega} \propto P_\omega^2 L^2 \text{sinc}^2(\Delta kL/2)$; requires phase matching for efficient conversion.
- Phase mismatch $\Delta k = (2\omega/c)(n_{2\omega} - n_\omega)$; coherence length $L_c = \pi/|\Delta k|$.
- SPDC (reverse of SHG) generates entangled photon pairs — essential for quantum photonic computing.

---

*References*

[1] Javid, U.A. et al. (2021). Ultrabroadband entangled photons on a nanophotonic chip. *Physical Review Letters*, 127(18), 183601. [DOI: 10.1103/PhysRevLett.127.183601]
