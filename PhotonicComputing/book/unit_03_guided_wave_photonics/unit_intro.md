# Unit III: Guided-Wave Photonics — Waveguides, Silicon Photonics, and Photonic Crystals

> *"The concept of a waveguide for light was laughed at. Everyone knew light couldn't be confined in a thin strand of glass. Then Kao showed the math."*
>
> — Paraphrase of the reception of Charles Kao's 1966 proposal for low-loss optical fiber

---

## What This Unit Is About

Light, as we have seen in Units I and II, is an electromagnetic wave that travels in straight lines in free space but bends, reflects, and refracts at material interfaces. The question this unit asks is: *can we build structures that guide light along prescribed paths the way wires guide electrons?*

The answer is yes, and the physics is total internal reflection combined with the mode theory of electromagnetic waveguides. The result is one of the most consequential technologies in human history: the optical fiber, which carries terabits of information across oceans, and the silicon photonic waveguide, which routes light on a chip smaller than a fingernail.

But guided-wave photonics is not only fiber and silicon waveguides. It is also photonic crystals — periodic dielectric structures that create photonic bandgaps analogous to electronic bandgaps in semiconductors — and metasurfaces that control the phase, amplitude, and polarization of light with subwavelength precision. These newer technologies are at the frontier of photonic computing, enabling new approaches to optical neural networks and analog signal processing that would not be possible with conventional waveguides.

---

## Three Chapters, Three Platforms

**Chapter 6: Waveguides and Fiber** develops the complete theory of guided modes in dielectric waveguides, from the planar slab waveguide (analytically soluble) to optical fiber (approximately soluble using the weakly-guiding approximation). We derive the dispersion relation, cutoff conditions, group velocity and group velocity dispersion, and the coupled-mode equations for directional couplers. We then apply this theory to optical fiber: the loss mechanisms (Rayleigh scattering, OH absorption, infrared absorption), the fiber amplifier (EDFA), and the dispersion management that makes modern WDM transmission possible.

**Chapter 7: Silicon Photonics** is the chapter that most directly addresses the photonic computing engineer's needs. We start with the silicon-on-insulator (SOI) platform: why the 450 × 220 nm strip waveguide became the standard, how to design directional couplers and ring resonators, and how to build the three key active devices — the MZI modulator, the ring modulator, and the on-chip photodetector. We give specific attention to the engineering tradeoffs (loss vs. confinement, bandwidth vs. extinction ratio, coherence vs. power) that determine which architecture suits which computation.

**Chapter 8: Photonic Crystals and Metasurfaces** introduces the newer guided-wave platforms. Photonic crystals use periodic dielectric structure to create photonic bandgaps; photonic crystal waveguides achieve ultra-slow group velocities (high group index) that enhance light-matter interaction. Metasurfaces implement arbitrary phase profiles in ultrathin layers by engineering the geometry of subwavelength resonators. Both technologies have been proposed for photonic neural networks (in the case of metasurfaces, they can directly implement a matrix-vector multiply in a layer of dielectric).

---

## The Engineering Stakes

This unit is where theory meets fabrication. The physics of Chapter 1–3 is timeless; the waveguide parameters in this unit are measured in nanometers and constrained by CMOS lithography limits, materials bandgaps, and fabrication yields. The silicon photonic community has made specific choices about waveguide dimensions, coupling architectures, and material systems that determine what photonic computing is possible today. This unit explains why those choices were made and what alternatives exist.

---

## References for the Unit Introduction

[1] Kao, K.C., & Hockham, G.A. (1966). "Dielectric-fibre surface waveguides for optical frequencies." *Proceedings of the IEE*, 113(7), 1151–1158. [The paper proposing low-loss optical fiber. Kao received the Nobel Prize in 2009.]

[2] Soref, R.A., & Lorenzo, J.P. (1986). "All-silicon active and passive guided-wave components for $\lambda$ = 1.3 and 1.6 μm." *IEEE Journal of Quantum Electronics*, 22(6), 873–879. [The founding paper of silicon photonics.]

[3] Reed, G.T., Mashanovich, G., Gardes, F.Y., & Thomson, D.J. (2010). "Silicon optical modulators." *Nature Photonics*, 4(8), 518–526. [Review of silicon modulator physics and architectures.]
