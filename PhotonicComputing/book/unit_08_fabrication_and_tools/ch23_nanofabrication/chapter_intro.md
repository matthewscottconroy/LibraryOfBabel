# Chapter 23: Nanofabrication for Photonics

> *"You don't design the chip you want. You design the chip the process lets you have."*
>
> — Paraphrase of a common sentiment among foundry engineers

---

## From GDSII to Glass and Silicon

A photonic chip begins its life as a GDSII file: a hierarchy of polygons on numbered layers, drawn on a 1 nm grid. Between that file and a working chip lie several hundred process steps executed over weeks in a cleanroom: photoresist is spun and exposed, plasmas carve the silicon, furnaces grow and deposit films, implanters fire dopant ions into precisely masked regions, and polishing pads planarize each layer before the next is built on top. The designer never touches any of this — and that is precisely why the designer must understand it. Every design rule in a PDK, every tolerance in a compact model, and every calibration loop in a photonic computing system exists because of something that happens (or fails to happen) in the fab.

This chapter follows the wafer through the process, in four stages. First, the **unit operations** of nanofabrication: lithography, etching, and deposition, along with planarization and implantation. Second, how those operations are assembled into a **silicon photonics foundry process**, and how you buy access to it through a multi-project wafer run. Third, **III-V integration**: how the one thing silicon cannot do — emit light efficiently — is grafted onto it by bonding, transfer printing, or direct epitaxial growth. Fourth, **packaging and testing**: getting light and electricity into and out of the chip, which routinely consumes more engineering effort (and more of the product cost) than the chip itself.

---

## The Central Tension: Analog Circuits from a Statistical Process

Nanofabrication is a statistical process. Linewidths vary by nanometers across a wafer; layer thicknesses drift from run to run; a plasma etch is never perfectly uniform from chip center to chip edge. Digital electronics was engineered to be immune to this — a transistor with a 5% threshold variation still computes correct Boolean logic. Photonic computing enjoys no such immunity:

1. **Optical phase is exquisitely sensitive to geometry.** A silicon waveguide's effective index changes by roughly $10^{-3}$ per nanometer of width. Over a 100 μm interferometer arm, a 1 nm width error accumulates ~0.6 rad of phase error — enough to scramble an MZI mesh that expects phases set to $10^{-2}$ rad precision.

2. **Resonant devices amplify the problem.** A microring filter with a 100 pm linewidth shifts by roughly 1 nm per nanometer of width error and ~2 nm per nanometer of thickness error: fabrication scatter is one to two orders of magnitude larger than the device's own spectral feature.

3. **Loss is a fabrication property, not a material property.** Bulk crystalline silicon is nearly lossless at 1550 nm; the 1–3 dB/cm of a real waveguide comes almost entirely from lithography- and etch-induced sidewall roughness.

The consequence, developed quantitatively in Section 23.2.3, is a design doctrine: *no photonic computing architecture may assume as-drawn dimensions*. Every serious system budgets tuning power, calibration time, and control electronics to absorb fabrication scatter — and those budgets, not the idealized optics, frequently dominate system power and cost.

---

## Chapter Structure

**Section 23.1 — Cleanroom Fundamentals**: Contamination control and wafers; lithography (photolithography, DUV, EUV, electron-beam); etching (wet and dry, RIE/ICP); deposition (PECVD, LPCVD, ALD), chemical-mechanical planarization, and ion implantation.

**Section 23.2 — The Silicon Photonics Foundry Process**: The standard SOI process flow and the PDK contract; multi-project wafer runs, costs, and turnaround; process variation, yield models, and what they imply for photonic computing.

**Section 23.3 — III-V Integration**: Why lasers need III-V semiconductors; flip-chip bonding, die-to-wafer bonding, and micro-transfer printing; heterogeneous III-V-on-silicon platforms and monolithic epitaxial growth.

**Section 23.4 — Packaging and Testing**: Fiber-chip coupling (grating couplers and edge couplers); electrical packaging from wire bonds to 3D integration; wafer-scale optical testing, thermal management, and reliability.
