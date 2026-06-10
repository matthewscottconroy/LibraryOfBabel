# 7.2.5 Microring Resonators

## The Ring as a Fabry-Pérot in Circular Geometry

A microring resonator is the integrated photonic analogue of the Fabry-Pérot cavity: light circulates in a closed waveguide loop, resonating when the round-trip optical path length is an integer multiple of the wavelength. Unlike the linear FP, the ring has no end mirrors — feedback is provided by the circular geometry itself.

**Resonance condition**: For a ring of radius $R$ and effective index $n_{eff}$:

$$m\lambda_m = 2\pi R n_{eff}, \quad m = 1, 2, 3, \ldots$$

or equivalently, resonance frequencies:

$$\omega_m = \frac{mc}{n_g R}$$

**Free spectral range**:

$$\Delta\nu_{FSR} = \frac{c}{2\pi n_g R} = \frac{c}{L_{ring} n_g}$$

For $R = 5$ μm, $n_g = 4.2$: $\Delta\nu_{FSR} = 3\times10^8/(2\pi \times 5\times10^{-6} \times 4.2) = 2.27$ THz → wavelength spacing 18.2 nm. This large FSR means only one resonance falls within the C-band (~35 nm) — the ring is well-separated from its next resonance.

## Transfer Function

For a ring resonator with coupling coefficient $\kappa$ to a bus waveguide (field coupling coefficient $t_c$, self-coupling coefficient $r_c$, with $t_c^2 + r_c^2 = 1$ for a lossless coupler), and round-trip field transmission $a$ (including loss), the through-port and drop-port transfer functions are:

**Through port** (all-pass configuration):

$$T_{through} = \frac{r_c^2 - 2r_c a\cos\phi + a^2}{1 - 2r_c a\cos\phi + r_c^2 a^2}$$

**At resonance** ($\phi = 2\pi m$):

$$T_{through} = \left(\frac{r_c - a}{1 - r_c a}\right)^2$$

**Critical coupling condition**: When $r_c = a$ (coupling loss equals round-trip loss), $T_{through} = 0$ — all input power is dissipated in the ring. This is the condition of maximum drop-port coupling (analogous to impedance matching in RF). Designing a ring for critical coupling requires knowing the round-trip loss $a$ precisely, which depends on fabrication-dependent waveguide loss — a challenging calibration problem.

## Resonance Width and Quality Factor

The FWHM linewidth of the resonance:

$$\delta\nu = \frac{\Delta\nu_{FSR}}{\mathcal{F}} = \frac{c(1 - r_c a)}{L\pi\sqrt{r_c a}}$$

The loaded Q factor:

$$Q_L = \frac{\nu_0}{\delta\nu} = \frac{\pi\sqrt{r_c a} m}{1 - r_c a}$$

For a ring with $a = 0.99$ (round-trip loss 0.09 dB), $r_c = 0.99$ (weak coupling), $m \approx 200$ (resonance number for $R = 5$ μm at 1550 nm), $Q_L \approx 30,000$. For ultra-low-loss Si₃N₄ rings: $Q > 10^7$.

## Rings for Photonic Computing

Ring resonators appear in photonic computing in three roles:

1. **Weight banks (ring modulators)**: Rings biased near resonance act as intensity modulators with very compact footprint (~10 μm radius) and high modulation efficiency (resonance enhancement). Section 7.3.3 covers ring modulators in detail.

2. **Wavelength filters (WDM demultiplexers)**: Each ring resonator can be tuned to drop a specific WDM channel from a bus waveguide. An array of rings, each at a different resonance wavelength, acts as a wavelength demultiplexer — routing each WDM channel to its own photodetector for parallel matrix readout.

3. **Phase accumulators in MZI arms**: Rings inserted into one arm of an MZI provide wavelength-selective phase shifts — enabling spectral routing functions.

**Thermal tuning and stability**: At $dn/dT = 1.87 \times 10^{-4}$ K$^{-1}$ for silicon, a 1°C temperature change shifts the ring resonance by:

$$\delta\lambda = \lambda \frac{1}{n_g}\frac{dn}{dT}\delta T = 1.55 \times 10^{-6} \times \frac{1.87\times10^{-4}}{4.2} \times 1 \approx 69 \text{ pm/K}$$

For a ring with 18 nm FSR and 0.1 nm linewidth: a 1.4°C temperature variation shifts the resonance by one linewidth — misaligning the ring from its target wavelength. Thermal control to ±0.1°C (via on-chip resistive heaters consuming ~1 mW) is the standard approach.

This ~1 mW/ring thermal tuning power sets the minimum static power for a ring-based photonic computing system: for a 64-ring weight bank, static tuning power ≈ 64 mW.
