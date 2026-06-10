# Unit IV: Information Theory and Optical Communications

> *"The fundamental problem of communication is that of reproducing at one point either exactly or approximately a message selected at another point."*
>
> — Claude Shannon, *A Mathematical Theory of Communication*, 1948

---

## What This Unit Is About

Units I through III have developed the physics of photonic systems: what light is, how lasers work, how photons are detected, how waveguides guide, how silicon photonics is fabricated, and how photonic crystals and metasurfaces manipulate wavefronts. We now turn to the question of what photonic systems are for — and in the 21st century, the primary purpose of photonic systems is carrying information.

This unit is about information: what it is mathematically, how much a physical channel can carry, how to encode it efficiently, how to protect it from noise, and how it flows through the optical networks that undergird the global internet, cloud computing, and AI inference infrastructure. The two chapters develop these ideas in sequence:

**Chapter 9 — Information Theory and Optical Channels**: Shannon's theory of information, the capacity of the optical channel, modulation formats from OOK to coherent QAM, WDM multiplexing, and forward error correction. The goal is to understand the information-theoretic limits that govern what optical systems can and cannot transmit — and how close current technology comes to those limits.

**Chapter 10 — Optical Interconnects**: The optical network from the data center down to the chip — a hierarchy of scales from 1000 km to 1 μm, each with different physics and engineering constraints. We examine the "interconnect bottleneck" in computing systems, co-packaged optics as the current solution, photonic network-on-chip as the future vision, and the gap between that vision and present hardware capability.

## Why Information Theory Matters for Photonic Computing

The connection between information theory and photonic *computing* (as opposed to photonic *communications*) may not be immediately obvious. After all, matrix multiplication and neural network inference are not communication problems in the ordinary sense.

But consider the following:

First, any analog photonic computing system — an MZI mesh performing matrix-vector multiplication — processes information whose precision is limited by noise. The signal-to-noise ratio of the photonic system, the effective number of bits (ENOB), the precision of analog weights: all of these are information-theoretic concepts. Shannon's framework provides the right language for quantifying what a photonic compute engine can do.

Second, any photonic computing system is embedded in a larger system that includes memory, processors, and networks. The utility of a photonic tensor processing unit depends on the rate at which data can be moved to and from it. If the interconnect bandwidth is the bottleneck, the compute performance is irrelevant. Understanding interconnect physics and information-theoretic limits is essential for honest evaluation of photonic computing system proposals.

Third, forward error correction (FEC) is how digital systems operate reliably near Shannon's capacity limit. When photonic computing is used for digital signal processing — decoding FEC codes, for example — the relevant question is whether photonic systems can perform decoding faster, at lower power, or more efficiently than electronic ones. Answering this requires understanding what FEC codes are and what they demand computationally.

---

## References

[1] Shannon, C.E. (1948). "A mathematical theory of communication." *Bell System Technical Journal*, 27(3), 379–423; 27(4), 623–656. [The foundational paper of information theory. Reproducible online in its original form; essential reading for anyone in communications, computing, or signal processing.]

[2] Agrawal, G.P. (2012). *Fiber-Optic Communication Systems*, 5th ed. Wiley. [The standard textbook for optical communications systems, covering modulation formats, WDM, amplifiers, and system design.]

[3] Miller, D.A.B. (2017). "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35(3), 346–396. [A key paper on the energy limits of optical communications and interconnects; essential for understanding where photonics can and cannot beat electronics.]
