# Chapter 8: Photonic Crystals, Metasurfaces, and Plasmonics

## The Limits of the Average

The waveguides, resonators, and modulators of Chapters 6 and 7 are built from materials with spatially uniform optical properties — silicon, silicon dioxide, silicon nitride. Their guiding and resonance behavior derives from the geometry of the interface between uniform media, not from the internal structure of the media themselves. This is an implicit assumption that has guided a century of photonics engineering, and it has been enormously productive.

But there is a complementary approach: rather than engineering the shape of interfaces between uniform media, engineer the internal structure of the medium itself. Make a material whose dielectric constant varies periodically on the scale of the optical wavelength. Scatter light off precisely arranged arrays of resonant sub-wavelength scatterers. Confine light in gaps much smaller than a wavelength using the collective oscillations of a free-electron plasma.

These are the ideas behind photonic crystals, metasurfaces, and plasmonics respectively. Together, they define a field sometimes called "structural photonics" — the use of geometric structure at the nano- and micro-scale to control electromagnetic waves in ways that no homogeneous material can match. The physics is richer than in the waveguide regime, the mathematics is more demanding, and the engineering possibilities are more dramatic.

## Why This Chapter Matters for Photonic Computing

The relevance of this material to photonic computing is both direct and foundational.

**Directly**: Photonic crystal cavities are the highest-Q, smallest-volume optical resonators known. They concentrate light into mode volumes of $(\lambda/2n)^3$ — ten million times smaller than a microring resonator — enabling extreme nonlinear interactions, ultra-low-threshold lasing, and the possibility of quantum optical operations at room temperature. Metasurfaces can implement arbitrary complex-valued transmission functions in a single patterned layer a few hundred nanometers thick, raising the prospect of a "diffractive optical network" where the entire computation happens as light passes through a stack of metasurface layers. Plasmonics concentrates optical energy below the diffraction limit, potentially enabling optical interconnects at electronic length scales.

**Foundationally**: The Bloch theorem for photonic crystals — the optical analog of the electronic band structure theory that underlies all of solid-state physics — is one of the most beautiful and consequential results in classical electromagnetism. Understanding it is understanding something deep about the nature of waves in periodic media, with implications that extend far beyond any specific application.

## Chapter Structure

This chapter develops three topics in parallel with a common theme: periodic or structured media as a route to extraordinary optical control.

**Section 8.1 — Photonic Crystals** begins with the 1D case: the Bragg reflector, which is the photonic crystal in its simplest form and an exact analogy to the quantum mechanical problem of an electron in a periodic potential. We develop the Bloch theorem for electromagnetic fields, introduce the photonic band structure, and explore the photonic bandgap — the range of frequencies for which no propagating modes exist in the crystal. We then examine 2D photonic crystal slabs (the geometry most relevant to integrated photonics) and slow-light waveguides, which exploit the flat bands near the bandgap edges to achieve dramatic group velocity reduction.

**Section 8.2 — Metasurfaces** introduces the concept of a metasurface: a 2D array of sub-wavelength resonant structures that impart a spatially varying phase, amplitude, or polarization transformation on incident light. We derive the Pancharatnam-Berry phase mechanism by which geometric phase — arising from the orientation of anisotropic scatterers — enables continuous phase control from 0 to $2\pi$ with a single layer. We then examine the application of metasurfaces to diffractive optical computing: the physical basis for D²NN (diffractive deep neural network) architectures.

**Section 8.3 — Plasmonics** develops the physics of surface plasmon polaritons (SPPs): hybrid electromagnetic-plasma oscillations that propagate along metal-dielectric interfaces. We derive the SPP dispersion relation from Maxwell's boundary conditions, show that SPPs are confined to the interface with an evanescent tail on both sides, and analyze the trade-off between confinement and propagation loss that is the central challenge of plasmonics. We close with an assessment of plasmonic approaches to photonic computing: where they offer genuine advantages and where the ohmic losses of metals are a hard physical barrier.

## A Note on Scope

This chapter is selective, not encyclopedic. Photonic crystals, metasurfaces, and plasmonics are each large fields with their own textbooks. We focus on the concepts most directly relevant to photonic computing and develop the physics with enough rigor to enable genuine understanding. For deeper coverage, the references in each section provide the path forward.

---

## References

[1] Yablonovitch, E. (1987). "Inhibited spontaneous emission in solid-state physics and electronics." *Physical Review Letters*, 58(20), 2059–2062. [One of the founding papers of the photonic crystal concept; proposes periodic dielectric structure as a means of controlling spontaneous emission.]

[2] John, S. (1987). "Strong localization of photons in certain disordered dielectric superlattices." *Physical Review Letters*, 58(23), 2486–2489. [The companion founding paper, proposing photon localization in disordered photonic structures.]

[3] Yu, N. & Capasso, F. (2014). "Flat optics with designer metasurfaces." *Nature Materials*, 13(2), 139–150. [The review that established modern metasurface science as a distinct field; essential reading.]

[4] Ritchie, R.H. (1957). "Plasma losses by fast electrons in thin films." *Physical Review*, 106(5), 874–881. [The original prediction of surface plasmon polaritons.]
