# Chapter 28: Further Reading and References

---

## Fundamental Limits

**Landauer, R. (1961). "Irreversibility and heat generation in the computing process." *IBM Journal of Research and Development*, 5, 183.**
The origin of the thermodynamic floor $k_B T \ln 2$ per erased bit — the limit Section 28.1.1 invokes only to show that engineering, not thermodynamics, is what binds photonic computing.

**Shannon, C.E. (1948). "A mathematical theory of communication." *Bell System Technical Journal*, 27, 379.**
The source of the capacity relation $B = \tfrac{1}{2}\log_2(1+\mathrm{SNR})$ behind the precision-energy trade-off of Section 28.1.2; every "bits cost photons" estimate descends from here.

**Caves, C.M. (1981). "Quantum-mechanical noise in an interferometer." *Physical Review D*, 23, 1693.**
The foundational analysis of the standard quantum limit and how squeezed light can beat it in one quadrature — the physics underlying Section 28.1.3 and the true cost of squeezing.

**Bérut, A., et al. (2012). "Experimental verification of Landauer's principle linking information and thermodynamics." *Nature*, 483, 187.**
The experiment that turned Landauer's principle from thought experiment into measured fact, confirming the reality of the floor discussed in Concept 1.

---

## Energy and Precision Accounting

**Miller, D.A.B. (2017). "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35, 346.**
The definitive statement of where device energy actually goes and why interconnect is photonics' natural role — the analytical basis of the interconnect-first thesis.

**Hamerly, R., Bernstein, L., Sludds, A., Soljačić, M., & Englund, D. (2019). "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9, 021032.**
The energy analysis that makes the $1/N$ amortization and the shot-noise-limited operating point concrete; the technical root of the converter-wall argument in Section 28.2.2.

**Nahmias, M.A., et al. (2020). "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26, 7701518.**
A careful survey of the energy, speed, and precision of photonic MAC schemes — the accounting framework Section 28.1.2 and 28.2.2 apply.

**Tait, A.N. (2022). "Quantifying power use in silicon photonic neural networks." *Physical Review Applied*, 17, 054029.**
An itemized power budget showing that lasers, conversion, and control dominate over the optical core — the empirical backbone of the converter-wall conclusion.

**Wang, T., et al. (2022). "An optical neural network using less than 1 photon per multiplication." *Nature Communications*, 13, 123.**
A working classifier operating below one photon per MAC, mapping the shot-noise floor of Section 28.1.3 onto a measured device.

---

## The Optical-Transistor and Memory Problems

**Miller, D.A.B. (2010). "Are optical transistors the logical next step?" *Nature Photonics*, 4, 3.**
The criteria a cascadable low-energy logic device must satisfy, and the argument that no all-optical switch meets them — the framing of the missing-transistor problem in Section 28.2.1.

**Nozaki, K., et al. (2010). "Sub-femtojoule all-optical switching using a photonic-crystal nanocavity." *Nature Photonics*, 4, 477.**
A landmark low-energy all-optical switch that nonetheless illustrates the cascadability and isolation gaps Miller's criteria expose.

**Ríos, C., et al. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9, 725.**
The demonstration that anchors Section 28.2.3: phase-change materials give genuine non-volatile, multi-level optical storage — and reveal its endurance and write-energy limits.

**Wuttig, M., Bhaskaran, H., & Taubner, T. (2017). "Phase-change materials for non-volatile photonic applications." *Nature Photonics*, 11, 465.**
The review of phase-change photonics that catalogs the physics behind the photonic-memory problem, including why endurance is the binding figure of merit.

**Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H., & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569, 208.**
Phase-change memory pressed into service as computation and on-chip learning — the ambitious edge of the memory candidate discussed in Section 28.2.3.

---

## Field Reviews and the Future

**McMahon, P.L. (2023). "The physics of optical computing." *Nature Reviews Physics*, 5, 717.**
The most disciplined statement of where optics can and cannot win; the single best companion to this chapter's analytical method.

**Shastri, B.J., et al. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15, 102.**
The standard map of the photonic-AI research base and the sober framing of what it can deliver, informing the outlook of Section 28.3.

**Marković, D., Mizrahi, A., Querlioz, D., & Grollier, J. (2020). "Physics for neuromorphic computing." *Nature Reviews Physics*, 2, 499.**
Places photonics within the wider physics-for-computing landscape, useful for locating optical approaches among competing analog paradigms.

**Berggren, K., et al. (2021). "Roadmap on emerging hardware and technology for machine learning." *Nanotechnology*, 32, 012002.**
A multi-author roadmap across device technologies for ML hardware — context for the heterogeneous-integration argument of Section 28.3.2.

**Bogaerts, W., et al. (2020). "Programmable photonic circuits." *Nature*, 586, 207.**
The review of general-purpose programmable meshes — the shared classical-quantum hardware platform of Section 28.3.1.

**Elshaari, A.W., Pernice, W., Srinivasan, K., Benson, O., & Zwiller, V. (2020). "Hybrid integrated quantum photonic circuits." *Nature Photonics*, 14, 285.**
The survey of hybrid quantum photonic integration that grounds the classical-quantum convergence and post-silicon-integration discussions.

**Moody, G., et al. (2022). "2022 Roadmap on integrated quantum photonics." *Journal of Physics: Photonics*, 4, 012501.**
The community roadmap for integrated quantum photonics; read alongside Bogaerts and Elshaari to see how classical and quantum platforms are converging.

**Wetzstein, G., et al. (2020). "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39.**
A cross-cutting perspective on optical inference — spanning free-space and integrated optics — useful for judging which workloads justify photonic compute.

---

## A Note on This Outlook's Shelf Life

The factual snapshot in this chapter is dated by construction: the energy figures, the state of phase-change endurance, the absence of a qualifying photonic transistor, and the industry's interconnect-first posture are all "as of roughly 2025," and some will be out of date before this book is old. What should not age is the method. End-to-end energy accounting, matched-baseline comparison, the separation of a physics result from a computing claim, and respect for the peer-review-to-press-release credibility gradient are tools, not facts — and they will let you evaluate photonic-computing claims that no one has made yet. Read the references above for what was known; keep the disciplines for what comes next.
