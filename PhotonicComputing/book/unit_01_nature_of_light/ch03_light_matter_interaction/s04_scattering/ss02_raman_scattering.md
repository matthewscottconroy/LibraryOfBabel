# 3.4.2 — Raman Scattering

## Inelastic Scattering and Phonons

Raman scattering (discovered by C.V. Raman in 1928 [1]) is *inelastic*: the scattered photon has a different frequency from the incident photon. The frequency shift corresponds to the creation or annihilation of a vibrational quantum (phonon) in the material.

**Stokes scattering**: incident photon at $\omega$ creates a phonon at $\Omega$ and scatters to $\omega - \Omega$ (lower frequency, red-shifted). The photon loses energy to the lattice.

**Anti-Stokes scattering**: incident photon at $\omega$ absorbs a thermally populated phonon at $\Omega$ and scatters to $\omega + \Omega$ (higher frequency, blue-shifted). Requires a pre-existing phonon; weaker at low temperatures.

The Raman shift $\Omega$ is characteristic of the material's vibrational spectrum — a fingerprint used for chemical identification. For silicon: $\Omega_\text{Si} \approx 520$ cm⁻¹ ($\approx 15.6$ THz), corresponding to the zone-center optical phonon. For silica: multiple overlapping bands from $300$–$1100$ cm⁻¹.

## Spontaneous vs. Stimulated Raman Scattering

**Spontaneous Raman scattering** is weak: the Raman cross-section is typically $10^{-30}$–$10^{-29}$ cm²/sr per molecule (10 orders of magnitude weaker than Rayleigh scattering). Spontaneous Raman is used for spectroscopy (Raman spectroscopy is an essential analytical tool in chemistry, materials science, and biology) but is too weak for optical amplification.

**Stimulated Raman scattering (SRS)** occurs at high pump intensities: when the pump wave ($\omega_p$) is intense, the Stokes wave ($\omega_s = \omega_p - \Omega$) that it generates can itself stimulate further Raman scattering, creating positive feedback. Above a threshold, the Stokes wave grows exponentially — Raman amplification.

The SRS threshold condition: the Raman gain exceeds the cavity (or waveguide) loss. The gain coefficient at the Stokes frequency:

$$g_R = \frac{2\omega_s n_2^R}{c n_p n_s}$$

where $n_2^R = \hbar\omega_s\chi_R''/(n_p n_s \varepsilon_0 c^2)$ and $\chi_R''$ is the imaginary part of the Raman susceptibility. For silica fiber: $g_R \approx 1 \times 10^{-13}$ m/W (peak at 13 THz downshift from pump); for silicon: $g_R \approx 4 \times 10^{-10}$ m/W (peak at 15.6 THz) — three orders of magnitude larger than silica.

## Raman Fiber Amplifiers

In silica fiber, the broad Raman gain spectrum (extending over $\sim 40$ THz) covers the entire telecom C and L bands when pumped at 1450–1480 nm. Raman fiber amplifiers are used in long-haul transmission systems to:
1. Boost signal levels between EDFA sites.
2. Provide gain in wavelength bands not covered by EDFAs (e.g., the S-band at 1480–1530 nm).
3. Reduce noise figure compared to EDFAs by providing distributed gain along the entire fiber span.

Raman amplifiers can be pumped in forward, backward, or bidirectional configurations. Backward pumping (pump and signal counter-propagate) gives lower noise figure (the amplified spontaneous emission noise builds up less).

## Silicon Raman Laser and Amplifier

The first silicon laser was demonstrated by Boyraz and Jalali (2004) [2] using stimulated Raman scattering in a silicon waveguide. This was a landmark result: silicon, which cannot lase by direct band-to-band emission (indirect bandgap), can lase via the Raman effect. Key points:

- Pump at $\omega_p$ → Raman Stokes output at $\omega_p - \Omega_\text{Si}$ (for 1550 nm pump: Stokes at $\sim 1686$ nm).
- Requires overcoming TPA-generated free-carrier absorption (the dominant loss mechanism at high pump powers).
- Solutions: pulsed operation (short pulses avoid free-carrier buildup), reverse-biased p-i-n junction (sweeps carriers out), or p-n junction with current injection to recombine carriers.
- Demonstrated CW silicon Raman laser: threshold pump power $\sim 200$ mW; output power $\sim$ tens of mW [3].

**For photonic computing**: A silicon Raman amplifier could compensate on-chip waveguide losses without requiring a III-V laser on chip. However, the wavelength shift ($\Omega_\text{Si} \approx 520$ cm⁻¹ = 76 nm shift at 1550 nm) means the signal must be at 1686 nm if the pump is at 1550 nm — or the pump wavelength must be at $\sim 1435$ nm to amplify at 1550 nm. This wavelength mismatch requires careful system design. The technique is promising but not yet widely adopted in photonic computing.

## Summary

- Raman scattering: inelastic; photon creates (Stokes) or absorbs (anti-Stokes) a phonon at $\Omega$.
- Silicon Raman shift: $\Omega_\text{Si} \approx 520$ cm⁻¹ = 15.6 THz; $g_R \approx 4 \times 10^{-10}$ m/W.
- Stimulated Raman scattering: exponential growth above threshold → Raman amplifier.
- Silicon Raman laser demonstrated in 2004 — first silicon laser.
- Potential for on-chip amplification without III-V integration; limited by TPA and carrier sweep-out.

---

*References*

[1] Raman, C.V. (1928). A new radiation. *Indian Journal of Physics*, 2, 387–398. [The original paper; Raman was awarded the 1930 Nobel Prize in Physics.]

[2] Boyraz, O. & Jalali, B. (2004). Demonstration of a silicon Raman laser. *Optics Express*, 12(21), 5269–5273. [DOI: 10.1364/OPEX.12.005269]

[3] Rong, H. et al. (2005). A continuous-wave Raman silicon laser. *Nature*, 433(7027), 725–728. [DOI: 10.1038/nature03351]
