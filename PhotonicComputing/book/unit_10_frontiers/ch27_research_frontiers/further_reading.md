# Chapter 27: Further Reading and References

---

## Field Reviews and Cross-Cutting Perspectives

**McMahon, P.L. (2023). "The physics of optical computing." *Nature Reviews Physics*, 5, 717–734.**
The single most important reference for this chapter: a rigorous account of what optical computing can and cannot win, and the source of much of the evaluative discipline applied throughout.

**Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15, 102–114.**
The standard field review, co-authored across the leading groups; the best map of the research base underneath every architecture in this chapter.

**Wetzstein, G., Ozcan, A., Gigan, S., Fan, S., Englund, D., Soljačić, M., Denz, C., Miller, D.A.B., & Psaltis, D. (2020). "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39–47.**
A perspective on optics for machine-learning inference, written by many of the scientists profiled in this chapter; the natural companion to the free-space and diffractive sections.

**Bogaerts, W., Pérez, D., Capmany, J., Miller, D.A.B., Poon, J., Englund, D., Morichetti, F., & Melloni, A. (2020). "Programmable photonic circuits." *Nature*, 586, 207–216.**
The reference for the cross-cutting programmable-photonics frontier — the MZI mesh generalized into a field-programmable photonic gate array.

---

## Photonic Ising Machines

**Lucas, A. (2014). "Ising formulations of many NP problems." *Frontiers in Physics*, 2, 5.**
The catalog of reductions that makes the Ising problem a *lingua franca* for combinatorial optimization; consult it before claiming any problem "maps to Ising for free."

**Marandi, A., Wang, Z., Takata, K., Byer, R.L., & Yamamoto, Y. (2014). "Network of time-multiplexed optical parametric oscillators as a coherent Ising machine." *Nature Photonics*, 8, 937–942.**
The founding demonstration of the measurement-feedback coherent Ising machine, and the origin of the OPO pulse as a binary spin.

**McMahon, P.L., et al. (2016). "A fully programmable 100-spin coherent Ising machine with all-to-all connections." *Science*, 354, 614–617.**
The CIM as a reprogrammable device with dense connectivity; read alongside the Inagaki paper below as the pair of 2016 milestones.

**Inagaki, T., et al. (2016). "A coherent Ising machine for 2000-node optimization problems." *Science*, 354, 603–606.**
The large-scale time-multiplexed CIM demonstrated on 2000-node instances — a scaling companion to McMahon et al.

**Honjo, T., et al. (2021). "100,000-spin coherent Ising machine." *Science Advances*, 7, eabh0952.**
The largest CIM to date; a useful specimen for Concept 2's argument that the machine's core is an electronic linear-algebra engine.

**Goto, H., Tatsumura, K., & Dixon, A.R. (2019). "Combinatorial optimization by simulating adiabatic bifurcations in nonlinear Hamiltonian systems." *Science Advances*, 5, eaav2372.**
Simulated bifurcation — the classical algorithm, inspired by the CIM's own dynamics, that every photonic Ising claim must now outrun.

**Pierangeli, D., Marcucci, G., & Conti, C. (2019). "Large-scale photonic Ising machine by spatial light modulation." *Physical Review Letters*, 122, 213902.**
A spatial-light-modulator route to a photonic Ising machine, connecting the Ising and free-space frontiers.

**Mohseni, N., McMahon, P.L., & Byrnes, T. (2022). "Ising machines as hardware solvers of combinatorial optimization problems." *Nature Reviews Physics*, 4, 363–379.**
The standard comparative survey across photonic, quantum, and digital Ising solvers; its sober theme is that competent classical baselines are hard to beat.

---

## Integrated Frequency Combs

**Herr, T., et al. (2014). "Temporal solitons in optical microresonators." *Nature Photonics*, 8, 145–152.**
The observation of temporal (dissipative Kerr) solitons in a microresonator — the experimental foundation of the usable microcomb.

**Kippenberg, T.J., Gaeta, A.L., Lipson, M., & Gorodetsky, M.L. (2018). "Dissipative Kerr solitons in optical microresonators." *Science*, 361, eaan8083.**
The definitive account of the dissipative Kerr soliton; the physics behind the chip-scale laser-bank substitution of Section 27.2.2.

**Gaeta, A.L., Lipson, M., & Kippenberg, T.J. (2019). "Photonic-chip-based frequency combs." *Nature Photonics*, 13, 158–169.**
The standard review of integrated combs — platforms, dispersion engineering, and applications.

**Xu, X., et al. (2021). "11 TOPS photonic convolutional accelerator for optical neural networks." *Nature*, 589, 44–51.**
A microcomb-driven convolutional accelerator: the comb-as-WDM-source thesis made concrete, and a good target for an end-to-end energy audit.

**Feldmann, J., et al. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589, 52–58.**
The companion phase-change photonic tensor core, combining a microcomb source with in-memory optical weights.

**Yuan, L., Lin, Q., Xiao, M., & Fan, S. (2018). "Synthetic dimension in photonics." *Optica*, 5, 1396–1405.**
The review that defined the synthetic-dimension program — computing and simulating *in* the frequency domain rather than merely across it.

**Dutt, A., et al. (2020). "A single photonic cavity with two independent physical synthetic dimensions." *Science*, 367, 59–64.**
The experimental realization of multiple synthetic dimensions in one cavity; the concrete counterpart to the Yuan et al. framework.

---

## Free-Space and Diffractive Computing

**Lin, X., et al. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361, 1004–1008.**
The diffractive deep neural network (D²NN) — the canonical fixed-function free-space computer and the reference for the fixed-versus-reconfigurable trade.

**Zhou, T., et al. (2021). "Large-scale neuromorphic optoelectronic computing with a reconfigurable diffractive processing unit." *Nature Photonics*, 15, 367–373.**
The reconfigurable counterpart to the D²NN, trading speed and energy for programmability.

**Rafayelyan, M., Dong, J., Tan, Y., Krzakala, F., & Gigan, S. (2020). "Large-scale optical reservoir computing for spatiotemporal chaotic systems prediction." *Physical Review X*, 10, 041037.**
Scattering-medium reservoir computing at large scale — optical "disorder" repurposed as a high-dimensional random projection.

**Wang, T., et al. (2022). "An optical neural network using less than 1 photon per multiplication." *Nature Communications*, 13, 123.**
A striking demonstration of optical inference at the shot-noise frontier; read with Chapter 25 for what the sub-photon figure does and does not include.

**Chang, J., Sitzmann, V., Dun, X., Heidrich, W., & Wetzstein, G. (2018). "Hybrid optical-electronic convolutional neural networks with optimized diffractive optics for image classification." *Scientific Reports*, 8, 12324.**
An early hybrid optical-electronic imaging classifier; a clear statement of the co-designed-optics-plus-digital-backend pattern.

---

## Topological and Non-Hermitian Photonics

**Lu, L., Joannopoulos, J.D., & Soljačić, M. (2014). "Topological photonics." *Nature Photonics*, 8, 821–829.**
The founding review of the field; the place to begin on band topology in photonic systems.

**Ozawa, T., et al. (2019). "Topological photonics." *Reviews of Modern Physics*, 91, 015006.**
The comprehensive, book-length review; the reference for the full taxonomy of topological photonic phases.

**Wang, Z., Chong, Y., Joannopoulos, J.D., & Soljačić, M. (2009). "Observation of unidirectional backscattering-immune topological electromagnetic states." *Nature*, 461, 772–775.**
The first photonic quantum-Hall analog — magneto-optic edge states — and the experimental root of topological protection in photonics.

**Rechtsman, M.C., et al. (2013). "Photonic Floquet topological insulators." *Nature*, 496, 196–200.**
Topological edge transport in an all-dielectric, magnetic-field-free system of helical waveguides.

**Bandres, M.A., et al. (2018). "Topological insulator laser: experiments." *Science*, 359, eaar4005.**
A topologically protected edge mode enforcing single-mode lasing — the clearest example of a topological principle yielding a device advantage.

**El-Ganainy, R., Makris, K.G., Khajavikhan, M., Musslimani, Z.H., Rotter, S., & Christodoulides, D.N. (2018). "Non-Hermitian physics and PT symmetry." *Nature Physics*, 14, 11–19.**
The standard introduction to gain-loss (parity-time) photonics and its exceptional points.

**Miri, M.-A., & Alù, A. (2019). "Exceptional points in optics and photonics." *Science*, 363, eaar7709.**
A focused review of exceptional-point physics and its proposed applications, including the sensing-enhancement debate flagged in Concept 9.

---

## 2D-Material Photonics

**Bonaccorso, F., Sun, Z., Hasan, T., & Ferrari, A.C. (2010). "Graphene photonics and optoelectronics." *Nature Photonics*, 4, 611–622.**
The agenda-setting review of graphene photonics; the origin of the broadband-absorption and gate-tunability arguments of Section 27.5.1.

**Liu, M., Yin, X., Ulin-Avila, E., Geng, B., Zentgraf, T., Ju, L., Wang, F., & Zhang, X. (2011). "A graphene-based broadband optical modulator." *Nature*, 474, 64–67.**
The first waveguide-integrated graphene electro-absorption modulator.

**Xia, F., Mueller, T., Lin, Y., Valdes-Garcia, A., & Avouris, P. (2009). "Ultrafast graphene photodetector." *Nature Nanotechnology*, 4, 839–843.**
The demonstration that established graphene's very high intrinsic detection bandwidth.

**Koppens, F.H.L., Mueller, T., Avouris, P., Ferrari, A.C., Vitiello, M.S., & Polini, M. (2014). "Photodetectors based on graphene, other two-dimensional materials and hybrid systems." *Nature Nanotechnology*, 9, 780–793.**
The comprehensive map of 2D-material detection mechanisms — photoconductive, photovoltaic, photothermoelectric, and bolometric.

**Romagnoli, M., Sorianello, V., Midrio, M., Koppens, F.H.L., Huyghebaert, C., Neumaier, D., Galli, P., Templ, W., D'Errico, A., & Ferrari, A.C. (2018). "Graphene-based integrated photonics for next-generation datacom and telecom." *Nature Reviews Materials*, 3, 392–414.**
The integration roadmap: transferring CVD graphene onto silicon and silicon-nitride photonics as an added active layer.

**Sun, Z., Martinez, A., & Wang, F. (2016). "Optical modulators with 2D layered materials." *Nature Photonics*, 10, 227–238.**
A comparative review of modulators built from graphene, TMDs, and black phosphorus — the interface case across the 2D-material family.

**Mak, K.F., & Shan, J. (2016). "Photonics and optoelectronics of 2D semiconductor transition metal dichalcogenides." *Nature Photonics*, 10, 216–226.**
The standard review of TMD optics, including excitons, valley physics, and emission.

**Mak, K.F., Lee, C., Hone, J., Shan, J., & Heinz, T.F. (2010). "Atomically thin MoS₂: a new direct-gap semiconductor." *Physical Review Letters*, 105, 136805.**
The discovery of the indirect-to-direct bandgap crossover in the monolayer limit.

**Wang, Q.H., Kalantar-Zadeh, K., Kis, A., Coleman, J.N., & Strano, M.S. (2012). "Electronics and optoelectronics of two-dimensional transition metal dichalcogenides." *Nature Nanotechnology*, 7, 699–712.**
The foundational review of the TMD family as electronic and optoelectronic materials.

**Wang, G., Chernikov, A., Glazov, M.M., Heinz, T.F., Marie, X., Amand, T., & Urbaszek, B. (2018). "Colloquium: Excitons in atomically thin transition metal dichalcogenides." *Reviews of Modern Physics*, 90, 021001.**
The reference on TMD excitons and valley physics — the many-body optics behind Section 27.5.2.

**Geim, A.K., & Grigorieva, I.V. (2013). "Van der Waals heterostructures." *Nature*, 499, 419–425.**
The manifesto for building designer materials by stacking 2D layers — the "atomic Lego" program.

**Novoselov, K.S., Mishchenko, A., Carvalho, A., & Castro Neto, A.H. (2016). "2D materials and van der Waals heterostructures." *Science*, 353, aac9439.**
The survey of the heterostructure field as it matured into a general assembly method.

---

## A Note on Reading the Frontier

More than any other chapter in this book, this one describes work still in motion, and its literature must be read with two disciplines held firmly in mind. First, respect the credibility gradient: a peer-reviewed journal paper reporting a measured device is a different object from a conference abstract, a preprint, or a press release announcing a record — and the frontier is where the temptation to blur them is greatest. Second, and more subtly, separate the *physics result* from the *computing claim*. A great many results cited above are superb physics whose computational advantage over a competent classical baseline is not yet established; both facts can be stated in the same sentence without embarrassment. When you read a new frontier paper, ask the three questions this chapter has practiced throughout — what is the baseline, is the physics result the same as the computing claim, and where do the interfaces set the true cost — and you will read it better than most of its press coverage.
