# Section 8.3: Plasmonics

The chapters on silicon photonics (Chapter 7) and photonic crystals (Section 8.1) deal with structures whose minimum dimensions are on the order of $\lambda/2n$ — the diffraction limit. In silicon at 1550 nm, this is approximately $\lambda/(2 \times 3.48) \approx 220$ nm. At this scale, waveguide modes can be confined to cross-sections of $0.1$–$1$ μm².

Plasmonics breaks through this limit. By coupling electromagnetic fields to the collective oscillations of electrons at a metal surface, plasmonics confines light to dimensions of 10–100 nm — far below the diffraction limit. This is not a matter of better engineering; it is a fundamentally different physical mechanism.

The physical basis for this extreme confinement is the **surface plasmon polariton (SPP)**: a hybrid mode consisting of the electromagnetic field and the collective plasma oscillation of free electrons at a metal-dielectric interface. The SPP field is evanescent on both sides of the interface — it decays exponentially away from the surface — and can be guided along the metal surface.

The potential for computing with plasmonics is real: nanoscale optical interconnects that carry information at the speed of light but at dimensions compatible with nanoscale electronics; photodetectors with sub-diffraction active areas; extreme nonlinear interactions in nanoscale gaps where field intensities can reach $10^{12}$ W/cm².

But plasmonics has a fundamental limitation that cannot be engineered away: ohmic loss. Free electrons in metals are not frictionless — they scatter off lattice defects, phonons, and surfaces, converting electromagnetic energy to heat. In gold or silver at 1550 nm, the propagation length of an SPP is only 10–100 μm before the signal is absorbed. This loss is not an artifact of poor fabrication; it is the intrinsic material property of all metals at optical frequencies.

The history of plasmonics is partly the story of a decade-long optimism about "plasmonic computing" that collided with this physical reality. Understanding both the genuine capabilities and the hard limits of plasmonics is essential for making honest assessments of what photonic computing can achieve at the nanoscale.

This section develops the physics of SPPs and plasmonics in three subsections:

**Subsection 8.3.1 — Surface Plasmon Polaritons**: The dispersion relation derived from Maxwell's boundary conditions, the SPP mode profile, and the propagation length vs. confinement tradeoff.

**Subsection 8.3.2 — Sub-Wavelength Confinement**: Gap plasmons, nanofocusing, and the field enhancement in metallic nanogaps. Applications to SERS, nonlinear optics, and nanoscale photodetection.

**Subsection 8.3.3 — Plasmonics for Computing**: An honest assessment of where plasmonics contributes genuinely to photonic computing (photodetectors, nanoscale modulators, near-field coupling) and where the loss problem is a hard barrier.
