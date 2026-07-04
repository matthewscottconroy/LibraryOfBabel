# Further Reading and References — Chapter 25: Co-Design and Benchmarking

The literature of this chapter is deliberately split between two shelves that seldom sit together: photonic-device papers that report optical-core physics, and computer-architecture and circuits papers that insist on whole-system measurement. Reading only the first shelf is how the field's five-orders-of-magnitude spread in efficiency claims came about. The tiers below are ordered to keep both shelves in view.

---

## Tier I: Essential — The System-Accounting Core

**Nahmias, M.A., Ferreira de Lima, T., Tait, A.N., Peng, H.-T., Shastri, B.J., & Prucnal, P.R. (2019/2020). "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518.**

The formal definition of the photonic MAC and the paper that established its energy-scaling laws, including the $O(N)$-conversion / $O(N^2)$-computation asymmetry on which the entire amortization argument of this chapter depends. *How to use it*: read it immediately after Section 25.1.1; it is the primary source for the master equation and for weight-stationary energy accounting.

---

**Al-Qadasi, M.A., Chrostowski, L., Shastri, B.J., & Shekhar, S. (2022). "Scaling up silicon photonic-based accelerators: challenges and opportunities." *APL Photonics*, 7(2), 020902.**

The independent full-system model that itemizes DAC, ADC, TIA, laser, and thermal-tuning power for MZI- and microring-based accelerators and reaches this chapter's conclusion by a different route: conversion and laser overheads dominate, and array size and precision set the crossover with electronics. *How to use it*: the quantitative backbone of Sections 25.1.2 and 25.2.1; work its energy tables alongside the worked example.

---

**Miller, D.A.B. (2017). "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35(3), 346–396.**

The canonical analysis of where optoelectronic energy actually goes. Miller shows that interface capacitance, not the optical process, dominates, and derives the attojoule-scale targets a competitive system must approach. *How to use it*: the physical justification for why integration technology (Section 25.1.2) multiplies the entire interface budget, and for the density and nonlinearity limits of Section 25.3.3.

---

**Hamerly, R., Bernstein, L., Sludds, A., Soljačić, M., & Englund, D. (2019). "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9(2), 021032.**

The strongest quantitative statement of the *pro*-photonics case: a standard-quantum-limit analysis showing that optical energy per MAC falls as $2^{2b}E_{\text{ph}}/N$, i.e. toward attojoules for large arrays, and the argument for coherent (homodyne) readout to reach the shot limit. *How to use it*: the counterweight to the tax accounting; it defines the ceiling of what photonics could achieve if every electronic penalty were removed.

---

**Reuther, A., Michaleas, P., Jones, M., Gadepally, V., Samsi, S., & Kepner, J. (2019, updated through 2022). "Survey and benchmarking of machine learning accelerators." *IEEE High Performance Extreme Computing Conference (HPEC)*.**

The shared coordinate system — peak performance versus power — on which any accelerator, electronic or photonic, can be located. *How to use it*: re-plot any photonic claim at the wall-plug boundary onto these axes; where it lands relative to the electronic frontier is the honest verdict.

---

**Reddi, V.J., et al. (2020). "MLPerf inference benchmark." *Proceedings of the 47th International Symposium on Computer Architecture (ISCA)*, 446–459.** (See also Mattson, P., et al. (2020). "MLPerf training benchmark." *MLSys*.)

The industry-standard methodology for accuracy-constrained, scenario-based, system-level inference benchmarking — fixed models and datasets, a hardware result within 99% of reference accuracy, and defined serving scenarios (single-stream, server, offline) with measured system power. *How to use it*: the template for how a photonic accelerator *should* be benchmarked and, so far, mostly has not been.

---

## Tier II: Highly Recommended

### Reviews and Landscape

**Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114.**

The standard review of noise, precision, training strategies, and device options for analog photonic AI hardware. The best single entry point to the field's own account of its challenges.

**Wetzstein, G., Ozcan, A., Gigan, S., Fan, S., Englund, D., Soljačić, M., Denz, C., Miller, D.A.B., & Psaltis, D. (2020). "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39–47.**

Positions optical computing explicitly as inference-side acceleration within electronic systems — the role-assignment thesis of Section 25.3.3, argued by many of the field's leaders together.

### Landmark Demonstrations (Read With the Auditor's Checklist)

**Shen, Y., Harris, N.C., Skirlo, S., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11(7), 441–446.**

The 56-MZI programmable mesh that launched the modern field, and a model of honest reporting: hardware accuracy (76.7%) stated against the digital baseline (91.7%). The often-quoted femtojoule energies are explicitly forward-looking device projections, not system measurements — a distinction this chapter insists upon.

**Xu, X., Tan, M., Corcoran, B., et al. (2021). "11 TOPS photonic convolutional accelerator for optical neural networks." *Nature*, 589, 44–51.**

The microcomb, time-wavelength-interleaved convolution engine — the exemplar of WDM parallelism as a throughput multiplier (Section 25.3.2). The 11 TOPS figure is a genuine aggregate; note that it ran on laboratory instrumentation, so no meaningful TOPS/W attaches to it.

**Feldmann, J., Youngblood, N., Karpov, M., et al. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589, 52–58.**

Non-volatile phase-change weights held in the optical datapath: zero static hold power and zero weight fetch, demonstrated — the existence proof behind the favorable branch of the worked example.

**Ashtiani, F., Geers, A.J., & Aflatouni, F. (2022). "An on-chip photonic deep neural network for image classification." *Nature*, 606, 501–506.**

A complete photonic pipeline classifying images in under 570 ps end-to-end — the latency exemplar of Section 25.3.1 and a model of correct whole-task, batch-1, on-chip latency reporting.

**Tait, A.N., Ferreira de Lima, T., Zhou, E., et al. (2017). "Neuromorphic photonic networks using silicon photonic weight banks." *Scientific Reports*, 7, 7430.**

The broadcast-and-weight architecture that turns WDM channels into parallel neurons; the reference design for the WDM-parallelism arguments of Section 25.3.2.

**Sludds, A., Bandyopadhyay, S., Chen, Z., et al. (2022). "Delocalized photonic deep learning on the internet's edge." *Science*, 378, 270–276.**

Weights streamed optically to receiver-only edge clients — a genuinely new point in the design space (compute where the data is, weights in flight) and a fine system-boundary exercise.

### Programmability, Calibration, and Error

**Bogaerts, W., Pérez, D., Capmany, J., Miller, D.A.B., Poon, J., Englund, D., Morichetti, F., & Melloni, A. (2020). "Programmable photonic circuits." *Nature*, 586, 207–216.**

The control-plane, calibration, and self-configuration requirements of large programmable meshes — the electronic burden that co-design must budget for.

**Bandyopadhyay, S., Hamerly, R., & Englund, D. (2021). "Hardware error correction for programmable photonics." *Optica*, 8(10), 1247–1255.**

Quantifies how component errors accumulate with mesh depth and demonstrates correction strategies — the precision analog of the loss-scaling problem in Section 25.2.2.

### The Electronic Cousins

**Sebastian, A., Le Gallo, M., Khaddam-Aljameh, R., & Eleftheriou, E. (2020). "Memory devices and applications for in-memory computing." *Nature Nanotechnology*, 15, 529–544.**

The analog-electronic in-memory-computing landscape — the closest cousin of photonic accelerators, sharing the DAC/ADC tax, the ENOB ceiling, and the same history of core-only claims deflated by system measurement.

**Jouppi, N.P., et al. (2017). "In-datacenter performance analysis of a tensor processing unit." *ISCA 2017*, 1–12;** and **Jouppi, N.P., et al. (2023). "TPU v4: an optically reconfigurable supercomputer for machine learning..." *ISCA 2023*.**

The exemplary delivered-versus-peak roofline accounting for a real accelerator, and the production deployment of optical circuit switching — the control case for "photonics already ships, as interconnect."

---

## Tier III: Circuits, Converters, and Methodological Foundations

**Horowitz, M. (2014). "1.1 Computing's energy problem (and what we can do about it)." *ISSCC Digest of Technical Papers*, 10–14.**

The canonical energy-per-operation and memory-access numbers (picojoule SRAM, nanojoule DRAM) used throughout this chapter's budgets. If you cite one number about digital energy, cite it from here.

**Walden, R.H. (1999). "Analog-to-digital converter survey and analysis." *IEEE Journal on Selected Areas in Communications*, 17(4), 539–550.**

The original ADC survey: the ENOB/SNDR formalism and the jitter-limited SNR analysis that impose the effective-bit ceiling on any analog optical output.

**Murmann, B. "ADC Performance Survey 1997–2023" (online dataset, https://web.stanford.edu/~murmann/adcsurvey.html);** and **Murmann, B. (2015). "The race for the extra decibel." *IEEE Solid-State Circuits Magazine*, 7(3), 58–66.**

The continuously updated empirical envelope of ADC energy per conversion-step — the hard constraint behind the single most expensive element of the conversion chain.

**Williams, S., Waterman, A., & Patterson, D. (2009). "Roofline: an insightful visual performance model for multicore architectures." *Communications of the ACM*, 52(4), 65–76.**

The peak-versus-bandwidth-bound framework this chapter adapts as the "conversion roofline" for analog accelerators, and the basis for distinguishing peak from delivered throughput.

**Sze, V., Chen, Y.-H., Yang, T.-J., & Emer, J.S. (2017). "Efficient processing of deep neural networks: a tutorial and survey." *Proceedings of the IEEE*, 105(12), 2295–2329.**

The electronic-accelerator dataflow taxonomy — weight-stationary, output-stationary, and the memory-hierarchy reasoning — that photonic architectures inherit wholesale.

**Coppinger, F., Bhushan, A.S., & Jalali, B. (1999). "Photonic time stretch and its application to analog-to-digital conversion." *IEEE Transactions on Microwave Theory and Techniques*, 47(7), 1309–1314.**

The founding paper of photonics *rescuing* the ADC — the purest instance of co-design in Section 25.3.2.

**Marpaung, D., Yao, J., & Capmany, J. (2019). "Integrated microwave photonics." *Nature Photonics*, 13(2), 80–90.**

Filtering, beamforming, and channelization of RF signals directly on the optical carrier — the application domain where the correct benchmark is spur-free dynamic range and beam count, not TOPS/W.

**Ramey, C. (2020). "Silicon photonics for artificial intelligence acceleration." *IEEE Hot Chips 32 Symposium*.**

A representative commercial system disclosure — a photonic tensor core with its electronic control stack — and a productive target for the auditor's checklist of Section 25.2.3.

**Prucnal, P.R., & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press.**

The field's first textbook: RF and real-time processing applications of weight-bank photonic processors, and the conceptual foundation for the neuromorphic strand of this chapter.

---

*The references above, together with those cited at the end of each subsection (25.1.1–25.3.3), form a complete bibliography for Chapter 25. The unit introduction's Feynman epigraph — "reality must take precedence over public relations, for nature cannot be fooled" — is the one-line summary of everything on these shelves.*
