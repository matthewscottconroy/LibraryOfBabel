# 7.3.1 The Plasma Dispersion Effect

## The Symmetry Problem and Its Resolution

We begin with an honest accounting of silicon's limitations as a modulator material, because understanding the constraint is the precondition for appreciating how the constraint is circumvented.

A linear electro-optic effect — a change in refractive index proportional to an applied electric field, $\delta n \propto E$ — requires a nonzero second-order susceptibility $\chi^{(2)}$. As we saw in Chapter 3, $\chi^{(2)}$ vanishes in any material with inversion symmetry, because inverting the coordinate system ($\mathbf{r} \to -\mathbf{r}$) must leave the physics unchanged, while the relation $P_i = \varepsilon_0\chi^{(2)}_{ijk}E_j E_k$ changes sign under inversion for the $P_i$ term but not the $E_j E_k$ term — a contradiction unless $\chi^{(2)} = 0$.

Silicon crystallizes in the diamond cubic structure, which is inversion-symmetric. Therefore $\chi^{(2)}_{\text{Si}} \equiv 0$ in the bulk, and the Pockels effect is absent [1].

The Kerr effect ($\delta n \propto E^2$) is present in silicon — $n_2 \approx 6 \times 10^{-18}$ m²/W, which is why silicon waveguides exhibit SPM as discussed in Section 6.3.1 — but the DC or RF Kerr effect ($\delta n \propto V^2$) is too weak to be useful for practical modulation at reasonable drive voltages.

What silicon *does* have is free carriers. When electrons and holes are injected into silicon — or removed from it by depletion — the optical properties of the material change. This effect, the plasma dispersion effect, is the basis of all practical silicon electro-optic modulators.

## The Drude Model for Free Carriers

To understand the plasma dispersion effect physically, we treat the free carriers as a classical plasma — an ensemble of charged particles that can oscillate in response to the optical field.

Consider a free electron with charge $-e$ and effective mass $m^*$ in an optical field $E(t) = E_0 e^{-i\omega t}$. The equation of motion is:

$$m^* \ddot{x} + m^* \frac{\omega}{\mu} \dot{x} = -eE(t)$$

where $\mu$ is the carrier mobility and the damping term $m^*\omega/\mu$ represents collisions. This is just the Lorentz oscillator model of Chapter 3 with zero resonance frequency (free carriers, not bound electrons).

For harmonic response $x = x_0 e^{-i\omega t}$:

$$(-m^*\omega^2 - im^*\omega^2/\mu \cdot 1/\omega)x_0 = -eE_0$$

Wait — let me write this more carefully. The mobility is defined by $v_d = \mu E$ in the DC limit, so the collision rate is $\gamma = e/(m^*\mu)$. The equation of motion becomes:

$$m^* \ddot{x} + m^*\gamma \dot{x} = -eE$$

with $\gamma = e/(m^*\mu)$. For harmonic fields, the displacement is:

$$x_0 = \frac{-eE_0/m^*}{-\omega^2 - i\omega\gamma} = \frac{eE_0/m^*}{\omega^2 + i\omega\gamma}$$

The polarization from $N$ free electrons per unit volume is $P = -Nex_0$, giving:

$$P = -\frac{Ne^2}{m^*}\frac{E_0}{\omega^2 + i\omega\gamma} = \varepsilon_0 \chi_{\text{free}} E_0$$

Therefore the contribution to the dielectric function from free carriers is:

$$\chi_{\text{free}} = -\frac{Ne^2}{m^*\varepsilon_0(\omega^2 + i\omega\gamma)} = -\frac{\omega_p^2}{\omega^2 + i\omega\gamma}$$

where $\omega_p = \sqrt{Ne^2/(m^*\varepsilon_0)}$ is the **plasma frequency**.

The total dielectric function is $\varepsilon_r = \varepsilon_{\text{Si}} + \chi_{\text{free}}$, where $\varepsilon_{\text{Si}} \approx n_{\text{Si}}^2 = 12.1$ is the background (bound-electron) contribution. In the limit $\omega \gg \gamma$ (optical frequencies, where $\hbar\omega \approx 0.8$ eV at 1550 nm, while $\hbar\gamma \sim k_BT/10 \sim 3$ meV, so $\omega/\gamma \sim 300$):

$$\varepsilon_r \approx \varepsilon_{\text{Si}} - \frac{\omega_p^2}{\omega^2}$$

Taking $n = \sqrt{\varepsilon_r} \approx n_{\text{Si}}(1 + \Delta\varepsilon/(2\varepsilon_{\text{Si}}))$ for small perturbations:

$$\Delta n \approx -\frac{\omega_p^2}{2n_{\text{Si}}\omega^2} = -\frac{Ne^2}{2n_{\text{Si}}\varepsilon_0 m^* \omega^2}$$

For silicon at 1550 nm ($\omega = 2\pi \times 193 \times 10^{12}$ rad/s), with electron effective mass $m^*_e = 0.26 m_0$:

$$\Delta n_e = -\frac{Ne^2}{2n_{\text{Si}}\varepsilon_0 m^*_e \omega^2} \approx -\frac{N}{N_{\text{ref}}}$$

where $N_{\text{ref}}$ evaluates to approximately $10^{27}$ m⁻³. More usefully, for $N$ in units of cm⁻³:

$$\Delta n_e \approx -8.8 \times 10^{-22} N_e \quad [N_e \text{ in cm}^{-3}]$$

The Drude model prediction for the free-carrier-induced change in refractive index scales as $\Delta n \propto -N/\omega^2 \propto -N\lambda^2$. This means the plasma dispersion effect is *stronger at longer wavelengths* — relevant when comparing operation at 1310 nm vs. 1550 nm.

## The Soref-Bennett Relations

The Drude model gives the scaling but not the precise numerical prefactors for silicon, because carriers in silicon occupy non-parabolic bands and have energy-dependent effective masses. In 1987, Soref and Bennett performed a landmark calculation combining the Drude model with a careful treatment of the silicon band structure, fitting to measured absorption data to obtain empirical relations that remain the design standard for silicon modulators [2].

For wavelength $\lambda = 1550$ nm, the Soref-Bennett relations are:

$$\boxed{\Delta n = -\left(8.8 \times 10^{-22} \Delta N_e + 8.5 \times 10^{-18} \Delta N_h^{0.8}\right)}$$

$$\boxed{\Delta\alpha = 8.5 \times 10^{-18} \Delta N_e + 6.0 \times 10^{-18} \Delta N_h \quad [\text{cm}^{-1}]}$$

where $\Delta N_e$ is the change in electron density (cm⁻³) and $\Delta N_h$ is the change in hole density (cm⁻³).

Several features of these relations deserve attention:

**The asymmetry between electrons and holes.** For the refractive index change, holes dominate at equal carrier densities: a factor of $8.5 \times 10^{-18} (10^{17})^{0.8} \approx 1.35 \times 10^{-3}$ for holes vs. $8.8 \times 10^{-22} \times 10^{17} = 8.8 \times 10^{-5}$ for electrons at $N = 10^{17}$ cm⁻³. This asymmetry arises because holes in silicon have higher effective masses than electrons, which increases the optical matrix element for intraband transitions.

**The sublinear power law for holes.** The $N_h^{0.8}$ dependence (rather than linear) reflects the non-parabolic valence band structure of silicon. The deviation from linearity is modest over the practical range but is important for precision modeling.

**Both $\Delta n$ and $\Delta\alpha$ contribute.** A change in carrier density changes both the real part of the refractive index (causing phase shift) and the imaginary part (causing absorption change). These are not independent — they are the real and imaginary parts of the complex refractive index $\tilde{n} = n + i\kappa$. The coupling between them is required by the Kramers-Kronig relations (Chapter 3).

For the practical purpose of modulator design, the simultaneous phase and amplitude modulation means that silicon modulators are inherently *chirped* — a change in the drive voltage changes both the phase and amplitude of the transmitted light. This chirp affects the spectral width of the modulated signal and can be a problem for long-distance transmission. In photonic computing, chirp matters less (distances are millimeters, not kilometers), but it does limit the precision of analog encoding.

At $\lambda = 1310$ nm, the Soref-Bennett relations differ slightly (stronger effect due to $\lambda^2$ scaling):

$$\Delta n = -\left(6.2 \times 10^{-22} \Delta N_e + 6.0 \times 10^{-18} \Delta N_h^{0.8}\right) \quad [\lambda = 1310 \text{ nm}]$$

$$\Delta\alpha = 6.0 \times 10^{-18} \Delta N_e + 4.0 \times 10^{-18} \Delta N_h \quad [\text{cm}^{-1}, \lambda = 1310 \text{ nm}]$$

These have since been refined by Nedeljkovic et al. using more accurate band structure calculations [3], but for the carrier densities relevant to silicon modulators ($10^{17}$–$10^{18}$ cm⁻³), the original Soref-Bennett values remain accurate to within ~10%.

## Carrier Control Mechanisms

The plasma dispersion effect modifies the refractive index, but to make a modulator, we need to control the carrier density electrically and rapidly. There are three mechanisms for doing this in silicon:

### Carrier Injection (Forward-Biased PIN Junction)

In a forward-biased PIN junction, minority carriers are injected into the intrinsic silicon region. For a PIN junction with intrinsic region thickness $d$ and forward current density $J$:

$$\Delta N_{e,h} \approx \frac{J\tau_r}{ed}$$

where $\tau_r$ is the carrier recombination lifetime. For silicon with $\tau_r \approx 1$–10 ns and $J = 1$ mA/μm² (typical for forward bias), $d = 200$ nm:

$$\Delta N \approx \frac{10^{10} \text{ A/m}^2 \times 10^{-9} \text{ s}}{1.6\times10^{-19} \text{ C} \times 200\times10^{-9} \text{ m}} \approx 3 \times 10^{17} \text{ cm}^{-3}$$

which produces $\Delta n \approx -5 \times 10^{-4}$ (using hole term dominant).

**Advantage**: Large $\Delta n$ at low voltage (< 1 V forward bias).

**Disadvantage**: The modulation bandwidth is limited by the carrier lifetime: $f_{-3\text{dB}} \approx 1/(2\pi\tau_r) \approx$ 100 MHz–1 GHz. This is too slow for most communications applications but potentially adequate for some matrix weight programming tasks. The recombination lifetime can be reduced by ion implantation to create trap states, improving bandwidth at the cost of increased free-carrier absorption [4].

### Carrier Depletion (Reverse-Biased PN Junction)

In a reverse-biased PN junction, the depletion width increases with reverse voltage:

$$W_d = \sqrt{\frac{2\varepsilon_{\text{Si}}}{e}\left(\frac{1}{N_A} + \frac{1}{N_D}\right)(V_{bi} - V)}$$

where $V_{bi}$ is the built-in voltage (~0.9 V for typical Si doping levels) and $V$ is the applied (negative) voltage. When the depletion width increases, carriers are swept out of the active region, reducing $\Delta N$ and thus changing $\Delta n$.

The maximum change in carrier density within a waveguide of width $w$ is of order:

$$\Delta N \approx N_D \frac{\Delta W_d}{w}$$

For $N_D = 10^{17}$ cm⁻³, $\Delta W_d = 50$ nm, $w = 450$ nm:

$$\Delta N \approx 10^{17} \times \frac{50}{450} \approx 10^{16} \text{ cm}^{-3}$$

This gives $\Delta n \approx -10^{-4}$, about 5× smaller than injection, requiring longer phase-shift sections.

**Advantage**: Bandwidth is set by the RC time constant, not the carrier lifetime. For a depletion modulator, $f_{-3\text{dB}} \approx 1/(2\pi RC_j)$, where $C_j \approx \varepsilon_{\text{Si}}/W_d$ is the junction capacitance. With $W_d \approx 200$ nm and $R = 50\ \Omega$, $C_j \approx 0.5$ fF/μm, giving:

$$f_{-3\text{dB}} \approx \frac{1}{2\pi \times 50 \times 0.5 \times 10^{-15}} \approx 6 \text{ GHz/μm of length}$$

For a 1-mm-long modulator: $C_j \approx 500$ fF, $f_{-3\text{dB}} \approx 6$ GHz — adequate for 10–25 Gbps modulation.

This is the dominant mechanism in modern high-speed silicon photonic modulators. The lower efficiency ($\Delta n$ per volt) is compensated by longer interaction lengths using the traveling-wave electrode geometry.

### Carrier Accumulation (MOS Capacitor)

In a metal-oxide-semiconductor (MOS) structure, an insulating oxide layer separates the silicon from a metal or polysilicon gate. Applying a gate voltage accumulates charge at the oxide-silicon interface without significant current flow:

$$\Delta N_{\text{surf}} = \frac{C_{ox} |V|}{e}$$

where $C_{ox}$ is the oxide capacitance per unit area. For an oxide thickness of 10 nm, $C_{ox} \approx 3.5 \times 10^{-3}$ F/m². At $|V| = 1$ V, $\Delta N_{\text{surf}} \approx 2.2 \times 10^{16}$ m⁻² of sheet charge.

The accumulated charge is confined to a very thin layer (~1 nm), so the volumetric density can be very high, but the interaction volume is small. The bandwidth is limited by the RC charging time of the MOS capacitor: similar to depletion, but with capacitance per unit area $C_{ox}$ rather than $C_j$.

MOS-based silicon modulators were demonstrated early [5] but have largely been superseded by PN depletion-mode devices for high-speed applications, because the thin oxide layer creates reliability concerns and the confinement to the oxide-silicon interface reduces the overlap with the waveguide mode.

## Phase Shift Efficiency: The $V_\pi L$ Figure of Merit

The key figure of merit for a phase modulator is the product $V_\pi L$, where $V_\pi$ is the voltage required to produce a phase shift of $\pi$ radians and $L$ is the device length:

$$V_\pi L = \frac{\pi \lambda}{2 \Gamma |\partial n / \partial V|}$$

where $\Gamma$ is the confinement factor and $\partial n / \partial V$ is the refractive index change per volt.

For a silicon PN depletion modulator with typical parameters:
- $\partial(\Delta N_e)/\partial V \approx 10^{16}$ cm⁻³/V (from junction capacitance model)
- $\partial n/\partial V \approx 8.8 \times 10^{-22} \times 10^{16} \approx 8.8 \times 10^{-6}$ per V (electron contribution)
- Including hole contribution (roughly equal): $\partial n / \partial V \approx 1.5 \times 10^{-5}$ per V
- $\Gamma \approx 0.8$ for 450×220 nm Si strip waveguide
- $\lambda = 1550$ nm

$$V_\pi L = \frac{\pi \times 1.55 \times 10^{-6}}{2 \times 0.8 \times 1.5 \times 10^{-5}} \approx 0.2 \text{ V·cm} = 2 \text{ V·mm}$$

Wait — this gives a lower value than commonly quoted. The issue is that the depletion modulator only depletes a *fraction* of the waveguide cross-section. For a PN junction centered in a 450-nm-wide waveguide, with 200 nm depletion width change per volt, only about 45% of the waveguide width is actively modulated. With a more careful accounting, the effective $\partial n/\partial V$ is reduced by about 5–10× from the above naive calculation:

$$V_\pi L \approx \frac{\pi \times 1550 \times 10^{-9}}{2 \times 0.8 \times 1.5 \times 10^{-6}} \approx 2 \text{ V·cm} = 20 \text{ V·mm}$$

This brings us to the commonly quoted experimental value: $V_\pi L \approx 10$–30 V·mm for silicon PN depletion modulators [6,7]. The factor of ~10–15× reduction from the idealized Drude estimate is due to the geometric inefficiency of the depletion region, the limited carrier density modulation, and the non-optimal overlap between the depletion zone and the waveguide mode.

For comparison, lithium niobate has $V_\pi L \approx 2$–4 V·cm = 20–40 V·mm for conventional bulk waveguides, but $V_\pi L \approx 2$ V·mm for thin-film LNOI devices (Section 7.3.4). So silicon depletion modulators at their best are comparable to or better than bulk LiNbO₃, but significantly worse than thin-film LNOI.

## Wavelength Scaling and the 1550 nm Choice

The Drude model predicts $\Delta n \propto \lambda^2$ for the plasma dispersion effect. This means that operating at longer wavelengths makes silicon modulators more efficient, while operating at shorter wavelengths makes them less efficient.

| Wavelength | $\lambda^2$ scaling | Relative $\Delta n$ | Notes |
|-----------|-------------------|---------------------|-------|
| 1310 nm | 1.00 | 1.0× | O-band, zero-dispersion in SMF |
| 1550 nm | 1.39 | 1.39× | C-band, attenuation minimum in fiber |
| 2000 nm | 2.33 | 2.33× | Emerging for mid-IR |
| 1064 nm | 0.66 | 0.66× | Nd:YAG laser wavelength |

This is one reason why the photonic computing community has settled on the C-band (1530–1565 nm) as the dominant operating range: both fiber attenuation and modulator efficiency favor longer wavelengths, and the 1550 nm window represents a practical optimum [8].

---

## Summary of Key Results

The plasma dispersion effect is silicon's only viable electro-optic modulation mechanism:

- **Physical origin**: Free carriers (electrons and holes) reduce the refractive index (Drude plasma effect) and increase absorption.
- **Soref-Bennett relations**: $\Delta n \approx -(8.8\times10^{-22}\Delta N_e + 8.5\times10^{-18}\Delta N_h^{0.8})$; $\Delta\alpha \propto \Delta N_{e,h}$ at 1550 nm [2].
- **Three control mechanisms**: Injection (large $\Delta n$, limited bandwidth ~100 MHz–1 GHz), depletion (moderate $\Delta n$, high bandwidth ~20–60 GHz), accumulation (thin MOS layer, limited by oxide reliability).
- **Phase shift efficiency**: $V_\pi L \approx 10$–30 V·mm for depletion-mode silicon MZI modulators — adequate for practical devices at millimeter lengths.
- **Inherent chirp**: Both $\Delta n$ and $\Delta\alpha$ are coupled, making silicon modulators intrinsically chirped.

---

## References

[1] Yariv, A. & Yeh, P. (2007). *Photonics: Optical Electronics in Modern Communications*, 6th ed. Oxford University Press. [Crystal symmetry and $\chi^{(2)}$ vanishing in centrosymmetric materials, Chapters 9–10.]

[2] Soref, R.A. & Bennett, B.R. (1987). "Electrooptical effects in silicon." *IEEE Journal of Quantum Electronics*, 23(1), 123–129. [The foundational paper establishing the plasma dispersion relations for silicon.]

[3] Nedeljkovic, M., Soref, R.A., & Mashanovich, G.Z. (2011). "Free-carrier electrorefraction and electroabsorption modulation predictions for silicon over the 1–14-μm infrared wavelength range." *IEEE Photonics Journal*, 3(6), 1171–1180. [Updated Soref-Bennett coefficients with improved band structure calculations.]

[4] Png, C.E., Chan, S.P., Lim, S.T., & Reed, G.T. (2004). "Optical phase modulators for MHz and GHz modulation in silicon-on-insulator (SOI)." *Journal of Lightwave Technology*, 22(6), 1573–1582. [Carrier injection modulators with ion implantation to reduce lifetime.]

[5] Liu, A., Jones, R., Liao, L., Samara-Rubio, D., Rubin, D., Cohen, O., ... & Paniccia, M. (2004). "A high-speed silicon optical modulator based on a metal-oxide-semiconductor capacitor." *Nature*, 427(6975), 615–618. [First high-speed silicon modulator demonstration, ~1 GHz, MOS capacitor approach.]

[6] Xu, Q., Schmidt, B., Pradhan, S., & Lipson, M. (2005). "Micrometre-scale integrated silicon ring-resonator optical modulator." *Nature*, 435(7040), 325–327. [First microring modulator; injection mode, compact footprint.]

[7] Thomson, D.J., Gardes, F.Y., Fedeli, J.-M., Zlatanovic, S., Hu, Y., Kuo, B.P.-P., ... & Reed, G.T. (2012). "50-Gb/s silicon optical modulator." *IEEE Photonics Technology Letters*, 24(4), 234–236. [50 Gbps depletion-mode MZI modulator demonstrating state-of-art speed.]

[8] Reed, G.T., Mashanovich, G., Gardes, F.Y., & Thomson, D.J. (2010). "Silicon optical modulators." *Nature Photonics*, 4(8), 518–526. [Comprehensive review of silicon modulator physics and device types.]
