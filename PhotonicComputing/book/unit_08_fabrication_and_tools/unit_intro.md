# Unit VIII: Fabrication and Simulation Tools — From Design Files to Working Chips

> *"What I cannot create, I do not understand."*
>
> — Found on Richard Feynman's blackboard at the time of his death, 1988

---

## What This Unit Is About

Theory without fabrication is philosophy. The seven units behind you developed the physics of photonic computing: Maxwell's equations, lasers, waveguides, information theory, interferometric matrix processors, neuromorphic architectures, and quantum photonics. Every one of those chapters quietly assumed that someone can actually *make* the devices — that a 450 × 220 nm silicon waveguide can be patterned with nanometer fidelity, that a directional coupler will split 50/50 as designed, that a III-V laser can end up on a silicon chip at all. This unit is about how that happens, and about the computational tools that let you predict device behavior *before* committing a design to a foundry run that costs tens of thousands of dollars and takes half a year.

The two chapters form a matched pair:

- **Chapter 23 — Nanofabrication for Photonics** follows a photonic chip through the fab: lithography (deep-UV, extreme-UV, and electron-beam), etching, deposition, planarization, and implantation; the foundry process-design-kit (PDK) model and multi-project wafer (MPW) runs that make fabrication accessible to a graduate student; process variation and yield; heterogeneous integration of III-V lasers onto silicon; and the unglamorous but decisive problems of fiber-chip coupling, electrical packaging, and testing.

- **Chapter 24 — Simulation and Design Tools** builds the simulation stack: rigorous Maxwell solvers (FDTD, FEM), the specialized methods that exploit waveguide structure (eigenmode expansion, beam propagation), circuit-level simulators that scale to thousands of components, and the inverse-design revolution — adjoint methods, topology optimization, and deep learning — that lets the computer discover device geometries no human would draw. It closes the loop with layout tools (gdsfactory, KLayout) and the tape-out flow that turns a verified design into a GDSII file a foundry will accept.

---

## Why a Computing Text Needs a Fabrication Unit

Photonic computing is *analog* computing, and analog computing is uniquely exposed to fabrication reality. A digital CMOS gate either switches or it doesn't; a photonic matrix multiplier degrades *continuously* with every nanometer of linewidth error, every degree of temperature drift, every tenth of a decibel of excess loss. Three consequences follow, and they organize this unit:

1. **Fabrication statistics are computing specifications.** A ±5 nm waveguide-width variation — routine even in a state-of-the-art foundry — shifts a microring resonance by nanometers, which is tens of linewidths for a high-Q ring. The precision of a photonic neural network's weights is set not by the designer's intent but by the joint distribution of linewidth, thickness, and temperature across the chip. Chapter 23 teaches you to compute these sensitivities and to budget the tuning power that compensates for them.

2. **Access is through foundries, and foundries impose a contract.** Almost nobody who designs photonic computing systems owns a fab. You design within a PDK, tape out on an MPW run, wait 4–9 months, and get back a handful of dies. A mistake that a simulation would have caught costs a career-relevant amount of calendar time. This is why the simulation chapter is not optional enrichment; it is how the field manages risk.

3. **Scale forces a hierarchy of models.** A rigorous FDTD simulation of a single ring resonator can occupy a GPU for the better part of an hour; a photonic tensor core contains thousands of coupled components. No single tool spans that range. The working method of the field — simulate components rigorously once, extract compact S-parameter models, compose them in a circuit simulator, and co-simulate with electronics — is exactly the methodology electronics developed decades ago, reborn with optical phase as the signal variable.

---

## The Skills You Will Acquire

This is the most practical unit of the book. By the end you should be able to:

- Estimate the resolution, cost, and turnaround of DUV, EUV, and e-beam lithography for a given device, and read an etch cross-section critically.
- Read a foundry PDK, plan an MPW submission, and lay out a chip with test structures that make the returned dies measurable.
- Propagate a linewidth distribution through to a resonance-wavelength distribution, and from there to a thermal tuning power budget or an MZI phase-error budget.
- Choose the right solver — FDTD, FEM, EME, BPM, or a circuit model — for a given structure, and estimate its memory and run-time cost before launching it.
- Extract S-parameters from a field simulation and assemble them into a circuit-level model of a full photonic processor.
- Set up an adjoint-based inverse design and understand the fabrication constraints that keep an optimized geometry manufacturable.
- Produce a DRC-clean GDSII layout in gdsfactory/KLayout and carry it through a tape-out checklist.

The exercises lean on free tools — MEEP for FDTD, gdsfactory and KLayout for layout, Python for everything else — so that every workflow in this unit can be practiced without a commercial license. Where commercial tools (Lumerical, COMSOL, Synopsys/Ansys products) are the industry standard, we say so and explain what they add.

---

## How This Unit Connects Backward and Forward

Chapter 23 grounds the device physics of Units II–III: the plasma-dispersion modulator of Chapter 7 exists because ion implantation can place a p-n junction inside a waveguide; the hybrid III-V laser of Chapter 4 exists because wafer bonding can marry indium phosphide to silicon. Chapter 24 operationalizes the mathematics of Unit I — FDTD and FEM are, in the end, just Maxwell's equations discretized — and supplies the modeling machinery that Units V–VI assumed whenever they quoted the fidelity of an MZI mesh under phase noise. Unit IX will then ask the system-level question: given what fabrication and simulation can deliver, how does a photonic processor actually compare to a GPU? You cannot answer that honestly without the numbers this unit provides.

---

## References for the Unit Introduction

[1] Chrostowski, L., & Hochberg, M. (2015). *Silicon Photonics Design: From Devices to Systems*. Cambridge University Press. [The standard reference for exactly the design-fabricate-test loop this unit teaches.]

[2] Bogaerts, W., & Chrostowski, L. (2018). "Silicon photonics circuit design: methods, tools and challenges." *Laser & Photonics Reviews*, 12(4), 1700237. [Review of the photonic design-automation stack, from compact models to layout.]

[3] Oskooi, A.F., Roundy, D., Ibanescu, M., Bermel, P., Joannopoulos, J.D., & Johnson, S.G. (2010). "Meep: A flexible free-software package for electromagnetic simulations by the FDTD method." *Computer Physics Communications*, 181(3), 687–702. [The open-source FDTD engine used throughout the exercises.]

[4] Molesky, S., Lin, Z., Piggott, A.Y., Jin, W., Vučković, J., & Rodriguez, A.W. (2018). "Inverse design in nanophotonics." *Nature Photonics*, 12(11), 659–670. [The review that frames Chapter 24's treatment of adjoint methods and topology optimization.]
