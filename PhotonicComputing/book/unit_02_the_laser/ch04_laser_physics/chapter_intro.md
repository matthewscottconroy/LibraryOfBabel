# Chapter 4: Laser Physics

> *"I just went ahead and built the thing."*
>
> — Theodore Maiman, on his decision to attempt the first laser
> after his grant proposal was rejected

---

## The Most Useful Device in Photonic Computing

The laser is not merely a light source. It is a device that produces light in a particular *state* — a state characterized by coherence, directionality, narrow spectrum, and the ability to carry information with high fidelity over long distances. These properties are not obtainable from any other source. A light bulb, a candle, a white LED, and the sun all produce incoherent light: photons emitted at random times, in random directions, with phases uncorrelated from one photon to the next. A laser produces photons in phase, at nearly the same frequency, traveling in the same direction. The difference between these two kinds of light is the difference between noise and signal — between thermal fluctuation and deliberate information encoding.

For photonic computing, this distinction is decisive. An MZI-based matrix multiplier works by interference: two optical paths are combined, and the relative phase determines the output amplitude. Interference requires coherence. A photonic neural network that uses wavelength-division multiplexing to carry multiple matrix rows simultaneously requires a light source stable enough in frequency that different wavelength channels remain distinguishable. A quantum photonic processor exploits two-photon interference, Hong-Ou-Mandel-style, to implement entangling gates — and this requires that photons from the source be identical, which means indistinguishable in frequency, arrival time, and polarization. In each of these cases, the laser is not just convenient; it is the physical requirement.

This chapter develops the full physics of laser operation, starting from the population inversion physics of Chapter 3 and adding the essential new ingredient: a resonant optical cavity that provides feedback, selects modes, and narrows the emission.

---

## The Four Questions This Chapter Answers

**Why can't you make a laser from a two-level atom?** This is not obvious. Stimulated emission — the mechanism that amplifies light — exists in a two-level system. Yet a two-level system cannot sustain a laser. The reason is thermodynamic, and understanding it is the key to understanding why every practical laser uses a three- or four-level pumping scheme.

**What does a resonant cavity add?** An optical gain medium alone is an amplifier, not a laser. The cavity provides frequency selectivity, directional feedback, and the positive reinforcement that drives stimulated emission to dominate over spontaneous emission. Without understanding resonant mode structure, you cannot understand why a DFB laser has a single frequency while a Fabry-Pérot laser has multiple, nor why VCSELs have such small thresholds.

**What determines laser noise and linewidth?** A perfect laser would have zero linewidth: a perfectly monochromatic wave. Real lasers have finite linewidth because spontaneous emission continuously injects random-phase photons into the cavity mode. The Schawlow-Townes formula, derived in this chapter, gives the fundamental quantum limit on laser linewidth. This number — often in the hertz-to-kilohertz range for semiconductor lasers — determines the coherence length available for interference-based photonic computing.

**Which laser architecture is right for which application?** Fabry-Pérot, DFB, DBR, VCSEL, external-cavity tunable — each has tradeoffs in linewidth, tunability, speed, power, and integration. The photonic computing engineer must know these tradeoffs to make informed design decisions.

---

## The Structure of This Chapter

**Section 4.1 — Population Inversion** reviews why inversion is necessary, why it cannot be achieved in a two-level system, and how three- and four-level systems circumvent this constraint. We develop the rate equations for gain and carrier density that are the workhorse of laser design.

**Section 4.2 — Optical Resonators** treats the cavity: longitudinal modes, mode spacing, finesse, Q factor, and Gaussian transverse modes. The stability criterion for a resonator geometry is derived and applied to practical resonator designs.

**Section 4.3 — Laser Operation** analyzes what happens above threshold: the threshold condition, slope efficiency, and the steady-state balance between gain and loss. We treat pulse generation by Q-switching and mode-locking, and derive the Schawlow-Townes linewidth.

**Section 4.4 — Laser Types** surveys the specific architectures used in photonic computing: DFB lasers (the standard single-frequency source), VCSELs (for 2D arrays and short-reach links), heterogeneously integrated III-V-on-silicon lasers (the current state of the art for on-chip sources), and microresonator frequency combs (the emerging multi-wavelength source for WDM photonic computing).

---

## Mathematical Prerequisites

This chapter builds on:
- **Chapter 3, Section 3.2**: Two-level systems, Einstein coefficients, population inversion
- **Chapter 2, Section 2.2**: Fabry-Pérot cavities, finesse, Q factor
- **Chapter 2, Section 2.6**: Gaussian beams and ABCD matrices

New mathematical tools introduced:
- **Rate equations**: coupled ODEs for population densities and photon number
- **Gain saturation**: modified rate equations above threshold
- **Langevin noise terms**: stochastic differential equations for laser phase noise (introduced phenomenologically; full treatment requires quantum optics beyond this book's scope)
