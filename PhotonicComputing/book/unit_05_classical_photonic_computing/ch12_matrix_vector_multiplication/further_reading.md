# Chapter 12: Further Reading and References

## Unitary Decompositions and Programmable Meshes

**Reck, M., Zeilinger, A., Bernstein, H.J., & Bertani, P. (1994).** "Experimental realization of any discrete unitary operator." *Physical Review Letters*, 73, 58–61.
*The founding theorem: any N×N unitary factors into 2×2 beam-splitter operations arranged in a triangular mesh. Short, readable, and endlessly cited.*

**Clements, W.R., Humphreys, P.C., Metcalf, B.J., Kolthammer, W.S., & Walmsley, I.A. (2016).** "Optimal design for universal multiport interferometers." *Optica*, 3(12), 1460–1465.
*The rectangular mesh: same MZI count as Reck, half the depth, balanced path loss. The default architecture of modern implementations; the decomposition algorithm is given explicitly.*

**Miller, D.A.B. (2013).** "Self-configuring universal linear optical component." *Photonics Research*, 1(1), 1–15.
*Shows that meshes can align themselves progressively using local feedback and power monitors, with no global computation or precise calibration. Conceptually foundational for practical large meshes.*

**Bogaerts, W., Pérez, D., Capmany, J., Miller, D.A.B., Poon, J., Englund, D., Morichetti, F., & Melloni, A. (2020).** "Programmable photonic circuits." *Nature*, 586, 207–216.
*Review of general-purpose programmable photonics — meshes as the FPGA of optics — situating matrix processors within the broader programmable-circuit agenda.*

---

## Photonic Neural Network Hardware — Coherent

**Shen, Y., et al. (2017).** "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446.
*The field-launching experiment analyzed in Section 12.3.2: 56-MZI processor, SVD architecture, vowel classification, and the error analysis that defined the research agenda.*

**Harris, N.C., et al. (2017).** "Quantum transport simulations in a programmable nanophotonic processor." *Nature Photonics*, 11, 447–452.
*The same processor running quantum walks — a vivid illustration that programmable linear optics serves classical and quantum computing alike.*

**Bandyopadhyay, S., Hamerly, R., & Englund, D. (2021).** "Hardware error correction for programmable photonics." *Optica*, 8(10), 1247–1255.
*Systematic taxonomy of MZI mesh errors and the correction algorithms that recover several bits of matrix fidelity. Primary reference for Section 12.2.4.*

**Hamerly, R., Bandyopadhyay, S., & Englund, D. (2022).** "Accurate self-configuration of rectangular multiport interferometers." *Physical Review Applied*, 18, 024019.
*Practical self-configuration for Clements meshes, extending Miller's progressive alignment to the rectangular topology.*

**Hamerly, R., Bernstein, L., Sludds, A., Soljačić, M., & Englund, D. (2019).** "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9, 021032.
*The coherent-detection alternative to meshes: homodyne multiplication scales to N ~ 10⁶ with sub-attojoule optical energy per MAC. Essential for understanding the large-N limit.*

---

## Photonic Neural Network Hardware — Incoherent and WDM

**Tait, A.N., Nahmias, M.A., Shastri, B.J., & Prucnal, P.R. (2014).** "Broadcast and weight: An integrated network for scalable photonic spike processing." *Journal of Lightwave Technology*, 32(21), 4029–4041.
*The WDM network protocol of Section 12.4.2. Defines weight banks, modulator neurons, and the scaling rules.*

**Tait, A.N., et al. (2017).** "Neuromorphic photonic networks using silicon photonic weight banks." *Scientific Reports*, 7, 7430.
*First silicon broadcast-and-weight system; recurrent dynamics quantitatively matching the programmed weight matrix.*

**Zhang, W., et al. (2022).** "Silicon microring synapses enable photonic deep learning beyond 9-bit precision." *Optica*, 9(5), 579–584.
*The weight-precision record, achieved with dithering feedback control of microrings. Read alongside the MZI error papers to understand why incoherent weights calibrate more easily.*

**Feldmann, J., et al. (2021).** "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589, 52–58.
*PCM crossbar weights + soliton microcomb inputs: non-volatile in-memory photonic computing at tera-MAC rates.*

**Xu, X., et al. (2021).** "11 TOPS photonic convolutional accelerator for optical neural networks." *Nature*, 589, 44–51.
*Time-wavelength interleaved convolution using a soliton crystal comb and dispersive delay — the highest-throughput photonic processor of its generation, built almost entirely from telecom components.*

**Huang, C., et al. (2021).** "A silicon photonic–electronic neural network for fibre nonlinearity compensation." *Nature Electronics*, 4, 837–844.
*The strongest application argument for WDM neuromorphics: processing wideband optical signals in flight, where electronics cannot follow.*

---

## Energy Analysis and System Perspective

**Nahmias, M.A., Ferreira de Lima, T., Tait, A.N., Peng, H.-T., Shastri, B.J., & Prucnal, P.R. (2020).** "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518.
*The most careful published accounting of photonic MAC energy, coherent and incoherent, including all conversion overheads. The antidote to headline TOPS figures.*

**Demirkiran, C., et al. (2023).** "An electro-photonic system for accelerating deep neural networks." *ACM Journal on Emerging Technologies in Computing Systems*, 19(4).
*Architecture-level analysis (ADEPT) of a full accelerator including DAC/ADC, SRAM, and scheduling — where photonic advantage survives system integration and where it evaporates.*

**Ramey, C. (2020).** "Silicon photonics for artificial intelligence acceleration." *IEEE Hot Chips 32 Symposium*.
*Lightmatter's Mars architecture disclosure; the best public window into commercial photonic accelerator engineering.*

**Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021).** "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15, 102–114.
*The standard survey of the whole landscape covered by Chapters 12–13; an excellent orientation read before diving into primary sources.*

**McMahon, P.L. (2023).** "The physics of optical computing." *Nature Reviews Physics*, 5, 717–734.
*A rigorous, skeptical assessment of where optical computing's advantages are physically real. Highly recommended as the capstone reading for this chapter.*

---

## Books

**Prucnal, P.R., & Shastri, B.J. (2017).** *Neuromorphic Photonics*. CRC Press.
*Book-length development of the broadcast-and-weight architecture and photonic spike processing; the standard text for the incoherent school.*

**Goodman, J.W. (2017).** *Introduction to Fourier Optics* (4th ed.). W.H. Freeman.
*Background for the free-space matrix multipliers (Chapter 11's 4f systems and Chapter 14's diffractive networks) that bracket this chapter's integrated approaches.*
