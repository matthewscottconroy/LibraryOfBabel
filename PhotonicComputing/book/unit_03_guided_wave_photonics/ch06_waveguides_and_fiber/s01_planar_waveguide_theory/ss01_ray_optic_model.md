# 6.1.1 Ray Optic Model and the TIR Condition

## Guidance by Total Internal Reflection

The simplest picture of waveguide guidance is the ray optic (or geometric optics) model. A ray of light traveling in a high-index slab ($n_1$) strikes the boundary with the lower-index cladding ($n_2 < n_1$) at grazing angle $\theta$ measured from the interface (or equivalently, at angle $90° - \theta$ from the normal).

From Chapter 2, TIR occurs when $\theta > \theta_c = \arcsin(n_2/n_1)$. For Si/SiO₂: $\theta_c = \arcsin(1.44/3.48) = 24.5°$.

A ray that undergoes TIR at both top and bottom interfaces of the slab will zig-zag along the waveguide, effectively propagating along the $z$-axis. The ray is guided.

## The Bounce Angle and Phase Condition

For a slab of thickness $d$, a ray at angle $\theta$ to the interface travels a transverse distance of $2d$ per round trip (one complete zig-zag). The transverse wavevector component is:

$$\kappa = n_1 k_0 \cos\theta$$

where $k_0 = \omega/c = 2\pi/\lambda$.

The ray picture is incomplete: not all bounce angles are allowed. Wave optics requires that the total phase accumulated in one round trip be a multiple of $2\pi$ (constructive interference condition). The phase accumulated includes:
- Propagation phase: $2\kappa d$ (two transverse traversals)
- Reflection phases: $2\phi_{TIR}$ at each interface (TIR introduces a Goos-Hänchen phase shift)

The self-consistency condition is:

$$2\kappa d + 2\phi_{TIR} = 2\pi m, \quad m = 0, 1, 2, \ldots$$

This quantizes the allowed values of $\kappa$, and hence the allowed propagation constants $\beta = n_1 k_0 \sin\theta$. Each allowed value corresponds to a *mode* of the waveguide.

**Insight**: The ray model correctly identifies TIR as the guidance mechanism and reveals that modes are discretely quantized, but it cannot predict the field profiles or the exact cutoff conditions without including the wave nature of the Goos-Hänchen reflection phase. The full wave theory of Section 6.1.2–6.1.3 is required.

## Numerical Aperture and Acceptance Angle

The numerical aperture of a waveguide is defined as:

$$\text{NA} = \sqrt{n_1^2 - n_2^2}$$

A ray in external medium ($n_0 = 1$) is accepted into the waveguide if its angle $\theta_{ext}$ from the waveguide axis satisfies:

$$n_0 \sin\theta_{ext} = \text{NA}$$

This is the acceptance angle. For Si/SiO₂: NA = $\sqrt{3.48^2 - 1.44^2} = \sqrt{12.11 - 2.07} = 3.17$ — formally > 1, meaning any angle from outside is accepted (the waveguide accepts all angles). This high NA is a consequence of the high index contrast of silicon photonics and means that input coupling from a fiber (NA ≈ 0.14) requires a mode converter (inverse taper or grating coupler), not just end-fire coupling.

For low-contrast waveguides (optical fiber: NA ≈ 0.13), $\theta_{ext} \approx \arcsin(0.13) \approx 7.5°$ — only rays within this narrow acceptance cone couple into the fiber.
