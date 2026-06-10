# 2.2.3 — The Fabry-Pérot Cavity

## Multiple-Beam Interference

The Fabry-Pérot (FP) cavity is formed by two parallel, partially reflective mirrors facing each other. Light bouncing back and forth between them undergoes multiple reflections, and all the reflected (or transmitted) beams interfere. Unlike the two-beam interference of the double slit, this is *multiple-beam interference*, and the intensity pattern it produces is qualitatively different: instead of smooth sinusoidal fringes, it produces sharp resonance peaks — narrow transmission windows separated by regions of high reflection.

This is the central resonance phenomenon of photonics. Fabry-Pérot resonators (and their integrated-photonics cousin, the ring resonator) are used as:
- Laser cavities (to select the lasing frequency)
- Optical filters (to select specific wavelength channels)
- Modulators (ring resonator modulators in silicon photonics)
- Sensors (measuring small changes in refractive index or length via resonance shifts)
- Weight banks in photonic neural networks (ring resonators set analog weights)

Understanding the FP cavity at the wave level — not just the formula, but the mechanism — is essential.

## The Transfer Matrix Approach

Consider a cavity formed by two mirrors with amplitude reflectances $r_1$ and $r_2$ (power reflectances $R_1 = |r_1|^2$, $R_2 = |r_2|^2$) and amplitude transmittances $t_1$, $t_2$. The cavity has length $L$ and contains a medium of refractive index $n$.

**Round-trip phase**: A wave traversing the cavity once accumulates phase $\delta/2 = nkL = n\omega L/c$. A round trip accumulates phase $\delta = 2nkL$.

**Round-trip amplitude**: After one round trip (without the mirror transmissions), the wave amplitude is multiplied by $r_1 r_2 e^{i\delta}$.

**Transmitted amplitude**: An input wave with amplitude $E_i$ is transmitted as a sum of contributions — the direct transmission plus waves that have made $m$ round trips before exiting:

$$E_t = t_1 t_2 E_i \sum_{m=0}^{\infty} (r_1 r_2)^m e^{im\delta} = \frac{t_1 t_2}{1 - r_1 r_2 e^{i\delta}} E_i$$

(using the geometric series $\sum_{m=0}^\infty x^m = 1/(1-x)$ for $|x| < 1$).

The transmitted *intensity* is $I_t = |E_t|^2 \cdot I_i / |E_i|^2$:

$$\frac{I_t}{I_i} = \frac{T_1 T_2}{|1 - r_1 r_2 e^{i\delta}|^2} = \frac{T_1 T_2}{(1 - \sqrt{R_1 R_2})^2 + 4\sqrt{R_1 R_2}\sin^2(\delta/2)}$$

where $T_{1,2} = |t_{1,2}|^2$ are the power transmittances.

## The Airy Function

For a symmetric lossless cavity ($R_1 = R_2 = R$, $T_1 = T_2 = T = 1-R$):

$$\frac{I_t}{I_i} = \frac{(1-R)^2}{(1-R)^2 + 4R\sin^2(\delta/2)} = \frac{1}{1 + \mathcal{F}^2 \sin^2(\delta/2)}$$

where we defined the *coefficient of finesse*:

$$\mathcal{F} = \frac{2\sqrt{R}}{1-R} \qquad \Rightarrow \qquad F = \frac{\pi\sqrt{R}}{1-R} \approx \frac{\pi}{1-R} \text{ (for high R)}$$

The function $1/(1 + \mathcal{F}^2\sin^2(\delta/2))$ is the *Airy function*. It equals 1 at resonance ($\delta = 2m\pi$, $m$ an integer) and has minimum value $(1-R)^2/(1+R)^2$ at anti-resonance. For high reflectance $R \to 1$, the peaks become sharp and the transmission between them approaches zero.

**Resonance condition**: $\delta = 2nkL = 2m\pi$ means $L = m\lambda/(2n)$, i.e., the cavity length equals an integer multiple of half-wavelengths. Equivalently:

$$\nu_m = m \cdot \frac{c}{2nL} = m \cdot \nu_\text{FSR}$$

where $\nu_\text{FSR} = c/(2nL)$ is the *free spectral range* (FSR) — the frequency spacing between adjacent resonances.

## Key Parameters: Finesse, Q Factor, FWHM

### Finesse

The *finesse* $F$ is defined as the ratio of the FSR to the FWHM (full width at half maximum) of a single resonance peak:

$$F = \frac{\nu_\text{FSR}}{\Delta\nu_\text{FWHM}}$$

For a lossless cavity with mirror reflectances $R_1$, $R_2$, the finesse is:

$$F = \frac{\pi (R_1 R_2)^{1/4}}{1 - \sqrt{R_1 R_2}}$$

High finesse means narrow peaks (high frequency resolution) and more round trips before photons exit. For $R = 0.99$ (99% reflectance mirrors): $F = \pi\sqrt{0.99}/(1-0.99) \approx 312$.

### Quality Factor

The quality factor $Q$ is the ratio of the resonant frequency to the FWHM:

$$Q = \frac{\nu}{\Delta\nu} = \frac{m \cdot \nu_\text{FSR}}{\nu_\text{FSR}/F} = mF$$

For a cavity of length $L = 1$ mm at $\lambda = 1550$ nm, $n = 3.5$ (silicon): $m = 2nL/\lambda = 2(3.5)(10^{-3})/(1550 \times 10^{-9}) \approx 4516$. With $F = 100$, $Q \approx 4.5 \times 10^5$. Silicon ring resonators have achieved $Q > 10^6$ in practice [1].

$Q$ has a physical interpretation: the cavity stores energy for a time $\tau = Q/\omega = Q\lambda/(2\pi c)$. For $Q = 10^6$ at 1550 nm: $\tau \approx 0.82$ ps, during which light makes approximately $\tau c/(2nL) \approx F/\pi \approx$ hundreds of round trips.

### Free Spectral Range

$$\nu_\text{FSR} = \frac{c}{2n_g L}$$

Note the use of the *group index* $n_g = n - \lambda \, dn/d\lambda$ rather than the phase index, because FSR is a frequency (not phase) spacing. For a silicon ring resonator with circumference $2\pi R = 100$ μm and $n_g = 4.2$:

$$\nu_\text{FSR} = \frac{3 \times 10^8}{4.2 \times 100 \times 10^{-6}} = 714 \text{ GHz}$$

This is the frequency spacing between ring resonances. Only input channels whose frequency matches a resonance are transmitted (and potentially coupled to or from the ring). The FSR sets the multiplexing range of the device.

## Ring Resonators in Silicon Photonics

The ring resonator is the on-chip implementation of the Fabry-Pérot cavity. A waveguide ring of circumference $C$ is placed close to a straight "bus" waveguide. Light from the bus evanescently couples into the ring with coupling efficiency determined by the gap between bus and ring. If the round-trip phase $\beta C = 2\pi m$ (resonance condition), light builds up in the ring and — by the reverse evanescent coupling — is partly extracted back to the bus.

**Transfer function**: At resonance, the ring acts as a notch filter: power is transferred from the bus waveguide to the ring (and if a second bus waveguide is present, to a "drop" port). For a single bus waveguide, the transmission past the ring (through port) has a Lorentzian dip at the resonant frequency:

$$T(\nu) = \frac{(\nu - \nu_0)^2 + (\Delta\nu/2)^2 \cdot (1 - \kappa^2 r^2)^2/\kappa^4 r^2}{(\nu - \nu_0)^2 + (\Delta\nu/2)^2}$$

At critical coupling ($\kappa^2 = 1 - r^2$ where $r$ is the single-pass loss amplitude), the on-resonance transmission drops to zero: all power is dissipated in the ring. This regime is used for high-extinction filters.

**As a modulator and weight element**: By injecting carriers into the ring (forward-biased p-n junction) or applying a voltage (reverse-biased), the refractive index of the ring waveguide is changed slightly via the plasma dispersion effect. This shifts the resonant frequency $\nu_0 = mc/(n_g C)$ by $\Delta\nu_0 = -\nu_0 \Delta n_g/n_g$. If the input laser frequency is fixed on the slope of the Lorentzian resonance, a small frequency shift changes the transmission significantly — this is the modulation mechanism of ring resonator modulators [2].

For photonic neural networks, ring resonators serve as analog weight elements: each ring is tuned to be on-resonance or off-resonance for a particular wavelength channel, thereby setting the transmission (weight) for that channel [3]. The weight is set by thermal tuning (a microheater shifting $n$ by the thermo-optic effect) or electro-optic tuning (carrier injection).

## Cavity Photon Lifetime and Bandwidth

A cavity with finesse $F$ and round-trip time $T_R = 2nL/c$ has a photon lifetime:

$$\tau_p = \frac{F \cdot T_R}{\pi} = \frac{2nLF}{\pi c}$$

The cavity resonance FWHM is $\Delta\nu = 1/(2\pi\tau_p) = c/(4\pi nLF) \cdot 2 = \nu_\text{FSR}/F$.

The photon lifetime sets the *modulation bandwidth* of the resonator-based modulator: the modulation frequency must be less than $1/(2\pi\tau_p)$ for the resonator to respond fully. High-Q (high-finesse) resonators have narrow linewidths and slow responses — there is a fundamental tradeoff between spectral selectivity and modulation bandwidth. Silicon ring modulators with bandwidths of 10–50 GHz have been demonstrated, corresponding to Q factors of $\sim 10^4$–$10^5$ [2].

## Summary

- Multiple-beam interference in a Fabry-Pérot cavity produces sharp Airy function resonances.
- Resonance condition: $L = m\lambda/(2n)$; resonant frequencies $\nu_m = mc/(2nL)$.
- Finesse $F = \pi R^{1/2}/(1-R)$ measures peak sharpness; Q factor $= mF$ measures energy storage time.
- Free spectral range $\nu_\text{FSR} = c/(2n_g L)$: frequency spacing between resonances.
- On-chip implementation: ring resonators in silicon photonics, used as filters, modulators, and weight elements in photonic neural networks.
- Tradeoff: high Q → narrow bandwidth → limited modulation speed.

---

*References*

[1] Borselli, M., Johnson, T.J., & Painter, O. (2005). Beyond the Rayleigh scattering limit in high-Q silicon microdisks. *Optics Express*, 13(5), 1515–1530. [DOI: 10.1364/OPEX.13.001515]

[2] Xu, Q., Schmidt, B., Pradhan, S., & Lipson, M. (2005). Micrometre-scale integrated silicon electro-optic modulator. *Nature*, 435(7040), 325–327. [DOI: 10.1038/nature03569] [First demonstration of a silicon ring resonator electro-optic modulator.]

[3] Tait, A.N., de Lima, T.F., Zhou, E., Wu, A.X., Nahmias, M.A., Shastri, B.J., & Prucnal, P.R. (2017). Neuromorphic photonic networks using silicon photonic weight banks. *Scientific Reports*, 7, 7430. [DOI: 10.1038/s41598-017-07754-z] [Demonstrates ring resonator weight banks for photonic neural networks.]
