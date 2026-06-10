# 4.2.3 Gaussian Transverse Modes and Q Factor

## Hermite-Gaussian Modes

A stable resonator sustains a discrete set of transverse electromagnetic modes, the Hermite-Gaussian (HG) modes TEM$_{mn}$. For a resonator with beam waist $w_0$ at the waist plane, the mode field profile is:

$$E_{mn}(x,y,z) = E_0 \, H_m\!\left(\frac{\sqrt{2}x}{w(z)}\right) H_n\!\left(\frac{\sqrt{2}y}{w(z)}\right) \exp\!\left(-\frac{x^2+y^2}{w(z)^2}\right) e^{i\phi_{mn}(z)}$$

where $H_m$ is the Hermite polynomial of order $m$, $w(z)$ is the Gaussian beam radius (from Chapter 2), and $\phi_{mn}$ is a phase that includes the Gouy phase with an additional $-(m+n+1)$ contribution.

The lowest-order mode, TEM$_{00}$, is a pure Gaussian beam. Higher-order modes TEM$_{mn}$ have $m+n$ additional intensity lobes and experience greater diffractive loss in a resonator with finite aperture. This provides transverse mode discrimination: a laser with an aperture smaller than the TEM$_{10}$ mode but larger than TEM$_{00}$ will oscillate in a single transverse mode.

## Resonant Frequencies Including Transverse Mode Structure

The resonant frequencies are:

$$\nu_{mnq} = \frac{c}{2n_g L}\left[q + \frac{(m+n+1)}{\pi}\arccos\sqrt{g_1 g_2}\right]$$

where $q$ is the longitudinal mode number and $(m,n)$ are the transverse mode indices. The transverse mode spacing $\Delta\nu_\perp$ is a fraction of the FSR, determined by $\arccos\sqrt{g_1 g_2}/\pi$. For a confocal cavity ($g_1 g_2 = 0$), $\Delta\nu_\perp = c/(4n_g L) = \Delta\nu_{FSR}/2$.

For semiconductor lasers, the waveguide confinement typically forces single transverse mode operation regardless of the resonator geometry. The waveguide dimensions are chosen so that only the fundamental mode is below cutoff — typically a ridge width of $<$3 μm for a 1550 nm InP laser.

## The Laser Linewidth: Schawlow-Townes Formula

The fundamental quantum limit on laser linewidth is set by spontaneous emission: each spontaneous emission event injects a photon with random phase into the lasing mode, diffusing the phase of the coherent field. The resulting linewidth (half-power full-width) is the Schawlow-Townes formula [1]:

$$\Delta\nu_{ST} = \frac{\hbar\omega_0 v_g^2 \alpha_{tot}^2}{4\pi P_{out}} \cdot n_{sp}$$

where:
- $\alpha_{tot} = \alpha_i + \alpha_m$ = total round-trip loss
- $P_{out}$ = output power
- $n_{sp} = N_2/(N_2 - N_1)$ = spontaneous emission factor (population inversion factor; = 1 for complete inversion)

An equivalent form using the cavity Q factor and photon lifetime:

$$\Delta\nu_{ST} = \frac{h\nu_0}{4\pi \tau_p^2 P_{out}} \cdot n_{sp}$$

**Interpretation**: The linewidth decreases as output power increases (more coherent photons dilute the effect of each random spontaneous emission event), and decreases as the photon lifetime increases (a longer-lived photon is more precisely defined in frequency, by time-frequency uncertainty).

**Modified Schawlow-Townes formula for semiconductors** (Henry, 1982 [2]): In semiconductor lasers, the linewidth is enhanced by a factor $(1 + \alpha_H^2)$:

$$\Delta\nu = \Delta\nu_{ST}(1 + \alpha_H^2)$$

where $\alpha_H$ is the linewidth enhancement factor (Henry factor), defined as the ratio of the change in real refractive index to the change in gain per unit carrier density:

$$\alpha_H = -\frac{dn/dN}{dg/dN} \cdot 2k_0$$

In InGaAsP at 1550 nm, $\alpha_H \approx 3$–5, so the actual linewidth is $(1 + 9)$ to $(1+25)$ = 10× to 26× the Schawlow-Townes limit. This gives typical semiconductor laser linewidths of **1–10 MHz** at moderate power levels (1–10 mW).

**Practical linewidths**:

| Laser type | Typical linewidth | Coherence length |
|---|---|---|
| Free-running Fabry-Pérot diode | 5–50 nm (multi-mode) | $<$ 0.1 mm |
| DFB diode (free-running) | 1–10 MHz | 30–300 m |
| DFB with optical feedback stabilization | 10–100 kHz | 3–30 km |
| External-cavity diode laser (ECDL) | 10 kHz – 100 Hz | 3 km – 3000 km |
| Fiber laser (e.g., NKT Koheras) | < 1 kHz | > 300 km |
| Narrow-linewidth fiber laser reference | 1–10 Hz | > 30,000 km |

**Photonic computing implication**: An MZI with path length difference $\Delta L$ has interference visibility $V \approx \exp(-\pi\Delta\nu \cdot n_g \Delta L/c)$. For $\Delta L = 1$ mm, $n_g = 1.5$, $\Delta\nu = 1$ MHz: $V \approx \exp(-\pi \times 10^6 \times 1.5 \times 10^{-3}/3\times10^8) = \exp(-0.016) \approx 0.984$ — negligible degradation. For $\Delta L = 10$ cm with a 10 MHz linewidth: $V \approx 0.2$ — severe degradation. Coherence length must exceed the maximum path length difference in the photonic computing circuit.

## References

[1] Schawlow, A.L., & Townes, C.H. (1958). "Infrared and optical masers." *Physical Review*, 112(6), 1940–1949.

[2] Henry, C.H. (1982). "Theory of the linewidth of semiconductor lasers." *IEEE Journal of Quantum Electronics*, 18(2), 259–264.
