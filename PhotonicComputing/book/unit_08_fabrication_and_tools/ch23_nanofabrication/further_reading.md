# Chapter 23: Further Reading and References

---

## Essential Textbooks

**Chrostowski, L. & Hochberg, M. (2015). *Silicon Photonics Design: From Devices to Systems*. Cambridge University Press.**
The single most useful book for this chapter and the next: it is organized around exactly the fabless design-fabricate-test loop taught here, with concrete treatment of fabrication variability, design-for-test layout conventions, and MPW practice. Written by two people who built the community's shared infrastructure (SiEPIC and OpSIS respectively). If you buy one book for this unit, buy this one.

**Franssila, S. (2010). *Introduction to Microfabrication*, 2nd ed. Wiley.**
The friendliest rigorous survey of cleanroom unit processes — lithography, etching, deposition, CMP, implantation — written to be read cover to cover. Ideal preparation for a first cleanroom course or a first foundry tape-out.

**Campbell, S.A. (2013). *Fabrication Engineering at the Micro- and Nanoscale*, 4th ed. Oxford University Press.**
The standard graduate fabrication text, deeper than Franssila on the physics of each process (plasma physics of RIE, implantation range theory, lithographic imaging).

**Reed, G.T. & Knights, A.P. (2004). *Silicon Photonics: An Introduction*. Wiley.**
Covers the silicon photonics platform including its fabrication foundations; still valuable for waveguide-process interactions.

---

## Reviews: Platform and Foundry Ecosystem

**Jalali, B. & Fathpour, S. (2006). "Silicon photonics." *Journal of Lightwave Technology*, 24(12), 4600–4615.**
An early and influential review of the silicon photonics platform by one of its founders, written just as foundry processes were coalescing.

**Hochberg, M. & Baehr-Jones, T. (2010). "Towards fabless silicon photonics." *Nature Photonics*, 4(8), 492–494.**
The manifesto for the MPW/PDK model this chapter teaches: standardized processes, shared runs, portable designs. Short, opinionated, and largely vindicated by the following decade.

**Bogaerts, W. & Chrostowski, L. (2018). "Silicon photonics circuit design: methods, tools and challenges." *Laser & Photonics Reviews*, 12(4), 1700237.**
Bridges this chapter and Chapter 24: how PDKs, compact models, layout automation, and variability analysis assemble into a design methodology.

---

## Primary Literature: Process Control and Variability

**Selvaraja, S.K., Bogaerts, W., Dumon, P., Van Thourhout, D., & Baets, R. (2010). "Subnanometer linewidth uniformity in silicon nanophotonic waveguide devices using CMOS fabrication technology." *IEEE Journal of Selected Topics in Quantum Electronics*, 16(1), 316–324.**
The classic quantitative study of what 193 nm CMOS lithography delivers for photonics: linewidth uniformity at the ~2 nm level and its consequences for device spectra. The methodological template for Section 23.2.3.

**Zortman, W.A., Trotter, D.C., & Watts, M.R. (2010). "Silicon photonics manufacturing." *Optics Express*, 18(23), 23598–23607.**
Measures resonance-wavelength variation of microrings across wafers and disentangles width from thickness contributions — empirical grounding for the sensitivity numbers used in this chapter.

---

## Primary Literature: III-V Integration

**Fang, A.W., Park, H., Cohen, O., Jones, R., Paniccia, M.J., & Bowers, J.E. (2006). "Electrically pumped hybrid AlGaInAs-silicon evanescent laser." *Optics Express*, 14(20), 9203–9210.**
The founding demonstration of the bond-then-process heterogeneous laser: unpatterned III-V bonded to SOI, cavity defined by silicon lithography. The insight that made laser integration a wafer-scale technology.

**Roelkens, G., Liu, L., Liang, D., Jones, R., Fang, A., Koch, B., & Bowers, J. (2010). "III-V/silicon photonics for on-chip and intra-chip optical interconnects." *Laser & Photonics Reviews*, 4(6), 751–779.**
Comprehensive review of bonding chemistries (direct and DVS-BCB adhesive), coupling structures, and early heterogeneous devices, from the two groups that defined the field.

**Komljenovic, T., Davenport, M., Hulme, J., Liu, A.Y., Santis, C.T., Spott, A., Srinivasan, S., Stanton, E.J., Zhang, C., & Bowers, J.E. (2016). "Heterogeneous silicon photonics integrated circuits." *Journal of Lightwave Technology*, 34(1), 20–35.**
The mature platform review: lasers, SOAs, modulators, detectors, and full circuits by heterogeneous integration.

**Chen, S., Li, W., Wu, J., Jiang, Q., Tang, M., Shutts, S., Elliott, S.N., Sobiesierski, A., Seeds, A.J., Ross, I., Smowton, P.M., & Liu, H. (2016). "Electrically pumped continuous-wave III-V quantum dot lasers on silicon." *Nature Photonics*, 10(5), 307–311.**
The monolithic milestone: CW, electrically pumped 1.3 μm InAs quantum-dot lasers grown epitaxially on silicon, with the dislocation-tolerance argument for quantum dots demonstrated in hardware.

---

## Primary Literature: Packaging and Coupling

**Marchetti, R., Lacava, C., Carroll, L., Gradkowski, K., & Minzioni, P. (2019). "Coupling strategies for silicon photonics integrated chips [Invited]." *Photonics Research*, 7(2), 201–239.**
The definitive tutorial-review of fiber-chip coupling: grating coupler theory and optimization (apodization, mirrors, 2D gratings) and edge-coupler design, with performance surveys.

**Carroll, L., Lee, J.-S., Scarcella, C., Gradkowski, K., Duperron, M., Lu, H., Zhao, Y., Eason, C., Morrissey, P., Rensing, M., Collins, S., Hwang, H.Y., & O'Brien, P. (2016). "Photonic packaging: transforming silicon photonic integrated circuits into photonic devices." *Applied Sciences*, 6(12), 426.**
From the Tyndall packaging group: fiber attach, micro-optics, electrical integration, and thermal management as a coherent engineering discipline — with the cost analyses behind this chapter's "the package is the computer" refrain.

**Lindenmann, N., Balthasar, G., Hillerkuss, D., Schmogrow, R., Jordan, M., Leuthold, J., Freude, W., & Koos, C. (2012). "Photonic wire bonding: a novel concept for chip-scale interconnects." *Optics Express*, 20(16), 17667–17677.**
Two-photon-polymerized free-form waveguides written between chips after placement — the idea that alignment can be corrected photonically rather than mechanically.

---

## Primary Literature: Monolithic Electronic-Photonic Integration

**Sun, C., Wade, M.T., Lee, Y., Orcutt, J.S., Alloatti, L., Georgas, M.S., ... Stojanović, V. (2015). "Single-chip microprocessor that communicates directly using light." *Nature*, 528(7583), 534–538.**
The "zero-change" landmark: a RISC-V processor and photonic transceivers fabricated in an unmodified 45 nm SOI CMOS process, communicating processor-to-memory over fiber.

**Atabaki, A.H., Moazeni, S., Pavanello, F., Gevorgyan, H., Notaros, J., Alloatti, L., ... Ram, R.J., Stojanović, V., & Popović, M.A. (2018). "Integrating photonics with silicon nanoelectronics for the next generation of systems on a chip." *Nature*, 556(7701), 349–354.**
Polysilicon-based photonics integrated into bulk-CMOS electronics — the complementary route to monolithic integration, in the substrate type where most logic actually lives.

---

## Lithography Context

**Wagner, C. & Harned, N. (2010). "EUV lithography: Lithography gets extreme." *Nature Photonics*, 4(1), 24–26.**
A compact, readable account of why EUV took decades: tin-plasma sources, all-reflective optics, and the engineering of the most complex tool in manufacturing.

---

## Online Resources

**AIM Photonics Academy (aimphotonics.academy)** — Course materials, PDK tutorials, and design-for-manufacturing content aligned with the AIM Photonics US foundry.

**Europractice (europractice-ic.com)** — Current MPW schedules, prices, and PDK access procedures for IMEC, CORNERSTONE, LioniX, SMART Photonics, and others; the practical starting point for a first tape-out in Europe.

**SiEPIC Program materials (siepic.ca and the SiEPIC-EBeam PDK on GitHub)** — Open design kits, layout tools, and course materials from Chrostowski's UBC program; includes the fabrication-variability-aware design flow used in this chapter's exercises.
