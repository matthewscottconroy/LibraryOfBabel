# 2.6.2 — Gaussian Beam Parameters

## The Gaussian Beam Solution

The Gaussian beam is the fundamental solution of the paraxial wave equation. We seek a solution of the form $u(r,z) = A(z)\exp[if(r,z)]$, where we expect the transverse profile to be Gaussian. The ansatz:

$$u(r,z) = A_0 \frac{w_0}{w(z)} \exp\left(-\frac{r^2}{w(z)^2}\right) \exp\left(-i\frac{kr^2}{2R(z)}\right) \exp(i\zeta(z))$$

where $r = \sqrt{x^2+y^2}$ is the transverse distance from the axis. Substituting into the paraxial wave equation and requiring consistency determines $w(z)$, $R(z)$, and $\zeta(z)$.

The result (which we state and interpret rather than derive in full detail):

## The Complex $q$-Parameter

The most compact way to present the Gaussian beam is through the *complex beam parameter* $q(z)$:

$$\frac{1}{q(z)} = \frac{1}{R(z)} - i\frac{\lambda}{\pi w(z)^2}$$

or equivalently, $q(z) = z - iz_R$ (measuring from the beam waist at $z = 0$), where $z_R = \pi w_0^2/\lambda$ is the *Rayleigh range*. Then:

$$\frac{1}{q(z)} = \frac{1}{z - iz_R} = \frac{z + iz_R}{z^2 + z_R^2} = \frac{z}{z^2+z_R^2} + i\frac{z_R}{z^2+z_R^2}$$

Comparing with the definition: $1/R(z) = z/(z^2+z_R^2)$ and $\lambda/(\pi w^2) = z_R/(z^2+z_R^2)$.

## Key Parameters

### Beam Waist $w_0$

The beam waist is the minimum radius of the Gaussian profile, occurring at $z = 0$. The intensity profile at $z = 0$ is:

$$I(r, 0) = I_0 e^{-2r^2/w_0^2}$$

where $I_0$ is the peak intensity. The $1/e^2$ intensity radius is $w_0$. (Note: the $1/e^2$ radius is the standard definition; the $1/e$ electric field radius is also $w_0$.)

### Rayleigh Range $z_R$

$$z_R = \frac{\pi w_0^2}{\lambda}$$

The Rayleigh range is the distance from the waist at which the beam area doubles and the peak intensity halves. For $z = z_R$: $w(z_R) = w_0\sqrt{2}$, $I_0(z_R) = I_0/2$.

**Worked example**: $w_0 = 10$ μm at $\lambda = 1550$ nm (in free space):

$$z_R = \frac{\pi (10 \times 10^{-6})^2}{1550 \times 10^{-9}} = \frac{\pi \times 10^{-10}}{1.55 \times 10^{-6}} \approx 202 \text{ μm}$$

The beam doubles its area after propagating only 202 μm — this is why a silicon chip output needs careful optics for coupling to a fiber.

For $w_0 = 5$ μm (typical single-mode fiber MFD is $\sim 10$ μm, but a smaller focused spot): $z_R \approx 50$ μm. For $w_0 = 50$ μm: $z_R \approx 5$ mm.

### Beam Radius $w(z)$

$$w(z) = w_0\sqrt{1 + \left(\frac{z}{z_R}\right)^2}$$

For $z \ll z_R$ (near field): $w(z) \approx w_0$ — nearly constant.
For $z \gg z_R$ (far field): $w(z) \approx w_0 z/z_R = \lambda z/(\pi w_0)$ — linear growth.

### Far-Field Divergence Angle

In the far field, $w(z) \approx \theta z$ where:

$$\theta = \frac{w_0}{z_R} = \frac{\lambda}{\pi w_0}$$

This is the *half-angle divergence* of the Gaussian beam. For $w_0 = 10$ μm: $\theta \approx 1550/(10,000\pi) \approx 0.049$ rad $\approx 2.8°$.

**The uncertainty principle for beams**: The product $w_0 \theta = \lambda/\pi$ is a constant — independent of the waist size. This is the optical analog of the Heisenberg uncertainty principle: the more tightly you focus a beam ($w_0$ small), the more rapidly it diverges ($\theta$ large). A Gaussian beam achieves the minimum possible product $w_0 \theta = \lambda/\pi$ — it is the diffraction-limited beam.

### Wavefront Radius of Curvature $R(z)$

$$R(z) = z\left[1 + \left(\frac{z_R}{z}\right)^2\right]$$

At $z = 0$ (the waist): $R(0) \to \infty$ — the wavefront is flat (planar).
At $z = z_R$: $R(z_R) = 2z_R$ — minimum radius of curvature.
For $z \gg z_R$: $R(z) \approx z$ — the wavefront is a sphere centered at the waist.

A lens focused at the waist should have focal length $f = R(z_\text{lens})$, the wavefront radius of curvature at the lens position.

### Gouy Phase $\zeta(z)$

$$\zeta(z) = \arctan\left(\frac{z}{z_R}\right)$$

The Gouy phase is an additional phase accumulated by the beam beyond the $kz$ plane wave phase. Across the entire beam (from $z = -\infty$ to $z = +\infty$), the total Gouy phase is $\pi$. This $\pi$ phase shift distinguishes a focused Gaussian beam from a plane wave and is observable in interferometric experiments.

**Significance for optical cavities**: The resonance condition for a cavity mode is that the round-trip phase is a multiple of $2\pi$. The Gouy phase contribution to the round-trip phase determines the frequency spacing between transverse modes (TEMpq modes) of the cavity. In a laser, this affects which transverse modes compete for gain.

## The Complete Gaussian Beam Expression

$$E(r,z) = E_0 \frac{w_0}{w(z)} \exp\left(-\frac{r^2}{w(z)^2}\right) \exp\left(-i\frac{kr^2}{2R(z)}\right) \exp[i(kz - \omega t + \zeta(z))]$$

Each factor has a clear physical meaning:
- $w_0/w(z)$: amplitude decreases as the beam expands (energy conservation)
- $\exp(-r^2/w^2)$: Gaussian transverse profile
- $\exp(-ikr^2/(2R))$: spherical wavefront curvature
- $\exp[i(kz - \omega t + \zeta)]$: propagating wave with Gouy phase

The intensity is $I(r,z) = I_0[w_0/w(z)]^2\exp(-2r^2/w(z)^2)$ — a Gaussian with $1/e^2$ radius $w(z)$ and peak intensity decreasing as $w_0^2/w(z)^2$ (area increase).

## Higher-Order Modes

The Gaussian beam is the TEM₀₀ mode. Higher-order solutions in Cartesian coordinates are the Hermite-Gaussian (HG) modes TEM$_{mn}$:

$$u_{mn}(x, y, z) \propto H_m\!\left(\frac{\sqrt{2}x}{w}\right) H_n\!\left(\frac{\sqrt{2}y}{w}\right) \exp\left(-\frac{x^2+y^2}{w^2}\right) \exp(\ldots)$$

where $H_m$ are Hermite polynomials. These modes form a complete orthonormal set for paraxial beams in Cartesian geometry. They are the transverse eigenmodes of a resonator with rectangular symmetry.

In cylindrical coordinates, the Laguerre-Gaussian (LG) modes are the appropriate basis:

$$u_{p\ell}(r,\phi,z) \propto \left(\frac{r\sqrt{2}}{w}\right)^{|\ell|} L_p^{|\ell|}\!\left(\frac{2r^2}{w^2}\right) \exp\left(-\frac{r^2}{w^2}\right) e^{i\ell\phi} \exp(\ldots)$$

where $L_p^{|\ell|}$ are associated Laguerre polynomials. The LG mode with topological charge $\ell$ has a phase singularity (vortex) at the beam center and carries orbital angular momentum $\ell\hbar$ per photon (Section 1.5.3).

## Summary

- Gaussian beam parameters: waist $w_0$, Rayleigh range $z_R = \pi w_0^2/\lambda$, divergence $\theta = \lambda/(\pi w_0)$, wavefront curvature $R(z) = z[1 + (z_R/z)^2]$, Gouy phase $\zeta = \arctan(z/z_R)$.
- Uncertainty principle: $w_0 \theta = \lambda/\pi$ (constant for all Gaussian beams — they are diffraction-limited).
- Near field ($z \ll z_R$): nearly constant width; far field ($z \gg z_R$): linear growth.
- Higher-order modes: HG (Cartesian) and LG (cylindrical) families form complete orthogonal sets.
