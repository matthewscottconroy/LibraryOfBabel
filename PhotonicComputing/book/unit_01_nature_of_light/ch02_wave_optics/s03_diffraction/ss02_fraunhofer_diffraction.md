# 2.3.2 — Fraunhofer Diffraction

## Single Slit

Consider a slit of width $a$ centered at the origin, illuminated by a plane wave. The aperture field is $E(x') = E_0$ for $|x'| \leq a/2$ and zero otherwise. The Fraunhofer diffraction pattern (the 1D Fourier transform of a rectangle function) is:

$$E(\theta) \propto \int_{-a/2}^{a/2} E_0 e^{-ikx'\sin\theta} dx' = E_0 a \, \text{sinc}\left(\frac{a\sin\theta}{\lambda}\right)$$

where $\text{sinc}(u) = \sin(\pi u)/(\pi u)$. The intensity pattern:

$$I(\theta) = I_0 \, \text{sinc}^2\left(\frac{a\sin\theta}{\lambda}\right)$$

**Zeros** at $a\sin\theta = m\lambda$ ($m = \pm 1, \pm 2, \ldots$), i.e., $\sin\theta = m\lambda/a$. The central maximum has angular width $2\lambda/a$ (between first zeros). **Interpretation**: A narrow slit ($a \ll \lambda$) diffracts broadly (wide central maximum); a wide slit ($a \gg \lambda$) diffracts narrowly (ray-optics limit). This is the Fourier uncertainty principle: spatial width $\times$ spatial frequency bandwidth = 1. Confining a wave in space forces it to spread in spatial frequency (angle).

**For photonics**: A waveguide output facet behaves like a slit (or, for 2D, a rectangular aperture). The divergence of the output beam is $\theta \approx \lambda/a$. For a 10 μm × 10 μm mode field (typical fiber core), $\theta \approx 9°$. For a 0.5 μm × 0.3 μm silicon nanowire mode, $\theta \approx 73°$ horizontal × $\sim 90°$ vertical. This enormous divergence means nearly all the light would be lost without a mode-size converter.

## Rectangular Aperture

A rectangular aperture of dimensions $a \times b$ produces the 2D sinc pattern:

$$I(x,y) = I_0 \, \text{sinc}^2\left(\frac{ax}{\lambda z}\right) \text{sinc}^2\left(\frac{by}{\lambda z}\right)$$

(in the paraxial limit at distance $z$). The pattern is separable in $x$ and $y$ — the 2D Fourier transform of a separable function is separable.

## Circular Aperture: The Airy Disk

For a circular aperture of diameter $D$ (radius $R = D/2$), the 2D Fourier transform in polar coordinates gives:

$$I(\theta) = I_0 \left[\frac{2J_1(\pi D\sin\theta/\lambda)}{\pi D\sin\theta/\lambda}\right]^2$$

where $J_1$ is the first-order Bessel function of the first kind. This pattern is the *Airy disk*. The first zero occurs at $J_1(x) = 0$ for $x = 1.22\pi$, i.e.:

$$\sin\theta_1 = \frac{1.22\lambda}{D}$$

The factor 1.22 (vs. 1.00 for a slit) arises from the circular geometry. The Rayleigh criterion for the resolution limit of an imaging system is that two point sources are just resolved when the Airy disk of one falls on the first minimum of the other: $\theta_\text{min} = 1.22\lambda/D$.

**Relevance to photonic computing**: The Airy disk pattern determines the spot size when focusing a Gaussian beam (approximately). For coupling laser light into a single-mode fiber (core diameter 8–10 μm), the focused spot must match the fiber mode profile. The coupling efficiency is the overlap integral between the focused beam profile and the fiber mode — imperfect matching leads to coupling loss. Alignment tolerance is on the order of the mode field diameter.

## The Fourier Transform Relationship

The most important fact about Fraunhofer diffraction, emphasized here as a standalone statement:

**The Fraunhofer diffraction pattern is the spatial Fourier transform of the aperture field.**

More precisely: if the aperture field is $E(x', y')$, then the field at angle $(\theta_x, \theta_y)$ in the Fraunhofer zone is:

$$E(\theta_x, \theta_y) \propto \hat{E}\left(\frac{\sin\theta_x}{\lambda}, \frac{\sin\theta_y}{\lambda}\right) = \iint E(x',y') \, e^{-i2\pi(x'\sin\theta_x + y'\sin\theta_y)/\lambda} \, dx' \, dy'$$

The spatial frequencies are $f_x = \sin\theta_x/\lambda \approx \theta_x/\lambda$ (paraxial). A spatial feature of size $a$ in the aperture corresponds to spatial frequency $f \sim 1/a$, diffracted at angle $\theta \sim \lambda/a$.

**Examples of the Fourier relationship**:
- Rectangle function (uniform slit) → sinc function
- Gaussian beam → Gaussian (Fourier transform of a Gaussian is a Gaussian — the reason Gaussian beams are so convenient)
- Periodic grating (comb function) → comb function (discrete diffraction orders)
- Phase mask (random phase, uniform amplitude) → spread-out pattern determined by the phase statistics

## Spatial Frequency and the Diffraction Limit

The spatial frequency of a grating or pattern $f = 1/\Lambda$ (cycles per unit length, where $\Lambda$ is the period) diffracts at angle $\sin\theta = f\lambda$. The maximum spatial frequency that can propagate (without evanescent decay) in free space is $f_\text{max} = 1/\lambda$ (when $\sin\theta = 1$, $\theta = 90°$). This is the diffraction limit: *the finest spatial feature resolvable by a conventional far-field optical system is approximately $\lambda$*.

Sub-wavelength features ($f > 1/\lambda$) generate evanescent waves that decay exponentially and do not reach the far field. Recovering them requires near-field techniques (SNOM, NSOM) or metamaterial superlenses.

For photonic integrated circuits: the waveguide pitch (center-to-center spacing) in a photonic chip is typically 2–5 μm, well above the diffraction limit but limited by the evanescent coupling between waveguides (Section 2.1.2). The grating coupler pitch is typically $\sim 600–700$ nm (close to $\lambda/n$ for 1550 nm in silicon), chosen to diffract light at the desired angle.

## Babinet's Principle

Babinet's principle states that the diffraction pattern of an aperture is the complement of the diffraction pattern of the opaque screen that fills the same area (except at the forward direction). Mathematically: if $E_\text{aperture}$ is the field through the aperture and $E_\text{complement}$ is the field through the complementary screen, then $E_\text{aperture} + E_\text{complement} = E_\text{incident}$ (the unobstructed wave). This is simply the superposition principle applied to the aperture and its complement.

Babinet's principle is used in the design of diffractive optical elements (DOEs): an amplitude mask that blocks certain spatial regions has a diffraction pattern computed from its complement. It is also the physical basis of anti-reflection coatings (destructive interference between the reflection from the complement aperture pattern and the direct reflection).

## Summary

- Single-slit Fraunhofer diffraction: $I \propto \text{sinc}^2(a\sin\theta/\lambda)$; zeros at $\sin\theta = m\lambda/a$.
- Circular aperture: Airy disk with first zero at $\sin\theta = 1.22\lambda/D$.
- The Fraunhofer diffraction pattern is the spatial Fourier transform of the aperture field.
- Diffraction limit: minimum resolvable feature size $\sim \lambda$ (corresponding to the maximum propagating spatial frequency $1/\lambda$).
- For photonic chip facets: enormous beam divergence ($\sim \lambda/a$ where $a$ is the mode size) requires mode converters.
