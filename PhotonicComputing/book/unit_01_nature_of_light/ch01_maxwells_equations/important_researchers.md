# Important Researchers — Chapter 1: Maxwell's Equations and Electromagnetic Waves

The physics in this chapter was built by a small number of people over roughly a century (1785–1888), each adding a necessary piece to a structure none of them could see whole. What follows is not a comprehensive history but a set of portraits — sufficient to give a sense of who these people were, what they contributed, and why their contributions matter specifically to photonic computing.

---

## Charles-Augustin de Coulomb (1736–1806)

Coulomb was a French military engineer and physicist who brought to physics the precision of engineering measurement. He invented the torsion balance — a device sensitive enough to measure the tiny forces between small charged spheres — and used it to establish, with quantitative rigor, that electrostatic force falls off as the inverse square of distance. His *Mémoires* of 1785–1789 established both the electric and magnetic inverse-square laws [1].

The contribution to photonic computing is indirect but foundational: Coulomb's law, when reformulated by Gauss, becomes the first of Maxwell's equations. Every electric field in every silicon photonic modulator, every calculation of the Pockels effect in lithium niobate, every determination of where charge accumulates at a dielectric interface — all trace back to the inverse-square law Coulomb measured with his torsion balance.

What is worth noting about Coulomb is the character of his contribution: he did not theorize about what electricity was; he measured what it *did*, carefully and quantitatively. The precision of his measurements made Gauss's reformulation possible. Experiment before theory.

---

## Carl Friedrich Gauss (1777–1855)

Gauss was one of the great mathematicians in history — the developer of the method of least squares, of non-Euclidean geometry, of number theory, and much else. His contributions to electromagnetism are among his less celebrated achievements, which is a comment on how extraordinary his mathematical work was.

Gauss's law — the statement that the total electric flux through any closed surface equals the enclosed charge divided by ε₀ — is a reformulation of Coulomb's law in integral form. The reformulation is not merely equivalent in content; it is vastly more powerful as a tool for calculation and as the basis for the differential form obtained by the divergence theorem. Gauss's law for magnetism (∇·**B** = 0, or its integral equivalent) is the statement that magnetic monopoles do not exist, which has deep consequences for the topology of magnetic field lines and for the transversality of electromagnetic waves.

Gauss also developed the mathematical theory of the divergence theorem (sometimes called Gauss's theorem), which is the bridge from the integral to the differential form of the first two of Maxwell's equations.

---

## Hans Christian Ørsted (1777–1851)

Ørsted was a Danish physicist who, in 1820, made what may be the most consequential accidental discovery in the history of electromagnetism: that a current-carrying wire deflects a compass needle. The observation was accidental (it occurred during a lecture demonstration), but Ørsted recognized its significance and immediately published his results [2].

The meaning of the observation: electric currents create magnetic fields. This was the first evidence that electricity and magnetism — previously studied as entirely separate phenomena — were related. Ørsted's discovery triggered an immediate experimental response from Ampère and others, and was the empirical basis for Ampère's law.

The connection to photonic computing: Ørsted's discovery is what makes electromagnetic waves possible. The wave propagation mechanism — Faraday's law says a changing magnetic field creates an electric field; Ampère-Maxwell says a changing electric field creates a magnetic field — requires that electricity and magnetism be coupled, which Ørsted's 1820 observation first revealed.

---

## André-Marie Ampère (1775–1836)

Ampère was a French physicist and mathematician who, within two weeks of hearing of Ørsted's discovery, had performed a comprehensive set of experiments establishing the quantitative relationship between current and magnetic field. Ampère was described by Maxwell himself as "the Newton of electricity," and the characterization is apt: as Newton axiomatized classical mechanics, Ampère axiomatized electrodynamics at the macroscopic level.

Ampère's law in its original form — that the line integral of **B** around a closed loop equals μ₀ times the enclosed current — is a correct description of magnetostatics. Its limitation (the failure for time-varying fields) was not Ampère's fault; the technology to probe time-varying fields at frequencies relevant to electromagnetic waves did not exist in his lifetime. The addition of the displacement current was Maxwell's correction, not a correction of any error Ampère made.

---

## Michael Faraday (1791–1867)

Faraday was one of the greatest experimental physicists of all time and one of the very few who made contributions both to physics and to chemistry of the first rank. He was largely self-educated — born to a blacksmith's family, he became a laboratory assistant at the Royal Institution at age twenty-one and never left. He could not do the mathematics that his contemporaries used; he thought entirely in terms of field lines and physical models, and it was this visual thinking that led him to conceive of the electromagnetic field.

His contributions relevant to this chapter:

**Electromagnetic induction (1831)**: Faraday discovered that a changing magnetic flux induces an electromotive force in a nearby circuit. This is Faraday's law, the third of Maxwell's equations. The discovery came through a series of careful experiments with iron rings, wire coils, and galvanometers.

**The field concept**: Faraday's most abstract contribution was the introduction of the electromagnetic field as a physical entity — not a mathematical convenience but a real thing, present in space, capable of storing energy. The field fills the space between charges and magnets; the forces are transmitted through the field, not at a distance. This was philosophically controversial in Faraday's time (Newton's gravity had been action at a distance), but it is the conceptual framework of all modern field theory. Without Faraday's concept of the field, Maxwell would have had no medium in which to embed his equations.

**The Faraday effect (1845)**: Faraday discovered that a magnetic field can rotate the plane of polarization of light passing through certain materials — the first direct experimental evidence that light and electromagnetism were related. This discovery motivated Maxwell's search for a unified theory.

For photonic computing, Faraday's law is not only a fundamental equation — the Faraday effect is the basis of the optical isolator, an essential component of any photonic system that uses a laser. The laser's output must be isolated from back-reflections that would destabilize the lasing mode; optical isolators (Faraday rotators combined with polarizers) perform this function.

---

## James Clerk Maxwell (1831–1879)

Maxwell was arguably the greatest theoretical physicist of the nineteenth century, and by Feynman's assessment the most significant scientific figure of that century by any measure. He died at forty-eight, of abdominal cancer — the same age and disease that killed his mother when he was eight. In a career of less than thirty years, he founded statistical mechanics, made the first durable color photograph, determined the composition of Saturn's rings, and unified electricity, magnetism, and optics into a single theory.

The specific contribution to this chapter is the introduction of the displacement current in 1865. The logical argument is explained in detail in Section 1.2 of this chapter: Ampère's law, as formulated, failed for time-varying fields, and the failure could be remedied only by adding a term proportional to ∂**E**/∂t. The addition was not forced by any experiment — no experiment had probed the relevant regime. It was forced by consistency with charge conservation and by Maxwell's mechanical model of the electromagnetic aether.

The consequences were: completion of the symmetry between the electric and magnetic fields in the curl equations; the existence of electromagnetic waves propagating at speed c = 1/√(μ₀ε₀) ≈ 3 × 10⁸ m/s; and, since this equaled the known speed of light, the identification of light as an electromagnetic wave.

Maxwell's 1865 paper is one of the most productive single papers in the history of physics. The wave equation appears in Part VI of that paper, and Maxwell notes, with what seems like deliberate understatement, that the agreement between the calculated speed of propagation and the measured speed of light is "not only a remarkable coincidence, but also a strong confirmation of the reality of the identity of light and electro-magnetic disturbances."

For photonic computing, Maxwell is not merely a historical figure. Photonic computing *is* applied Maxwell's equations. The reason we use 1550 nm light, the reason waveguides work, the reason interferometers can perform matrix multiplication, the reason polarization carries additional information — these are all direct consequences of Maxwell's four equations and their solutions.

---

## Oliver Heaviside (1850–1925)

Heaviside was a self-taught English electrical engineer and mathematician who reformulated Maxwell's original theory — twenty equations in twenty unknowns — into the compact four-equation form used today [3]. This is not a trivial notational simplification: Heaviside's reformulation using vector calculus (which he partly developed himself, independently of Gibbs) reveals the structure of the theory in a way Maxwell's original formulation does not.

Heaviside also introduced the concept of impedance, developed transmission line theory, and made substantial contributions to operator calculus. He spent much of his career in poverty and obscurity, corresponding with a small circle of physicists and engineers while living in relative isolation.

The four equations on Heaviside's formulation are the ones taught in every physics course today. The beautiful symmetry between the curl equations — Faraday's law and the Ampère-Maxwell law — is visible only in Heaviside's formulation. When physicists and engineers say "Maxwell's equations," they mean Heaviside's reformulation of Maxwell's equations.

---

## Heinrich Hertz (1857–1894)

Hertz was a German physicist who, between 1886 and 1888, produced and detected electromagnetic waves in the laboratory, confirming Maxwell's prediction [4]. He used an oscillating spark discharge to generate waves at centimeter wavelengths (what we would now call UHF radio), and he detected them with a resonant receiver loop. He then systematically demonstrated that these waves exhibited reflection, refraction, standing waves, and polarization — exactly the properties predicted for electromagnetic waves.

Hertz's contribution to photonic computing is the experimental validation of the entire theoretical framework. Before Hertz, Maxwell's equations predicted the existence of electromagnetic waves; after Hertz, it was established. Every claim made on the basis of Maxwell's equations — including all the claims about light propagation in waveguides, fiber optics, and photonic chips — ultimately rests on Hertz's experimental confirmation.

Hertz died at thirty-six, of a blood disease. He did not live to see the consequences of his discovery: wireless telegraphy (Marconi), radio, radar, and ultimately the global telecommunications infrastructure that photonic computing inhabits.

---

## Richard A. Soref (1936–)

Soref is an American physicist who is often called "the father of silicon photonics." His 1987 paper with Bennett [5] provided the empirical formulas relating the free-carrier plasma dispersion effect in silicon to changes in the real and imaginary parts of the refractive index — the "Soref-Bennett equations" that are the basis for virtually all silicon electro-optic modulators.

This contribution is directly relevant to photonic computing: the ability to modulate light in silicon waveguides using the plasma dispersion effect is what makes silicon-based photonic processors possible. Without a mechanism to modulate light in silicon (which has no linear electro-optic effect due to its centrosymmetric crystal structure), photonic computing on the dominant semiconductor platform would be infeasible.

Soref spent much of his career at AFRL (Air Force Research Laboratory) and later at the University of Massachusetts Boston. His later work predicted the extension of silicon photonics to mid-infrared wavelengths and laid groundwork for the silicon-germanium photonic platform.

---

## Notes

[1] Coulomb, C.-A. (1785). Premier mémoire sur l'électricité et le magnétisme. *Histoire de l'Académie Royale des Sciences*, 569–577.

[2] Ørsted, H.C. (1820). Experiments on the effect of a current of electricity on the magnetic needle. *Annals of Philosophy*, 16, 273–277.

[3] Heaviside, O. (1893). *Electromagnetic Theory*, Vol. 1. The Electrician Publishing Company.

[4] Hertz, H. (1888). Über Strahlen elektrischer Kraft. *Annalen der Physik*, 271(12), 769–783.

[5] Soref, R.A. & Bennett, B.R. (1987). Electrooptical effects in silicon. *IEEE Journal of Quantum Electronics*, 23(1), 123–129.
