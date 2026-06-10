# 2.5.3 — The van Cittert-Zernike Theorem

## The Problem: Coherence from an Incoherent Source

Thermal light sources — lamps, LEDs, the sun — are spatially incoherent at their emitting surface: different atoms emit independently, with no fixed phase relationship. The mutual intensity at the source surface is:

$$J_\text{source}(\mathbf{r}_1', \mathbf{r}_2') = I(\mathbf{r}_1') \delta^{(2)}(\mathbf{r}_1' - \mathbf{r}_2')$$

(zero correlation between any two distinct points). This is completely incoherent.

Yet, as this incoherent light propagates to a distant observation plane, it develops spatial coherence. The moon, the sun, and distant stars are spatially incoherent sources, but the light they deliver to Earth is partially coherent — Michelson was able to measure stellar diameters by observing interference fringes in starlight! How does an incoherent source produce coherent light at a distance?

The answer is the **van Cittert-Zernike theorem** (independently derived by Pieter van Cittert in 1934 and Frits Zernike in 1938 [1, 2]).

## Statement of the Theorem

Consider an incoherent, quasi-monochromatic source of total intensity distribution $I(\boldsymbol{\xi})$ in the source plane at $z = 0$. The mutual intensity at two points $\mathbf{r}_1$ and $\mathbf{r}_2$ in a distant observation plane at $z = R \gg \sqrt{A_\text{source}/\lambda}$ (the far field) is:

$$J(\mathbf{r}_1, \mathbf{r}_2) = \frac{k^2}{4\pi^2 R^2} \int I(\boldsymbol{\xi}) \, e^{i\frac{k}{R}(\boldsymbol{\xi} \cdot \mathbf{r}_1 - \boldsymbol{\xi} \cdot \mathbf{r}_2)} \, d^2\boldsymbol{\xi}$$

$$= C \hat{I}\left(\frac{k(\mathbf{r}_1 - \mathbf{r}_2)}{R}\right)$$

where $\hat{I}$ is the 2D Fourier transform of the source intensity distribution.

**Stated plainly**: The degree of spatial coherence in the far field from an incoherent source is the *Fourier transform of the source intensity profile*. The argument of the Fourier transform is the separation vector $\mathbf{r}_1 - \mathbf{r}_2$ scaled by $k/R = 2\pi/(\lambda R)$.

## Intuitive Explanation

Why does propagation build up coherence? Consider two observation points $\mathbf{r}_1$ and $\mathbf{r}_2$ in the far field. Light from any point $\boldsymbol{\xi}$ of the source arrives at $\mathbf{r}_1$ with phase $kr_1(\boldsymbol{\xi})$ and at $\mathbf{r}_2$ with phase $kr_2(\boldsymbol{\xi})$. The phase difference is $k[r_1(\boldsymbol{\xi}) - r_2(\boldsymbol{\xi})] \approx k\boldsymbol{\xi}\cdot(\mathbf{r}_1 - \mathbf{r}_2)/R$ (in the far field).

Now sum over all source points, each emitting independently (incoherent source):

$$J(\mathbf{r}_1, \mathbf{r}_2) \propto \int I(\boldsymbol{\xi}) e^{ik\boldsymbol{\xi}\cdot(\mathbf{r}_1-\mathbf{r}_2)/R} d^2\boldsymbol{\xi}$$

Each source point contributes a phasor that rotates as $\boldsymbol{\xi}$ varies. The integral — a Fourier transform — adds these phasors. For $\mathbf{r}_1 = \mathbf{r}_2$ (zero separation), all phasors point in the same direction and add coherently (giving total intensity). For $|\mathbf{r}_1 - \mathbf{r}_2| = \lambda R/D$ (where $D$ is the source diameter), the phasors rotate through roughly $2\pi$ and the integral begins to cancel — reducing coherence.

The spatial coherence length at distance $R$ from a source of angular diameter $\theta = D/R$:

$$l_c \approx \frac{R\lambda}{D} = \frac{\lambda}{\theta}$$

This is the result stated in Section 2.5.2, now with a derivation.

## Examples

**The Sun** ($D = 1.39 \times 10^9$ m, $R = 1.5 \times 10^{11}$ m, $\lambda = 550$ nm):
$$l_c = \frac{5.5 \times 10^{-7} \times 1.5 \times 10^{11}}{1.39 \times 10^9} \approx 60 \text{ μm}$$

Young's double slit can show solar interference fringes if the slit separation is less than 60 μm — confirming the van Cittert-Zernike theorem.

**Stellar diameter measurement (Michelson stellar interferometer)**: By increasing the separation of two apertures (input mirrors) in a stellar interferometer until fringes disappear, the angular diameter $\theta$ of a star can be determined from $\theta = \lambda/l_c$. Michelson measured the angular diameter of Betelgeuse in 1920 as $\theta \approx 47$ milli-arcseconds, using mirror separations up to 6.1 m [3]. This was the first direct measurement of a stellar diameter.

## The Theorem as Fourier Relationship

The van Cittert-Zernike theorem is another manifestation of the Fourier relationship between source properties and far-field coherence:

- **Fraunhofer diffraction** (Section 2.3.2): Far-field intensity = Fourier transform of aperture field amplitude.
- **Van Cittert-Zernike theorem**: Far-field spatial coherence = Fourier transform of source intensity distribution.

These are the same mathematical structure, applied to different physical scenarios. The common thread is propagation: free-space propagation over large distances performs a Fourier transform on any property of the field (amplitude, intensity distribution, coherence).

## Application: Designing Illumination for Photonic Chips

When coupling an incoherent source (LED, broadband laser) to a photonic chip, the spatial coherence of the source at the coupling aperture determines how many independent spatial modes are coupled in. For a single-mode waveguide input (coupling to one mode), only the fraction of light within one coherence area $l_c^2$ contributes to the guided mode. All other modes are lost to radiation.

For a single-mode fiber-coupled source: only the light within the fiber mode area (typically $\sim 50$ μm² for single-mode fiber at 1550 nm) is accepted. If the illumination source has coherence area larger than the fiber mode area, coupling efficiency is limited by mode overlap, not coherence. If the coherence area is smaller than the fiber mode area (e.g., an LED), each coherence cell contributes independently, and coupling efficiency is proportional to $l_c^2 / A_\text{mode}$.

This is why LEDs couple poorly to single-mode fibers (coherence area $\ll$ fiber mode area), while single-mode lasers couple efficiently (coherence area $\gg$ any relevant aperture).

## Summary

- Van Cittert-Zernike theorem: far-field spatial coherence = Fourier transform of source intensity profile.
- Spatial coherence length $l_c = \lambda/\theta_s$, where $\theta_s$ is the source angular diameter.
- Incoherent sources develop spatial coherence upon propagation (through the Fourier transform mechanism).
- Practical consequence: single-mode fibers accept only spatially coherent light; LED-to-fiber coupling is inherently inefficient.

---

*References*

[1] van Cittert, P.H. (1934). Die wahrscheinliche Schwingungsverteilung in einer von einer Lichtquelle direkt oder mittels einer Linse beleuchteten Ebene. *Physica*, 1(1–6), 201–210.

[2] Zernike, F. (1938). The concept of degree of coherence and its application to optical problems. *Physica*, 5(8), 785–795.

[3] Michelson, A.A. & Pease, F.G. (1921). Measurement of the diameter of α-Orionis with the interferometer. *Astrophysical Journal*, 53, 249–259. [The first stellar interferometry measurement of a stellar diameter.]
