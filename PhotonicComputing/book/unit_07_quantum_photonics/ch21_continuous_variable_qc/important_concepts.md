# Important Concepts — Chapter 21: Continuous-Variable and Xanadu's Quantum Computing

## Quadrature Operators and Vacuum Noise

$$\hat{x} = \frac{\hat{a}+\hat{a}^\dagger}{\sqrt{2}}, \qquad \hat{p} = \frac{\hat{a}-\hat{a}^\dagger}{i\sqrt{2}}, \qquad [\hat{x},\hat{p}] = i$$

Vacuum (shot-noise) variance: $(\Delta x)^2 = (\Delta p)^2 = 1/2$, saturating $\Delta x\,\Delta p \geq 1/2$. Quadratures are field amplitudes relative to a local-oscillator phase; conventions for the vacuum variance ($1/2$, $1$, or $1/4$) vary across the literature — always check.

## Squeezed States

$\hat{S}(r)$: $\Delta x \to e^{-r}/\sqrt{2}$, $\Delta p \to e^{+r}/\sqrt{2}$; generated deterministically by parametric down-conversion. Decibel measure $S_\text{dB} \approx 8.69\,r$; record: **15 dB** (Vahlbruch et al., 2016). Loss $\eta_\text{loss}$ mixes in vacuum, $(\Delta x)^2 \to (1-\eta_\text{loss})(\Delta x)^2_\text{sq} + \eta_\text{loss}/2$: **loss, not pump power, caps squeezing** (5% loss ⇒ ≤13 dB).

## Homodyne and Heterodyne Detection

Balanced homodyne: subtracted photocurrent $\propto |\beta|\,\hat{x}_\theta$; LO phase selects the quadrature; efficiency $>99\%$, room temperature, GHz bandwidth. Heterodyne (dual homodyne) measures $\hat{x}$ and $\hat{p}$ simultaneously at the mandatory cost of one added vacuum unit per quadrature.

## Gaussian States = Means + Covariances

Gaussian state ↔ Gaussian Wigner function ↔ $(\boldsymbol{\mu}, \sigma)$ with uncertainty $\sigma + \frac{i}{2}\Omega \geq 0$. Gaussian unitaries act symplectically: $\sigma \to S\sigma S^T$, $S\Omega S^T = \Omega$. Bloch-Messiah: any Gaussian unitary = interferometer → single-mode squeezers → interferometer. Two-mode squeezing gives EPR correlations $\text{Var}(\hat{x}_a - \hat{x}_b) = \text{Var}(\hat{p}_a + \hat{p}_b) = e^{-2r}$.

## CV Cluster States and Time-Domain Multiplexing

Nullifiers $\hat{p}_j - \sum_{k\in N(j)}\hat{x}_k$ with variance $e^{-2r}$; homodyne measurements drive Gaussian MBQC. Time multiplexing (one squeezer + delay loops) produced **$>10^6$-mode 1D** and **$\sim 10^4$–$10^5$-mode 2D** cluster states — the largest entangled states on any platform. Finite squeezing injects noise $\propto e^{-2r}$ per measurement step.

## The Gaussian No-Go (CV Gottesman-Knill)

Gaussian inputs + Gaussian operations + Gaussian measurements ⇒ efficiently classically simulable (Bartlett-Sanders-Braunstein-Nemoto, 2002); the Wigner function stays positive and evolves as a classical probability. Quantum advantage requires non-Gaussianity: a cubic-or-higher gate (e.g., $e^{i\gamma\hat{x}^3}$; Lloyd-Braunstein universality), a non-Gaussian state (GKP), or a non-Gaussian measurement (photon counting ⇒ GBS). Wigner negativity is the resource marker.

## GKP Encoding (Qubit in an Oscillator)

$$|0_L\rangle \propto \sum_s |x = 2s\sqrt{\pi}\rangle, \qquad |1_L\rangle \propto \sum_s |x = (2s+1)\sqrt{\pi}\rangle$$

Stabilizers: displacements by $2\sqrt{\pi}$; logical Paulis: displacements by $\sqrt{\pi}$. Any displacement $|u| < \sqrt{\pi}/2$ is corrected by measuring quadratures mod $\sqrt{\pi}$ (analog syndrome bonus). **All logical Cliffords are Gaussian**; the non-Gaussian burden sits entirely in state preparation. Logical flip per round $\approx \text{erfc}(\sqrt{\pi}/2\sqrt{2}\sigma)$: $\sim 7\times10^{-5}$ at 10 dB. Thresholds: 20.5 dB (Menicucci 2014), ~10 dB range with analog decoding and architectural optimization. Demonstrated: trapped ions (2019), microwave cavities (2020; beyond break-even 2023), propagating light (2024).

## Time-Multiplexed Architecture (Borealis)

One squeezer at 6 MHz; modes = pulses (167 ns apart); three delay loops ($\tau$, $6\tau$, $36\tau$) with programmable beam splitters give 216 modes and three-range couplings; 16 TES photon-number-resolving channels. GBS samples in 36 μs vs. estimated 9,000 years classically (2022) — later narrowed by loss-exploiting classical samplers. Fault-tolerant roadmap (Blueprint 2021; Aurora 2025): multiplexed GKP factories + CV cluster fabric + homodyne decoding, modular over fiber.

## Differentiable Quantum Programming

QNode: $f(\boldsymbol{\theta}) = \langle\hat{A}\rangle_{\boldsymbol{\theta}}$ embedded in autodiff graphs (PennyLane). Parameter-shift rule: $\partial_\theta f = \frac{1}{2}[f(\theta+\frac{\pi}{2}) - f(\theta-\frac{\pi}{2})]$ — exact hardware gradients; CV versions exist for Gaussian gates with quadratic observables. Strawberry Fields backends embody the theory: Gaussian backend = polynomial covariance propagation; Fock backend = exponential $O(D^N)$, needed exactly when circuits become non-Gaussian.

## CV Algorithms and Their Caveats

GBS ↔ hafnians ↔ perfect matchings: dense subgraph/max clique heuristics, graph kernels, molecular vibronic spectra — no established end-to-end practical advantage; classical GBS-inspired samplers compete. CV-QNN layer = interferometer–squeezers–interferometer–displacement (affine map via Bloch-Messiah) + non-Gaussian activation; without the activation it is classically simulable. Quantum kernels $k(x,x') = |\langle\phi(x)|\phi(x')\rangle|^2$ must be simultaneously classically hard and data-relevant.

## CV versus DV Tradeoffs

CV: deterministic squeezed sources, deterministic Gaussian entanglers, near-ideal room-temperature homodyne, million-mode entanglement — but continuous noise in every operation and hard non-Gaussian state factories. DV: heralded discrete errors and native non-Gaussianity (photons) — but probabilistic sources and gates, cryogenic detection. Each is cheap where the other is expensive; fault-tolerant designs hybridize (bosonic codes below, qubit logic above).
