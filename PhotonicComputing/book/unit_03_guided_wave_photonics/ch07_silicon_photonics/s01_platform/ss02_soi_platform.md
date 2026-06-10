# 7.1.2 The SOI Platform: Substrate, Device Layer, and Waveguide Geometry

## Silicon-on-Insulator Substrate

The standard silicon photonic substrate is a **silicon-on-insulator (SOI) wafer**:
- Handle wafer: 725 μm thick Si (mechanical support)
- Buried oxide (BOX): 2–3 μm thick SiO₂ (lower cladding; provides optical isolation from handle)
- Device layer: 220 nm (or 300 nm) thick Si (the photonic layer)

The BOX must be thick enough that the optical mode of a Si waveguide does not leak into the handle wafer. For a 450 × 220 nm strip waveguide with $A_{eff} \approx 0.14$ μm², the evanescent tail into SiO₂ decays with $1/e$ length $1/\gamma \approx 100$ nm. A 2 μm BOX attenuates the field at the BOX/handle interface by $e^{-2000/100} = e^{-20} \approx 2\times10^{-9}$ — effectively zero leakage.

## Why 220 nm Device Layer?

The 220 nm device layer thickness is not arbitrary — it is the result of engineering optimization. The key tradeoff:

- **Thinner device layer** (< 200 nm): Weaker vertical confinement → larger mode → lower loss from waveguide sidewall roughness; but also weaker horizontal confinement → must reduce width → smaller mode → actually higher sensitivity to width variations.

- **Thicker device layer** (> 250 nm): Supports higher-order vertical modes beyond cutoff point, complicating passive device design; also limits modulator bandwidth (thicker p-i-n junctions have larger capacitance per unit area).

**220 nm is a sweet spot**: At 450 nm width × 220 nm height, the TE₀ mode has $n_{eff} \approx 2.4$, the next mode (TE₁) is below cutoff, the mode area is $\approx 0.14$ μm², and the TM₀ mode has significantly different effective index (allowing efficient polarization splitting). The 300 nm SOI platform (used by some foundries) gives more manufacturing margin but supports more polarization-sensitive effects.

## Waveguide Geometries

**Strip (wire) waveguide**: Rectangular silicon core with SiO₂ cladding on all sides. Standard cross-section: 450 nm × 220 nm. High confinement → compact bends → dense integration. But high loss (~1–3 dB/cm from sidewall roughness from the lithographic edge).

**Rib (ridge) waveguide**: Silicon core with slab silicon "wings" on either side. Cross-section: 500 nm wide × 220 nm tall rib on 90 nm slab. The slab reduces the effective index contrast horizontally, relaxing sidewall roughness sensitivity. Lower loss (~0.3–1 dB/cm) but larger minimum bend radius (~50 μm). Used in the passive interconnects of low-loss photonic circuits; difficult to dope efficiently for active devices.

**Slot waveguide**: Two Si rails separated by a narrow (~100 nm) gap filled with a lower-index material. The electric field is strongly enhanced in the slot (discontinuity of the normal E-field component at the Si/slot interface). Used for enhanced light-matter interaction with gas or liquid analytes, and for electro-optic modulation with nonlinear polymers.

**Suspended waveguide**: Si membrane waveguide with air cladding on all sides. Very high confinement, zero sidewall oxide losses, but mechanically fragile and incompatible with most CMOS processes.

## Top Cladding Options

The top cladding of silicon photonic waveguides significantly affects performance:
- **Air top cladding**: Lower index → stronger confinement, higher $\Delta n$ asymmetry → polarization sensitivity; enables thermo-optic tuning (air above, SiO₂ below → strong thermal gradient)
- **SiO₂ top cladding** (standard): Symmetric cladding → polarization-independent propagation; protects waveguides from environmental exposure; required for chemical passivation of Si surfaces
- **Si₃N₄ top cladding**: Used in some processes for stress engineering or to adjust the effective index

Most silicon photonic PDKs use SiO₂ top cladding as the default for passive waveguides and air or SiO₂ for active (p-n junction) regions.
