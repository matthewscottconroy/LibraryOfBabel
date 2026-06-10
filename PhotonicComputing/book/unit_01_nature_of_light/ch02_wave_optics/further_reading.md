# Further Reading and References — Chapter 2: Wave Optics

## Tier I: Essential References

**Goodman, J.W. (2005). *Introduction to Fourier Optics*, 3rd ed. Roberts & Company.**

The definitive textbook on Fourier optics and coherent optical processing. Goodman covers scalar diffraction theory (Chapters 3–4), the frequency analysis of optical imaging (Chapter 6), holography (Chapter 9), and coherent optical information processing (Chapter 8). His treatment of the 4f system, spatial filtering, and matched filtering is the standard reference for free-space optical computing. The mathematical treatment is thorough and rigorous; the physical intuition is excellent.

*How to use this for photonic computing*: Chapter 6 on Fourier transforms by lenses is essential. Chapter 8 on coherent optical processing is the foundation for understanding analog optical computing.

---

**Born, M. & Wolf, E. (1999). *Principles of Optics*, 7th ed. Cambridge University Press.**

The encyclopedic reference for classical optics. Chapter 7 (elements of the theory of diffraction), Chapter 10 (partially coherent light), and Chapter 12 (diffraction of light by ultrasonic waves, i.e., acousto-optic effects) are most relevant. Born and Wolf's treatment of coherence theory (Chapter 10) is rigorous and comprehensive, developing the mutual coherence function and van Cittert-Zernike theorem in full generality.

---

**Saleh, B.E.A. & Teich, M.C. (2019). *Fundamentals of Photonics*, 3rd ed. Wiley.**

The standard advanced undergraduate/graduate text on photonics. Chapter 2 (wave optics), Chapter 3 (beam optics — Gaussian beams and ABCD matrices), Chapter 4 (Fourier optics), and Chapter 6 (resonator optics) all cover the material of this chapter in detail at a level slightly below Born & Wolf but with more engineering applications. Chapter 9 on photon optics is the bridge to the quantum treatment.

---

**Siegman, A.E. (1986). *Lasers*. University Science Books.**

The authoritative reference on laser physics. Chapters 15–20 (Gaussian beam propagation, ABCD matrices, resonator modes) are the basis for Section 2.6. Siegman's treatment of ABCD matrices for Gaussian beams is definitive and detailed. The physical intuition is excellent. This book is the standard reference in laser engineering and in photonics broadly.

---

## Tier II: Highly Recommended

### On Interference and Resonators

**Yariv, A. & Yeh, P. (2006). *Photonics: Optical Electronics in Modern Communications*, 6th ed. Oxford University Press.**

Strong on guided-wave devices and resonators. Chapter 4 (optical resonance and Fabry-Pérot etalons) and Chapter 8 (coupled-mode theory) are relevant here. The treatment of ring resonators and their application to filtering and modulation is practical and clear.

---

**Haus, H.A. (1984). *Waves and Fields in Optoelectronics*. Prentice-Hall.**

A compact, rigorous text by one of the leading figures in photonics theory. Haus's treatment of coupled-mode theory is the foundation for understanding mode coupling in directional couplers, ring resonators, and distributed feedback structures. Essential for understanding the physical basis of the beam splitter matrix and its $\pi/2$ phase relationship.

---

### On Coherence Theory

**Mandel, L. & Wolf, E. (1995). *Optical Coherence and Quantum Optics*. Cambridge University Press.**

The comprehensive reference on classical and quantum coherence. Part I (chapters 1–7) develops classical coherence theory — correlation functions, spectral representations, the van Cittert-Zernike theorem, intensity interferometry. Part II begins the quantum treatment. At 1200 pages, this is a research-level reference, but the introductions to each chapter are accessible and informative.

---

### On Polarization

**Collett, E. (1993). *Polarized Light: Fundamentals and Applications*. Marcel Dekker.**

A practical reference on polarization, covering Jones calculus, Stokes parameters, Mueller matrices, and their measurement. More applied than theoretical; good for understanding how polarization measurements are actually performed.

---

**Yariv, A. (1988). *Quantum Electronics*, 3rd ed. Wiley.**

Chapter 9 (theory of optical waveguides) is the foundation for understanding mode propagation, polarization splitting, and birefringence in integrated photonic waveguides. Yariv's treatment of anisotropic waveguides and electrooptic effects is the basis for modulator design.

---

### On Photonic Neural Networks and MZI Meshes

**Shen, Y. et al. (2017). Deep learning with coherent nanophotonic circuits. *Nature Photonics*, 11(7), 441–446.** [DOI: 10.1038/nphoton.2017.93]

The paper that launched the modern era of photonic neural network hardware. Demonstrates a 4×4 MZI mesh implementing a linear classifier on a photonic chip. The architecture is based on the Reck decomposition. Read this paper after understanding the MZI physics (Section 2.2.4) — you will see exactly how the wave physics translates to computational capability.

---

**Reck, M., Zeilinger, A., Bernstein, H.J., & Bertani, P. (1994). Experimental realization of any discrete unitary operator. *Physical Review Letters*, 73(1), 58–61.** [DOI: 10.1103/PhysRevLett.73.58]

The theoretical foundation for MZI mesh photonic computing: proves that any $N \times N$ unitary matrix can be decomposed into a product of $2 \times 2$ unitary rotations (MZIs), implementable in a triangular array. The paper is short, clear, and foundational.

---

**Clements, W.R. et al. (2016). Optimal design for universal multiport interferometers. *Optica*, 3(12), 1460–1465.** [DOI: 10.1364/OPTICA.3.001460]

Improves on Reck et al. with a rectangular (balanced) decomposition that minimizes the depth of the MZI mesh and the maximum optical path length. For large $N$, this significantly reduces fabrication sensitivity and crosstalk. The current standard architecture for photonic matrix processors.

---

## Tier III: Primary Historical Sources

**Young, T. (1804). The Bakerian lecture: Experiments and calculations relative to physical optics. *Philosophical Transactions of the Royal Society of London*, 94, 1–16.**

Young's original interference paper. He introduces the principle of interference, uses the double-slit experiment to measure wavelengths, and gives numerical results consistent with modern measurements. The clarity and economy of the argument are remarkable.

---

**Fresnel, A.-J. (1818). Mémoire sur la diffraction de la lumière. *Annales de Chimie et de Physique*, 1, 239–281.**

Fresnel's diffraction theory, combining Huygens' construction with Young's interference principle. The Huygens-Fresnel integral is introduced here for the first time. Fresnel's derivation of the diffraction pattern of a circular obstacle (with the bright central spot) is contained in this memoir.

---

**Zernike, F. (1938). The concept of degree of coherence and its application to optical problems. *Physica*, 5(8), 785–795.**

Zernike introduces the "degree of coherence" as a physical quantity and derives (jointly with van Cittert) the theorem relating spatial coherence to source size. This paper establishes coherence theory as a quantitative subject.

---

**Jones, R.C. (1941). A new calculus for the treatment of optical systems. *Journal of the Optical Society of America*, 31(7), 488–493.** [DOI: 10.1364/JOSA.31.000488]

The first paper in Jones's six-paper series on polarization calculus. Introduces the Jones vector and Jones matrix formalism. Short (six pages), clear, and immediately applicable.

---

**Zehnder, L. (1891). Ein neuer Interferenzrefraktor. *Zeitschrift für Instrumentenkunde*, 11, 275–285.**
**Mach, L. (1892). Über einen Interferenzrefraktor. *Zeitschrift für Instrumentenkunde*, 12, 89–93.**

The original papers on the Mach-Zehnder interferometer. Worth reading for the original engineering design and the applications Zehnder and Mach envisioned — utterly unlike what the MZI is used for today.

---

*The references above, together with those cited in the chapter text, form a complete bibliography for Chapter 2.*
