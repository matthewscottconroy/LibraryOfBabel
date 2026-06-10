# 2.3.1 — The Huygens-Fresnel Principle

## Huygens' Construction (1678)

Christiaan Huygens proposed, in his *Traité de la Lumière* (1678), a geometric construction for predicting how a wavefront propagates: every point on a wavefront at time $t$ acts as a source of secondary spherical wavelets. At a later time $t + dt$, the new wavefront is the envelope of all these secondary wavelets. Huygens used this to explain reflection and refraction geometrically.

The construction is correct but incomplete: it explains propagation and refraction, but does not naturally explain diffraction or interference. It also predicts forward-propagating wavelets but not backward-propagating ones (which should exist by symmetry). These gaps were filled by Fresnel.

## Fresnel's Extension (1818)

Fresnel combined Huygens' construction with the principle of interference: the amplitude at any observation point is the superposition (with phase) of all the Huygens wavelets from the wavefront [1]. This is the Huygens-Fresnel principle:

$$E(P) = \frac{-i}{\lambda} \iint_\Sigma \frac{E(Q)}{r_{QP}} K(\chi) \, e^{ikr_{QP}} \, dA$$

where:
- $\Sigma$ is the wavefront (or aperture)
- $Q$ is a point on $\Sigma$; $P$ is the observation point
- $r_{QP} = |P - Q|$ is the distance from $Q$ to $P$
- $K(\chi)$ is the obliquity factor (Kirchhoff's modification, discussed below)
- The factor $-i/\lambda$ ensures correct normalization

The key addition over Huygens: the phase factor $e^{ikr_{QP}}$ accounts for the phase accumulated by each wavelet traveling distance $r_{QP}$. Adding wavelets with their phases produces interference, which is what gives diffraction patterns their structure.

## Kirchhoff's Rigorous Formulation

Gustav Kirchhoff (1883) put the Huygens-Fresnel principle on a rigorous mathematical basis using Green's theorem [2]. Starting from the Helmholtz equation $(\nabla^2 + k^2)E = 0$, he derived the *Kirchhoff diffraction integral*:

$$E(P) = \frac{1}{4\pi} \iint_\Sigma \left[ E(Q) \frac{\partial}{\partial n}\left(\frac{e^{ikr}}{r}\right) - \frac{e^{ikr}}{r}\frac{\partial E(Q)}{\partial n} \right] dA$$

In the *Fraunhofer* limit (large $r$, so the Green's function looks like a plane wave at $P$), this simplifies to Huygens-Fresnel with obliquity factor:

$$K(\chi) = \frac{1}{2}(1 + \cos\chi)$$

where $\chi$ is the angle between the normal to $\Sigma$ and the direction to $P$. This factor is nearly 1 for forward propagation ($\chi \approx 0$) and 0 for backward propagation ($\chi = \pi$) — resolving Huygens' backward-wave problem.

**The practical content**: For paraxial geometries ($\chi \ll 1$), $K(\chi) \approx 1$ and the Kirchhoff integral reduces to:

$$E(P) \approx \frac{-i}{\lambda} \iint_\text{aperture} E(Q) \frac{e^{ikr_{QP}}}{r_{QP}} \, dA$$

This integral, evaluated over the open aperture (with $E = 0$ on the opaque screen), gives the diffracted field everywhere.

## The Fresnel and Fraunhofer Limits

The approximations made in evaluating the Kirchhoff integral depend on the geometry. Two standard approximations:

### Fresnel (Near-Field) Approximation

When the observation point $P$ is at distance $z$ from the aperture plane, with transverse position $(x, y)$, and the aperture point $Q$ is at $(x', y')$:

$$r_{QP} = \sqrt{z^2 + (x-x')^2 + (y-y')^2} \approx z\left[1 + \frac{(x-x')^2 + (y-y')^2}{2z^2}\right]$$

This is valid when the higher-order terms $(x-x')^4/(8z^3\lambda) \ll 1$. The resulting integral:

$$E(x,y,z) = \frac{e^{ikz}}{i\lambda z} \iint E(x',y') \, e^{i\frac{k}{2z}[(x-x')^2+(y-y')^2]} \, dx' \, dy'$$

is the *Fresnel diffraction integral*. It is a convolution of $E(x', y')$ with the quadratic phase kernel $h(x, y) = e^{ik(x^2+y^2)/(2z)}$.

### Fraunhofer (Far-Field) Approximation

When the observation distance satisfies $z \gg k(x'^2 + y'^2)_\text{max}/2$ (the quadratic terms in $x'$ and $y'$ are negligible), the Fresnel integral further simplifies:

$$E(x,y,z) = \frac{e^{ikz}}{i\lambda z} e^{i\frac{k}{2z}(x^2+y^2)} \iint E(x',y') \, e^{-i\frac{2\pi}{\lambda z}(xx'+yy')} \, dx' \, dy'$$

$$= \frac{e^{ikz}}{i\lambda z} e^{i\frac{k}{2z}(x^2+y^2)} \, \hat{E}\left(\frac{x}{\lambda z}, \frac{y}{\lambda z}\right)$$

where $\hat{E}(f_x, f_y) = \iint E(x',y') e^{-i2\pi(f_x x' + f_y y')} dx' dy'$ is the **two-dimensional spatial Fourier transform** of the aperture field, evaluated at spatial frequencies $f_x = x/(\lambda z)$, $f_y = y/(\lambda z)$.

**The central result**: In the Fraunhofer limit, the far-field diffraction pattern is the *spatial Fourier transform* of the aperture field. This is not an approximation valid only at extreme distances; it is exact whenever the observation plane is in the Fraunhofer zone, and a lens brings the Fraunhofer zone to a finite distance.

The Fraunhofer condition ($z \gg a^2/\lambda$, where $a$ is aperture size) for a 1 mm aperture at $\lambda = 1$ μm requires $z \gg 1$ m — quite far. But a converging lens of focal length $f$ focuses the Fraunhofer pattern at its back focal plane, regardless of $f$. We are always in the Fraunhofer regime when we are looking at the back focal plane of a lens.

## Why This Matters for Photonic Computing

The Huygens-Fresnel principle establishes that diffraction is unavoidable in any wave-optical system. Any aperture or finite wavefront will diffract. More specifically:

1. **Waveguide mode sizes**: The mode of a silicon nanowire waveguide is determined by the balance between total internal reflection confinement (ray optics) and diffraction (wave optics that would spread the beam). The equilibrium mode size is the solution to the paraxial wave equation (Section 2.6.1).

2. **Far-field beam properties**: When light exits a photonic chip facet (a waveguide end), it diffracts into free space. The angle of divergence is $\theta \approx \lambda/w$, where $w$ is the mode field diameter. For a 450 nm silicon waveguide, $w \approx 0.5$ μm, giving $\theta \approx 1550/500 \approx 3$ rad — nearly the full hemisphere. This explains why coupling between a photonic chip and an optical fiber is challenging and requires mode adapters (inverse tapers, grating couplers).

3. **Diffractive neural networks**: In diffractive neural network (D2NN) architectures, a multi-layer cascade of diffractive apertures (phase and amplitude masks) performs optical computation via controlled diffraction [3]. The diffraction physics is described exactly by the Fresnel integral, and the D2NN training procedure optimizes the phase profiles of the masks by backpropagating through the Fresnel propagator.

---

*References*

[1] Fresnel, A.-J. (1818). Mémoire sur la diffraction de la lumière. *Annales de Chimie et de Physique*, 1, 239–281.

[2] Kirchhoff, G. (1883). Zur Theorie der Lichtstrahlen. *Annalen der Physik*, 254(4), 663–695. [DOI: 10.1002/andp.18832540409]

[3] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). All-optical machine learning using diffractive deep neural networks. *Science*, 361(6406), 1004–1008. [DOI: 10.1126/science.aat8084] [The original D2NN paper, demonstrating optical classification at the speed of light.]
