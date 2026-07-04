# 17.3.1 Field Quantization

## Strategy: Modes First, Quanta Second

Classical electromagnetism in a source-free region reduces, as shown in Chapter 1, to the wave equation, and any field in a bounded region can be expanded in a discrete set of **modes** — solutions of Maxwell's equations with the right boundary conditions, each with a definite spatial profile and frequency. For a cubic cavity of volume $V$ with periodic boundary conditions, the modes are plane waves labeled by wavevector $\mathbf{k}$ (with $\omega_k = c|\mathbf{k}|$) and polarization $\lambda \in \{1, 2\}$; for a photonic chip they are the guided modes of Unit III; for a laser cavity, the resonator modes of Chapter 4. The expansion turns Maxwell's field equations into an independent equation for each mode amplitude $\alpha_{\mathbf{k}\lambda}(t)$:

$$\ddot{\alpha}_{\mathbf{k}\lambda} + \omega_k^2\,\alpha_{\mathbf{k}\lambda} = 0$$

— a harmonic oscillator per mode, with the field energy $\frac{1}{2}\int (\varepsilon_0 E^2 + B^2/\mu_0)\,dV$ decomposing into a sum of oscillator energies. That is the entire physical content of field quantization: *the electromagnetic field is a collection of harmonic oscillators, one per mode.* Quantization then consists of applying Section 17.2 to each of them — promote each mode amplitude to an annihilation operator $\hat{a}_{\mathbf{k}\lambda}$ with

$$[\hat{a}_{\mathbf{k}\lambda},\, \hat{a}^\dagger_{\mathbf{k}'\lambda'}] = \delta_{\mathbf{k}\mathbf{k}'}\,\delta_{\lambda\lambda'}, \qquad [\hat{a}_{\mathbf{k}\lambda},\, \hat{a}_{\mathbf{k}'\lambda'}] = 0$$

The Hamiltonian and electric field operator become

$$\hat{H} = \sum_{\mathbf{k},\lambda} \hbar\omega_k\left(\hat{a}^\dagger_{\mathbf{k}\lambda}\hat{a}_{\mathbf{k}\lambda} + \frac{1}{2}\right)$$

$$\hat{\mathbf{E}}(\mathbf{r},t) = \sum_{\mathbf{k},\lambda} \mathcal{E}_k\left(\hat{a}_{\mathbf{k}\lambda}\, e^{i(\mathbf{k}\cdot\mathbf{r} - \omega_k t)} + \hat{a}^\dagger_{\mathbf{k}\lambda}\, e^{-i(\mathbf{k}\cdot\mathbf{r} - \omega_k t)}\right)\boldsymbol{\epsilon}_{\mathbf{k}\lambda}, \qquad \mathcal{E}_k = \sqrt{\frac{\hbar\omega_k}{2\varepsilon_0 V}}$$

(Heisenberg picture; $\boldsymbol{\epsilon}_{\mathbf{k}\lambda}$ are unit polarization vectors, $\boldsymbol{\epsilon}\cdot\mathbf{k} = 0$ enforcing transversality.) The classical complex amplitude of Chapters 1–2 has become the operator $\hat{a}$, exactly as the Heisenberg dynamics of 17.2.2 anticipated. The multimode state space is **Fock space**: a photon number for every mode, $|n_{\mathbf{k}_1\lambda_1}, n_{\mathbf{k}_2\lambda_2}, \ldots\rangle$.

## What a Photon Is (and Is Not)

In this construction a **photon** is one quantum of excitation of one mode: $\hat{a}^\dagger_{\mathbf{k}\lambda}|0\rangle$ is "one photon in mode $(\mathbf{k},\lambda)$." The photon inherits the mode's shape — it is as extended, as monochromatic, as polarized as the mode it excites. A photon is *not* a small ball traveling along a ray; localized single-photon wavepackets exist, but they are superpositions $\sum_k c_k\, \hat{a}_k^\dagger|0\rangle$ over many frequency modes, as spread out in time as their spectrum is narrow. This mode-first ontology resolves at a stroke the puzzles of Chapter 3's naive photon picture, and it has an engineering corollary that governs all of Chapters 18–20: *photons are only as identical as their modes are identical.* Two photons interfere as quanta (Hong-Ou-Mandel, Section 18.2.2) only to the degree that their spatial, spectral, temporal, and polarization mode functions overlap — indistinguishability is a mode-engineering problem, which is why source design (Chapter 19) obsesses over spectral purity.

**The scale of one photon's field** is set by $\mathcal{E}_k = \sqrt{\hbar\omega/2\varepsilon_0 V}$ — the "electric field per photon," which grows as the mode volume shrinks.

**Worked example.** For a 1550-nm mode ($\hbar\omega = 1.28\times10^{-19}$ J) confined in a high-Q silicon microresonator with $V = 1\ \mu\text{m}^3 = 10^{-18}\ \text{m}^3$:

$$\mathcal{E} = \sqrt{\frac{1.28\times 10^{-19}}{2 \times 8.85\times 10^{-12} \times 10^{-18}}} \approx 8.5\times 10^{4}\ \text{V/m}$$

Nearly $10^5$ V/m of field per photon — the reason nanophotonic cavities can push light-matter coupling to the single-quantum level (Chapter 19's cavity QED), and a design rule linking Unit III's mode-volume engineering directly to quantum physics.

## Vacuum Fluctuations

The multimode vacuum $|0\rangle$ (every mode empty) has $\langle 0|\hat{\mathbf{E}}|0\rangle = 0$ but

$$\langle 0|\hat{E}^2|0\rangle = \sum_{\mathbf{k},\lambda}\mathcal{E}_k^2 \;\neq\; 0$$

each mode contributing its zero-point variance — the $\hat{a}\hat{a}^\dagger$ ordering term that no normal-ordering convention can remove from a *squared* field. These **vacuum fluctuations** are not formal residue; they are the working explanation of measurable physics:

- **Spontaneous emission** is stimulated emission driven by vacuum fluctuations; engineering the local mode density changes the rate (the Purcell effect, Chapter 19).
- **Shot noise** in coherent detection is vacuum noise entering the unused port of the beam splitter or homodyne detector (Sections 18.2.1, 18.3.3) — the quantum noise floor of every analog photonic computer in Unit V.
- **Parametric fluorescence**: SPDC (Section 18.3.1) is literally the amplification of vacuum fluctuations in a nonlinear crystal; every entangled-photon-pair source is a vacuum amplifier.
- **Squeezing** (17.3.3) redistributes vacuum noise between quadratures — you cannot delete it, but you can herd it.

The total zero-point energy $\sum_k \hbar\omega_k/2$ diverges; for everything in this book only energy *differences* and fluctuation *correlations* matter, so the divergence is harmlessly subtracted (its one celebrated observable consequence, the Casimir force between conductors, arises from boundary-condition-dependent differences).

## Quadrature Operators

For a single mode (dropping labels), define the Hermitian **quadratures** — the quantum versions of the in-phase and in-quadrature amplitudes of Chapter 2's phasor decomposition:

$$\hat{X}_1 = \frac{\hat{a} + \hat{a}^\dagger}{2}, \qquad \hat{X}_2 = \frac{\hat{a} - \hat{a}^\dagger}{2i}, \qquad [\hat{X}_1, \hat{X}_2] = \frac{i}{2}$$

so the field of the mode is $\hat{E} \propto \hat{X}_1\cos\omega t + \hat{X}_2\sin\omega t$. The Robertson relation (17.1.2) gives

$$\Delta X_1\,\Delta X_2 \;\geq\; \frac{1}{4}$$

with the vacuum saturating it symmetrically: $\Delta X_1 = \Delta X_2 = \frac{1}{2}$. (Conventions differ across the literature — some authors use $\hat{x} = (\hat{a}+\hat{a}^\dagger)/\sqrt{2}$ with vacuum variance $1/2$; we use the $1/2$-amplitude convention above, matching the outline of this book, and will flag any formula where the choice matters.) Quadratures are what **homodyne detectors** measure (Section 18.3.2), and they are the continuous variables of Chapter 21: where discrete-variable photonics computes with photon numbers of modes, CV photonics computes with the $\hat{X}_1, \hat{X}_2$ phase plane of each mode. The two architectures of photonic quantum computing are two choices of basis in the same Fock space built on this page.
