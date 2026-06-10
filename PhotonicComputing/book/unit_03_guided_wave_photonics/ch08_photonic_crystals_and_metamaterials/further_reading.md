# Chapter 8: Further Reading and References

---

## Essential Textbooks

**Joannopoulos, J.D., Johnson, S.G., Winn, J.N., & Meade, R.D. (2008). *Photonic Crystals: Molding the Flow of Light*, 2nd ed. Princeton University Press.**
The standard textbook for photonic crystals. Available free online at ab-initio.mit.edu/book. Covers the master equation, Bloch theorem, band structures, defect modes, and device applications with exceptional rigor and clarity. Chapters 2–4 are the mathematical foundation; Chapters 8–9 cover 2D slabs.

**Maier, S.A. (2007). *Plasmonics: Fundamentals and Applications*. Springer.**
The standard graduate reference for plasmonics. Derives SPP dispersion from first principles, covers localized surface plasmons, waveguiding, and applications. Honest about the loss limitations. Chapter 2 covers SPP dispersion; Chapter 7 covers applications.

**Born, M. & Wolf, E. (1999). *Principles of Optics*, 7th ed. Cambridge University Press.**
Chapter 1 covers transfer matrices for multilayer systems; the classical treatment remains the most rigorous.

---

## Highly Recommended

**Yu, N. & Capasso, F. (2014). "Flat optics with designer metasurfaces." *Nature Materials*, 13(2), 139–150.**
The review article that defined metasurface science as a mature discipline. Covers mechanisms (resonant, propagation, geometric phase), applications (lenses, beam steering, holograms), and design principles. Essential reading for anyone working with metasurfaces.

**Baba, T. (2008). "Slow light in photonic crystals." *Nature Photonics*, 2(8), 465–473.**
The best concise review of slow light: physics, mechanisms, bandwidth-loss tradeoffs, and device applications. 10 pages that save 100 hours of literature searching.

---

## Primary Literature: Photonic Crystals

**Yablonovitch, E. (1987). "Inhibited spontaneous emission in solid-state physics and electronics." *Physical Review Letters*, 58(20), 2059–2062.**
The founding paper. Derives the photonic bandgap concept from the spontaneous emission suppression perspective.

**John, S. (1987). "Strong localization of photons in certain disordered dielectric superlattices." *Physical Review Letters*, 58(23), 2486–2489.**
The companion founding paper from the localization perspective; introduced the connection between photonic crystals and condensed matter physics.

**Noda, S., Tomoda, K., Yamamoto, N., & Chutinan, A. (2000). "Full three-dimensional photonic bandgap crystals at near-infrared wavelengths." *Science*, 289(5479), 604–606.**
The first demonstration of a 3D photonic bandgap structure at near-infrared wavelengths; establishes the engineering feasibility of 3D photonic crystals.

**Asano, T., Song, B.-S., & Noda, S. (2006). "Analysis of the experimental Q factors (~1 million) of photonic crystal nanocavities." *Optics Express*, 14(5), 1996–2002.**
The $Q > 10^6$ photonic crystal cavity demonstration; the benchmark for highest-Q chip-scale resonators.

---

## Primary Literature: Metasurfaces

**Yu, N., Genevet, P., Kats, M.A., Aieta, F., Tetienne, J.-P., Capasso, F., & Gaburro, Z. (2011). "Light propagation with phase discontinuities: Generalized laws of reflection and refraction." *Science*, 334(6054), 333–337.**
The paper that launched modern metasurface science. Introduces the generalized Snell's law and demonstrates it with V-shaped gold antennas.

**Khorasaninejad, M., Chen, W.T., Devlin, R.C., Oh, J., Zhu, A.Y., & Capasso, F. (2016). "Metalenses at visible wavelengths: Diffraction-limited focusing and subwavelength resolution imaging." *Science*, 352(6290), 1190–1194.**
TiO₂ metalens achieving >80% focusing efficiency at 532 nm; demonstrates that metasurfaces can match conventional lenses in performance.

**Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008.**
The D²NN paper. Essential reading; note the physics carefully: the "deep" network here is of limited representational power without added nonlinearity.

**Hughes, T., Williamson, I.A.D., Minkov, M., & Fan, S. (2019). "Wave physics as an analog recurrent neural network." *Science Advances*, 5(12), eaay6946.**
A careful theoretical analysis showing that linear optical networks (including D²NNs) lack the expressive power of nonlinear networks. Provides the formalism for understanding what optical linear systems can and cannot compute.

---

## Primary Literature: Plasmonics

**Ritchie, R.H. (1957). "Plasma losses by fast electrons in thin films." *Physical Review*, 106(5), 874–881.**
The original prediction of surface plasmon polaritons. A beautiful theoretical paper that is short enough to read in one sitting.

**Ebbesen, T.W., Lezec, H.J., Ghaemi, H.F., Thio, T., & Wolff, P.A. (1998). "Extraordinary optical transmission through sub-wavelength hole arrays." *Nature*, 391(6668), 667–669.**
The extraordinary optical transmission paper that launched the modern plasmonic era.

**Barnes, W.L., Dereux, A., & Ebbesen, T.W. (2003). "Surface plasmon subwavelength optics." *Nature*, 424(6950), 824–830.**
A comprehensive review of SPP physics and early applications; accessible and historically important.

**Haffner, C., Heni, W., Fedoryshyn, Y., Niegemann, J., Melikyan, A., Elder, D.L., ... & Leuthold, J. (2015). "All-plasmonic Mach–Zehnder modulator enabling optical high-speed communication at the microscale." *Nature Photonics*, 9(8), 525–528.**
The OEO plasmonic modulator at ETH Zürich; the most compelling engineering case for applying plasmonics to photonic computing.

---

## Software Tools

**MPB (MIT Photonic Bands)**: Band structure solver for photonic crystals. Open-source at mpb.readthedocs.io. The standard tool for photonic crystal design.

**MEEP**: FDTD simulation for Maxwell's equations on complex geometries. Open-source at meep.readthedocs.io. Handles photonic crystals, metasurfaces, and plasmonic structures.

**Lumerical FDTD/RCWA**: Commercial tools widely used in industry for metasurface and plasmonic device simulation.
