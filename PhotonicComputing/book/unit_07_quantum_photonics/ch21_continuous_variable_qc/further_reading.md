# Further Reading and References — Chapter 21: Continuous-Variable and Xanadu's Quantum Computing

## Tier I: Essential References

**Weedbrook, C., Pirandola, S., García-Patrón, R., Cerf, N.J., Ralph, T.C., Shapiro, J.H., & Lloyd, S. (2012). Gaussian quantum information. *Reviews of Modern Physics*, 84, 621–669.**

The standard reference for everything Gaussian: covariance matrices, symplectic transformations, Gaussian channels, Gaussian measurements, and Gaussian protocols (teleportation, cloning, CV-QKD). If you internalize one paper from this chapter, make it this one — the notation of Section 21.1.2 follows it closely, and it is the field's shared vocabulary.

*How to use it*: Sections II–IV for the formalism; Section VI for Gaussian channels (the loss model used throughout photonics); Section VIII for CV-QKD, which reappears in Chapter 22.

---

**Braunstein, S.L. & van Loock, P. (2005). Quantum information with continuous variables. *Reviews of Modern Physics*, 77, 513–577.**

The earlier of the two great CV reviews, broader in protocol coverage (teleportation, dense coding, entanglement swapping, error correction in CV) and closer to the experimental optics. Its treatment of CV teleportation and EPR entanglement complements Weedbrook et al.'s channel-theoretic emphasis.

---

**Gottesman, D., Kitaev, A., & Preskill, J. (2001). Encoding a qubit in an oscillator. *Physical Review A*, 64, 012310.**

The GKP paper. Remarkably readable for a work of such consequence: the code, its stabilizer structure, error-correction circuits, Clifford-gate Gaussianity, and even the symmetrized (hexagonal-lattice) variants are all here. Read it after Section 21.1.3 and be struck by how much of the 2020s photonic roadmap was laid out in 2001.

---

**Lloyd, S. & Braunstein, S.L. (1999). Quantum computation over continuous variables. *Physical Review Letters*, 82, 1784–1787.**

The founding document of CV quantum computing: quadratic Hamiltonians close under commutation (Gaussian world); one cubic generator unlocks arbitrary polynomial Hamiltonians (universality). Four pages, and the entire field's structure follows from its algebra.

---

**Madsen, L.S., et al. (2022). Quantum computational advantage with a programmable photonic processor. *Nature*, 606, 75–81.**

The Borealis paper: time-domain multiplexing, three delay loops, 216 squeezed modes, TES photon counting, and the 36 μs-versus-9,000-years advantage claim. Read alongside a loss-based classical rebuttal (e.g., Oh et al., *Nature Physics* 2024) to see how advantage claims are stress-tested.

---

## Tier II: Highly Recommended

### On the CV Formalism and Simulation

**Serafini, A. (2017). *Quantum Continuous Variables: A Primer of Theoretical Methods*. CRC Press.**
The graduate textbook for the symplectic toolbox — Williamson normal form, Gaussian channels, entanglement measures for Gaussian states — with exercises. The book-length companion to the two RMP reviews.

**Bartlett, S.D., Sanders, B.C., Braunstein, S.L., & Nemoto, K. (2002). Efficient classical simulation of continuous variable quantum information processes. *Physical Review Letters*, 88, 097904.**
The CV Gottesman-Knill theorem: the four-page proof that the all-Gaussian world is classical, and therefore the map of where CV quantum advantage cannot live.

### On Squeezing and Cluster States

**Vahlbruch, H., Mehmet, M., Danzmann, K., & Schnabel, R. (2016). Detection of 15 dB squeezed states of light... *Physical Review Letters*, 117, 110801.**
The squeezing world record, with a masterclass in loss budgeting; also a clever application of squeezing to calibrate photodiode quantum efficiency absolutely.

**Asavanant, W., et al. (2019). Generation of time-domain-multiplexed two-dimensional cluster state. *Science*, 366, 373–376; and Larsen, M.V., et al. (2019). Deterministic generation of a two-dimensional cluster state. *Science*, 366, 369–372.**
Back-to-back demonstrations of universal-topology CV cluster states with $\sim 10^4$–$10^6$ modes — the scalability argument for CV in experimental form.

### On GKP-Based Fault Tolerance

**Menicucci, N.C. (2014). Fault-tolerant measurement-based quantum computing with continuous-variable cluster states. *Physical Review Letters*, 112, 120504.**
First proof that finite squeezing suffices for fault tolerance (threshold 20.5 dB), converting "how much squeezing do we need?" from philosophy into engineering.

**Bourassa, J.E., et al. (2021). Blueprint for a scalable photonic fault-tolerant quantum computer. *Quantum*, 5, 392.**
Xanadu's full-stack architecture: multiplexed GKP factories, hybrid cluster states, passive linear optics, homodyne decoding. The reference design behind Section 21.2.1's roadmap discussion.

**Konno, S., et al. (2024). Logical states for fault-tolerant quantum computation with propagating light. *Science*, 383, 289–293.**
First GKP-like logical states in propagating optical modes — the missing hardware ingredient, demonstrated in embryo.

### On Software and Algorithms

**Killoran, N., Izaac, J., Quesada, N., Bergholm, V., Amy, M., & Weedbrook, C. (2019). Strawberry Fields: A software platform for photonic quantum computing. *Quantum*, 3, 129.**
The CV software stack paper; pairs with Bergholm et al., arXiv:1811.04968 (PennyLane) and Schuld et al., *Physical Review A*, 99, 032331 (2019) on parameter-shift gradients.

**Killoran, N., Bromley, T.R., Arrazola, J.M., Schuld, M., Quesada, N., & Lloyd, S. (2019). Continuous-variable quantum neural networks. *Physical Review Research*, 1, 033063.**
The CV-QNN architecture: Bloch-Messiah as the SVD of a weight matrix, non-Gaussian gates as activations.

**Hamilton, C.S., Kruse, R., Sansoni, L., Barkhofen, S., Silberhorn, C., & Jex, I. (2017). Gaussian boson sampling. *Physical Review Letters*, 119, 170501.**
Where GBS was defined and tied to the hafnian; the theoretical basis of both Jiuzhang and Borealis.

---

## Historical Note

CV quantum information began not with computing but with *EPR*: the 1935 Einstein-Podolsky-Rosen argument was formulated in continuous position-momentum variables, and the two-mode squeezed state is its laboratory realization. The through-line runs from EPR (1935) to squeezed-light generation (Slusher et al., 1985), CV teleportation (Furusawa et al., 1998), the CV computing proposal (Lloyd & Braunstein, 1999), GKP (2001), CV cluster states (Menicucci et al., 2006), the megamode cluster experiments (2013–2019), and Borealis (2022). It is one of the cleanest examples in physics of a foundational puzzle maturing into an engineering discipline.
