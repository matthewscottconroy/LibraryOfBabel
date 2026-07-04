# 23.2.1 The Standard SOI Process Flow and the PDK

## The Flow, End to End

A full active silicon photonics process — the kind offered by IMEC, AIM Photonics, AMF, GlobalFoundries, or Tower — runs on 200 mm or 300 mm SOI wafers (220 nm device layer, 2–3 μm BOX) and executes, in order:

1. **Waveguide lithography and etch.** DUV 193 nm lithography (Section 23.1.1) patterns a hard mask; ICP-RIE (Section 23.1.2) performs the full 220 nm etch for strip waveguides. Separate mask levels perform partial etches (~70 nm for grating couplers; ~160 nm rib etch leaving a ~60 nm slab for modulator contacts).
2. **Implants and activation.** A sequence of masked implants (p, n, p⁺, n⁺, plus intermediate levels) builds modulator junctions and heater-adjacent contacts; a rapid thermal anneal activates them.
3. **Germanium epitaxy.** Selective epitaxial Ge growth in oxide windows forms photodetectors; further implants and anneal complete the p-i-n diode.
4. **Contacts and metallization.** Tungsten vias, one or two aluminum/copper metal levels, TiN heater layer, and bond/probe pads.
5. **Cladding, planarization, passivation.** PECVD SiO₂ cladding with CMP between levels; final passivation and pad opening.
6. **Optional back-end modules**: deep trenches for edge-coupler facets, substrate undercuts for thermal isolation, SiN secondary waveguide layer, bumps for flip-chip.

The output is a wafer whose every photonic device — waveguide, ring, modulator, detector, heater — was built from the *same shared layers*, which is why a change requested for one device (say, a deeper partial etch to improve grating couplers) ripples through every other device and is essentially never granted. The process is the constitution; devices are legislation.

**Passive-only processes** (waveguides + cladding, no implants/Ge/metal) are cheaper and faster, and are the standard vehicle for research on meshes, filters, and inverse-designed components where modulation and detection happen off-chip.

## The PDK: A Contract, Not a Suggestion

The **process design kit** is the foundry's formal interface to designers. A silicon photonic PDK contains:

- **Layer definitions** mapping GDSII layer/datatype numbers to physical steps (waveguide etch, partial etch, implants, metals, keep-out and tiling-control layers).
- **Design rules (DRC deck)**: minimum widths and spacings per layer (e.g., ~150–200 nm minimum feature and gap on the waveguide level for 193 nm lithography), enclosure rules (metal over via, implant over waveguide), density bounds from CMP and etch loading (Section 23.1.3), and antenna rules protecting gate dielectrics during plasma steps.
- **Parameterized cells (pcells) / fixed cells**: foundry-validated layouts — grating couplers, edge couplers, MMIs, phase shifters, modulators, detectors — with *measured* performance distributions. The deepest sentence in any PDK: **only components in the PDK are guaranteed to work.** Custom devices are permitted but are your risk entirely.
- **Compact models** for circuit simulation (Chapter 24): S-parameters for passives, electro-optic models for actives, often with statistical corners.
- **Verification decks**: DRC scripts (run in KLayout, Calibre, or the foundry's tool of choice) and, in mature flows, LVS (layout-versus-schematic) extraction so that the *netlist implied by your polygons* can be checked against the schematic you simulated — a discipline imported from electronics that catches the classic photonic disaster: a waveguide that looks connected but isn't, or an unintended 2 nm gap at a cell boundary.

## GDSII: The File Format of Everything

The design you submit is a **GDSII** file (or its modern compressed successor, **OASIS**): a hierarchy of cells containing polygons and cell references, with integer vertex coordinates on a database grid (1 nm is standard). Three photonic-specific consequences:

1. **Everything is polygons.** GDSII has no native circles or curves; your ring resonator is a many-vertex polygon, and the vertex discretization must be fine enough (sub-nm chord error) not to add roughness of its own. Layout tools handle this, but exported/re-imported geometry can silently coarsen it.
2. **The 1 nm grid quantizes geometry.** A directional coupler whose optimal gap is 197.4 nm will be drawn at 197 nm; snapping errors at angled interfaces can create sliver polygons that violate DRC.
3. **Hierarchy is power.** A 64×64 MZI mesh is one MZI cell referenced 2016 times, not 2016 copies — which is what makes layout, DRC, and mask writing tractable.

## The Design Flow

The full loop a photonic computing chip traverses, tying this chapter to the next:

1. **System partition**: decide what is optical (the matrix multiply, the routing) and what is electronic (drivers, ADCs, control).
2. **Schematic and circuit simulation**: compose PDK compact models in a photonic circuit simulator (Section 24.2) and verify the transfer function, loss budget, and tuning budget.
3. **Custom component design** where the PDK falls short: FDTD/FEM/EME simulation (Section 24.1), possibly inverse design (Section 24.3).
4. **Layout**: generate the GDSII — by hand in KLayout for small cells, programmatically in gdsfactory (or the foundry's supported flow: Luceda IPKISS, Synopsys OptoDesigner, Cadence/Lumerical flows) for anything with more than a handful of components. Automated waveguide routing with bend-radius and length-matching constraints plays the role that wire routing plays in electronics, with the added twist that *path length is phase* and often must be matched to sub-micron precision across an interferometer.
5. **Verification**: DRC clean; LVS or netlist extraction re-simulation ("simulate what you drew, not what you meant"); manual review of every non-PDK structure.
6. **Tape-out**: merge in the foundry's frame (alignment marks, PCM structures — Section 23.2.3), fill tiling, generate final OASIS/GDSII, upload, and sign the design-rule waiver forms for anything intentionally non-compliant.
7. **Post-fab test** (Section 23.4): wafer- or die-level measurement, model back-annotation, and the next iteration.

The fabless discipline this flow encodes — standardized processes, shared runs, portable designs — is what turned silicon photonics from a heroic laboratory art into an engineering field [Hochberg & Baehr-Jones, "Towards fabless silicon photonics," *Nature Photonics*, 2010], in conscious imitation of the fabless-electronics revolution that MOSIS began in the 1980s.
