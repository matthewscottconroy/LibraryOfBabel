# Important Concepts: Chapter 3 — Light-Matter Interaction

---

## 1. The Lorentz Oscillator Model

**Core idea**: The macroscopic optical response of a dielectric is modeled by treating each atom as a classical harmonic oscillator: a bound electron driven by the electric field, restoring force proportional to displacement, and a damping term representing radiation and collisional losses.

**The model equation**:
$$m\ddot{x} + m\gamma\dot{x} + m\omega_0^2 x = -eE_0 e^{-i\omega t}$$

**Key results**:
- Complex susceptibility: $\chi(\omega) = \omega_p^2/(\omega_0^2 - \omega^2 - i\gamma\omega)$ where $\omega_p^2 = Ne^2/(m\varepsilon_0)$
- Real part (dispersion): peaks off-resonance, S-shaped around $\omega_0$
- Imaginary part (absorption): Lorentzian peak at $\omega_0$, FWHM = $\gamma$
- Quality factor of resonance: $Q = \omega_0/\gamma$

**Limiting cases**:
- **Far below resonance** ($\omega \ll \omega_0$): $n$ approximately constant and real (transparent, normal dispersion)
- **Near resonance** ($\omega \approx \omega_0$): anomalous dispersion, strong absorption
- **Far above resonance** ($\omega \gg \omega_0$): $n \to 1$ (all media become transparent to hard X-rays)
- **Free carriers** ($\omega_0 = 0$, Drude model): $n^2 = 1 - \omega_p^2/(\omega^2 + i\gamma\omega)$, relevant for silicon plasma dispersion

**Photonic computing relevance**: The Sellmeier equation (multi-resonance Lorentz fit) gives the refractive index used in all waveguide design. The Drude model gives the plasma dispersion effect used in silicon electro-optic modulators.

---

## 2. Kramers-Kronig Relations

**Core idea**: Causality — that a material cannot respond before it is driven — constrains the real and imaginary parts of the susceptibility to be Hilbert transforms of each other. No physical medium can have absorption at one frequency without having associated dispersion at all other frequencies.

**The relations**:
$$n(\omega) - 1 = \frac{2}{\pi}\text{P.V.}\int_0^\infty \frac{\omega' \kappa(\omega')}{\omega'^2 - \omega^2} d\omega'$$

$$\kappa(\omega) = -\frac{2\omega}{\pi}\text{P.V.}\int_0^\infty \frac{n(\omega') - 1}{\omega'^2 - \omega^2} d\omega'$$

**Four consequences that matter**:
1. **No absorption without dispersion**: A medium that absorbs light at any frequency must have non-trivial refractive index dispersion at nearby frequencies.
2. **No flat refractive index**: A dispersion-free medium over all frequencies would be physically impossible.
3. **f-sum rule**: $\int_0^\infty \chi''(\omega)\,d\omega = \pi\omega_p^2/2$, constraining total absorption.
4. **Silicon modulator chirp**: The plasma dispersion effect changes both $n$ (phase) and $\kappa$ (absorption) simultaneously. Phase modulation in a silicon modulator necessarily introduces amplitude modulation. This is not a design flaw; it is a consequence of causality.

**Derivation origin**: Analytic continuation of $\chi(\omega)$ into the complex plane, combined with the Cauchy integral formula. Causality $\Rightarrow$ $\chi(\tau) = 0$ for $\tau < 0$ $\Rightarrow$ $\chi(\omega)$ is analytic in the upper half-plane $\Rightarrow$ KK relations follow from contour integration.

---

## 3. Einstein A and B Coefficients

**Core idea**: Thermal equilibrium of a two-level atom with radiation requires three processes: spontaneous emission (A), stimulated emission (B₂₁), and stimulated absorption (B₁₂). Their rates are not independent.

**Einstein relations**:
$$B_{12} = B_{21}$$
$$A_{21} = \frac{\hbar\omega^3}{\pi^2 c^3} B_{21}$$

**Physical meaning**:
- $B_{12} = B_{21}$: stimulated emission and stimulated absorption have equal rates at equal populations — a consequence of the symmetry of the interaction Hamiltonian
- $A_{21} \propto \omega^3$: spontaneous emission is faster at higher frequencies (why lasers at visible wavelengths are harder to build than IR lasers with similar gain cross-sections)
- Spontaneous emission = stimulated emission by vacuum fluctuations (QED perspective)

**Laser threshold condition** (four-level system):
$$\sigma(\omega) \cdot \Delta N \cdot L_g \geq \text{round-trip losses}$$

where $\sigma(\omega) = \hbar\omega B_{21} g(\omega)$ is the stimulated emission cross-section and $g(\omega)$ is the lineshape.

**Photonic computing relevance**: EDFA gain and noise figure, semiconductor laser rate equations, single-photon source design, quantum memory lifetime.

---

## 4. Population Inversion

**Core idea**: Lasing requires $N_2 > N_1$ (more atoms in the upper level than the lower level). This cannot be achieved in a two-level system at thermal equilibrium. It requires a pumping scheme (three-level or four-level) that breaks detailed balance.

**Three-level vs. four-level**:
- **Three-level** (e.g., ruby, Er:glass at 1550 nm): lower laser level is the ground state; inversion requires pumping >50% of atoms out of the ground state; inherently less efficient
- **Four-level** (e.g., Nd:YAG, most diode lasers at telecom wavelengths): lower laser level empties rapidly to a ground sublevel; inversion achieved at much lower pump rates; threshold is lower; more efficient

**Saturation**: Above threshold, stimulated emission depletes the inversion; gain saturates as:
$$g = \frac{g_0}{1 + I/I_{sat}}$$
where $I_{sat} = \hbar\omega/(2\sigma\tau)$.

**EDFA noise figure**: A fully inverted EDFA (all atoms in upper level) has quantum-limited noise figure $F = 2n_{sp} = 2$ (3 dB). At partial inversion, $F > 3$ dB. Three-level operation at 1550 nm means practical EDFAs have noise figures of 4–6 dB.

---

## 5. Nonlinear Susceptibilities: $\chi^{(2)}$ and $\chi^{(3)}$

**The expansion**:
$$\mathbf{P} = \varepsilon_0\left[\chi^{(1)}\mathbf{E} + \chi^{(2)}\mathbf{E}\mathbf{E} + \chi^{(3)}\mathbf{E}\mathbf{E}\mathbf{E} + \cdots\right]$$

**$\chi^{(2)}$ (second-order)**:
- Tensor of rank 3 with up to 27 components (reduced by symmetry)
- Vanishes in centrosymmetric materials (e.g., silicon, amorphous silica)
- Non-zero in LiNbO₃ ($d_{33} = 27$ pm/V), GaAs, AlGaAs, BaTiO₃, KTP
- Processes: SHG, sum-frequency generation, difference-frequency generation, SPDC, electro-optic (Pockels) effect
- SPDC: one pump photon → two entangled photons (signal + idler); the primary source of entangled photon pairs for quantum photonic computing

**$\chi^{(3)}$ (third-order)**:
- Present in all materials (no symmetry restriction)
- In silica fiber: $n_2 = 2.6 \times 10^{-20}$ m²/W; in silicon: $n_2 = 6 \times 10^{-18}$ m²/W (230× larger)
- Processes: Kerr effect (self-phase modulation, cross-phase modulation), four-wave mixing, third-harmonic generation, two-photon absorption
- Kerr nonlinear index: $n = n_0 + n_2 I$

**Key constraint for photonic computing**:
- Silicon's $\chi^{(3)}$ is large but is accompanied by two-photon absorption at 1550 nm ($\beta_{TPA} \approx 5 \times 10^{-12}$ m/W), which generates free carriers and causes additional loss
- Figure of merit: $\text{FOM} = n_2/(\lambda \beta_{TPA}) \approx 0.4$ for Si at 1550 nm (below unity — TPA is a problem)
- Si₃N₄ has FOM >> 1 (no TPA at 1550 nm), making it the preferred platform for Kerr-effect photonic processing

---

## 6. Phase Matching

**Core idea**: Nonlinear optical processes require energy conservation (frequency matching) AND momentum conservation (phase matching) to be efficient. Energy conservation is automatic; momentum conservation (equivalently, phase matching) is not.

**Phase mismatch**: $\Delta k = k(\omega_3) - k(\omega_1) - k(\omega_2)$

**SHG efficiency** (undepleted pump):
$$\eta \propto L^2 \cdot \text{sinc}^2(\Delta k L / 2)$$

Maximum at $\Delta k = 0$; coherence length $L_c = \pi/|\Delta k|$.

**Methods to achieve phase matching**:
1. **Birefringence phase matching**: use different polarizations with different $n$ to satisfy $\Delta k = 0$
2. **Quasi-phase matching (QPM)**: periodically invert $\chi^{(2)}$ (PPLN) so poling period $\Lambda = 2L_c$ compensates the mismatch; effective $d_{eff} = (2/\pi)d_{33}$
3. **Waveguide dispersion engineering**: modify waveguide geometry to tune effective index at each frequency

**Photonic computing relevance**: SPDC (entangled photon generation for quantum photonics) requires phase matching. Electro-optic modulation at 100+ GHz requires velocity matching between the microwave drive and the optical wave.

---

## 7. The Nonlinear Schrödinger Equation

**Governing equation** (anomalous dispersion, lossless):
$$i\frac{\partial A}{\partial z} = \frac{\beta_2}{2}\frac{\partial^2 A}{\partial t^2} - \gamma|A|^2 A$$

where $A(z,t)$ is the slowly-varying envelope, $\beta_2$ is the group velocity dispersion, and $\gamma = n_2\omega/(cA_{eff})$ is the nonlinear coefficient.

**Competing effects**:
- $\beta_2 > 0$ (normal dispersion): blue components travel faster → pulse broadens, no soliton
- $\beta_2 < 0$ (anomalous dispersion): red components travel faster
- $\gamma|A|^2$ (Kerr): self-phase modulation chirps the pulse
- Balance of anomalous GVD and Kerr SPM → soliton

**Length scales**:
- Dispersion length: $L_D = T_0^2/|\beta_2|$ (where $T_0$ is pulse width)
- Nonlinear length: $L_{NL} = 1/(\gamma P_0)$
- Soliton number: $N^2 = L_D/L_{NL} = \gamma P_0 T_0^2/|\beta_2|$

**Fundamental soliton** ($N = 1$):
$$A(z,t) = \sqrt{P_0}\,\text{sech}(t/T_0)\,e^{iz/(2L_D)}$$

propagates indefinitely without broadening. Required peak power: $P_0 = |\beta_2|/(\gamma T_0^2)$.

**Microresonator dissipative Kerr solitons**: driven-dissipative version of NLSE with pump and loss terms; produces stabilized frequency combs for WDM-based photonic computing.

---

## 8. Rayleigh, Raman, and Brillouin Scattering: Key Numbers

| Scattering process | Physical origin | Frequency shift | Cross-section/gain |
|---|---|---|---|
| Rayleigh | Density fluctuations, elastic | 0 (elastic) | $\propto \lambda^{-4}$ |
| Raman | Optical phonons, inelastic | ~13 THz (silica) | $g_R \approx 10^{-13}$ m/W |
| Brillouin | Acoustic phonons, inelastic | ~11 GHz (silica) | $g_B \approx 5\times10^{-11}$ m/W |

**Rayleigh scattering**:
- Loss: $\alpha_R = A/\lambda^4$ with $A \approx 0.78$ dB·km$^{-1}$·μm$^4$ for silica
- At 1550 nm: 0.14 dB/km (close to total fiber loss of ~0.18 dB/km — Rayleigh dominates)
- Minimum at ~1570 nm; sets fundamental limit on fiber transmission loss

**Raman scattering**:
- Gain peak at 13.2 THz (Stokes), anti-Stokes at -13.2 THz (thermally suppressed)
- SRS threshold: $g_R P_{th} L_{eff}/A_{eff} \approx 16$
- Used constructively: Raman fiber amplifiers (1450 nm pump → 1550 nm gain), silicon Raman laser

**Brillouin scattering**:
- Narrow gain bandwidth (~20 MHz); very high gain coefficient ($g_B = 500 \times g_R$)
- SBS threshold: $P_{th} \approx 21 A_{eff}/(g_B L_{eff}) \approx 1$–5 mW in SMF
- SBS is the primary limit on single-channel launch power in coherent optical transmission
- BOTDA/BOTDR sensing: $\nu_B$ shifts 1.1 MHz/°C, 500 MHz/% strain; distributed sensing over 100+ km

---

## 9. The Transparency Window and Platform Selection

Why 1550 nm is the operating wavelength of photonic computing:

| Property | 1310 nm | 1550 nm |
|---|---|---|
| Silica Rayleigh loss | ~0.33 dB/km | ~0.14 dB/km |
| Silica IR absorption | Negligible | Negligible |
| Total fiber loss | ~0.35 dB/km | ~0.18 dB/km |
| EDFA amplification | No (1550 nm band) | Yes (C-band 1530–1565 nm) |
| SMF-28 GVD | Near-zero (ideal for single-channel) | Anomalous (requires management for WDM) |
| Si waveguide GVD | Normal | Anomalous (near zero-dispersion with geometry) |
| Silicon transparency | Yes | Yes |
| Two-photon absorption in Si | Negligible ($E_{ph} < E_g/2$) | Negligible ($\hbar\omega = 0.8$ eV, $E_g = 1.12$ eV) |

1550 nm wins: lowest fiber loss + EDFA availability. The convergence of these two properties in the same spectral window is not a coincidence — EDFA development in the 1980s was driven by the loss minimum at 1550 nm.

---

## 10. Key Numerical Values

| Quantity | Value | Significance |
|---|---|---|
| Silicon $n_2$ | $6 \times 10^{-18}$ m²/W | Large Kerr nonlinearity |
| Silicon $\beta_{TPA}$ | $5 \times 10^{-12}$ m/W | Two-photon absorption (problem) |
| Silicon FOM ($n_2/\lambda\beta_{TPA}$) | ~0.4 | Below unity: TPA limits nonlinear processing |
| Si₃N₄ $n_2$ | $2.4 \times 10^{-19}$ m²/W | Moderate Kerr |
| Si₃N₄ $\beta_{TPA}$ | ~0 at 1550 nm | No TPA (advantage) |
| Silica fiber $n_2$ | $2.6 \times 10^{-20}$ m²/W | Standard single-mode fiber |
| Silica fiber $g_R$ | $\approx 10^{-13}$ m/W | Raman gain |
| Silica fiber $g_B$ | $\approx 5 \times 10^{-11}$ m/W | Brillouin gain (500× Raman) |
| SBS threshold (25 km SMF) | ~1.5 mW | Power limit for coherent transmission |
| Brillouin shift $\nu_B$ | ~11 GHz at 1550 nm | BOTDA frequency reference |
| EDFA noise figure (best) | 3 dB (quantum limit) | Amplifier noise floor |
| Soliton power (1 ps, SMF) | ~15 mW | Fundamental soliton condition |
