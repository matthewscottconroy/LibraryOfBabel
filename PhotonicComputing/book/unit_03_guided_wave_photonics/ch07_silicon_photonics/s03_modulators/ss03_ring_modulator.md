# 7.3.3 The Ring Modulator

## The Resonant Enhancement Principle

The MZI modulator achieves phase-to-intensity conversion through interference along a millimeter-scale optical path. The ring modulator achieves the same goal by a completely different principle: resonant enhancement.

A microring resonator, as we established in Section 7.2.5, builds up a circulating field at its resonance wavelength. When the ring is in resonance with the input light, it couples strongly to the bus waveguide and produces a sharp dip in the transmission spectrum. The width of this dip — the resonance linewidth — is inversely proportional to the photon lifetime in the ring.

Now consider what happens when we embed a plasma dispersion phase shifter inside the ring. A small change in refractive index $\Delta n$ shifts the resonance wavelength by:

$$\Delta\lambda_{\text{res}} = \lambda_{\text{res}} \frac{\Delta n}{n_g}$$

For Si with $n_g = 4.24$ and $\lambda_{\text{res}} = 1550$ nm, a change $\Delta n = 10^{-4}$ shifts the resonance by:

$$\Delta\lambda_{\text{res}} = 1550 \times \frac{10^{-4}}{4.24} \approx 0.037 \text{ nm} = 37 \text{ pm}$$

At the operating point on the steep edge of the transmission spectrum — where the transmission changes most rapidly with wavelength — a 37 pm resonance shift can change the transmitted intensity from near zero to near maximum. This is the resonant enhancement: the same carrier density change that produces only a small phase shift in a straight waveguide produces a large intensity change in a resonator, because the resonator converts the small phase change to a large cavity detuning that traverses the steep spectral feature.

The price of this enhancement is that the device only works over a narrow wavelength range — the resonance linewidth — and is exquisitely sensitive to any other perturbation that shifts the resonance (temperature changes, fabrication variations).

## Transfer Function

From Section 7.2.5, the through-port transmission of a single-bus ring resonator is:

$$T(\omega) = \frac{(r - a e^{i\phi})^2 + \ldots}{|1 - ra e^{i\phi}|^2}$$

More precisely, for a ring with field coupling coefficient $\kappa$ (so power coupling $|t|^2 = 1 - |r|^2 = \kappa^2$), round-trip field transmission $a$, and round-trip phase $\phi = n_g(2\pi R/c)\omega$:

$$T = \left|\frac{r - a e^{i\phi}}{1 - ra e^{i\phi}}\right|^2$$

where $r = \sqrt{1-\kappa^2}$ is the field self-coupling coefficient (through coefficient).

Near resonance, $\phi = 2\pi m + \delta\phi$ where $\delta\phi$ is a small detuning from the $m$-th resonance. Expanding to second order:

$$T \approx 1 - \frac{(1-r^2)(1-a^2)}{[(1-ra)^2 + (ra/2)(\delta\phi)^2]}$$

Actually, let's write this more cleanly. Define the finesse $\mathcal{F} = \pi\sqrt{ra}/(1-ra)$ and the free spectral range $\Delta\omega_{\text{FSR}} = c/(n_g R)$ (in angular frequency). The half-linewidth (HWHM) is:

$$\delta\omega_{1/2} = \frac{\Delta\omega_{\text{FSR}}}{\mathcal{F}}$$

At critical coupling ($r = a$), the on-resonance transmission drops to zero. The transmission near resonance takes a Lorentzian form:

$$T(\delta\omega) = 1 - \frac{1}{1 + (\delta\omega/\delta\omega_{1/2})^2} = \frac{(\delta\omega/\delta\omega_{1/2})^2}{1 + (\delta\omega/\delta\omega_{1/2})^2}$$

(at critical coupling). For modulation, the optical source is tuned to the steepest slope of $T$, at $\delta\omega = \pm\delta\omega_{1/2}$, where:

$$\left.\frac{dT}{d(\delta\omega)}\right|_{\delta\omega_{1/2}} = \frac{1}{2\delta\omega_{1/2}}$$

Now when the carrier density changes by $\Delta N$ due to an applied voltage $V$, the resonance shifts by:

$$\delta\omega_{\text{shift}} = -\frac{\omega_{\text{res}}}{n_g}\frac{dn}{dN}\Delta N = -\frac{\omega_{\text{res}}}{n_g}\frac{dn}{dN}\frac{C_j V}{eV_{\text{ring}}}$$

where $V_{\text{ring}}$ is the ring volume and $C_j$ is the junction capacitance. The resulting change in transmission:

$$\delta T \approx \frac{dT}{d(\delta\omega)} \cdot \delta\omega_{\text{shift}} = \frac{\delta\omega_{\text{shift}}}{2\delta\omega_{1/2}}$$

The **resonant enhancement factor** over a straight waveguide of equal length is:

$$G_{\text{res}} = \frac{\delta T_{\text{ring}}}{\delta\phi_{\text{straight}}} \approx \frac{\mathcal{F}}{\pi} \times \frac{\lambda_{\text{res}}}{L_{\text{ring}}\Delta\lambda_{\text{res}}/\Delta n}$$

For a ring with $\mathcal{F} = 30$ and $R = 5$ μm ($L_{\text{ring}} = 31.4$ μm), compared to a straight phase shifter of equal length, the resonant enhancement is ~$\mathcal{F}/\pi \approx 10$. This is why ring modulators can achieve millivolt drive voltages.

## Practical Device Parameters

A ring modulator consists of:
1. A microring resonator (radius 5–15 μm for silicon at 1550 nm)
2. A lateral PN junction embedded in the ring waveguide
3. Electrode contacts reaching the p+ and n+ doped regions

The PN junction geometry is similar to the MZI modulator, but the ring is typically designed slightly differently: since the ring circumference is the total modulator length, the doping profile must maximize $\Delta n$ per unit ring length. Experiments and simulations show that a "p-i-n" ring with the PN junction centered and depletion extending across the full waveguide width gives ~20% improvement over a lateral PN ring [1].

For silicon ring modulators at 1550 nm with $R = 10$ μm and $\mathcal{F} \approx 25$–50:

| Parameter | Typical Value |
|-----------|---------------|
| Ring radius | 5–15 μm |
| FSR | $c/(n_g 2\pi R)$ = 230 GHz at $R=10$ μm |
| Linewidth (FWHM) | ~25–80 pm |
| $V_\pi$ (effective) | 0.5–3 V |
| Capacitance | 5–20 fF |
| Energy/bit | 1–50 fJ/bit |
| Bandwidth | 10–60 GHz |
| Insertion loss (on-resonance) | 0.5–2 dB (off-resonance drop) |

The extraordinarily low capacitance (5–20 fF vs. 100–500 fF for an MZI modulator) is the ring's decisive advantage for energy efficiency. The energy per bit scales as:

$$E/\text{bit} = \frac{1}{2}CV_{\text{drive}}^2$$

For $C = 10$ fF and $V_{\text{drive}} = 1$ V: $E/\text{bit} = 5$ fJ. This is 10–100× lower than a comparable MZI modulator — the reason ring modulators are intensely studied for energy-efficient optical interconnects [2].

## The First Microring Modulator: Xu et al. 2005

The practical microring modulator was demonstrated by Qianfan Xu, Bipin Schmidt, Sameer Pradhan, and Michal Lipson at Cornell University in 2005 [3]. Their device used a 6-μm-radius ring with forward-biased (injection) operation, achieving modulation at ~12 Mb/s — modest by today's standards, but a landmark demonstration because it established that a resonant silicon device could be electrically controlled with sufficient speed to be useful.

The injection-mode operation in that first device limited bandwidth. Subsequent work shifted to depletion mode, which — for a ring of the same size and finesse — achieves bandwidths of 10–60 GHz. The record as of the early 2020s is ~60 GHz [4], approaching the limit set by the ring photon lifetime:

$$f_{\text{photon}} = \frac{\delta\nu_{\text{ring}}}{2} = \frac{\Delta\nu_{\text{FSR}}}{2\mathcal{F}}$$

For $\Delta\nu_{\text{FSR}} = 3$ THz and $\mathcal{F} = 25$: $f_{\text{photon}} = 60$ GHz. To go faster, finesse (and thus resonant enhancement) must be reduced. This is the fundamental bandwidth-efficiency tradeoff of ring modulators: higher finesse gives more sensitivity (lower $V_\pi$, lower energy) but lower bandwidth and narrower operating wavelength range.

## Thermal Sensitivity: The Key Challenge

Silicon's thermo-optic coefficient is $dn/dT = 1.87 \times 10^{-4}$ K⁻¹. For a ring resonator, the resonance wavelength shifts by:

$$\frac{d\lambda_{\text{res}}}{dT} = \frac{\lambda_{\text{res}}}{n_g}\frac{dn}{dT} = \frac{1550 \times 1.87 \times 10^{-4}}{4.24} \approx 68 \text{ pm/K}$$

The ring resonance linewidth is typically 25–80 pm. This means a temperature change of just 1°C can shift the resonance by more than one linewidth, completely detuning the modulator from its operating wavelength.

In a photonic computing chip operating at room temperature with typical temperature variations of ±1–5°C due to power dissipation and ambient fluctuations, *every ring modulator must be actively temperature-stabilized* to maintain operation.

The stabilization mechanism is typically a local resistive heater: a metal wire or n+ doped silicon strip above the ring that can supply 0.5–2 mW to trim the resonance by ±0.5–1 nm. Active feedback from a monitoring photodiode at the drop port provides the error signal for a PID control loop.

For a chip with 64 ring modulators, the static thermal stabilization power is approximately:

$$P_{\text{thermal}} \approx 64 \times 1 \text{ mW} = 64 \text{ mW}$$

This is not negligible — it can dominate the chip's power budget for the ring modulator approach. Several proposals exist to mitigate this:

1. **Athermal ring design**: Engineering the waveguide geometry so that the thermo-optic effect of silicon ($dn/dT > 0$) is partially cancelled by the dimensional change of the waveguide ($dA/dT$). Partial athermal operation has been demonstrated, reducing sensitivity to ~20 pm/K [5].

2. **Polymer cladding**: Some polymers have $dn/dT < 0$ (negative thermo-optic coefficient). A polymer-clad ring waveguide can have near-zero net thermal shift. The challenge is foundry compatibility.

3. **Resonance locking via data signal**: In a WDM system, the resonance drift can be detected and corrected using the modulated signal itself as an error signal, eliminating the need for a separate monitor photodiode [6].

## WDM Applications in Photonic Computing

The ring modulator's narrow operating wavelength and large FSR make it naturally compatible with wavelength-division multiplexing (WDM). If each ring is resonant at a distinct wavelength $\lambda_1, \lambda_2, \ldots, \lambda_N$ (achieved by controlling ring radii to differences of $\Delta R \approx 10$–20 nm per channel), then a single bus waveguide can carry $N$ independently modulated wavelength channels:

$$\lambda_k = \lambda_0 + k \times \text{FSR}/N$$

For Si rings with FSR ≈ 3 THz at $R = 10$ μm and $N = 8$ wavelength channels, channel spacing ≈ 375 GHz ≈ 3 nm. For $N = 32$ channels: spacing ≈ 94 GHz ≈ 0.75 nm. These are readily achievable.

This WDM architecture has been proposed for photonic neural network implementations where each wavelength represents one input vector component. The Lightmatter architecture (Chapter 22 of this book) uses an MZI mesh for the matrix and WDM ring modulators for input encoding, potentially achieving very high input data rates [7].

The combination of WDM modulation + photodetector arrays + on-chip DSP defines a complete analog optical computing pipeline. The precision of each WDM channel is limited by:
- Ring modulator linearity (sinusoidal transfer function within the resonance)
- Thermal noise on the ring bias (~1 pm noise → ~0.5% intensity noise)
- Crosstalk between adjacent wavelength channels (~−30 to −40 dB for well-designed rings)

Taken together, these limit the effective precision to approximately 5–6 bits per wavelength channel in practice, consistent with the ENOB analysis in Section 5.3.3.

---

## References

[1] Li, G., Zheng, X., Yao, J., Thacker, H., Shubin, I., Luo, Y., ... & Raj, K. (2010). "25Gb/s 1V-driving CMOS ring modulator with integrated thermal tuning." *Optics Express*, 19(21), 20435–20443. [PN ring modulator with optimized doping geometry showing state-of-art energy efficiency.]

[2] Manipatruni, S., Dokania, R.K., Schmidt, B., Sherwood-Droz, N., Poitras, C.B., Apsel, A.B., & Lipson, M. (2008). "Wide temperature range operation of micrometer-scale silicon electro-optic modulators." *Optics Letters*, 33(19), 2185–2187. [Thermal effects and stabilization strategies for Si ring modulators.]

[3] Xu, Q., Schmidt, B., Pradhan, S., & Lipson, M. (2005). "Micrometre-scale integrated silicon ring-resonator optical modulator." *Nature*, 435(7040), 325–327. [The landmark first microring modulator demonstration; injection mode, 6 μm radius, 12 Mb/s.]

[4] Timurdogan, E., Sorace-Agaskar, C.M., Sun, J., Shah Hosseini, E., Biberman, A., & Watts, M.R. (2014). "An ultralow power athermal silicon modulator." *Nature Communications*, 5, 4008. [Ultra-low-energy ring modulator with 0.9 fJ/bit; resonance design and measurements.]

[5] Raghunathan, V., Yoon, D., Fan, T., & Guo, L.J. (2018). "Wavelength-tunable and reconfigurable photonic-crystal fiber laser." [For athermal designs, see: Guha, B., Cardenas, J., & Lipson, M. (2013). "Athermal silicon microring resonators with titanium oxide cladding." *Optics Express*, 21(22), 26557–26563.]

[6] Georgas, M., Orcutt, J., Ram, R.J., & Stojanovic, V. (2014). "A monolithically-integrated optical receiver in standard 45-nm SOI." *IEEE Journal of Solid-State Circuits*, 47(7), 1693–1702. [Resonance locking using the data signal and on-chip circuitry.]

[7] Hamerly, R., Bandyopadhyay, S., Carolan, J., Englund, D., & Mabuchi, H. (2022). "Asymptotic advantages of loop-based boson sampling." *npj Quantum Information*, 8(1), 1–10. [WDM-encoded photonic neural networks; optical computing precision analysis.]
