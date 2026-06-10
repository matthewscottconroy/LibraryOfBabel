# 2.2.2 — Young's Double-Slit Experiment

## Historical and Conceptual Significance

Thomas Young performed his double-slit experiment in 1801 and reported the results in 1804 [1]. The experiment was decisive in the wave-versus-particle debate about the nature of light. Newton's corpuscular theory could not explain the dark bands in the illuminated region behind the slits — particles that pass through either slit should illuminate the screen, so the dark bands required two beams of light to *cancel*, which implied they could subtract as waves can, not just add as particles would.

Young's experiment is not merely historical. It is the clearest possible demonstration of the wave principle of superposition, and it introduces the concept of spatial coherence in a physically transparent way. We will see that Young's experiment is the direct physical underpinning of the van Cittert-Zernike theorem (Section 2.5.3) and, through that theorem, of the spatial mode structure of any photonic computing system.

## Geometry and Path Length Difference

Two narrow slits, separated by distance $d$, are illuminated by a coherent monochromatic source. A screen is placed at distance $L \gg d$ from the slits. A point $P$ on the screen at height $y$ subtends an angle $\theta \approx y/L$ from the axis.

The optical path length from slit 1 (at $+d/2$) to point $P$ is $r_1$; from slit 2 (at $-d/2$) is $r_2$. In the Fraunhofer (far-field) limit $L \gg d^2/\lambda$:

$$\Delta r = r_2 - r_1 \approx d\sin\theta \approx \frac{dy}{L}$$

The phase difference is:

$$\Delta\phi = \frac{2\pi}{\lambda}\Delta r = \frac{2\pi d}{\lambda} \cdot \frac{y}{L} = \frac{2\pi d \sin\theta}{\lambda}$$

With equal amplitude $E_0$ from each slit, the intensity pattern on the screen is:

$$I(\theta) = 4I_0 \cos^2\left(\frac{\pi d \sin\theta}{\lambda}\right)$$

where $I_0 = |E_0|^2/2$ is the intensity from a single slit. The factor of 4 at $\Delta\phi = 0$ (central maximum): two coherent equal sources add in amplitude, giving twice the amplitude and four times the intensity. The spatial frequency of the fringes is $d/\lambda$: fringes are spaced by $\Delta y = \lambda L/d$.

## Measuring the Wavelength

Young's key application: measure the fringe spacing $\Delta y$, know $d$ and $L$, solve for $\lambda$:

$$\lambda = \frac{d \cdot \Delta y}{L}$$

Young used this to measure the wavelengths of different colors of visible light [1], giving values in good agreement with modern measurements. This was the first measurement of the wavelength of light.

**Worked example**: Slit separation $d = 0.1$ mm, screen distance $L = 1$ m, fringe spacing observed $\Delta y = 6.0$ mm. Then $\lambda = (10^{-4} \text{ m})(6.0 \times 10^{-3} \text{ m})/(1 \text{ m}) = 600$ nm (orange light).

## Single-Slit Diffraction Envelope

The analysis above assumed the slits were infinitely narrow (point sources). For slits of finite width $a$, each slit produces a diffraction pattern (Section 2.3.2). The double-slit pattern is the product of the interference factor (from the two-slit geometry) and the single-slit diffraction envelope:

$$I(\theta) = 4I_0 \cos^2\left(\frac{\pi d \sin\theta}{\lambda}\right) \cdot \left[\frac{\sin(\pi a \sin\theta/\lambda)}{\pi a \sin\theta/\lambda}\right]^2$$

The first factor produces the narrow, closely spaced interference fringes; the second factor is the broad diffraction envelope. Some interference maxima coincide with diffraction minima and are "missing orders."

For photonic chip applications: a waveguide array (the analogy of multiple slits) produces an interference pattern that is the discrete Fourier transform of the field amplitudes at the waveguide outputs. This is the principle of the *phased array* optical antenna — used in LiDAR, optical communications, and in some architectures for optical neural networks.

## Coherence and the Double Slit

What if the source is not perfectly coherent — say, it has finite spectral width $\Delta\lambda$ or finite angular extent $\Delta\alpha$?

**Temporal coherence** (finite $\Delta\lambda$): Different wavelengths produce fringe patterns with slightly different spacings. When many wavelengths are superposed (as in a thermal source), the fringes wash out for large path differences but are visible near $\Delta r = 0$. Fringes remain visible only for $|\Delta r| < L_c = \lambda^2/\Delta\lambda$ (the coherence length). For sunlight ($\Delta\lambda \approx 300$ nm centered at 600 nm), $L_c \approx 1.2$ μm — a very small coherence length. For a laser with linewidth $\Delta\lambda \approx 10^{-6}$ nm, $L_c \approx 3 \times 10^8$ m — essentially unlimited.

**Spatial coherence** (finite source size): If the source subtends angle $\Delta\alpha$ at the slit plane, each point of the source produces fringes shifted by a different amount. The fringes add incoherently (they have no fixed phase relationship). When the shift per point equals half a fringe period, the fringes are completely washed out: $\Delta\alpha \cdot d = \lambda/2$, giving the *spatial coherence condition* $d < \lambda/(2\Delta\alpha)$. The maximum slit separation for which fringes are visible is the *spatial coherence length* $l_c = \lambda/(2\Delta\alpha)$.

For a distant star ($\Delta\alpha \approx 5 \times 10^{-8}$ rad): $l_c \approx 6$ m. Michelson used this to measure stellar diameters with a stellar interferometer. For a solar LED die ($\Delta\alpha \approx 0.1$ rad): $l_c \approx 3$ μm. For a single-mode laser: $\Delta\alpha \to 0$, $l_c \to \infty$.

**Relevance to photonic computing**: The spatial coherence of the input optical field determines whether two different spatial modes of a photonic system can interfere. A photonic chip fed by a coherent single-mode laser has the same coherence everywhere on-chip; all waveguide modes can interfere with each other. A chip fed by an incoherent source (like a broadband LED) cannot use interference for computation. Coherent photonic computing architectures require coherent laser illumination — this is a fundamental hardware requirement.

## The Quantum Version: Wave-Particle Duality

It is worth noting, without dwelling on it here, that the double-slit experiment works even with single photons sent through one at a time. Each individual photon lands at a point on the screen (particle-like), but the accumulated pattern of thousands of single-photon events reproduces the interference fringe pattern (wave-like) [2]. This is the essential quantum mystery of wave-particle duality: each photon interferes with itself, not with other photons.

For photonic computing, the implication is that quantum optical processors based on single photons (Unit VII) must handle this wave-particle duality carefully. Classical photonic computing uses many photons (coherent states), where the quantum fluctuations are suppressed and the classical wave picture is adequate.

## Summary

- Young's double-slit produces fringes with spacing $\Delta y = \lambda L/d$ and intensity $I = 4I_0\cos^2(\pi d\sin\theta/\lambda)$.
- The experiment demonstrates wave superposition and allows wavelength measurement.
- Fringe visibility requires temporal coherence ($|\Delta r| < L_c$) and spatial coherence ($d < l_c$).
- Coherent (laser) illumination is a prerequisite for interference-based photonic computing.

---

*References*

[1] Young, T. (1804). The Bakerian lecture: Experiments and calculations relative to physical optics. *Philosophical Transactions of the Royal Society of London*, 94, 1–16.

[2] Grangier, P., Roger, G., & Aspect, A. (1986). Experimental evidence for a photon anticorrelation effect on a beam splitter. *Europhysics Letters*, 1(4), 173–179. [DOI: 10.1209/0295-5075/1/4/004] [Demonstrates single-photon interference.]
