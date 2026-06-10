# Chapter 6: Waveguides and Fiber

> *"The world is connected by a web of glass threads thinner than a human hair, each capable of carrying more information in a second than all the telephone conversations in the United States on a busy day in 1980."*
>
> — Adapted from common statements on fiber capacity, circa 2000

---

## The Guided Photon

A photon in free space travels in a straight line — useful for some computations, but impractical for routing signals around a chip or across an ocean. The optical waveguide solves this problem by exploiting total internal reflection to keep light confined within a high-index core.

The theory of optical waveguides is both classical (Maxwell's equations applied to piecewise-homogeneous media) and remarkably elegant. The key insight is that confinement quantizes the transverse degrees of freedom: only certain discrete modes can propagate without diffraction, just as only certain electron states exist in a quantum well. The mode structure — how many modes are supported, what their field profiles are, how their phase velocities depend on frequency — determines almost everything about how light propagates in a waveguide.

This chapter develops this theory in full, from the simplest case (the planar slab waveguide, analytically soluble) to the circular optical fiber (soluble in terms of Bessel functions) to the silicon strip waveguide (typically solved numerically). We then apply the theory to optical fiber — the medium that carries essentially all long-distance optical communication in the world today.

---

## Four Sections, Four Questions

**Section 6.1 — Planar Waveguide Theory** answers: *How are electromagnetic modes classified and calculated in the simplest waveguide geometry?* We derive the transverse resonance condition and the characteristic equation for TE and TM modes, the cutoff condition, and the confinement factor that determines the effective gain or loss in an active waveguide.

**Section 6.2 — Optical Fiber** answers: *How do waveguide principles extend to cylindrical fiber, and what limits fiber transmission?* We treat step-index and graded-index fibers, the weakly guiding approximation and LP modes, single-mode conditions, attenuation mechanisms (the 0.18 dB/km minimum at 1550 nm — a physical law, not an engineering target), and chromatic dispersion and its management.

**Section 6.3 — Nonlinear Effects in Fiber** connects to Chapter 3: *How do the Kerr effect, Raman, and four-wave mixing manifest in long-fiber transmission systems, and how are they managed?* These effects set the limits on WDM channel power and spacing in fiber-optic links from data centers to photonic computing network interconnects.

**Section 6.4 — Optical Amplifiers** answers: *How is loss compensated in long-reach systems, and what are the noise implications?* EDFA gain physics, noise figure, and distributed Raman amplification are treated with enough detail to understand the amplifier chain designs used in real fiber systems.

---

## Connection to Photonic Computing

Optical fiber is the medium connecting photonic computing chips to the outside world: to memory systems, to other chips in a multi-chip photonic processor, and to network infrastructure. Understanding fiber characteristics — especially loss, dispersion, and the nonlinear limitations on power — is prerequisite to designing the I/O of any photonic computing system.

More directly, the *silicon photonic waveguide* (Chapter 7) is the on-chip analogue of optical fiber: a high-index-contrast core (Si, $n = 3.48$) surrounded by a lower-index cladding (SiO₂, $n = 1.44$). The mode theory developed in this chapter applies directly, with the quantitative differences arising from the much stronger confinement (450 nm core vs. 9 μm core in SMF-28) and the resulting much larger waveguide dispersion.
