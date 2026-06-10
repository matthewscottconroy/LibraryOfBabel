# 7.5.1 Silicon Nitride (Si₃N₄)

## Why Silicon Nitride?

Silicon nitride appeared as a waveguide material almost as a footnote to silicon photonics development — an encapsulation layer and etch mask in early SOI processes — and then quietly became one of the most important photonic platforms in its own right. The reason is a combination of properties that silicon conspicuously lacks.

The key comparison between Si and Si₃N₄ for photonic waveguides at 1550 nm:

| Property | Si | Si₃N₄ | Significance |
|----------|-----|---------|-------------|
| Refractive index $n$ | 3.478 | 1.997 | Si has 4× stronger confinement |
| Bandgap | 1.12 eV | ~5 eV | Si₃N₄ transparent UV–mid-IR |
| Two-photon absorption $\beta_{TPA}$ | 0.5–0.8 cm/GW | ~0 | Si₃N₄ has essentially no TPA at 1550 nm |
| Nonlinear index $n_2$ | $6\times10^{-18}$ m²/W | $2.5\times10^{-19}$ m²/W | Si is 24× more nonlinear |
| Thermo-optic $dn/dT$ | $1.87\times10^{-4}$ K⁻¹ | $2.5\times10^{-5}$ K⁻¹ | Si₃N₄ is 7.5× more thermally stable |
| $\chi^{(2)}$ | 0 | 0 | Both centrosymmetric (no Pockels) |
| Propagation loss | 1–3 dB/cm | 0.01–1 dB/cm | Si₃N₄ can be much lower loss |

The combination of no TPA, low thermo-optic coefficient, and low propagation loss makes Si₃N₄ the platform of choice for:
- **Low-noise frequency combs** (microresonator combs for WDM photonic computing)
- **Precision analog photonic circuits** (where TPA-induced nonlinearity and thermo-optic drift in Si degrade precision)
- **High-power optical circuits** (Si is TPA-limited above ~10 mW; Si₃N₄ handles watts)
- **Ultrabroadband photonics** (Si₃N₄ transparent at 400 nm–4 μm, vs. Si's 1.1–8 μm)

## Waveguide Parameters

Silicon nitride waveguides are typically deposited by low-pressure chemical vapor deposition (LPCVD) or plasma-enhanced CVD (PECVD), then patterned by reactive ion etching. The stoichiometry Si₃N₄ (3:4 ratio) is the target, though slight deviations produce SiN$_x$ materials with different indices.

The standard waveguide cross-section for high-confinement Si₃N₄ at 1550 nm is approximately 800 × 400 nm (width × height), deposited on a SiO₂ lower cladding. The refractive indices:
- Si₃N₄ core: $n = 1.997$ at 1550 nm
- SiO₂ cladding: $n = 1.444$ at 1550 nm
- Numerical aperture: $\text{NA} = \sqrt{1.997^2 - 1.444^2} = \sqrt{3.988 - 2.085} \approx 1.38$

The effective index for the fundamental TE mode in an 800 × 400 nm waveguide: $n_{\text{eff}} \approx 1.65$, with group index $n_g \approx 1.9$.

For single-mode operation, the waveguide must be narrow enough to cut off the second-order TE mode. For a Si₃N₄ slab waveguide of height 400 nm:

$$V = \frac{2\pi \times 400 \times 10^{-9} \times 1.38}{1550 \times 10^{-9}} \approx 2.24$$

With $V_c(\text{LP}_{11}) = 2.405$, this slab is just below cutoff — single-mode operation. For the 2D strip waveguide at 800 nm width, the TE₁₀ mode is guided; the TE₂₀ cutoff width is ~1.2 μm. This is less restrictive than silicon's 450-nm constraint.

## Anomalous Dispersion Engineering

One of Si₃N₄'s most consequential advantages for photonic computing is its engineerable dispersion. Microresonator Kerr frequency combs (Section 4.4.4) require anomalous group velocity dispersion at the pump wavelength. Silicon has anomalous dispersion at 1550 nm for narrow waveguides, but TPA kills the comb state. Si₃N₄ has the same qualitative dispersion tunability but without TPA.

The group velocity dispersion for a Si₃N₄ waveguide depends on its geometry. The waveguide dispersion contribution $D_W$ (which can be anomalous for appropriate cross-sections) adds to the material dispersion $D_M$ (normal in Si₃N₄ near 1550 nm):

$$D = D_M + D_W$$

For an 800 × 1550 nm cross-section (width × height), $D \approx -10$ ps/(nm·km) — slightly normal. For an 800 × 400 nm cross-section, $D \approx +30$ ps/(nm·km) — anomalous. The transition from normal to anomalous dispersion occurs near a height of ~700 nm for typical widths [1].

Once anomalous dispersion is achieved, the phase-matching condition for four-wave mixing in a microresonator is satisfied, and Kerr combs can form. The key parameter is the **second-order dispersion coefficient** $\beta_2 = -D\lambda^2/(2\pi c)$. For anomalous dispersion ($D > 0$), $\beta_2 < 0$, enabling the modulation instability that seeds comb formation.

## Si₃N₄ Microresonator Combs

The combination of anomalous dispersion, low propagation loss, and no TPA makes Si₃N₄ the leading platform for **dissipative Kerr soliton (DKS) microresonator combs** — coherent frequency combs where all comb lines are phase-locked and equally spaced in frequency.

For a Si₃N₄ ring resonator with:
- Radius $R = 100$ μm
- Propagation loss $\alpha = 0.1$ dB/cm (state-of-art)
- Loaded Q: $Q = 2\pi R n_g / (\alpha_{\text{coupling}}L + \alpha_{\text{prop}}L) \approx 10^6$
- Intrinsic Q: $Q_0 = 2\pi R n_g / (\alpha_{\text{prop}} L) \approx 3\times10^6$

The FSR is:

$$\text{FSR} = \frac{c}{n_g \cdot 2\pi R} = \frac{3\times10^8}{1.9 \times 2\pi \times 100\times10^{-6}} \approx 252 \text{ GHz}$$

Each DKS comb state covers a bandwidth of ~2 THz (set by the pump-resonator detuning and dispersion), providing approximately $2000/252 \approx 8$ comb lines. For broader combs, smaller rings (larger FSR) are used at the cost of higher threshold power.

The threshold power for Kerr comb formation in a Si₃N₄ resonator [2]:

$$P_{\text{threshold}} = \frac{\omega_0 n_0 A_{\text{eff}} Q_L^2 \omega_0}{c n_2 Q_c Q_0^2}$$

For $R = 100$ μm, $A_{\text{eff}} \approx 1$ μm², $Q_0 = 3\times10^6$, $Q_L = 10^6$, $Q_c \approx 1.5\times10^6$:

$$P_{\text{threshold}} \approx 50\text{–}200 \text{ mW}$$

This is accessible with semiconductor pump lasers or erbium-doped fiber amplifiers. State-of-art DKS combs on Si₃N₄ achieve:
- 100+ comb lines with < 1 dB power variation
- Comb coherence: < 1 Hz relative linewidth between adjacent lines
- Pump-to-comb efficiency: 1–10%

For WDM photonic computing, a DKS comb provides a coherent multi-wavelength source: all comb lines are phase-locked and can be individually modulated, demultiplexed, processed, and detected. This enables massively parallel optical computation where the number of simultaneous matrix operations scales with the number of comb lines [3].

## Low-Loss Si₃N₄ and the Stoichiometry Challenge

The main challenge in Si₃N₄ waveguide fabrication is achieving low propagation loss. The fundamental limitation is the Si-H and N-H absorption at 1550 nm (overtones of Si-H and N-H stretching modes near 3000 cm⁻¹). These bonds are introduced during CVD deposition.

For LPCVD Si₃N₄ deposited at 800°C:
- Stoichiometric Si₃N₄: few Si-H and N-H bonds, low absorption loss at 1550 nm
- But high tensile stress (~1 GPa), leading to film cracking for thickness > ~300–400 nm

For thicker films needed for anomalous dispersion (800–1600 nm), stress mitigation is essential. The **photonic Damascene process**, developed at EPFL, deposited Si₃N₄ in pre-etched SiO₂ trenches, allowing thick films with controlled stress relief [4]. This process enabled Si₃N₄ waveguide losses of < 0.001 dB/cm — among the lowest of any waveguide platform.

State-of-art Si₃N₄ propagation losses:
- Damascene process: 0.001–0.01 dB/cm
- Standard LPCVD/etch: 0.1–0.5 dB/cm
- PECVD Si₃N₄: 0.5–5 dB/cm (higher due to H bonds from low-temperature deposition)

## Si₃N₄ in Photonic Computing

Si₃N₄ is used in photonic computing in three primary roles:

1. **Passive routing layer on top of silicon**: In heterogeneous Si/Si₃N₄ platforms (e.g., GlobalFoundries 45CLO), Si₃N₄ is deposited above the Si device layer and provides low-loss routing over long distances, power splitters, and broadband couplers. The Si layer handles high-speed electro-optic functions; Si₃N₄ handles the linear optical network.

2. **Microresonator comb source**: A Si₃N₄ ring or racetrack resonator, pumped by an external laser, generates a DKS comb that seeds a WDM photonic matrix multiplier. The comb provides far more spectral channels (up to 100+) than a laser array, with perfect frequency registration between channels.

3. **Ultra-low-loss delay lines and filters**: Applications requiring very long optical path lengths (e.g., for large matrix implementations via time-domain encoding, or for optical beamforming) use Si₃N₄ for its loss of 0.001–0.1 dB/cm, where silicon's 1–3 dB/cm would be prohibitive.

---

## References

[1] Pfeiffer, M.H.P., Herkommer, C., Liu, J., Morais, T., Zervas, M., Zernickel, M., Kippenberg, T.J. (2018). "Photonic Damascene process for integrated high-Q microresonator based nonlinear photonics." *Optica*, 3(1), 20–25. [Si₃N₄ photonic Damascene with ultra-low loss; dispersion engineering for DKS combs.]

[2] Kippenberg, T.J., Holzwarth, R., & Diddams, S.A. (2011). "Microresonator-based optical frequency combs." *Science*, 332(6029), 555–559. [Review of microresonator Kerr combs including Si₃N₄ threshold and dispersion analysis.]

[3] Feldmann, J., Youngblood, N., Karpov, M., Gehring, H., Li, X., Stappers, M., ... & Bhaskaran, H. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589(7840), 52–58. [PCM + WDM comb-based photonic matrix multiplication demonstration.]

[4] Pfeiffer, M.H.P., Kordts, A., Brasch, V., Zernickel, M., Geiselmann, M., Jost, J.D., & Kippenberg, T.J. (2016). "Photonic Damascene process for integrated high-Q microresonator based nonlinear photonics." *Optica*, 3(1), 20–25. [The definitive reference for photonic Damascene Si₃N₄ fabrication and ultra-low loss.]
