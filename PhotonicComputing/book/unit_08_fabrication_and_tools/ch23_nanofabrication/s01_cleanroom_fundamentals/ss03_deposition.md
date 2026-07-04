# 23.1.3 Deposition, Planarization, and Implantation

Etching removes; this subsection is about adding — dielectric films that clad and protect waveguides, metals that heat and contact them, dopants that turn passive silicon into modulators — and about the planarization step that makes layer-upon-layer construction possible at all.

## Chemical Vapor Deposition: PECVD and LPCVD

In chemical vapor deposition (CVD), precursor gases react at the wafer surface to grow a film.

**PECVD (plasma-enhanced CVD)** uses a plasma to drive the chemistry at low temperature (250–400 °C). It is the workhorse for the SiO₂ **top cladding** deposited over finished waveguides (SiH₄ + N₂O) and for silicon nitride films (SiH₄ + NH₃). The low temperature is the point: PECVD can be applied *after* metals and germanium are on the wafer. The price is film quality — PECVD films are non-stoichiometric and hydrogen-rich, and in nitride the **N–H bond has a vibrational overtone near 1520 nm**, absorbing precisely in the telecom band. PECVD SiNx waveguides therefore show a loss peak in the C-band unless the hydrogen is driven out by high-temperature annealing (which the thermal budget may forbid) or the chemistry is deuterated.

**LPCVD (low-pressure CVD)** runs hot (700–800 °C for Si₃N₄ from dichlorosilane + NH₃) in a furnace, producing dense, stoichiometric, low-hydrogen films — the material of the ultra-low-loss Si₃N₄ platforms of Chapter 7. The constraints: the temperature restricts LPCVD to the front end of the process (before metal), and stoichiometric Si₃N₄ carries ~1 GPa of tensile stress, cracking when grown thicker than ~300–400 nm. The photonic Damascene process (Chapter 7) and multi-step deposition-with-anneal cycles exist precisely to sneak past this crack limit for the ~800 nm films that nonlinear photonics wants.

**ALD (atomic layer deposition)** grows films one self-limiting monolayer at a time — e.g., trimethylaluminum + H₂O yielding ~0.1 nm of Al₂O₃ per cycle. It is slow but delivers three unique properties: sub-nanometer thickness control, perfect conformality (uniform coating of trenches and gaps of any aspect ratio), and low temperature (150–300 °C). Photonic uses: gate dielectrics in MOSCAP modulators, conformal passivation, post-fabrication trimming of ring resonances by adding controlled cladding thickness, and high-quality coatings inside etched facets.

**Epitaxy** deserves mention as CVD's aristocratic cousin: single-crystal growth registered to the substrate lattice. Selective epitaxial growth of **germanium** in oxide windows (followed by cyclic annealing to sweep dislocations to the interface, tolerating the 4% Ge/Si lattice mismatch) is how every silicon photonics platform builds its photodetectors, and III-V epitaxy is the subject of Section 23.3.

## Physical Deposition: Metals

Metals arrive by **evaporation** (thermal or e-beam, line-of-sight, paired with lift-off patterning in research fabs) or **sputtering** (plasma-ejected atoms, better step coverage, standard in production), with **electroplating** for thick layers. A photonic BEOL (back end of line) typically includes: TiN or NiCr **resistive heaters** placed ~1–2 μm above waveguides (the thermo-optic phase shifters of Chapter 7), tungsten **vias**, one or two levels of aluminum or copper **interconnect**, and Al or Au **pads** for probing, wire bonding, or bump attach. Once the first metal is down, every subsequent step must stay below roughly 400–450 °C — the **thermal budget** that dictates the entire ordering of a process flow: oxidation and LPCVD first, then implant anneals, then germanium, then metals, then only low-temperature PECVD and ALD.

## Chemical-Mechanical Planarization (CMP)

CMP polishes the wafer flat with a rotating pad and a chemically active slurry, combining mechanical abrasion and chemistry. It appears wherever topography must be erased:

- After cladding deposition over etched waveguides, so the next lithography step sits within the scanner's ~200–300 nm depth of focus (Section 23.1.1);
- To planarize between **multiple waveguide layers** — e.g., SiN-on-SOI platforms that stack a nitride routing layer above the silicon layer, enabling low-loss waveguide crossings-by-avoidance in large photonic meshes;
- To prepare surfaces for **wafer bonding** (Section 23.3), which demands sub-0.5 nm RMS roughness;
- In the Damascene nitride process, where the waveguide itself is defined by polishing back an overfilled trench.

CMP is imperfect over varying pattern density (dishing over wide features, erosion over dense ones), which is why PDKs impose **density rules** and add dummy "tiling" fill shapes — those mysterious little squares covering every empty region of a fabricated photonic chip. Designers must keep tiling exclusion zones around sensitive devices (rings, gratings) since fill shapes perturb the local effective index and etch/CMP loading.

## Ion Implantation

Implantation fires ionized dopants (B, P, As) into the wafer at keV–MeV energies through a resist mask, with dose (ions/cm²) and energy setting concentration and depth. It is how the **p-n junction of a depletion modulator** is placed *inside* a rib waveguide with ~100 nm lateral precision: a sequence of masked implants builds p, n, p⁺, n⁺, and contact regions with the junction positioned at the optical-mode peak for maximal overlap (recall the Soref–Bennett physics of Chapter 7 — the doping profile *is* the modulator design). Implantation damages the crystal; a **rapid thermal anneal** (~1000 °C, seconds) repairs the lattice and activates dopants, which is why implants precede germanium growth and all metallization.

Implant doses trade modulation efficiency against optical loss: free carriers absorb (~1 dB/cm per ~10¹⁷ cm⁻³ of overlap-weighted doping, wavelength-dependent), so a modulator's $V_\pi L$ and its insertion loss are two ends of the same design lever.

## The Designer's Summary

Three numbers from this subsection follow you into every later chapter:

1. **Thermal budget** (~400 °C after metal) — explains what films your process can and cannot have, and why hydrogen-related absorption haunts back-end nitride.
2. **Film uniformity** (percent-level thickness control; ±few nm for partial etches and claddings) — enters the variability models of Section 23.2.3 on equal footing with lithographic CD control, and for thickness-sensitive devices (rings: ~2 nm resonance shift per nm of Si thickness) it dominates.
3. **Density rules and exclusion zones** — the visible fingerprints of CMP and etch loading on your layout, enforced by the DRC deck of Section 23.2.1.
