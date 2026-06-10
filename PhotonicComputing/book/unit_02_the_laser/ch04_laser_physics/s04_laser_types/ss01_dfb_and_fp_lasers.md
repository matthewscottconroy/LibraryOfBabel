# 4.4.1 DFB and Fabry-Pérot Semiconductor Lasers

## The Fabry-Pérot Semiconductor Laser

The simplest semiconductor laser is a rectangular chip of direct-bandgap material with cleaved facets acting as mirrors. The cleaved facet reflectivity for InP ($n \approx 3.17$) is:

$$R = \left(\frac{n-1}{n+1}\right)^2 = \left(\frac{2.17}{4.17}\right)^2 \approx 0.27$$

This 27% reflectivity is sufficient for lasing because the gain per pass ($e^{\Gamma g_{th} L}$) can be large for short cavities (L = 250–500 μm) and high material gain.

A Fabry-Pérot laser oscillates in multiple longitudinal modes simultaneously, as discussed in Section 4.2.1. The spectral output consists of a comb of modes within the gain bandwidth, separated by ~1–2 nm, with the dominant mode shifting with temperature and current. This makes FP lasers unsuitable for:
- WDM systems (multiple modes occupy multiple channels)
- Long-reach coherent transmission (multi-mode output severely reduces coherence)
- High-precision photonic computing (uncertain operating wavelength)

FP lasers are, however, simple, cheap, and used in low-cost optical interconnects (e.g., multimode fiber links within data center racks at 850 nm).

## The Distributed Feedback (DFB) Laser

The DFB laser solves the multi-mode problem by incorporating a Bragg grating directly into the laser waveguide. The grating provides frequency-selective reflection at the Bragg wavelength:

$$\Lambda = \frac{m\lambda_B}{2n_{eff}}$$

where $\Lambda$ is the grating period, $m$ is the diffraction order, $\lambda_B$ is the Bragg wavelength, and $n_{eff}$ is the effective refractive index of the waveguide mode. For $\lambda_B = 1550$ nm, $n_{eff} = 3.2$, first-order grating: $\Lambda = 242$ nm.

The grating provides distributed feedback — reflections from each grating period add coherently at $\lambda_B$ and destructively at other wavelengths. The reflection spectrum of the grating is a Gaussian (or sinc-squared) centered on $\lambda_B$ with bandwidth approximately $\Delta\lambda_{BW} \approx \lambda_B^2/(n_{eff} L)$ for a grating of length $L$.

**Single-mode operation**: A DFB laser with sufficient grating coupling strength ($\kappa L > 1$, where $\kappa$ is the coupling coefficient in cm$^{-1}$) operates in a single longitudinal mode at $\lambda_B$, rejecting all other modes by > 30–40 dB (side-mode suppression ratio, SMSR).

**Quarter-wave shift (QWS) DFB**: A standard DFB grating has two degenerate modes on either side of the stopband. Adding a $\lambda/4$ phase shift at the center of the grating breaks this degeneracy and forces single-mode operation in the center of the stopband. QWS DFBs are the standard architecture for > 40 dB SMSR, narrow-linewidth semiconductor lasers.

## DFB Laser Parameters

| Parameter | Typical value | Significance for photonic computing |
|---|---|---|
| Threshold current | 5–20 mA | Sets minimum operating power |
| Operating wavelength | ITU-T grid ±0.1 nm | WDM channel assignment |
| Wavelength temperature coefficient | ~0.1 nm/°C | Requires temperature stabilization (TEC) |
| Free-running linewidth | 1–10 MHz | Sets coherence length (~30–300 m) |
| Linewidth (with optical feedback) | 10–100 kHz | Extended coherence length |
| SMSR | > 40 dB | Single-mode purity |
| Relative intensity noise (RIN) | –145 to –155 dBc/Hz | Analog noise floor |
| Modulation bandwidth | 20–30 GHz (direct) | On-off keying data rate |
| Wall-plug efficiency | 20–35% | Power budget |
| Output power | 1–20 mW | Available optical power |

## Temperature Stabilization

The DFB Bragg wavelength is temperature-sensitive primarily through the thermal dependence of $n_{eff}$:

$$\frac{d\lambda_B}{dT} = \lambda_B \frac{1}{n_{eff}}\frac{dn_{eff}}{dT} \approx 0.10 \text{ nm/°C for InP at 1550 nm}$$

For a WDM system with 100 GHz (0.8 nm) channel spacing, a ±4 nm wavelength stability window means the temperature must be controlled to ±4 nm / (0.1 nm/°C) = ±40°C. For 50 GHz (0.4 nm) spacing: ±20°C. In practice, photonic computing systems using WDM use thermoelectric coolers (TECs) to maintain laser temperature to ±0.1°C, consuming ~500 mW per laser — comparable to or exceeding the optical output power itself.

This temperature stabilization power is one of the practical challenges for large-scale WDM photonic computing: a 64-wavelength system with 64 individually temperature-controlled DFB lasers consumes ~32 W just for wavelength stabilization.

## The Distributed Bragg Reflector (DBR) Laser

A close variant of the DFB, the DBR laser separates the gain section from the reflector sections. Two Bragg grating mirrors (DBRs) act as wavelength-selective end mirrors, with a separate current-pumped gain section between them. Injecting current into the DBR sections changes the carrier density and hence the grating effective index, tuning the Bragg wavelength. DBR lasers achieve wider tuning ranges than DFBs (~10 nm) without the temperature dependence, using fast current tuning instead.

For photonic computing applications requiring wavelength-agile sources (e.g., reconfigurable WDM matrix processors), sampled-grating DBR (SGDBR) and other widely tunable laser architectures achieve tuning ranges of 40–80 nm with millisecond-scale switching [1].

## References

[1] Coldren, L.A. (2000). "Monolithic tunable diode lasers." *IEEE Journal of Selected Topics in Quantum Electronics*, 6(6), 988–999.
