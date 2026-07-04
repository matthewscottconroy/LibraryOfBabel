# Section 24.1: Numerical Electromagnetic Methods

At the bottom of the modeling hierarchy sit the rigorous solvers: methods that assume nothing about the field beyond Maxwell's equations and the geometry you supply. They are the ground truth against which every faster, more approximate model is calibrated — and they are expensive, which is exactly why the rest of the chapter exists to avoid overusing them.

The four methods in this section divide along two axes. The first is **time versus frequency domain**: FDTD marches the fields forward in time and recovers a broadband spectrum from a single pulsed run, while FEM solves one frequency at a time but handles resonances and eigenproblems with unmatched precision. The second is **general versus structured**: FDTD and FEM make no assumption about the shape of the device and will simulate anything, at a cost that grows with the enclosed volume; EME and BPM assume the structure is nearly invariant along a propagation axis and exploit that assumption to simulate devices — centimeter-long tapers, arrayed-waveguide gratings — that a volumetric solver could never afford.

Choosing among them is the practical skill this section teaches. A resonator's quality factor, a grating's back-reflection, a sharp bend's radiation loss: FDTD or FEM. A waveguide's mode profile and effective index: an FEM or finite-difference mode solver. A slowly varying, low-reflection, long structure: EME if reflections matter, BPM if they do not. Get this choice right and a simulation runs in minutes; get it wrong and it runs for a week, or silently returns nonsense.

- **24.1.1** — Finite-Difference Time-Domain (FDTD): the Yee grid, the Courant limit, and broadband spectra in one run
- **24.1.2** — The Finite Element Method (FEM): conforming meshes, weak forms, and mode solving
- **24.1.3** — Eigenmode Expansion and Beam Propagation: exploiting the propagation axis
