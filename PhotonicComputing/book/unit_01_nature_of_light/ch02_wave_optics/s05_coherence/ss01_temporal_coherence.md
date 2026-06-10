# 2.5.1 — Temporal Coherence

## The Mutual Coherence Function

Consider the electric field at a fixed point in space, $E(t)$. This is a random process for any real light source. The *temporal mutual coherence function* (or *autocorrelation function*) is:

$$\Gamma(\tau) = \langle E^*(t) E(t + \tau) \rangle$$

where $\langle \cdot \rangle$ denotes time averaging over an interval long compared to the optical period but short compared to the slow fluctuations. The argument $\tau$ is the time delay.

At $\tau = 0$: $\Gamma(0) = \langle |E(t)|^2 \rangle = I$ (the intensity).

The *complex degree of temporal coherence* (the normalized autocorrelation):

$$\gamma(\tau) = \frac{\Gamma(\tau)}{\Gamma(0)} = \frac{\langle E^*(t) E(t+\tau) \rangle}{\langle |E(t)|^2 \rangle}$$

By the Cauchy-Schwarz inequality, $|\gamma(\tau)| \leq 1$ for all $\tau$. At $\tau = 0$, $|\gamma(0)| = 1$ always.

**Physical meaning**: $|\gamma(\tau)|$ is the fringe visibility in a Michelson interferometer with path difference $c\tau$ (one arm delayed by $\tau$ relative to the other). For a source with $|\gamma(\tau)| = 0$ for $|\tau| > \tau_c$, fringes are visible only for path differences less than $c\tau_c$.

## The Coherence Time and Coherence Length

The *coherence time* $\tau_c$ is the characteristic time over which $\gamma(\tau)$ decreases from 1 to approximately $1/e$:

$$\tau_c = \int_0^\infty |\gamma(\tau)|^2 d\tau$$

The *coherence length* is $L_c = c\tau_c$.

For common sources:
| Source | Linewidth $\Delta\nu$ | Coherence time $\tau_c$ | Coherence length $L_c$ |
|--------|----------------------|------------------------|------------------------|
| Thermal source (filtered, $\Delta\lambda = 1$ nm at 632 nm) | $\sim 750$ GHz | $\sim 1.3$ ps | $\sim 0.4$ mm |
| LED ($\Delta\lambda = 30$ nm at 850 nm) | $\sim 12$ THz | $\sim 80$ fs | $\sim 25$ μm |
| Single-mode diode laser ($\Delta\nu = 1$ MHz) | 1 MHz | $\sim 1$ μs | $\sim 300$ m |
| Stabilized DFB laser ($\Delta\nu = 1$ kHz) | 1 kHz | $\sim 1$ ms | $\sim 300$ km |
| Nd:YAG mode-locked laser (10 ps pulse) | $\sim 44$ GHz | $\sim 10$ ps | $\sim 3$ mm |

For photonic computing circuits with path differences of $\sim 1$ μm to $\sim 1$ mm, a standard DFB laser with $\Delta\nu < 1$ MHz and $L_c > 300$ m is far more than sufficient.

## The Wiener-Khinchin Theorem

The power spectral density $S(\nu)$ (the spectrum of the light source) is the Fourier transform of the autocorrelation function:

$$S(\nu) = \int_{-\infty}^{\infty} \Gamma(\tau) e^{-i2\pi\nu\tau} d\tau$$

This is the **Wiener-Khinchin theorem** [1, 2]. It states that the temporal coherence function and the power spectrum are a Fourier transform pair.

**Consequences**:
1. A broad spectrum (large $\Delta\nu$) corresponds to a rapidly decaying $\Gamma(\tau)$ — short coherence time.
2. A narrow spectrum (small $\Delta\nu$) corresponds to slowly decaying $\Gamma(\tau)$ — long coherence time.
3. The coherence time and bandwidth are related by the time-bandwidth product: $\tau_c \cdot \Delta\nu \geq 1/(2\pi)$ (the exact value depends on the definitions, but the order of magnitude is always $\tau_c \sim 1/\Delta\nu$).

This is the temporal analog of the spatial-frequency uncertainty principle from diffraction.

**Practical use**: The coherence function (and hence the visibility of interference fringes) can be measured by scanning the delay $\tau$ in a Michelson interferometer and recording the output intensity. The Fourier transform of the fringe visibility vs. delay gives the source spectrum $S(\nu)$. This is *Fourier transform spectroscopy* — the basis of instruments like FTIR spectrometers, which measure molecular absorption spectra with high resolution.

## Line Shapes

The spectrum of a laser line has a specific profile depending on the broadening mechanism:

**Lorentzian** (homogeneous broadening, natural linewidth, pressure broadening):

$$S(\nu) \propto \frac{1}{(\nu - \nu_0)^2 + (\Delta\nu/2)^2}$$

$$\Rightarrow \quad \gamma(\tau) \propto e^{i2\pi\nu_0\tau} e^{-\pi|\tau|\Delta\nu}$$

Coherence decays exponentially; coherence time $\tau_c = 1/(\pi\Delta\nu)$.

**Gaussian** (inhomogeneous broadening, Doppler broadening):

$$S(\nu) \propto e^{-(\nu - \nu_0)^2/(2\sigma_\nu^2)}$$

$$\Rightarrow \quad \gamma(\tau) \propto e^{i2\pi\nu_0\tau} e^{-2\pi^2\sigma_\nu^2\tau^2}$$

Coherence decays as a Gaussian; coherence time $\tau_c = 1/(2\pi\sigma_\nu)$.

**Why this matters**: A laser with a Lorentzian lineshape has $|\gamma(\tau)| = e^{-\pi|\tau|\Delta\nu}$, which decays only to $e^{-\pi} \approx 0.04$ after one coherence time $1/\Delta\nu$. The fringe visibility measured in a Michelson interferometer decreases gradually with path difference — it doesn't abruptly go to zero. This means that for path differences somewhat larger than $L_c$, fringes are still partially visible (with visibility $< e^{-\pi} \approx 4\%$). In a photonic chip, this residual coherence can cause unwanted interference effects.

## Phase Noise and Laser Linewidth

The finite linewidth of a laser arises primarily from *phase noise* — random fluctuations in the optical phase caused by spontaneous emission events into the lasing mode. The Schawlow-Townes formula gives the fundamental quantum noise limit to the laser linewidth [3]:

$$\Delta\nu_\text{ST} = \frac{\pi h\nu (\Delta\nu_c)^2}{P_\text{out}}$$

where $\Delta\nu_c = c/(2nLF)$ is the cold-cavity linewidth (Section 2.2.3) and $P_\text{out}$ is the output power. For a 1 mW semiconductor laser with cavity length 300 μm and finesse 100: $\Delta\nu_\text{ST} \sim 1$ MHz — consistent with observed DFB laser linewidths.

**For photonic computing**: Phase noise from the laser translates directly into phase errors in the optical fields throughout the photonic network. In an MZI with one arm longer than the other by $\Delta L$, a phase noise event at the laser perturbs the MZI's differential phase by $\Delta\phi = 2\pi\Delta L \cdot \Delta\nu/c$. For $\Delta L = 1$ mm and $\Delta\nu = 1$ MHz: $\Delta\phi \approx 2\pi \times 10^{-3} \times 10^6/(3 \times 10^8) = 2\pi \times 3.3 \times 10^{-6}$ rad — negligible. But if $\Delta L = 10$ cm and $\Delta\nu = 100$ kHz: $\Delta\phi = 2\pi \times 3.3 \times 10^{-5}$ rad, still small. Phase noise is generally not a limiting factor for current photonic computing chips, but becomes relevant for long-coherence-length applications or very high-precision analog computations.

## Summary

- Temporal coherence function $\Gamma(\tau) = \langle E^*(t)E(t+\tau)\rangle$; degree of coherence $\gamma(\tau) = \Gamma(\tau)/\Gamma(0)$.
- Coherence time $\tau_c \sim 1/\Delta\nu$; coherence length $L_c = c\tau_c$.
- Wiener-Khinchin theorem: spectrum and autocorrelation are a Fourier transform pair.
- Lorentzian line → exponentially decaying $|\gamma|$; Gaussian line → Gaussian-decaying $|\gamma|$.
- For photonic chip path differences $\sim 1$ mm, standard DFB lasers ($\Delta\nu < 1$ MHz, $L_c > 300$ m) provide more than sufficient coherence.

---

*References*

[1] Wiener, N. (1930). Generalized harmonic analysis. *Acta Mathematica*, 55, 117–258.

[2] Khinchin, A. (1934). Korrelationstheorie der stationären stochastischen Prozesse. *Mathematische Annalen*, 109, 604–615.

[3] Schawlow, A.L. & Townes, C.H. (1958). Infrared and optical masers. *Physical Review*, 112(6), 1940–1949. [DOI: 10.1103/PhysRev.112.1940] [The paper proposing the laser; the Schawlow-Townes linewidth formula appears here.]
