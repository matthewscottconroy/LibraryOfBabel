# Further Reading and References — Chapter 1: Maxwell's Equations and Electromagnetic Waves

The works listed here fall into three tiers. The first tier is *essential*: these are books and papers that any serious student of photonic computing should own, read carefully, and return to repeatedly throughout their career. The second tier is *highly recommended*: excellent sources that go deeper on specific topics treated in this chapter. The third tier is *primary historical sources*: the original papers in which the physics was first developed, invaluable for understanding what was actually claimed and why.

---

## Tier I: Essential References

### Textbooks on Electromagnetism

**Griffiths, D.J. (2017). *Introduction to Electrodynamics*, 4th ed. Cambridge University Press.**

This is the standard introductory text at the advanced undergraduate level and is almost universally used in the United States. Griffiths is extraordinarily clear: he explains not just *what* the equations say but *why* we should believe them, and he is honest about the places where the subject is subtle. His treatment of the displacement current (Section 7.3) is a model of how to motivate a physical addition to a mathematical structure. His chapters on electromagnetic waves (Chapter 9) cover much of what this chapter covers. If you find the present treatment too compressed, Griffiths is the place to go. The exercises in Griffiths are superb and substantially more numerous than what is provided here.

*How to read it for this chapter*: Chapter 2 (electrostatics), Chapter 5 (magnetostatics), Chapters 7–9 (electrodynamics and waves). His Chapter 9 on electromagnetic waves is particularly important for the material in Sections 4 and 5 of this chapter.

---

**Jackson, J.D. (1999). *Classical Electrodynamics*, 3rd ed. Wiley.**

Jackson is the graduate-level standard. It is formidable — dense, mathematical, comprehensive — but it is the source that most physicists mean when they say "Jackson." The treatment of energy and momentum in electromagnetic fields (Chapters 6 and 8), the radiation from accelerating charges (Chapter 9), and the discussion of wave propagation in media (Chapters 7 and 9) are unsurpassed in their rigor and depth. The problems are notorious for their difficulty; working even a subset of them is a graduate education in itself.

*How to read it for this chapter*: Chapter 6 (Maxwell's equations and conservation laws), Chapter 7 (plane electromagnetic waves and wave propagation). Jackson uses Gaussian units in some editions; be attentive to unit conventions.

---

**Born, M. & Wolf, E. (1999). *Principles of Optics*, 7th ed. Cambridge University Press.**

Born and Wolf is the canonical reference for classical optics. At 1000 pages, it is an encyclopedia — but an organized and rigorous one. The treatment of electromagnetic theory in Chapter 1 is terse but authoritative. Chapters 7 (elements of the theory of diffraction), 10 (partially coherent light), and 12 (diffraction of light by ultrasonic waves) are particularly relevant to later chapters in this book. The historical notes scattered throughout are invaluable.

*Note*: Born and Wolf uses the physics convention $e^{+i\omega t}$ for time dependence, opposite to the convention used in this book. Be careful about sign conventions when translating.

---

**Feynman, R.P., Leighton, R.B., & Sands, M. (1964). *The Feynman Lectures on Physics*, Vol. II. Addison-Wesley. (Available free online at feynmanlectures.caltech.edu)**

Feynman's lectures are not a textbook in the conventional sense — they are lectures, with all the directness and occasional imprecision that implies. But they offer something no conventional textbook provides: Feynman's physical intuition, which is the deepest in modern physics. His first chapter on electromagnetism (Vol. II, Chapter 1) opens with the quote used in the Unit Introduction of this book and sets out, with extraordinary clarity, why Maxwell's equations matter. His Chapter 18 on the Maxwell's equations themselves, Chapter 20 on the wave equation, and Chapter 27 on field energy and momentum are essential reading — not because they cover topics not covered elsewhere, but because Feynman makes you feel you understand them.

*How to read it for this chapter*: Vol. II, Chapters 1, 2, 4, 6, 7, 10, 13, 15, 18, 20, 27.

---

### References for Photonic Computing Applications

**Saleh, B.E.A. & Teich, M.C. (2019). *Fundamentals of Photonics*, 3rd ed. Wiley.**

The standard advanced undergraduate/graduate text on photonics. Covers lasers, fiber optics, modulators, detectors, waveguides, and more in depth. The electromagnetic foundations (Chapter 5 on electromagnetic optics, Chapter 6 on resonator optics) build on precisely what is developed in this chapter. Anyone intending to proceed to the later units of this book should have a copy.

---

**Yariv, A. & Yeh, P. (2006). *Photonics: Optical Electronics in Modern Communications*, 6th ed. Oxford University Press.**

Another essential photonics reference, stronger on devices and fiber optics than Saleh & Teich. The treatment of mode coupling and coupled-mode theory is particularly thorough and is the foundation for understanding MZI-based computing (Unit V) and many waveguide devices.

---

**Reed, G.T., Mashanovich, G., Gardes, F.Y., & Thomson, D.J. (2010). Silicon optical modulators. *Nature Photonics*, 4(8), 518–526.** [DOI: 10.1038/nphoton.2010.179]

The review paper on silicon modulators that is the standard entry point into the field. Explains the plasma dispersion effect (Soref-Bennett relations), device architectures (ring resonators, MZIs, electro-absorption), and performance metrics. Essential for understanding why silicon is the dominant platform for photonic computing.

---

## Tier II: Highly Recommended

### On the Electromagnetic Foundation

**Purcell, E.M. & Morin, D.J. (2013). *Electricity and Magnetism*, 3rd ed. Cambridge University Press.**

Purcell's classic text, updated by Morin, derives the magnetic field as a relativistic consequence of Coulomb's law — a beautiful unification that clarifies the deep relationship between electricity and magnetism. The derivation is not standard but is physically illuminating. The level is similar to Griffiths; the perspective is different and complementary.

---

**Stratton, J.A. (1941). *Electromagnetic Theory*. McGraw-Hill.**

An older, more formal text that remains useful for its rigor. Stratton's treatment of boundary conditions, Green's functions, and expansion of fields in terms of eigenfunctions is authoritative. Less readable than Griffiths or Jackson but valuable for deeper mathematical questions.

---

**Zangwill, A. (2013). *Modern Electrodynamics*. Cambridge University Press.**

A newer graduate-level text that many find more accessible than Jackson. Excellent treatment of dispersive media and the Kramers-Kronig relations (Chapter 18). Recommended for students who want Jackson-level rigor with more pedagogical support.

---

### On Optical Properties of Materials

**Palik, E.D., ed. (1998). *Handbook of Optical Constants of Solids*. Academic Press.**

The definitive tabulation of refractive index data for materials relevant to photonics. If you need the complex refractive index of silicon, silica, silicon nitride, lithium niobate, indium phosphide, or any other common photonic material, this is where to look. The introductory chapter on how the data is measured and compiled is itself instructive.

---

**Wooten, F. (1972). *Optical Properties of Solids*. Academic Press.**

A classic text connecting the macroscopic optical constants (n, κ) to the microscopic electronic structure. The treatment of the Drude model, the Lorentz oscillator model, and the Kramers-Kronig relations is clear and systematic. Short at under 200 pages, but dense with useful material.

---

**Soref, R.A. & Bennett, B.R. (1987). Electrooptical effects in silicon. *IEEE Journal of Quantum Electronics*, 23(1), 123–129.** [DOI: 10.1109/JQE.1987.1073206]

The foundational paper on the plasma dispersion effect in silicon. Provides the empirical formulas relating changes in carrier density to changes in refractive index and absorption coefficient at 1300 nm and 1550 nm. These "Soref-Bennett equations" are the basis for virtually all silicon modulator designs. Every photonic computing engineer should read this paper.

---

**Kramers, H.A. (1927). La diffusion de la lumière par les atomes. *Atti del Congresso Internazionale dei Fisici*, Como, September 1927, Vol. 2, pp. 545–557.**

**Kronig, R. de L. (1926). On the theory of dispersion of x-rays. *Journal of the Optical Society of America*, 12(6), 547–557.** [DOI: 10.1364/JOSA.12.000547]

The two original papers establishing the relations between real and imaginary parts of the susceptibility. Reading them is an education in the power of causality as a physical principle. Kronig (1926) is the more accessible; Kramers (1927) is more general.

---

### On Electromagnetic Momentum and Optical Forces

**Ashkin, A., Dziedzic, J.M., Bjorkholm, J.E., & Chu, S. (1986). Observation of a single-beam gradient force optical trap for dielectric particles. *Optics Letters*, 11(5), 288–290.** [DOI: 10.1364/OL.11.000288]

The paper introducing the single-beam optical trap (optical tweezers). Ashkin was awarded the Nobel Prize in Physics in 2018 for this work. The underlying physics is the gradient force from the Maxwell stress tensor — a direct consequence of the energy carried by electromagnetic fields.

---

**Aspelmeyer, M., Kippenberg, T.J., & Marquardt, F. (2014). Cavity optomechanics. *Reviews of Modern Physics*, 86(4), 1391–1452.** [DOI: 10.1103/RevModPhys.86.1391]

The definitive review of optomechanics, covering radiation pressure effects in optical cavities. Relevant to photonic computing as an engineering constraint (thermal and mechanical noise in high-Q resonators) and as a research direction (phonon-photon interfaces). Long and thorough; read the introduction and Section II for the relevant physics.

---

## Tier III: Primary Historical Sources

These papers represent some of the most important original contributions to the physics developed in this chapter. Reading them in the original is instructive both historically and technically.

---

**Coulomb, C.-A. (1785). Premier mémoire sur l'électricité et le magnétisme. *Histoire de l'Académie Royale des Sciences*, 569–577.**

Coulomb's original presentation of the inverse-square law for electrostatic forces, derived from measurements with his torsion balance. The experiment is elegant; the result — that force falls as 1/r² — is the empirical foundation of Gauss's law for the electric field.

---

**Faraday, M. (1832). Experimental researches in electricity. *Philosophical Transactions of the Royal Society of London*, 122, 125–162.** [DOI: 10.1098/rstl.1832.0006]

The first of Faraday's series of papers on electromagnetic induction. Faraday's style is completely non-mathematical, which is both unusual and instructive — he thinks entirely in terms of field lines and flux, concepts he invented. The introduction of the field concept, buried within the experimental description, is the most consequential single contribution to nineteenth-century physics.

---

**Maxwell, J.C. (1865). A dynamical theory of the electromagnetic field. *Philosophical Transactions of the Royal Society of London*, 155, 459–512.** [DOI: 10.1098/rstl.1865.0008]

Maxwell's masterwork: the paper in which he introduced the displacement current, unified electricity, magnetism, and optics, and predicted the existence of electromagnetic waves traveling at speed c = 1/√(μ₀ε₀). The argument is not presented in the compact four-equation form we use today (that is Heaviside's contribution); Maxwell's original version has twenty equations in twenty unknowns. But the physical reasoning is explicit and the prediction of electromagnetic waves is unmistakable.

---

**Hertz, H. (1888). Über die Ausbreitung der elektrischen Kraft. *Annalen der Physik*, 270(7), 551–569.** [DOI: 10.1002/andp.18882700702]

Hertz's paper reporting the experimental confirmation of electromagnetic wave propagation — the verification of Maxwell's prediction, twenty-three years after it was made. Hertz measured reflection, refraction, standing waves, and polarization of the waves produced by his oscillator. The clarity and simplicity of the experimental design are a model.

---

**Poynting, J.H. (1884). On the transfer of energy in the electromagnetic field. *Philosophical Transactions of the Royal Society of London*, 175, 343–361.** [DOI: 10.1098/rstl.1884.0016]

The paper introducing the Poynting vector S = E × H and establishing that electromagnetic energy flows in the direction of S. The derivation is reproduced in simplified form in Section 5.1 of this chapter. Poynting's result was immediately recognized as important — it gave electromagnetic energy a definite location in space, which was philosophically radical.

---

**Miya, T., Terunuma, Y., Hosaka, T., & Miyashita, T. (1979). Ultimate low-loss single-mode fibre at 1.55 μm. *Electronics Letters*, 15(4), 106–108.** [DOI: 10.1049/el:19790077]

The paper reporting the first demonstration of silica optical fiber with loss below 0.2 dB/km at 1550 nm, approaching the Rayleigh scattering limit. This measurement is the reason that 1550 nm became the standard telecommunications wavelength and, consequently, the standard wavelength for photonic computing.

---

**Kao, K.C. & Hockham, G.A. (1966). Dielectric-fibre surface waveguides for optical frequencies. *Proceedings of the IEE*, 113(7), 1151–1158.** [DOI: 10.1049/piee.1966.0189]

The paper proposing silica optical fiber as a communications medium, showing that impurity-limited attenuation (rather than the fundamental silica absorption) was the practical limitation, and that sub-20 dB/km loss was achievable. This paper launched the entire field of optical fiber communications. Kao was awarded the Nobel Prize in Physics in 2009 for this work.

---

**Beth, R.A. (1936). Mechanical detection and measurement of the angular momentum of light. *Physical Review*, 50(2), 115–125.** [DOI: 10.1103/PhysRev.50.115]

The first direct measurement of the spin angular momentum of light (±ℏ per photon for circular polarization), using a sensitive torsion pendulum. The experiment is a beautiful demonstration that angular momentum is a real, measurable property of the electromagnetic field.

---

**Allen, L., Beijersbergen, M.W., Spreeuw, R.J.C., & Woerdman, J.P. (1992). Orbital angular momentum of light and the transformation of Laguerre-Gaussian laser modes. *Physical Review A*, 45(11), 8185–8189.** [DOI: 10.1103/PhysRevA.45.8185]

The paper establishing that light beams with helical phase fronts carry orbital angular momentum of ℓℏ per photon, where ℓ is the topological charge. This result, published in 1992, was largely unexpected and opened the field of OAM optics. The paper is clearly written and accessible.

---

## A Note on How to Read Primary Sources

Primary sources reward patience. Maxwell's 1865 paper is not organized the way we would organize a modern physics paper; his twenty-equation system is unwieldy compared to Heaviside's four-equation reformulation. Hertz's papers are short and concrete — read them for the experimental design. Faraday's papers are entirely non-mathematical; read them for the physical intuition that preceded the mathematics by a generation.

The value of reading primary sources is not that they are clearer than modern textbooks (they are often not). The value is that they show you how physics was actually constructed: by people who did not know the answer, who had to choose which experiments to do and which theoretical moves to make, and who were sometimes wrong before they were right. This is the condition of everyone working at the frontier — including the photonic computing researchers whose work appears in the later chapters of this book.

---

*The references above form the core bibliography for Chapter 1. Complete bibliographic entries for all works cited in the chapter text are listed at the end of the relevant subsection files, in addition to being compiled here.*
