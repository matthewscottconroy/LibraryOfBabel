# Chapter 20: Further Reading and References

---

## Reviews: Start Here

**Kok, P., Munro, W.J., Nemoto, K., Ralph, T.C., Dowling, J.P., & Milburn, G.J. (2007). "Linear optical quantum computing with photonic qubits." *Reviews of Modern Physics*, 79(1), 135–174.**
The canonical review of the whole subject: dual-rail encoding, the KLM protocol, the NS gate and nondeterministic CZ, gate teleportation, and cluster-state methods, written by several of the field's architects. The single best starting point for Sections 20.1–20.3, and still the reference the community reaches for.

---

## Primary Literature: LOQC and the KLM Protocol

**Knill, E., Laflamme, R., & Milburn, G.J. (2001). "A scheme for efficient quantum computation with linear optics." *Nature*, 409(6816), 46–52.**
The founding paper. Proves that linear optics, single-photon sources, and photon-counting detectors suffice for scalable universal quantum computation, via measurement-induced nonlinearity and teleportation-boosted gates. Dense but foundational reading for Section 20.2.

**Gottesman, D. & Chuang, I.L. (1999). "Demonstrating the viability of universal quantum computation using teleportation and single-qubit operations." *Nature*, 402(6760), 390–393.**
The gate-teleportation primitive KLM borrowed: apply a gate to the entangled resource before teleporting, and the data emerges gated. The conceptual seed of both Section 20.2.3 and all of measurement-based computing.

**Knill, E. (2005). "Quantum computing with realistically noisy devices." *Nature*, 434(7029), 39–44.**
Fault-tolerance thresholds for noisy gates, including the postselection-and-teleportation architecture that made LOQC's error budget look survivable — essential background to Section 20.5.

**Reck, M., Zeilinger, A., Bernstein, H.J., & Bertani, P. (1994). "Experimental realization of any discrete unitary operator." *Physical Review Letters*, 73(1), 58–61.**
The triangular decomposition of an arbitrary $N$-mode unitary into $N(N-1)/2$ beam splitters and phase shifters — the recipe for building the interferometers of Sections 20.1 and 20.4.

**Clements, W.R., Humphreys, P.C., Metcalf, B.J., Kolthammer, W.S., & Walmsley, I.A. (2016). "Optimal design for universal multiport interferometers." *Optica*, 3(12), 1460–1465.**
The rectangular mesh that halves the optical depth of Reck's design, lowering loss — the layout used in modern integrated boson samplers and photonic processors.

---

## Primary Literature: Cluster States and Measurement-Based Computing

**Raussendorf, R. & Briegel, H.J. (2001). "A one-way quantum computer." *Physical Review Letters*, 86(22), 5188–5191.**
The paper that introduced computation by adaptive single-qubit measurement on a cluster state — the model of Section 20.3.2.

**Briegel, H.J. & Raussendorf, R. (2001). "Persistent entanglement in arrays of interacting particles." *Physical Review Letters*, 86(5), 910–913.**
Defines cluster states and their entanglement properties; the structural companion to the one-way-computer paper (Section 20.3.1).

**Raussendorf, R., Browne, D.E., & Briegel, H.J. (2003). "Measurement-based quantum computation on cluster states." *Physical Review A*, 68(2), 022312.**
The complete formalism: measurement patterns, byproduct operators, feed-forward, and universality proofs. The reference derivation behind Section 20.3.2's rotation gadget.

**Nielsen, M.A. (2004). "Optical quantum computation using cluster states." *Physical Review Letters*, 93(4), 040503.**
Marries KLM gates to the cluster-state model and cuts the resource overhead by more than an order of magnitude — the hinge between Sections 20.2 and 20.3.

**Browne, D.E. & Rudolph, T. (2005). "Resource-efficient linear optical quantum computation." *Physical Review Letters*, 95(1), 010501.**
Introduces the type-I and type-II fusion gates for building photonic cluster states from small pieces — the operational core of Sections 20.3.1 and 20.3.3.

**Kieling, K., Rudolph, T., & Eisert, J. (2007). "Percolation, renormalization, and quantum computing with nondeterministic gates." *Physical Review Letters*, 99(13), 130501.**
Recasts cluster growth with probabilistic fusions as a percolation problem, and shows renormalization recovers a perfect lattice above threshold — the theory behind Section 20.3.3.

**Gimeno-Segovia, M., Shadbolt, P., Browne, D.E., & Rudolph, T. (2015). "From three-photon GHZ states to ballistic universal quantum computation." *Physical Review Letters*, 115(2), 020502.**
Shows that tiny (three-photon) resource states and boosted fusion percolate into a universal cluster with no feed-forward — the ballistic scheme underlying the FBQC roadmap.

**Bartolucci, S., Birchall, P., Bombin, H., Cable, H., Dawson, C., Gimeno-Segovia, M., Johnston, E., Kieling, K., Nickerson, N., Pant, M., Pastawski, F., Rudolph, T., & Sparrow, C. (2023). "Fusion-based quantum computation." *Nature Communications*, 14, 912.**
The definitive statement of FBQC: constant-size resource states, fusion networks, and fault tolerance folded into the fusion pattern. The architectural blueprint of Sections 20.3.3 and 20.5.3.

---

## Primary Literature: Boson Sampling

**Valiant, L.G. (1979). "The complexity of computing the permanent." *Theoretical Computer Science*, 8(2), 189–201.**
Proves the permanent is #P-hard — the complexity-theoretic bedrock on which the entire boson-sampling hardness argument stands (Section 20.4.1).

**Aaronson, S. & Arkhipov, A. (2011). "The computational complexity of linear optics." *Proceedings of the 43rd ACM Symposium on Theory of Computing (STOC)*, 333–342.** (Journal version: *Theory of Computing*, 9, 143–252, 2013.)
The paper that launched the field: exact and approximate hardness of boson sampling, the polynomial-hierarchy-collapse argument, and the permanent-of-Gaussians and anti-concentration conjectures. Long, but the source for Section 20.4.1.

**Hamilton, C.S., Kruse, R., Sansoni, L., Barkhofen, S., Silberhorn, C., & Jex, I. (2017). "Gaussian boson sampling." *Physical Review Letters*, 119(17), 170501.**
Introduces GBS — squeezed-state inputs and Hafnian statistics — the experimentally friendlier variant of Section 20.4.2 that all the large demonstrations adopted.

**Broome, M.A., Fedrizzi, A., Rahimi-Keshari, S., Dove, J., Aaronson, S., Ralph, T.C., & White, A.G. (2013). "Photonic boson sampling in a tunable circuit." *Science*, 339(6121), 794–798.**
**Spring, J.B., Metcalf, B.J., Humphreys, P.C., et al. (2013). "Boson sampling on a photonic chip." *Science*, 339(6121), 798–801.**
**Tillmann, M., Dakić, B., Heilmann, R., Nolte, S., Szameit, A., & Walther, P. (2013). "Experimental boson sampling." *Nature Photonics*, 7(7), 540–544.**
**Crespi, A., Osellame, R., Ramponi, R., et al. (2013). "Integrated multimode interferometers with arbitrary designs for photonic boson sampling." *Nature Photonics*, 7(7), 545–549.**
The four 2013 proof-of-principle demonstrations (three and four photons) that established boson sampling in the laboratory — the opening of Section 20.4.3.

**Zhong, H.-S., Wang, H., Deng, Y.-H., et al. (2020). "Quantum computational advantage using photons." *Science*, 370(6523), 1460–1463.**
Jiuzhang 1.0: up to 76 detected photons in 100-mode GBS, and the first photonic claim of quantum computational advantage.

**Zhong, H.-S., Deng, Y.-H., Qin, J., et al. (2021). "Phase-programmable Gaussian boson sampling using stimulated squeezed light." *Physical Review Letters*, 127(18), 180502.**
Jiuzhang 2.0: 113 photons across 144 modes, with programmable phases.

**Madsen, L.S., Laudenbach, F., Askarani, M.F., et al. (2022). "Quantum computational advantage with a programmable photonic processor." *Nature*, 606(7912), 75–81.**
Xanadu's Borealis: a time-multiplexed, fully programmable 216-mode GBS machine — the scalable, reconfigurable counterpoint to the Jiuzhang bulk-optics approach.

---

## Primary Literature: Error Correction and Architecture

**Dennis, E., Kitaev, A., Landahl, A., & Preskill, J. (2002). "Topological quantum memory." *Journal of Mathematical Physics*, 43(9), 4452–4505.**
The foundational analysis of the surface/toric code as a memory, including the mapping of error correction to statistical-mechanics phase transitions that fixes the threshold — background for Section 20.5.2.

**Varnava, M., Browne, D.E., & Rudolph, T. (2006). "Loss tolerance in one-way quantum computation via counterfactual error correction." *Physical Review Letters*, 97(12), 120501.**
Tree-cluster encodings that infer the state of a lost photon from its neighbours, tolerating loss rates approaching the 50% erasure ceiling — the loss-tolerance engine of Section 20.5.2.

**Raussendorf, R., Harrington, J., & Goyal, K. (2007). "Topological fault-tolerance in cluster state quantum computation." *New Journal of Physics*, 9(6), 199.**
The RHG lattice: a 3D cluster state whose measurement implements the surface code in $(2{+}1)$ dimensions — the foliation idea central to photonic fault tolerance.

**Bombin, H., Dawson, C., Mishmash, R.V., Nickerson, N., Pastawski, F., & Roberts, S. (2021). "Interleaving: Modular architectures for fault-tolerant photonic quantum computing." *arXiv:2103.08612*.**
Time-multiplexing the fault-tolerant lattice through a small hardware bank using fiber delays — the space-for-time trade that makes million-qubit photonic machines a throughput problem (Section 20.5.3).

---

## Textbooks

**Kok, P. & Lovett, B.W. (2010). *Introduction to Optical Quantum Information Processing*. Cambridge University Press.**
The dedicated textbook for this chapter's material: linear optics, KLM, cluster states, and photonic error correction, developed from first principles with worked detail.

**Nielsen, M.A. & Chuang, I.L. (2010). *Quantum Computation and Quantum Information* (10th Anniversary Edition). Cambridge University Press.**
The standard reference for the quantum-information background — stabilizers, teleportation, the circuit model, and error-correction fundamentals — assumed throughout the chapter.
