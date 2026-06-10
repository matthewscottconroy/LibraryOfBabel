# Chapter 7: Silicon Photonics

> *"Silicon photonics exploits the fact that silicon is transparent at 1550 nm, and nearly everything else about it is a problem to be engineered around."*
>
> — Paraphrase of a common sentiment in the silicon photonics community

---

## The Chip-Scale Photonics Platform

Silicon photonics is the technology that makes photonic computing physically achievable at chip scale. By fabricating photonic devices — waveguides, modulators, couplers, detectors — using the same manufacturing processes as CMOS microelectronics (180 nm to 45 nm lithography, silicon-on-insulator substrates, standard chemical vapor deposition and etch processes), silicon photonics leverages the fabrication infrastructure of the semiconductor industry: the billion-dollar fabs, the nanometer-scale precision, the yield improvements from decades of process refinement, and the potential for monolithic co-integration of photonic and electronic circuits.

The result is photonic chips with hundreds or thousands of components, fabricated with the same design methodology as electronic ICs: schematic capture, simulation, layout versus schematic (LVS) checks, and tape-out through a multi-project wafer (MPW) service.

This chapter covers the silicon photonic platform in sufficient detail for a photonic computing engineer to understand and design chip-scale optical circuits.

---

## The Central Tension

Silicon photonics is powerful but imperfect. Its strengths — CMOS compatibility, high index contrast, mature fabrication, existing infrastructure — come with fundamental limitations:

1. **Silicon cannot lase** (indirect bandgap): requires off-chip or heterogeneously integrated III-V sources
2. **Silicon modulation requires the plasma dispersion effect**, which is the most efficient but also the most lossy and chirpy mechanism available — far inferior to the Pockels effect in LiNbO₃
3. **Germanium is needed for detection**, adding a separate epitaxial process
4. **High index contrast means high sensitivity to fabrication variations**: a ±5 nm variation in waveguide width shifts a ring resonance by ~0.3 nm — a significant effect for tight-tolerance WDM systems
5. **Thermal sensitivity**: silicon's thermo-optic coefficient ($dn/dT = 1.87 \times 10^{-4}$ K$^{-1}$) means that a 1°C temperature change shifts a ring resonance by ~80 pm — requiring active thermal tuning in any thermally sensitive application

These limitations are the engineering constraints within which photonic computing must operate. Understanding them quantitatively is the key to designing systems that work.

---

## Chapter Structure

**Section 7.1 — The Platform**: Why silicon? SOI substrate, waveguide geometry, materials constants.

**Section 7.2 — Passive Components**: Waveguide design, directional couplers, MMI couplers, MZIs, ring resonators, AWGs.

**Section 7.3 — Modulators**: The plasma dispersion effect, MZI modulators, ring modulators, and lithium niobate modulators (the alternative that avoids silicon's limitations).

**Section 7.4 — Switches and Tunable Devices**: Thermo-optic tuning, MEMS switches, and phase-change material switches for photonic computing reconfiguration.

**Section 7.5 — Beyond Silicon**: Si₃N₄, lithium niobate on insulator (LNOI), and InP platforms for specialized photonic computing applications.
