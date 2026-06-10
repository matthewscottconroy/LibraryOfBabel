# Further Reading and References: Chapter 3 — Light-Matter Interaction

The literature of light-matter interaction divides naturally into three layers: the classical treatment of optical response (Lorentz oscillator through Kramers-Kronig), the quantum theory of emission and amplification (Einstein through laser physics), and nonlinear optics (from the first SHG experiment to microresonator combs). The references below trace this stratification, from the essential textbooks through the primary research papers.

---

## Tier 1 — Essential Textbooks

**Saleh, B.E.A., & Teich, M.C. (2019). *Fundamentals of Photonics*, 3rd ed. Wiley-Interscience.**

The single most comprehensive textbook on photonics for engineers and applied physicists. Covers the Lorentz oscillator, dispersion, laser physics, and nonlinear optics with a consistent notation and level of rigor appropriate for advanced undergraduates. The third edition updates the photonic computing and fiber optics sections significantly. Chapters 5–7 (guided waves), 14–16 (laser physics), and 19 (nonlinear optics) are directly relevant to this chapter. Every photonic engineer should own a copy.

**Boyd, R.W. (2020). *Nonlinear Optics*, 4th ed. Academic Press.**

The authoritative textbook on nonlinear optics. Boyd covers χ⁽²⁾ and χ⁽³⁾ processes, coupled wave equations, phase matching, stimulated scattering, the nonlinear Schrödinger equation, and ultrafast optics with exceptional clarity and completeness. The fourth edition adds a chapter on quantum and nano-scale nonlinear optics. This is the primary reference for everything in Section 3 of this chapter (nonlinear optics) and the text to which all practitioners eventually return when they need a rigorous treatment of a nonlinear process.

**Agrawal, G.P. (2019). *Nonlinear Fiber Optics*, 6th ed. Academic Press.**

The definitive treatment of nonlinear optics in optical fibers: the nonlinear Schrödinger equation, solitons, modulational instability, stimulated Raman and Brillouin scattering, four-wave mixing, and microresonator combs. Agrawal's numerical methods appendix (the split-step Fourier method) is the standard algorithm for pulse propagation simulation. For anyone doing fiber-optic nonlinear optics computationally, this book and the companion *Fiber-Optic Communication Systems* (Agrawal, 6th ed., 2021) are essential.

**Allen, L., & Eberly, J.H. (1975). *Optical Resonance and Two-Level Atoms*. Wiley.** (Dover reprint 1987.)

A concise and rigorous treatment of the semiclassical theory of light-matter interaction: Rabi oscillations, the Bloch equations, free induction decay, photon echo, and dressed states. Written before laser cooling, before quantum information, before photonic computing — but the physics is timeless. Section 2 (Sections 2.1–2.6) is the cleanest derivation of the optical Bloch equations in the literature. For readers who want to understand why a laser amplifier behaves as it does at the level of individual atoms, this is the book.

---

## Tier 2 — Highly Recommended

**Yariv, A., & Yeh, P. (2006). *Photonics: Optical Electronics in Modern Communications*, 6th ed. Oxford University Press.**

Yariv and Yeh bridges the gap between the classical electromagnetism of Chapter 1 and the practical optoelectronics of semiconductor lasers, optical amplifiers, and photonic circuits. Strong on guided-wave optics, electro-optic modulators, and semiconductor laser theory. Chapters 5 (optical resonance and lasers) and 8–9 (electrooptics and acousto-optics) are particularly relevant. The treatment of the complex susceptibility and the connection to semiconductor band structure (Chapter 15) is excellent.

**Wooten, F. (1972). *Optical Properties of Solids*. Academic Press.**

A compact, rigorous derivation of the optical constants of solids from the Lorentz oscillator through quantum mechanical perturbation theory. The derivation of the Kramers-Kronig relations (Chapter 1) and the sum rules (Chapter 2) are among the clearest in the literature. This book is out of print but available in university libraries and through Dover-style reprint services. For the reader who wants to understand the quantum mechanical justification for the Lorentz model from first principles, Wooten Chapter 4 is the place to go.

**Shen, Y.R. (1984). *The Principles of Nonlinear Optics*. Wiley-Interscience.**

The comprehensive reference for nonlinear optics at the level of graduate theory. More mathematical and more complete than Boyd in some respects, particularly on the symmetry analysis of nonlinear susceptibility tensors and on the quantum mechanical derivation of χ⁽ⁿ⁾. Less readable than Boyd for first contact with the subject. Essential for anyone developing new nonlinear optical devices or doing theoretical work on nonlinear photonic systems.

**Siegman, A.E. (1986). *Lasers*. University Science Books.**

At 1283 pages, Siegman's *Lasers* is the most complete treatment of laser physics ever written. It covers rate equations, gain media, beam propagation (Gaussian beams done in depth), laser noise, Q-switching, mode locking, and much more. For the photonic computing engineer who uses lasers as sources and needs to understand linewidth, noise, coherence, and mode structure, Chapters 3–7 (laser amplification), 16–18 (Gaussian beams and optical resonators), and 25–28 (laser noise) are indispensable. Appendix A (matrix methods) is also excellent.

**Palik, E.D. (Ed.) (1985). *Handbook of Optical Constants of Solids*. Academic Press.**

The standard reference for the complex refractive index $\tilde{n} = n + i\kappa$ of optical materials as a function of wavelength. Essential for computing absorption, dispersion, and the Soref-Bennett plasma dispersion coefficients from first principles for any specific material. Silicon, silica, germanium, III-V semiconductors, lithium niobate, and many others are tabulated with references to primary measurements. When you need a number, this is where to look.

---

## Tier 3 — Primary Literature: Foundational Papers

**Lorentz, H.A. (1909). *The Theory of Electrons*. Teubner.** (Dover reprint available.)
The book form of Lorentz's electron theory, including the oscillator model of optical response. A remarkable document of the pre-quantum era: Lorentz derives dispersion, absorption, and the magneto-optical Kerr effect from classical electron dynamics, obtaining results that survive almost intact into quantum mechanics.

**Kramers, H.A. (1927). "La diffusion de la lumière par les atomes." *Atti del Congresso Internazionale dei Fisici*, Como, 2, 545–557.**
The original derivation of the dispersion relations by Kramers, based on the correspondence principle. Combined with Kronig's independent derivation (Kronig, R. de L. (1926). "On the theory of dispersion of X-rays." *Journal of the Optical Society of America*, 12(6), 547–557), these papers established the Kramers-Kronig relations.

**Einstein, A. (1917). "Zur Quantentheorie der Strahlung." *Physikalische Zeitschrift*, 18, 121–128.**
The paper introducing stimulated emission, A and B coefficients, and their thermodynamic relations. The most consequential paper for laser physics. Read the original: it is short, clear, and contains one of Einstein's characteristic uses of thermodynamic consistency as a constraint — he derives what the relations between A and B *must* be from the requirement that the system approach thermal equilibrium at large $t$, without knowing anything about the specific matrix element.

**Franken, P.A., Hill, A.E., Peters, C.W., & Weinreich, G. (1961). "Generation of optical harmonics." *Physical Review Letters*, 7(4), 118–119.**
Two pages. The founding paper of experimental nonlinear optics: second-harmonic generation in quartz using a ruby laser. The total energy of the second-harmonic signal in the original experiment was approximately 10⁻⁸ W. The field it opened generates billions of dollars of devices annually.

**Bloembergen, N., & Pershan, P.S. (1962). "Light waves at the boundary of nonlinear media." *Physical Review*, 128(2), 606–622.**
The theoretical framework for nonlinear optics at interfaces: boundary conditions for nonlinear polarization, the phase matching conditions, and the geometry of second-harmonic generation. The beginning of systematic nonlinear optics theory.

**Zakharov, V.E., & Shabat, A.B. (1972). "Exact theory of two-dimensional self-focusing and one-dimensional self-modulation of waves in nonlinear media." *Soviet Physics JETP*, 34(1), 62–69.**
The inverse scattering transform applied to the NLSE: proof of exact integrability and derivation of the soliton solutions. A tour de force of mathematical physics that also describes what you observe in a nonlinear fiber.

**Mollenauer, L.F., Stolen, R.H., & Gordon, J.P. (1980). "Experimental observation of picosecond pulse narrowing and solitons in optical fibers." *Physical Review Letters*, 45(13), 1095–1098.**
The first experimental observation of optical solitons: a 7 ps pulse propagating without broadening through 700 m of single-mode fiber. Initiated the field of soliton communications and is the direct experimental confirmation of Zakharov and Shabat's theory.

**Raman, C.V., & Krishnan, K.S. (1928). "A new type of secondary radiation." *Nature*, 121(3048), 501–502.**
One page. The discovery of the Raman effect. The paper is almost shockingly brief — it announces the discovery, gives the key observation (frequency-shifted scattering not explainable by fluorescence), and draws the comparison to the Compton effect in X-rays. The Nobel Prize followed two years later, the fastest ever awarded from discovery to prize.

---

## Tier 4 — Primary Literature: Photonic Computing Applications

**Soref, R.A., & Bennett, B.R. (1987). "Electrooptical effects in silicon." *IEEE Journal of Quantum Electronics*, 23(1), 123–129.**
[DOI: 10.1109/JQE.1987.1073206]
The foundational paper for silicon photonic modulation: empirical equations for the change in real and imaginary parts of the refractive index as a function of free-carrier density in silicon. All silicon modulators use these equations. A consequence of Kramers-Kronig applied to the plasma dispersion effect in a specific material.

**Boyraz, O., & Jalali, B. (2004). "Demonstration of a silicon Raman laser." *Optics Express*, 12(21), 5269–5273.**
The first laser fabricated in silicon, using stimulated Raman scattering and a reverse-biased p-i-n junction to suppress two-photon absorption-generated free carriers. Proved that silicon could be an active optical medium.

**Del'Haye, P., Schliesser, A., Arcizet, O., Wilken, T., Holzwarth, R., & Kippenberg, T.J. (2007). "Optical frequency comb generation from a monolithic microresonator." *Nature*, 450, 1214–1217.**
The founding paper of microresonator frequency combs: a single CW laser driving a silica microtoroid produces a comb spanning 500 nm. Demonstrated that integrated resonators could generate stable frequency combs without mode-locked lasers.

**Herr, T., Brasch, V., Jost, J.D., Wang, C.Y., Kondratiev, N.M., Kippenberg, T.J., & Gorodetsky, M.L. (2014). "Temporal solitons in optical microresonators." *Nature Photonics*, 8(2), 145–152.**
[DOI: 10.1038/nphoton.2013.343]
The demonstration of dissipative Kerr solitons in microresonators: stable, coherent frequency combs arising from the balance of Kerr gain, anomalous dispersion, parametric gain, and cavity loss. The stable soliton comb state is the most useful for applications including photonic computing.

**Marin-Palomo, P., et al. (2017). "Microresonator-based optical frequency combs for high-speed coherent data transmission." *Nature*, 546, 274–279.**
[DOI: 10.1038/nature22387]
Demonstrates Kerr microcombs used as WDM sources in coherent optical communications: 179 Gbit/s per channel, 50 channels. This paper is the proof-of-concept for using microcombs as the light source in WDM-based photonic matrix processors.

**Otterstrom, N.T., Behunin, R.O., Kittlaus, E.A., Wang, Z., & Rakich, P.T. (2018). "A silicon Brillouin laser." *Science*, 360(6393), 1113–1116.**
[DOI: 10.1126/science.aar6113]
The first Brillouin laser in silicon, exploiting stimulated Brillouin scattering in a silicon waveguide ring resonator. Generates extremely narrow-linewidth light (sub-Hz linewidth) in a CMOS-compatible platform. Relevant to photonic computing as an on-chip ultra-narrow-linewidth source.

---

## A Note on the Primary Literature in Nonlinear Optics

The field of nonlinear optics was created in a brief, intense period from 1960 to 1975. In this span, the key phenomena (SHG, SFG, OPA, SRS, SBS, self-phase modulation, four-wave mixing) were all discovered experimentally and explained theoretically, largely by groups at Bell Labs (Bloembergen, Terhune, Armstrong), MIT, and Stanford. The collection of foundational papers from this era is remarkably compact: approximately 20 papers account for most of the physics of χ⁽²⁾ and χ⁽³⁾ nonlinear optics as it is understood today.

For the reader who wishes to trace the original development, the most important journals are: *Physical Review Letters* (1958–1975 archives), *Physical Review* (same period), and *Applied Physics Letters*. Boyd's *Nonlinear Optics* provides excellent historical notes and original citations. Shen's *Principles of Nonlinear Optics* contains the most complete bibliography of the theoretical literature.

The soliton literature has a different character: it is primarily mathematical physics from the Soviet school (Zakharov, Shabat, Manakov) combined with experimental fiber optics from Bell Labs (Mollenauer, Stolen, Gordon). Both Agrawal's *Nonlinear Fiber Optics* and the review by Hasegawa and Tappert (1973, *Applied Physics Letters*) are good entry points.

---

## Computing Further References

Beyond the textbooks, the following review articles are useful bridges to the research literature:

**Moss, D.J., Morandotti, R., Gaeta, A.L., & Lipson, M. (2013). "New CMOS-compatible platforms based on silicon nitride and Hydex for nonlinear optics." *Nature Photonics*, 7(8), 597–607.**
[DOI: 10.1038/nphoton.2013.183]
Review of Si₃N₄ and Hydex as nonlinear platforms: low loss, high FOM, CMOS-compatible. The go-to reference for understanding why Si₃N₄ has largely displaced silicon for Kerr-effect photonic processing.

**Kippenberg, T.J., Holzwarth, R., & Diddams, S.A. (2011). "Microresonator-based optical frequency combs." *Science*, 332(6029), 555–559.**
[DOI: 10.1126/science.1193968]
The review that established microresonator combs as a distinct field: physics, platforms (silica, MgF₂, Si₃N₄), and applications.
