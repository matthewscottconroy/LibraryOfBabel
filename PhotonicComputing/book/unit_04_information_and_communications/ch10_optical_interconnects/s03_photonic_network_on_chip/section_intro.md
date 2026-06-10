# Section 10.3: Photonic Network-on-Chip

## What This Section Is About

The previous two sections of this chapter have been about inter-chip and inter-server optical interconnects: the links and networks that connect separate computing nodes. This section turns inward — to the chip, and specifically to the vision of replacing the electrical wires *inside* a chip with optical waveguides.

This is a more speculative domain than the inter-chip interconnect, where optical technology has been in production for decades. Photonic networks-on-chip (PNoC) are not yet deployed in commercial processors. But the physics arguments for them are compelling, the engineering challenges are formidable, and the research community has made substantial progress in the past decade. Understanding the concept, the architecture options, and the remaining challenges is essential for anyone who wants to evaluate the claims made for photonic computing at the chip level.

The fundamental motivation is the same as for inter-chip interconnects, but the numbers are more extreme. Inside a modern processor, the wires that distribute the clock signal consume more power than all of the logic gates combined [1]. The wires that carry data between processor cores, cache levels, and memory controllers carry tens of terabits per second of aggregate bandwidth. As chip dimensions scale and clock frequencies rise (or more precisely, as more and more cores are packed onto a chip), the electrical interconnect inside the chip becomes the dominant power consumer and performance bottleneck.

Optics cannot solve this with the same physical mechanism it uses for long-distance interconnects — at chip scale, the *latency* advantage of photons over electrons vanishes (at 1 cm, the propagation time is 50 ps in copper vs. 66 ps in Si waveguide — copper actually wins on a straight line), and the energy advantage requires sub-fJ/bit transceivers that have not yet been demonstrated in silicon photonics. But optics has other potential advantages: wavelength-division multiplexing allows many parallel data streams to share a single waveguide, potentially providing enormous bandwidth density; and optical modulation can be *non-contact* (electro-optic effect without carrier injection/extraction), potentially avoiding the RC charging energy that dominates electrical links.

This section contains three subsections:

**Subsection 10.3.1: The PNoC Concept** develops the energy and bandwidth analysis that motivates PNoC, explains what it would need to achieve to be better than electrical alternatives, and examines where the current state of the art falls short.

**Subsection 10.3.2: WDM Routing Architectures** covers the specific circuit topologies proposed for PNoC — broadcast-and-select, wavelength-routing, and optical bus architectures — and analyzes their bandwidth-energy tradeoffs.

**Subsection 10.3.3: Integration Challenges** addresses the practical obstacles: laser power, thermal management, modulator energy, detector sensitivity, and the fundamental question of whether silicon photonics can be integrated with sub-10nm CMOS without compromising either.

---

*References for this section introduction are given within the subsections.*

[1] Weste, N.H.E., & Harris, D.M. (2015). *CMOS VLSI Design: A Circuits and Systems Perspective* (4th ed.). Pearson. [Standard reference; states that clock distribution can consume 30–40% of total chip power in high-speed designs.]
