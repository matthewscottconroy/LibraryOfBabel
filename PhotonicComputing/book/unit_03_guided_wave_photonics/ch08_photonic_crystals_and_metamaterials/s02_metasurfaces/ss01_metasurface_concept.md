# 8.2.1 The Metasurface Concept and Generalized Snell's Law

## Beyond Snell's Law

The classical Snell's law of refraction,

$$n_1 \sin\theta_1 = n_2 \sin\theta_2$$

applies when a plane wave at angle $\theta_1$ encounters a uniform planar interface between two media with indices $n_1$ and $n_2$. The physics behind it is phase matching: the component of the wavevector parallel to the interface must be continuous across the boundary.

What if the interface is not uniform — if it imparts a spatially varying phase shift $\phi(x)$ across the wavefront? In 2011, Yu, Capasso, and colleagues [1] showed that this modifies the refraction law to:

$$n_2\sin\theta_t - n_1\sin\theta_i = \frac{\lambda}{2\pi}\frac{d\phi}{dx}$$

where $d\phi/dx$ is the phase gradient along the interface. This **generalized Snell's law** shows that a phase gradient along a surface can redirect transmitted (and reflected) light to arbitrary angles, independent of the refractive indices of the surrounding media.

The same principle applies to reflection: a reflected beam's angle is modified by the surface phase gradient. And for thin samples, the transmitted phase can be manipulated to create arbitrary wavefront shapes — focusing, defocusing, steering, or holographic imaging — in a single flat layer.

## The Metasurface: A Flat Phase Plate

A metasurface is a surface that implements an arbitrary phase function $\phi(x, y)$ with sub-wavelength spatial resolution. It consists of an array of sub-wavelength resonant structures — "meta-atoms" — each designed to impart a specific local phase shift.

For a planar metasurface designed to focus light at a distance $f$ (a flat metalens), the required phase profile is:

$$\phi(x, y) = \frac{2\pi}{\lambda}\left(\sqrt{x^2 + y^2 + f^2} - f\right)$$

This is the phase that a wavefront must accumulate to converge at the focal point $(0, 0, f)$. A conventional lens achieves this through varying thickness $h(x,y)$: $\phi(x,y) = (2\pi/\lambda)(n-1)h(x,y)$. A metasurface achieves it through varying meta-atom geometry, with thickness < 1 μm.

## Phase Control Mechanisms

Several mechanisms allow sub-wavelength structures to achieve phase shifts from 0 to $2\pi$:

### Resonant Phase (Lorentzian)

A resonant structure (antenna, cavity, pillar) has a phase response that sweeps from $0$ to $\pi$ on passing through its resonance. By engineering the resonance frequency (by changing geometry), any phase in $[0, \pi]$ can be achieved. Two resonance mechanisms combined (electric and magnetic dipole resonances, as in Huygens' metasurfaces) can give $[0, 2\pi]$ [2].

The resonant phase response near a single resonance is:

$$\phi(\omega) = \arctan\left(\frac{2(\omega - \omega_0)\tau}{1}\right) - \frac{\pi}{2}$$

where $\omega_0$ is the resonance frequency and $\tau$ is the decay time. This sweeps from $-\pi/2$ to $+\pi/2$ as $\omega$ sweeps through $\omega_0$ — only a $\pi$ range. To get full $2\pi$, a second resonance (magnetic, or higher-order) must be overlapped with the first.

### Propagation Phase (Pillar Height)

A dielectric pillar of height $h$ and effective index $n_{\text{eff}}$ imparts a phase:

$$\phi = \frac{2\pi n_{\text{eff}} h}{\lambda}$$

By varying $h$ while keeping the cross-section fixed, the phase changes continuously with $h$. For a range $\Delta h$ such that $n_{\text{eff}}\Delta h = \lambda$, a full $2\pi$ range is achievable.

However, varying pillar *height* is difficult to fabricate (it requires gray-scale lithography or multiple etch steps). More practical is varying the pillar *width* (or diameter for circular pillars) while keeping height fixed. The effective index $n_{\text{eff}}$ of the guided mode inside the pillar depends on its diameter, giving a phase variation of up to $2\pi$ over the range of accessible diameters [3].

For a cylindrical silicon (or TiO₂) pillar of diameter $D$ and height $h = 600$ nm on a glass substrate, surrounded by air:
- At $D = 80$ nm: $n_{\text{eff}} \approx n_{\text{air}} = 1$, phase ≈ $2\pi h/\lambda$
- At $D = 200$ nm: $n_{\text{eff}} \approx 2.0$, phase ≈ $2\pi \times 2.0 \times h/\lambda$
- Phase variation: $\Delta\phi \approx 2\pi (2.0 - 1.0) \times 600/\lambda$

For $\lambda = 1550$ nm: $\Delta\phi \approx 2\pi \times 0.39$ — only 39% of $2\pi$. To get the full range requires taller pillars or higher-index materials.

For TiO₂ ($n \approx 2.35$) pillars at $\lambda = 633$ nm and $h = 600$ nm: $\Delta\phi \approx 2\pi \times (2.35-1.0) \times 600/633 \approx 2\pi \times 1.28 > 2\pi$ — more than adequate. This is why TiO₂ is a popular material for visible metasurfaces [4].

For Si pillars at 1550 nm ($n = 3.478$), a height of only $h \approx 350$ nm suffices for $2\pi$ phase range: $\Delta\phi = 2\pi(3.478-1.0) \times 350/1550 \approx 2\pi \times 0.56$ — not quite. With $h = 500$ nm: $\Delta\phi \approx 2\pi \times 0.8$. For $h = 600$ nm: essentially $2\pi$. Practical silicon metasurfaces for 1550 nm use pillar heights of 500–800 nm.

## Transmission and Reflection Efficiency

A metasurface is not perfectly transmitting: some fraction of the incident power is reflected, absorbed, or scattered into unwanted orders. The efficiency depends on the polarization, wavelength, and meta-atom design.

For the best dielectric (TiO₂) metasurfaces at visible wavelengths:
- Transmission efficiency: 80–95%
- Focusing efficiency (fraction of transmitted power in the focal spot): 80–90% [3]

For silicon metasurfaces at 1550 nm:
- Transmission efficiency: 70–90%
- Focusing efficiency: 60–80%

These efficiencies are limited by:
1. Reflection at the air-metasurface interfaces (reducible with anti-reflection coatings)
2. Scattering into higher diffraction orders (minimized by dense sub-wavelength packing)
3. Absorption in the meta-atom material (TiO₂ is better than Si at visible wavelengths due to lower absorption near the bandgap)

## Metasurfaces vs. Diffractive Optical Elements

The pre-cursor to the metasurface is the **diffractive optical element (DOE)** — a blazed grating or kinoform etched into glass or plastic, operating by diffraction. DOEs have been used since the 1980s for beam steering, phase coding, and holography.

The difference is spatial resolution:
- DOE: feature sizes > $\lambda$ (typically 1–10 μm), multiple etch levels for blazing
- Metasurface: feature sizes $\lambda/5$ to $\lambda/10$ (100–300 nm), single-layer fabrication

The sub-wavelength feature size gives metasurfaces two advantages: (1) diffraction into unwanted orders is suppressed (sub-wavelength gratings have only the zeroth order in transmission), and (2) the phase control is local and continuous, not step-quantized. The corresponding disadvantage: fabrication requires e-beam lithography or deep-UV lithography with ~50-nm resolution.

---

## References

[1] Yu, N., Genevet, P., Kats, M.A., Aieta, F., Tetienne, J.-P., Capasso, F., & Gaburro, Z. (2011). "Light propagation with phase discontinuities: Generalized laws of reflection and refraction." *Science*, 334(6054), 333–337. [The foundational metasurface paper; introduces the generalized Snell's law via phase-gradient metasurfaces using V-shaped gold antennas.]

[2] Kerker, M., Wang, D.-S., & Giles, C.L. (1983). "Electromagnetic scattering by magnetic spheres." *Journal of the Optical Society of America*, 73(6), 765–767. [Theoretical basis for Huygens' condition (equal electric and magnetic dipole responses); the foundation of low-reflection high-transmission metasurfaces.]

[3] Khorasaninejad, M., Chen, W.T., Devlin, R.C., Oh, J., Zhu, A.Y., & Capasso, F. (2016). "Metalenses at visible wavelengths: Diffraction-limited focusing and subwavelength resolution imaging." *Science*, 352(6290), 1190–1194. [TiO₂ metalens with >80% efficiency at 532 nm; landmark demonstration of high-efficiency flat optics.]

[4] Devlin, R.C., Khorasaninejad, M., Chen, W.T., Oh, J., & Capasso, F. (2016). "Broadband high-efficiency dielectric metasurfaces for the visible spectrum." *Proceedings of the National Academy of Sciences*, 113(38), 10473–10478. [TiO₂ metasurface design methodology for full 0–2π phase control at visible wavelengths.]
