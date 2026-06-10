# Further Reading and References: Chapter 5 — Photodetectors

---

## Tier 1 — Essential Textbooks

**Saleh, B.E.A., & Teich, M.C. (2019). *Fundamentals of Photonics*, 3rd ed. Wiley.**
Chapter 18 (Photodetection) provides a clear and complete treatment of all detector types covered here, with consistent notation. The receiver SNR analysis and the coherent vs. direct detection comparison in Sections 18.4–18.5 are particularly useful. Chapter 21 covers optical communication systems and receiver sensitivity in the context of digital transmission.

**Keiser, G. (2021). *Optical Fiber Communications*, 5th ed. McGraw-Hill.**
Chapters 6 (Photodetectors) and 7 (Optical Receiver Operation) give a practical, engineering-focused treatment. The SNR analysis and sensitivity calculations are presented with enough detail to apply directly to photonic computing output stage design. Keiser is consistently clear, well-organized, and oriented toward design rather than just physics.

**Cova, S., Ghioni, M., Lotito, A., Rech, I., & Zappa, F. (2004). "Evolution and prospects for single-photon avalanche diodes and quenching circuits." *Journal of Modern Optics*, 51(9-10), 1267–1288.**
A comprehensive review of SPAD technology: physics, fabrication, quenching circuits, performance metrics, and applications. The standard reference for anyone designing SPAD-based systems.

---

## Tier 2 — Highly Recommended

**Stillman, G.E., & Wolfe, C.M. (1977). "Avalanche photodiodes." In *Semiconductors and Semimetals*, Vol. 12. Academic Press.**
The foundational treatment of APD physics: impact ionization, the Tager-McIntyre theory of excess noise, and the gain-bandwidth product. Classic reference for APD design.

**Agrawal, G.P. (2021). *Fiber-Optic Communication Systems*, 6th ed. Academic Press.**
Chapters 4–5 treat photodetectors and optical receivers with attention to noise analysis and sensitivity. The treatment is more application-focused than Saleh & Teich, which makes it more immediately useful for system-level design.

**Hadfield, R.H. (2009). "Single-photon detectors for optical quantum information applications." *Nature Photonics*, 3(12), 696–705.**
An accessible review of single-photon detector technologies (APD, SPAD, SNSPD, TES) comparing performance metrics relevant to quantum information processing. This is the best single review article for understanding the detector landscape for quantum photonic computing.

---

## Tier 3 — Primary Literature: Foundational Papers

**Einstein, A. (1905). "Über einen die Erzeugung und Verwandlung des Lichtes betreffenden heuristischen Gesichtspunkt." *Annalen der Physik*, 322(6), 132–148.**
The photoelectric effect paper. Read it for the clean argument: light comes in quanta, and a quantum below the threshold frequency cannot eject an electron regardless of intensity. This paper contains the conceptual foundation of all photon detection.

**Johnson, J.B. (1928). "Thermal agitation of electricity in conductors." *Physical Review*, 32(1), 97–109.**
**Nyquist, H. (1928). "Thermal agitation of electric charge in conductors." *Physical Review*, 32(1), 110–113.**
The measurement and theory of thermal noise. Johnson and Nyquist were colleagues at Bell Labs; their simultaneous publications are a model of the experimental-theoretical partnership. The Nyquist theorem in these papers is the direct ancestor of the fluctuation-dissipation theorem in modern statistical mechanics.

**Schottky, W. (1918). "Über spontane Stromschwankungen in verschiedenen Elektrizitätsleitern." *Annalen der Physik*, 362(23), 541–567.**
The original shot noise paper. Schottky derived the formula $\langle i^2\rangle = 2eI\Delta f$ from the Poisson statistics of electron emission. Reading the original gives context for how fundamental this result is.

---

## Tier 4 — Primary Literature: Photonic Computing Specific

**Yin, T., Cohen, R., Morse, M.M., Sarid, G., Chetrit, Y., Rubin, D., & Paniccia, M.J. (2007). "31 GHz Ge n-i-p waveguide photodetectors on silicon-on-insulator substrate." *Optics Express*, 15(21), 13965–13971.**
One of the key early papers demonstrating high-bandwidth Ge-on-Si waveguide photodetectors compatible with silicon photonic platforms.

**Goltsman, G.N., Okunev, O., Chulkova, G., Lipatov, A., Semenov, A., Smirnov, K., Voronov, B., Dzardanov, A., Williams, C., & Sobolewski, R. (2001). "Picosecond superconducting single-photon optical detector." *Applied Physics Letters*, 79(6), 705–707.**
The first SNSPD paper. Two pages, remarkable performance for its time, and the foundation of a technology that now achieves >98% efficiency.

**Reddy, D.V., Nerem, R.R., Nam, S.W., Mirin, R.P., & Verma, V.B. (2020). "Superconducting nanowire single-photon detectors with 98% system detection efficiency at 1550 nm." *Optica*, 7(12), 1649–1653.**
The current record holder for SNSPD system detection efficiency. The cavity-integrated WSi nanowire design that achieved 98% SDE at 1550 nm.

**Bandyopadhyay, S., et al. (2022). "Single chip photonic deep neural network with accelerated training." *arXiv:2208.01623*.**
Reports the SNR and precision (effective number of bits) measured in an experimentally demonstrated silicon photonic MZI matrix processor. Directly quantifies the noise sources discussed in this chapter in a real photonic computing system.

**Miscuglio, M., & Sorger, V.J. (2020). "Photonic tensor cores for machine learning." *Applied Physics Reviews*, 7(3), 031404.**
[DOI: 10.1063/5.0001942]
Section 4 of this review discusses the precision limitations of analog photonic computing from the perspective of all noise sources (shot, thermal, RIN, modulator) in an integrated context. The treatment directly connects detector noise to achievable ENOB in photonic accelerators.
