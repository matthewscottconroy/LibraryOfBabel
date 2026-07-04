# Chapter 24: Further Reading and References

---

## Essential Textbooks

**Taflove, A. & Hagness, S.C. (2005). *Computational Electrodynamics: The Finite-Difference Time-Domain Method*, 3rd ed. Artech House.**
The definitive treatment of FDTD, from the Yee algorithm and stability analysis through PML boundaries, dispersive materials, and near-to-far-field transforms. If you use FDTD seriously, this is the reference on the shelf.

**Jin, J.-M. (2014). *The Finite Element Method in Electromagnetics*, 3rd ed. Wiley–IEEE Press.**
The standard graduate text on FEM for electromagnetics: weak formulations, edge elements and the elimination of spurious modes, and eigenvalue (mode) problems. The theoretical backing for every waveguide mode solver.

**Chrostowski, L. & Hochberg, M. (2015). *Silicon Photonics Design: From Devices to Systems*. Cambridge University Press.**
Spans this chapter and the last: compact-model extraction, circuit simulation, design-for-test, and the practical layout-to-tape-out flow, with worked silicon-photonics examples. The single most useful book for the unit.

**Okamoto, K. (2006). *Fundamentals of Optical Waveguides*, 2nd ed. Academic Press.**
Rigorous treatment of waveguide modes, coupled-mode theory, the beam propagation method, and arrayed-waveguide gratings — the analytic foundations behind EME and BPM.

---

## Foundational Method Papers

**Yee, K.S. (1966). "Numerical solution of initial boundary value problems involving Maxwell's equations in isotropic media." *IEEE Transactions on Antennas and Propagation*, 14(3), 302–307.**
Three pages that created FDTD: the staggered electric/magnetic grid that bears Yee's name and the leapfrog time march still used, essentially unchanged, in every FDTD code today.

**Berenger, J.-P. (1994). "A perfectly matched layer for the absorption of electromagnetic waves." *Journal of Computational Physics*, 114(2), 185–200.**
The PML — the artificial absorber that lets a finite grid mimic open space with reflections below $-60$ dB, without which FDTD of radiating structures would be impractical.

**Oskooi, A.F., Roundy, D., Ibanescu, M., Bermel, P., Joannopoulos, J.D. & Johnson, S.G. (2010). "Meep: A flexible free-software package for electromagnetic simulations by the FDTD method." *Computer Physics Communications*, 181(3), 687–702.**
The paper for the open-source FDTD engine used in this chapter's exercises, notable for its subpixel-smoothing scheme that recovers accuracy at curved boundaries.

**Laporte, F., Dambre, J. & Bienstman, P. (2019). "Highly parallel simulation and optimization of photonic circuits in time and frequency domain based on the deep-learning framework PyTorch." *Scientific Reports*, 9, 5918.**
Introduces **photontorch**: a differentiable photonic circuit simulator that recasts a circuit as a sparse recurrent network, enabling gradient-based training of photonic systems — the basis of Section 24.2.2's discussion of differentiable circuit simulation.

---

## Reviews: Design Automation and Inverse Design

**Bogaerts, W. & Chrostowski, L. (2018). "Silicon photonics circuit design: methods, tools and challenges." *Laser & Photonics Reviews*, 12(4), 1700237.**
The map of the photonic design-automation stack: compact models, circuit simulation, variability analysis, and layout. Bridges Chapters 23 and 24.

**Molesky, S., Lin, Z., Piggott, A.Y., Jin, W., Vučković, J. & Rodriguez, A.W. (2018). "Inverse design in nanophotonics." *Nature Photonics*, 12(11), 659–670.**
The review that frames Section 24.3: adjoint sensitivity, topology optimization, fabrication constraints, and the outlook for computer-discovered photonic devices. Start here for inverse design.

**Jiang, J., Chen, M. & Fan, J.A. (2021). "Deep neural networks for the evaluation and design of photonic devices." *Nature Reviews Materials*, 6(8), 679–700.**
The authoritative survey of machine learning in photonic design: forward surrogates, inverse and generative networks, and where each helps or fails.

**So, S., Badloe, T., Noh, J., Bravo-Abad, J. & Rho, J. (2020). "Deep learning enabled inverse design in nanophotonics." *Nanophotonics*, 9(5), 1041–1057.**
A complementary, application-oriented review of deep-learning inverse design, strong on metasurfaces and generative models.

---

## Primary Literature: Adjoint and Topology Optimization

**Lalau-Keraly, C.M., Bhargava, S., Miller, O.D. & Yablonovitch, E. (2013). "Adjoint shape optimization applied to electromagnetic design." *Optics Express*, 21(18), 21693–21701.**
An influential and accessible statement of the adjoint method for photonics, with the two-simulation gradient made concrete.

**Piggott, A.Y., Lu, J., Lagoudakis, K.G., Petykiewicz, J., Babinec, T.M. & Vučković, J. (2015). "Inverse design and demonstration of a compact and broadband on-chip wavelength demultiplexer." *Nature Photonics*, 9(6), 374–377.**
The landmark demonstration: a fabricated $2.8\times2.8\ \mu\text{m}$ topology-optimized demultiplexer separating 1300 and 1550 nm — the device that convinced the field inverse design was real.

**Hughes, T.W., Minkov, M., Williamson, I.A.D. & Fan, S. (2018). "Adjoint method and inverse design for nonlinear nanophotonic devices." *ACS Photonics*, 5(12), 4781–4787.**
Extends the adjoint method to nonlinear devices and, with its companion work, underpins the open-source differentiable simulator **ceviche**.

**Su, L., Vercruysse, D., Skarda, J., Sapra, N.V., Petykiewicz, J.A. & Vučković, J. (2020). "Nanophotonic inverse design with SPINS: Software architecture and practical considerations." *Applied Physics Reviews*, 7(1), 011407.**
The practitioner's guide to fabrication-aware topology optimization — filtering, projection, minimum-feature and robustness constraints — that makes inverse-designed devices actually manufacturable.

---

## Primary Literature: Deep Learning for Photonics

**Peurifoy, J., Shen, Y., Jing, L., Yang, Y., Cano-Renteria, F., DeLacy, B.G., Joannopoulos, J.D., Tegmark, M. & Soljačić, M. (2018). "Nanophotonic particle simulation and inverse design using artificial neural networks." *Science Advances*, 4(6), eaar4206.**
An early, clean demonstration of a neural-network forward surrogate and its inversion for design.

**Liu, D., Tan, Y., Khoram, E. & Yu, Z. (2018). "Training deep neural networks for the inverse design of nanophotonic structures." *ACS Photonics*, 5(4), 1365–1369.**
Introduces the **tandem network** that resolves the non-uniqueness trap of naïve inverse networks — the architecture discussed in Section 24.3.3.

---

## Software and Documentation

**Meep** (meep.readthedocs.io) — Open-source FDTD and mode solving with a Python API, including the `meep.adjoint` inverse-design module and `harminv` resonance extraction.

**Tidy3D** (docs.flexcompute.com) — Cloud-GPU FDTD from Flexcompute, with a built-in adjoint plugin; changes what problem sizes count as routine.

**Ansys Lumerical Knowledge Base** (optics.ansys.com) — Documentation and application examples for FDTD, MODE (FDE and EME), and INTERCONNECT, the commercial standard flow.

**gdsfactory** (gdsfactory.github.io) — Open-source parametric layout in Python, with netlist extraction and links to circuit simulators (SAX) and DRC.

**KLayout** (klayout.de) — The open-source mask editor and DRC/LVS engine used for photonic layout verification and tape-out.

**SAX** (flaport.github.io/sax) — A JAX-based differentiable S-parameter circuit simulator integrated with the gdsfactory ecosystem.
