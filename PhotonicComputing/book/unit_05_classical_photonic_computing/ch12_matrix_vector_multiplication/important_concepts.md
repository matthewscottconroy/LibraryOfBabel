# Chapter 12: Important Concepts

## The Case for Optical Linear Algebra

**Matrix-vector multiplication is the target because it is linear, parallel, and dominant.** Modern AI inference spends >99% of its operations on MACs. A coherent optical system performs linear superposition by default: each input mode couples to every output mode with a complex coefficient, so an $N \times N$ multiply completes in one optical transit ($\sim$10–100 ps) regardless of $N$. The $O(N^2)$ arithmetic is done by wave propagation; only the $O(N)$ encoding and readout cost energy.

**The optical MAC has two physical implementations.** Coherent: fields interfere at couplers, and balanced detection extracts the complex inner product $A_1^* A_2$. Incoherent: powers on distinct wavelengths are attenuated by weights and summed as photocurrent at a detector. Coherent computing preserves phase (complex matrices, unitaries); incoherent computing trades phase for robustness.

**Analog precision is an energy variable, not a constant.** Shot noise sets $\text{SNR} = \bar{n}$ (detected photons per symbol), so every additional bit of ENOB costs 6 dB more optical energy per symbol. Photonic processors deliver 4–8 ENOB — sufficient for neural inference (INT8-class), insufficient for scientific computing. Precision, throughput, and optical power form a three-way tradeoff that must be quoted together.

---

## The MZI Mesh

**One MZI is a programmable $2\times2$ unitary; $N(N-1)/2$ of them make any $N\times N$ unitary.** The parameter count matches the dimension of $U(N)$ exactly. Reck's triangular mesh (depth $2N-3$) proved existence in 1994; Clements' rectangular mesh (depth $N$, balanced path lengths) is the practical standard since 2016.

**Mesh error is entangled error.** Every matrix element depends on every phase shifter, so per-device errors $\sigma$ compound to relative output error $\sim\sqrt{N(N-1)/2}\,\sigma$. Coupler imbalance ($\pm$1–3%), DAC phase resolution, and thermo-optic drift ($\sim$0.08 rad/K per 100 μm shifter) each contribute; uncorrected meshes achieve $\sim$4–5 bits of matrix fidelity, error-corrected and self-configured meshes 6–8 bits.

**Thermo-optic static power is the silicon mesh's structural weakness.** At $\sim$10 mW per $\pi$ shifter, a 64-mode mesh dissipates watts just holding its weights. Phase-change materials (non-volatile, zero hold power), Pockels shifters (TFLN/BTO, femtojoule reconfiguration), and MEMS are the candidate cures — each trading update speed, loss, or process maturity.

---

## SVD and Non-Unitary Matrices

**Arbitrary matrices = rotation × scaling × rotation.** $W = U\Sigma V^\dagger$ maps directly to mesh–modulator column–mesh. Passive optics cannot amplify, so singular values are normalized by $\sigma_{\max}$ and the global scale is restored electronically.

**Non-unitarity is optical loss; condition number is dynamic range.** The $\Sigma$ stage deliberately dumps the power fraction $1 - (\sigma_i/\sigma_{\max})^2$ from each mode. An ill-conditioned matrix ($\kappa \gg 1$) starves its weak singular channels of photons and therefore of ENOB. Analog photonic linear algebra is condition-number sensitive in a way floating point is not.

**Low-rank structure is fabricatable compression.** By Eckart–Young, keeping the top $r$ singular values gives the optimal rank-$r$ approximation; the hardware shrinks from $O(N^2)$ to $O(Nr)$ MZIs with proportionally less depth loss. Training directly in the (mesh-phase, singular-value) parameterization eliminates decomposition error entirely.

---

## The 2017 Watershed and Its Lessons

**Shen et al. 2017 demonstrated the full coherent stack — and quantified its gap.** 56 thermo-optic MZIs, SVD-programmed $4\times4$ layers, electronic nonlinearity: 76.7% vowel-classification accuracy vs. 91.7% digital. The diagnosis (phase encoding error + detection noise) and the remedy directions (error correction, noise-aware training, self-configuration) defined the following decade.

**Inference degrades gracefully under analog noise.** Classification needs the right class to win the argmax, not numerical exactness; a 5% matrix error costs points, not collapse — and training with noise in the loop recovers most of them. This graceful degradation is the single property that makes analog photonic AI plausible at all.

---

## Wavelength-Multiplexed Computing

**A detuned microring is an analog multiplier; a balanced photodetector makes it signed.** Ring transmission sets $|w|$; routing drop and through ports to a balanced detector pair extends the range to $w \in [-1, 1]$. All wavelengths sum automatically in the photocurrent. Channel count is bounded by FSR/spacing ($N \sim$ 10–30 per bus) and crosstalk by the Lorentzian tails ($\sim$1% at $Q = 10^4$, 100 GHz grid).

**Broadcast-and-weight turns weight banks into recurrent networks.** Each neuron owns a wavelength, hears all others through its weight bank, and retransmits through its modulator — whose transfer function doubles as the activation nonlinearity and whose O/E/O stage restores signal levels (the cascadability that all-optical logic lacked). Loop latencies of $\sim$1 ns make it the natural fabric for RF-rate and in-flight optical signal processing.

**Incoherent weights calibrate better than coherent ones.** One ring = one parameter = one local feedback loop: >9-bit weight precision demonstrated. A mesh matrix element is a global interference property of dozens of phases. Expressivity (coherent) and controllability (incoherent) pull in opposite directions.

**Parallelism can be spent in space, wavelength, or time.** The 10+ TOPS demonstrations of 2021 used wavelength (PCM crossbar + soliton comb; Feldmann) and time-wavelength interleaving over dispersive fiber (Xu) rather than large space-domain circuits — leveraging telecom's fastest components instead of fabricating giant meshes.

**Weight update speed is the unsolved constraint common to every architecture.** Thermo-optic: μs–100 μs. PCM: μs–ms writes, limited endurance. Depletion tuning: fast but weak. All current photonic matrix engines are therefore inference accelerators with quasi-static weights; training remains electronic (Chapter 13 examines the escape routes).
