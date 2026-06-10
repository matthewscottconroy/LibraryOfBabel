# 3.1.3 — Dispersion and Absorption

## Normal and Anomalous Dispersion

The *dispersion* of a medium is the variation of its refractive index with frequency (or wavelength). The sign and magnitude of $dn/d\omega$ (or $dn/d\lambda$) have profound consequences for optical pulse propagation.

**Normal dispersion**: $dn/d\omega > 0$ (equivalently, $dn/d\lambda < 0$). The refractive index increases with increasing frequency — blue light travels more slowly than red light. This is the behavior of most optical materials in their transparency window, far from any resonance. It is "normal" in the sense that it causes a prism to refract blue light more than red (the familiar rainbow-like dispersion seen in glass).

**Anomalous dispersion**: $dn/d\omega < 0$ (equivalently, $dn/d\lambda > 0$). The refractive index *decreases* with increasing frequency. This occurs near an absorption resonance, in the region where $\omega > \omega_0$ and the oscillator response contributes a negative $\chi_e'$ with magnitude increasing with frequency.

From the Lorentz oscillator:

$$\frac{d\chi_e'}{d\omega} = \frac{2\omega_p^2\omega[(\omega_0^2-\omega^2)^2 - \gamma^2\omega^2/2 - ...]}{[(\omega_0^2-\omega^2)^2+\gamma^2\omega^2]^2}$$

In the transparent region far below resonance ($\omega \ll \omega_0 - \gamma$): $d\chi_e'/d\omega > 0$, so $dn/d\omega > 0$ (normal dispersion).

In the absorption band ($|\omega - \omega_0| \lesssim \gamma$): the dispersive $\chi_e'$ curve slopes negatively through zero at $\omega = \omega_0$, giving $dn/d\omega < 0$ (anomalous dispersion).

Above the resonance ($\omega \gg \omega_0 + \gamma$): $\chi_e' < 0$ and $|d\chi_e'/d\omega|$ is small; normal dispersion resumes.

## Group Velocity and Group Velocity Dispersion

The *group index* $n_g = n - \lambda\,dn/d\lambda = n + \omega\,dn/d\omega$ determines the group velocity $v_g = c/n_g$. For anomalous dispersion ($dn/d\omega < 0$): $n_g < n$, and light pulses travel faster than in normal dispersion. Near a strong resonance, $n_g$ can become very large (*slow light*) or even less than 1 (*fast light* — not a violation of relativity; the group velocity can exceed $c$ but the signal velocity cannot [1]).

**Group velocity dispersion (GVD)**:

$$\beta_2 = \frac{d^2k}{d\omega^2} = \frac{d}{d\omega}\left(\frac{n_g}{c}\right) = \frac{1}{c}\frac{dn_g}{d\omega}$$

$\beta_2 > 0$: *normal* GVD (lower frequencies travel faster — red leads blue). $\beta_2 < 0$: *anomalous* GVD (higher frequencies travel faster — blue leads red).

For silica fiber, $\beta_2 = 0$ (zero-dispersion point) at $\lambda \approx 1270$ nm for bulk silica and at $\lambda = 1310$ nm for standard single-mode fiber (waveguide dispersion shifts the zero). The choice of 1550 nm for telecom (in the anomalous dispersion regime, $\beta_2 \approx -20$ ps²/km for standard SMF) enables soliton propagation (Section 3.3.5) but requires dispersion compensation for long-haul links.

## The Sellmeier Dispersion of Silica

The three-term Sellmeier equation for fused silica (Malitson 1965):

$$n^2(\lambda) = 1 + \frac{0.6961663\,\lambda^2}{\lambda^2 - 0.0684043^2} + \frac{0.4079426\,\lambda^2}{\lambda^2 - 0.1162414^2} + \frac{0.8974794\,\lambda^2}{\lambda^2 - 9.8961609^2}$$

(wavelengths in μm). At $\lambda = 1550$ nm: $n = 1.4440$, consistent with Table 1 in Chapter 1.

The *dispersion parameter* $D$ (more commonly used in fiber optics than $\beta_2$):

$$D = -\frac{\lambda}{c}\frac{d^2n}{d\lambda^2} = -\frac{2\pi c}{\lambda^2}\beta_2 \quad \text{(ps/nm/km)}$$

For standard single-mode fiber at 1550 nm: $D \approx +17$ ps/(nm·km), meaning that a 1 nm wavelength difference in a pulse causes 17 ps of additional delay per km of fiber. Over 1000 km, a pulse with 1 nm bandwidth spreads to 17 ns — ruinous for high-speed data unless compensated.

## The Zero-Dispersion Wavelength

The zero-dispersion wavelength $\lambda_0$ (where $D = 0$, equivalently $\beta_2 = 0$) is the wavelength at which the material dispersion (from the Sellmeier equation, normal for silica) and waveguide dispersion (from the waveguide geometry, which contributes anomalous dispersion for small cores) cancel. By adjusting the waveguide design (core radius and index contrast), $\lambda_0$ can be shifted over a range of hundreds of nm.

**Dispersion-shifted fiber (DSF)**: $\lambda_0$ shifted to 1550 nm. Minimizes dispersion at the transmission wavelength — advantageous for single-channel links but problematic for WDM (channels at different wavelengths experience different delays, and nonlinear effects like four-wave mixing are enhanced near $\lambda_0$).

**Dispersion-compensating fiber (DCF)**: high $|D|$ with sign opposite to standard SMF. Short lengths of DCF compensate the dispersion accumulated over long spans of SMF.

## Absorption in Silicon

Silicon is transparent at wavelengths above its bandgap ($\lambda_g = hc/E_g = 1127$ nm for $E_g = 1.12$ eV). Below 1127 nm, photons have enough energy to promote electrons from the valence band to the conduction band — linear absorption.

At 1550 nm (below the bandgap by 200 nm, i.e., $\Delta E = 0.16$ eV): intrinsic silicon has negligible one-photon absorption. However:

**Two-photon absorption (TPA)**: two 1550 nm photons together have energy $2 \times 0.80 = 1.60$ eV > $E_g = 1.12$ eV. At high intensities, two photons can be absorbed simultaneously (a nonlinear process), generating free carriers. The TPA coefficient of silicon at 1550 nm is $\beta_\text{TPA} \approx 0.5$ cm/GW [2]. For a 1 mm waveguide with mode area $A_\text{eff} = 0.1$ μm² carrying 100 mW peak power: intensity $I = P/A = 10^{12}$ W/m² = $10^8$ W/cm², giving TPA-induced absorption $\alpha_\text{TPA} = \beta_\text{TPA} I = 0.5 \times 10^{-8} = 5 \times 10^{-9}$ cm⁻¹ — negligible at low power. But at higher powers or for ultrashort pulses, TPA becomes significant and generates free carriers that further absorb the signal (free-carrier absorption, FCA).

## Material Absorption in the Telecom Window

| Material | Absorption at 1550 nm | Primary cause |
|----------|----------------------|---------------|
| Silica fiber | 0.2 dB/km ($4.6 \times 10^{-5}$ cm⁻¹) | Rayleigh scattering (density fluctuations) |
| Silicon waveguide | 2–3 dB/cm (0.46–0.69 cm⁻¹) | Sidewall roughness scattering |
| SiO₂ cladding | $< 0.001$ dB/cm | Negligible |
| Si₃N₄ waveguide | 0.001–0.01 dB/cm | Low; material absorption near band edge |
| LiNbO₃ | $< 0.1$ dB/cm | Material absorption; OH contamination |
| InP (intrinsic) | $< 1$ dB/cm | Below bandgap; residual impurity absorption |

The key message: the dominant loss mechanism in silicon waveguides is *scattering* (not material absorption), arising from nanometer-scale roughness on the etched sidewalls. This is why improving silicon waveguide fabrication focuses on reducing sidewall roughness — this is a materials and lithography challenge, not a fundamental optical limit.

## Summary

- Normal dispersion: $dn/d\omega > 0$; below resonance. Anomalous: $dn/d\omega < 0$; above resonance.
- GVD $\beta_2 = d^2k/d\omega^2$: determines pulse spreading in dispersive media.
- Silica fiber zero-dispersion wavelength: $\sim 1270$ nm (material); $\sim 1310$ nm (standard SMF).
- Silicon transparent above 1127 nm; dominant loss in Si waveguides is sidewall roughness scattering.

---

*References*

[1] Brillouin, L. (1960). *Wave Propagation and Group Velocity*. Academic Press. [Classic treatment of signal velocity, group velocity, and information transmission in dispersive media.]

[2] Dinu, M., Quochi, F., & Garcia, H. (2003). Third-order nonlinearities in silicon at telecom wavelengths. *Applied Physics Letters*, 82(18), 2954–2956. [DOI: 10.1063/1.1571665]
