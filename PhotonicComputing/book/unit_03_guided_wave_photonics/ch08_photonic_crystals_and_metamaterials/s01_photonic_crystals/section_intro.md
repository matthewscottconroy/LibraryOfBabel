# Section 8.1: Photonic Crystals

In 1987, almost simultaneously, Eli Yablonovitch at Bell Communications Research and Sajeev John at Princeton published papers proposing that a periodic dielectric structure — a "photonic crystal" — could control the spontaneous emission of atoms and localize photons in ways impossible in homogeneous media. The key idea was an analogy: just as an electron moving through the periodic potential of an atomic lattice develops an energy band structure with forbidden energy gaps, photons moving through a periodic dielectric medium develop a *photonic band structure* with forbidden frequency gaps. In a photonic bandgap material, certain frequencies of light cannot propagate in any direction.

This was a conceptual revolution. The message was that the photon, like the electron, responds to periodic structure by forming bands and gaps — and that these bands and gaps can be engineered by designing the structure. Spontaneous emission of an atom placed inside a photonic bandgap material is suppressed, because there are no electromagnetic modes into which the atom can emit. Conversely, if the crystal has a defect — a missing hole, an extra rod — a localized mode appears inside the gap, like an impurity state inside a semiconductor band gap. Light in this defect mode is trapped.

The implications for photonic computing are substantial:
- Photonic crystal microcavities have quality factors $Q > 10^6$ in volumes as small as $(\lambda/2n)^3$, enabling extreme light-matter coupling for low-threshold lasers and nonlinear optics.
- Photonic crystal waveguides (line defects in a 2D crystal) can slow light to a group velocity of $c/100$ or less, enhancing nonlinear interactions by $v_g^{-2}$ — enabling compact, high-efficiency optical modulators.
- Photonic crystal slabs, which can be fabricated on standard silicon photonics platforms, provide ultra-compact optical routing with unique dispersion properties.

This section develops the physics from the simplest case (1D Bragg reflector) through the Bloch theorem to 2D photonic crystal slabs and slow-light waveguides.

**Subsection 8.1.1 — Bragg Reflector (1D)**: The Fabry-Perot analogy and the origin of photonic bandgaps in alternating dielectric layers.

**Subsection 8.1.2 — Bloch's Theorem for Photons**: The mathematical framework: Floquet-Bloch modes, reciprocal lattice, Brillouin zone, and photonic band structure.

**Subsection 8.1.3 — 2D Photonic Crystal Slabs**: The practical geometry for integrated photonics. Air holes in silicon, triangular lattice, and photonic bandgap engineering.

**Subsection 8.1.4 — Slow Light**: Group velocity at band edges, the slow-light enhancement factor, and applications to compact nonlinear devices.
