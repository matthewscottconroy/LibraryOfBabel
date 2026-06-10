# Chapter 7: Further Reading and References

---

## Essential Textbooks

**Reed, G.T. & Knights, A.P. (2004). *Silicon Photonics: An Introduction*. Wiley.**
The foundational textbook for silicon photonics, written by two of the field's principal architects. Covers waveguide physics, fabrication, modulators, and detectors with a rigor and depth appropriate for graduate students. The modulator chapters remain valuable despite the age of the book; the physics hasn't changed, though the performance numbers have improved.

**Lipson, M., Koyama, F., & Soref, R.A. (Eds.) (various years). *Selected Papers on Silicon Photonics*. SPIE.**
A collection of landmark papers in silicon photonics; useful for tracing the historical development of the field.

**Pavesi, L. & Lockwood, D.J. (Eds.) (2004). *Silicon Photonics*. Springer.**
A multi-author volume covering the breadth of silicon photonics, including early work on nonlinear effects and optical amplification in silicon.

---

## Highly Recommended

**Saleh, B.E.A. & Teich, M.C. (2007). *Fundamentals of Photonics*, 2nd ed. Wiley.**
The chapter on waveguides (Chapter 8) is the clearest treatment of planar and optical fiber waveguide theory in any textbook. The chapters on electro-optic and acousto-optic modulation (Chapters 18–19) provide the theoretical background for both silicon and LiNbO₃ modulators.

**Yariv, A. & Yeh, P. (2007). *Photonics: Optical Electronics in Modern Communications*, 6th ed. Oxford.**
Strong on the physics of guided-wave devices and crystal optics relevant to LiNbO₃ and LNOI. The chapters on coupled-mode theory and electro-optic modulation are the authoritative source for the Jones matrix and waveguide coupling calculations in this chapter.

**Hecht, J. (2016). *City of Light: The Story of Fiber Optics*. Oxford University Press.**
A narrative history of fiber optics from idea to global telecommunications infrastructure. Not a textbook, but essential context for understanding why certain engineering choices were made and why 1550 nm won over competing wavelength windows.

---

## Primary Literature: Silicon Photonics Foundations

**Soref, R.A. & Bennett, B.R. (1987). "Electrooptical effects in silicon." *IEEE Journal of Quantum Electronics*, 23(1), 123–129.**
The paper that established the plasma dispersion relations for silicon. Every silicon electro-optic modulator designed since 1987 uses these relations (or their more recent refinements by Nedeljkovic et al. 2011).

**Liu, A., Jones, R., Liao, L., Samara-Rubio, D., Rubin, D., Cohen, O., ... & Paniccia, M. (2004). "A high-speed silicon optical modulator based on a metal-oxide-semiconductor capacitor." *Nature*, 427(6975), 615–618.**
The paper that demonstrated silicon photonics could achieve real modulation speeds. Intel's MOS capacitor modulator operating at ~1 GHz. The paper is notable for its engineering rigor and for its industrial origin.

**Xu, Q., Schmidt, B., Pradhan, S., & Lipson, M. (2005). "Micrometre-scale integrated silicon ring-resonator optical modulator." *Nature*, 435(7040), 325–327.**
The microring modulator demonstration that launched a thousand derivative papers. Showed that silicon resonators could function as high-contrast electro-optic devices despite silicon's lack of $\chi^{(2)}$.

**Thomson, D.J., Gardes, F.Y., Fedeli, J.-M., Zlatanovic, S., Hu, Y., Kuo, B.P.-P., ... & Reed, G.T. (2012). "50-Gb/s silicon optical modulator." *IEEE Photonics Technology Letters*, 24(4), 234–236.**
The 50 Gbps MZI depletion modulator that demonstrated silicon was competitive with LiNbO₃ bulk modulators for high-speed communications.

---

## Primary Literature: LNOI and Advanced Modulators

**Wang, C., Zhang, M., Chen, X., Bertrand, M., Shams-Ansari, A., Chandrasekhar, S., ... & Lončar, M. (2018). "Integrated lithium niobate electro-optic modulators operating at CMOS-compatible voltages." *Nature*, 562(7725), 101–104.**
The paper that established LNOI as a serious platform for high-performance photonic systems. $V_\pi L = 2.2$ V·cm, >100 GHz, zero chirp. Essential reading for anyone designing photonic computing hardware.

**Xu, M., He, M., Zhang, H., Jian, J., Pan, Y., Liu, X., ... & Cai, X. (2020). "High-performance coherent optical modulators based on thin-film lithium niobate platform." *Nature Communications*, 11(1), 3911.**
210 GHz bandwidth LNOI modulator — a result that seemed impossible for an on-chip device just a decade prior.

---

## Primary Literature: Phase-Change Materials

**Rios, C., Stegmaier, M., Hosseini, P., Wang, D., Scherer, T., Wright, C.D., Bhaskaran, H., & Pernice, W.H.P. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9(11), 725–732.**
First integrated PCM optical memory. Demonstrates the principle of non-volatile optical state storage using GST on a silicon photonic waveguide.

**Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H., & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569(7755), 208–214.**
PCM photonic synapse with 34 analog levels and demonstrated plasticity (Hebbian learning). The first proof-of-concept neuromorphic PCM photonic device.

**Feldmann, J., Youngblood, N., Karpov, M., Gehring, H., Li, X., Stappers, M., ... & Bhaskaran, H. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589(7840), 52–58.**
The landmark 4×4 PCM photonic matrix demonstration with WDM input, on-chip Ge detection, and vowel classification. The best current demonstration of non-volatile in-memory photonic computing.

---

## Primary Literature: Silicon Nitride and Frequency Combs

**Kippenberg, T.J., Holzwarth, R., & Diddams, S.A. (2011). "Microresonator-based optical frequency combs." *Science*, 332(6029), 555–559.**
The comprehensive review that established microresonator combs as a major research direction. Covers Si₃N₄, MgF₂, and other platforms.

**Herr, T., Brasch, V., Jost, J.D., Wang, C.Y., Kondratiev, N.M., Gorodetsky, M.L., & Kippenberg, T.J. (2014). "Temporal solitons in optical microresonators." *Nature Photonics*, 8(2), 145–152.**
The discovery of dissipative Kerr soliton combs — the coherent, single-soliton state that produces a flat, equidistant frequency comb. Essential for WDM photonic computing.

**Pfeiffer, M.H.P., Kordts, A., Brasch, V., Zernickel, M., Geiselmann, M., Jost, J.D., & Kippenberg, T.J. (2016). "Photonic Damascene process for integrated high-Q microresonator based nonlinear photonics." *Optica*, 3(1), 20–25.**
The photonic Damascene process for ultra-low-loss Si₃N₄; enables Q factors > 10⁷ and propagation losses < 0.01 dB/cm.

---

## Reviews and Roadmaps

**Reed, G.T., Mashanovich, G., Gardes, F.Y., & Thomson, D.J. (2010). "Silicon optical modulators." *Nature Photonics*, 4(8), 518–526.**
The comprehensive review of silicon modulator physics, device types, and performance state-of-art as of 2010. Still the best single reference for the physics of the field.

**Bogaerts, W., De Heyn, P., Van Vaerenbergh, T., De Vos, K., Kumar Selvaraja, S., Claes, T., ... & Baets, R. (2012). "Silicon microring resonators." *Laser & Photonics Reviews*, 6(1), 47–73.**
The canonical review of silicon microring resonators, covering design, fabrication, thermal effects, and applications.

**Lončar, M. & colleagues. (2021). Multiple LNOI platform papers in *Nature Photonics* and *Nature Reviews Physics*.**
Lončar's group publishes regular reviews of the LNOI platform. The most current review supersedes any specific paper cited here; search for "lithium niobate photonics review" + the current year.
