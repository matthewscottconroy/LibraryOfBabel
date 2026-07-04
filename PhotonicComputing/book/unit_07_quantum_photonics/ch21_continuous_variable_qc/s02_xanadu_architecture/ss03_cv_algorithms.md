# 21.2.3 — Quantum Machine Learning and CV Algorithms

## What Do You Run on a Squeezed-Light Machine?

A CV processor's native operations — Gaussian transformations plus photon counting — do not look like the Shor/Grover gate model. The algorithm families proposed for near-term CV hardware instead exploit exactly what the hardware produces: samples from hafnian-weighted distributions (GBS), and trainable phase-space transformations (CV neural networks and kernels).

## Gaussian Boson Sampling as a Subroutine

GBS is more than a benchmark; because the sampling distribution is governed by matrix hafnians, any problem whose structure maps onto hafnians can be attacked by sampling:

- **Graph problems.** Encode a graph's adjacency matrix $A$ into the Gaussian state (via its doubled form; the mapping exists whenever $A$ is symmetric). The probability of detecting a photon pattern that "marks" a subgraph scales with the *hafnian of the corresponding submatrix* — and $\text{haf}(A_S)$ counts the perfect matchings of subgraph $S$. Dense subgraphs have many matchings, so GBS samples land preferentially on dense regions: a stochastic proposal generator for **dense-$k$-subgraph** and **maximum clique** heuristics (Arrazola & Bromley, 2018 [1]). Related mappings give graph-similarity kernels and point processes with repulsive/clustered statistics.
- **Molecular vibronic spectra.** Huh et al. showed that the Franck-Condon profile of a molecule's electronic transition — a sum over vibrational mode overlaps that is classically expensive — equals the output photon distribution of a GBS device programmed with the molecule's Duschinsky rotation and displacement parameters [2]. This remains the most physically natural GBS "application": the device *is* an analog simulator of the molecule's oscillator physics.

The honest caveat, continuing Section 21.2.1: these are *heuristics*, and for each of them, classical "GBS-inspired" samplers that mimic the low-order correlations of GBS output often perform comparably on the downstream task. No end-to-end practical advantage from GBS applications has been established; what is established is only the hardness of exact sampling itself.

## CV Quantum Neural Networks

The CV platform admits an unusually literal quantum generalization of a neural network (Killoran et al., 2019 [3]). A classical layer computes $\mathbf{y} = \varphi(W\mathbf{x} + \mathbf{b})$: affine map, then nonlinearity. Define the CV quantum layer as

$$\mathcal{L} = \Phi \circ D(\boldsymbol{\alpha}) \circ U_2 \circ S(\mathbf{r}) \circ U_1$$

where $U_1, U_2$ are interferometers (orthogonal/symplectic transformations), $S(\mathbf{r})$ is a bank of single-mode squeezers (the singular values), $D(\boldsymbol{\alpha})$ displacements (the bias), and $\Phi$ a fixed non-Gaussian gate per mode (Kerr or cubic phase — the activation function). By the Bloch-Messiah decomposition (Section 21.1.2), the Gaussian part $D \circ U_2 \circ S \circ U_1$ enacts precisely an arbitrary *affine transformation* on the quadrature vector, with $U_2\,\text{diag}(e^{\pm r})\,U_1$ playing the role of the SVD of the weight matrix $W$. A CV-QNN is therefore a neural network whose activations are quantum states of light: it inherits the photonic matrix-multiplication advantages of Unit V *and* can, in principle, create entanglement and interference inaccessible to its classical counterpart. Stacked layers are trained end to end with parameter-shift gradients in PennyLane; the same architecture specializes to quantum generative adversarial networks — with quantum generators trained against classical or quantum discriminators [4, 5] — and to quantum autoencoders and classifiers.

Two structural cautions. First, *remove the non-Gaussian activation $\Phi$ and the entire network is Gaussian*, hence classically simulable by covariance bookkeeping — a CV-QNN without its Kerr layers is just a noisy linear-optical neural network. The quantum expressivity lives entirely in $\Phi$, which is also the hardest gate to realize (current hardware substitutes measurement-based non-Gaussianity). Second, variational quantum models face generic trainability hazards — barren plateaus, and "dequantization" results showing many quantum ML models can be matched by classical surrogates — so claims of learning advantage should be treated as research hypotheses, not established results.

## Quantum Kernel Methods

Rather than training a deep quantum circuit, kernel methods use the quantum device only to *evaluate inner products in a quantum feature space* (Schuld & Killoran, 2019 [6]). Encode a classical datum $x$ into a quantum state $|\phi(x)\rangle$ — in CV, for example, by driving displacements and squeezers with $x$ — and define the kernel

$$k(x, x') = \left|\langle \phi(x) | \phi(x') \rangle\right|^2$$

estimated on hardware by preparing $|\phi(x)\rangle$, applying the inverse encoding of $x'$, and measuring the vacuum-return probability. A classical support-vector machine then works with this kernel. The approach is attractive because it needs shallow circuits and inherits SVM convergence theory; it is powerful only if the quantum kernel is *both* hard to compute classically *and* well matched to the data — a pair of conditions that has proven hard to satisfy simultaneously on natural datasets. CV feature maps built from Gaussian encodings alone yield classically computable kernels (the simulability theorem again); non-Gaussian encodings evade it at the cost of hardware difficulty.

## CV versus DV: The Tradeoff Ledger

This chapter and Chapter 20 describe two photonic routes to quantum computing. Their complementary strengths:

| Aspect | CV (squeezed light) | DV (single photons) |
|---|---|---|
| State generation | Deterministic (parametric squeezer) | Probabilistic/heralded sources (Ch. 19) |
| Entangling operations | Deterministic Gaussian (beam splitters on squeezed light) | Probabilistic fusion/KLM gates, needs multiplexing |
| Measurement | Homodyne: $>99\%$ efficient, GHz, room temperature | SNSPD/PNR: $\sim 98\%$, cryogenic |
| Demonstrated entanglement scale | $10^6$ modes (1D cluster), $\sim 10^5$ (2D) | tens of photons |
| Error character | Continuous Gaussian noise from finite squeezing; *every* operation slightly imperfect | Discrete photon loss/erasure; heralded operations either succeed cleanly or flag failure |
| Error correction route | GKP-in-oscillator; squeezing threshold of order 10 dB | Qubit codes over dual-rail/fusion networks; loss thresholds of order 1–10% per component |
| Non-Gaussian / non-Clifford resource | GKP or cubic-phase state preparation (hard, probabilistic) | Single photons themselves are non-Gaussian; magic states via distillation |
| Classical simulability trap | All-Gaussian circuits | Clifford/linear-optics-only circuits |

The symmetry of the last rows is the deep point: **each platform is cheap exactly where the other is expensive.** CV gets deterministic entanglement and near-perfect measurement but pays with analog noise in every operation and a brutally hard non-Gaussian state factory; DV gets clean, heralded discrete errors but pays with probabilistic everything. Both converge, at the fault-tolerant end, on hybrid designs: Xanadu's GKP qubits carried by CV cluster states are decoded like DV surface codes, while PsiQuantum's fusion networks borrow multiplexing tricks pioneered in time-domain CV optics. The likely future is not "CV versus DV" but bosonic codes and discrete codes composed — oscillators supplying hardware-efficient error suppression at the bottom, qubit logic on top.

## Summary

- GBS applications map hafnian structure onto problems: dense subgraph/max clique heuristics, graph kernels, and molecular vibronic spectra — promising physics, but no established end-to-end practical advantage; classical GBS-inspired samplers are close competitors.
- CV-QNNs realize affine-plus-nonlinearity layers natively in phase space (Bloch-Messiah = SVD of the weight matrix); all quantum expressivity resides in the non-Gaussian activation.
- Quantum kernel methods need only shallow encodings but require kernels that are simultaneously classically hard and data-relevant; Gaussian-only encodings are classically computable.
- CV vs DV: deterministic resources and ideal detection versus heralded discreteness — complementary cost structures converging on hybrid bosonic-qubit architectures.

---

*References*

[1] Arrazola, J.M. & Bromley, T.R. (2018). Using Gaussian boson sampling to find dense subgraphs. *Physical Review Letters*, 121(3), 030503. [DOI: 10.1103/PhysRevLett.121.030503]

[2] Huh, J., Guerreschi, G.G., Peropadre, B., McClean, J.R., & Aspuru-Guzik, A. (2015). Boson sampling for molecular vibronic spectra. *Nature Photonics*, 9(9), 615–620. [DOI: 10.1038/nphoton.2015.153]

[3] Killoran, N., Bromley, T.R., Arrazola, J.M., Schuld, M., Quesada, N., & Lloyd, S. (2019). Continuous-variable quantum neural networks. *Physical Review Research*, 1(3), 033063. [DOI: 10.1103/PhysRevResearch.1.033063]

[4] Lloyd, S. & Weedbrook, C. (2018). Quantum generative adversarial learning. *Physical Review Letters*, 121(4), 040502. [DOI: 10.1103/PhysRevLett.121.040502]

[5] Dallaire-Demers, P.-L. & Killoran, N. (2018). Quantum generative adversarial networks. *Physical Review A*, 98(1), 012324. [DOI: 10.1103/PhysRevA.98.012324]

[6] Schuld, M. & Killoran, N. (2019). Quantum machine learning in feature Hilbert spaces. *Physical Review Letters*, 122(4), 040504. [DOI: 10.1103/PhysRevLett.122.040504]

[7] Bromley, T.R., et al. (2020). Applications of near-term photonic quantum computers: software and algorithms. *Quantum Science and Technology*, 5(3), 034010. [DOI: 10.1088/2058-9565/ab8504] [Survey of GBS application algorithms in Strawberry Fields.]
