# 23.4.1 Fiber-Chip Coupling: Grating Couplers and Edge Couplers

## The Mode-Mismatch Problem

Standard single-mode fiber (SMF-28) carries a nearly Gaussian mode of **10.4 μm mode-field diameter** at 1550 nm. A silicon strip waveguide confines a mode of roughly **0.5 × 0.3 μm**. Butt them together and the coupling efficiency is the squared overlap integral of the two fields,

$$\eta = \frac{\left|\int E_1^* E_2\, dA\right|^2}{\int |E_1|^2 dA \int |E_2|^2 dA}$$

which for a ~20× linear size mismatch evaluates to roughly $\eta \sim (2 w_1 w_2/(w_1^2+w_2^2))^2 \approx 1\%$ per axis — a catastrophic ~20 dB loss, before Fresnel reflection. Every fiber-chip coupling scheme is a mode-size transformer, and there are two great families.

## Grating Couplers

A **grating coupler** is a periodically corrugated (partially etched) region of waveguide that diffracts the guided mode upward into a fiber held near-vertically above the chip surface. The design equation is phase matching along the surface — the grating equation:

$$n_{eff} - n_{c}\sin\theta = \frac{m\lambda}{\Lambda}$$

with $n_{eff}$ the grating region's average effective index, $n_c$ the cladding/superstrate index, $\theta$ the fiber angle from vertical, $m$ the diffraction order (1 in practice), and $\Lambda$ the period.

**Worked example.** Target 1550 nm at $\theta = 10°$ in air ($n_c = 1$), with a shallow-etched grating of $n_{eff} = 2.85$:

$$\Lambda = \frac{\lambda}{n_{eff} - \sin\theta} = \frac{1550\ \text{nm}}{2.85 - 0.174} = 579\ \text{nm}$$

The small tilt (8–12° is customary) breaks the degeneracy between upward diffraction and second-order back-reflection into the waveguide, which would otherwise sit exactly at the design wavelength.

Characteristic performance of a standard foundry 1D grating coupler: **insertion loss −2 to −4 dB**, **1-dB bandwidth ~30–40 nm**, strongly **polarization-selective** (TE gratings reject TM). The loss has two understandable parts: directionality (power diffracted down toward the substrate rather than up — improved by optimizing BOX thickness for constructive interference, or adding a bottom mirror) and mode-profile mismatch (a uniform grating radiates an exponential profile, not a Gaussian — fixed by *apodizing* the grating strength along its length). Engineered designs with apodization and substrate mirrors reach below −1 dB in the literature; 2D gratings (two superimposed orthogonal gratings) split arbitrary fiber polarization into two TE on-chip paths, solving polarization diversity at the cost of ~1 dB extra loss. See Marchetti et al. [*Photonics Research*, 2019] for a thorough survey of the design space.

Grating couplers' operational superpower is **wafer-scale test**: they work anywhere on the die surface, before dicing, with a fiber array simply lowered onto the wafer — the enabler of the automated measurement flows in Section 23.4.2. Their liabilities: bandwidth (painful for broadband WDM systems and O+C-band devices), polarization dependence, and a spectral response that shifts ~1 nm per nm of etch-depth or period error (Section 23.2.3).

## Edge Couplers

An **edge coupler** (inverse taper) attacks the mismatch from the side: the silicon waveguide narrows — counterintuitively — to a fine tip (~100–200 nm) as it approaches the chip facet. As the core shrinks below cutoff-scale dimensions, the mode *deconfines*, ballooning into a large, nearly circular field guided weakly by the tip and by an overlying low-index waveguide (SiON, SU-8, or an oxide "cane"), sized to match either a **lensed fiber** (2.5–5 μm spot) or, with multi-stage spot-size converters, standard SMF.

Performance: **0.5–2 dB** loss routinely, records well below 0.5 dB; **bandwidth of hundreds of nanometers** (the taper is adiabatic, not resonant); **low polarization dependence** with symmetric designs. The costs: real estate at the die perimeter only; a **facet** must be created (deep-etched trench in-line, or dicing/polishing) with optical quality and controlled distance to the tip; anti-reflection considerations at the facet; and tighter alignment tolerance.

## Alignment, Tolerances, and Assembly Cost

The 1-dB misalignment tolerance is approximately the mode-field radius scaled by $\sqrt{\ln(10^{0.1})}$ ≈ 0.48 — as a Gaussian overlap $\eta(\delta) = e^{-\delta^2/w^2}$ falls by 1 dB when $\delta \approx 0.48\,w$:

| Interface | Mode size (1/e² radius) | ~1-dB lateral tolerance |
|---|---|---|
| Grating coupler ↔ SMF | ~5.2 μm | ±2.5 μm |
| Edge coupler ↔ lensed fiber (3 μm spot) | ~1.5 μm | ±0.7 μm |
| Edge coupler ↔ SMF via SSC | ~5 μm | ±2.4 μm |

Sub-micron tolerances demand **active alignment** — piezo stages maximizing transmitted power while UV-cured epoxy sets — which is slow, serial, and the single largest reason photonic packaging is expensive. The industry's countermeasures: **V-groove arrays** (KOH-etched, Section 23.1.2) that passively seat fiber ribbons at lithographic accuracy; **fiber arrays** (127/250 μm pitch glass blocks) attached in one active-alignment step for N fibers rather than N steps; self-aligning mechanical stops; and **photonic wire bonding** — free-form polymer waveguides written *in situ* by two-photon polymerization between fiber and chip after loose placement, converting mechanical alignment error into a photonically corrected path [Lindenmann et al., *Optics Express*, 2012].

## A Link Budget, Because Everything Above Is a Line Item

Consider a photonic accelerator die fed by an external 50 mW (+17 dBm) laser, computing with a 6-stage MZI mesh, read out by on-chip detectors:

| Element | Loss |
|---|---|
| Laser-to-fiber + isolator + fiber routing | 1.5 dB |
| Fiber → chip (edge coupler) | 1.5 dB |
| On-chip distribution: 2 cm at 2 dB/cm | 4.0 dB |
| Mesh: 6 MZI stages × 0.3 dB | 1.8 dB |
| Excess splitter/crossing losses | 1.0 dB |
| **Total to detector** | **9.8 dB** |

Of +17 dBm launched, +7.2 dBm (~5 mW) reaches the detector plane — before the mesh's *intentional* signal-dependent attenuation, and divided among output channels. Fan the mesh out to 64 outputs (−18 dB) and each detector sees ~80 μW: fine for a p-i-n receiver at GHz rates, but the shot-noise-limited bit depth calculations of Chapter 9 now bind. The two largest controllable entries are coupling and propagation loss — which is why this chapter's fabrication minutiae (roughness, facet quality, coupler apodization) appear on equal footing with architecture in any honest photonic computing power budget.
