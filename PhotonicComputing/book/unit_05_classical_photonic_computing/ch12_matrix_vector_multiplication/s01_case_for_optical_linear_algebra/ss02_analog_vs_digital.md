# Subsection 12.1.2: Analog vs. Digital Computing

## Orientation

The MZI mesh computes a matrix-vector product using continuous physical quantities — optical field amplitudes — rather than discrete digital numbers. This makes it an *analog computer* in the classical sense, with all the associated advantages (parallelism, potential energy efficiency) and disadvantages (noise sensitivity, limited precision, calibration requirements). Understanding the analog-digital tradeoff is essential for evaluating photonic processor claims honestly.

---

## 12.1.2.1 What Analog Computation Means

### Precision and Signal-to-Noise Ratio

An analog computing element performs a mathematical operation on a physical quantity (voltage, current, optical field amplitude). The precision of the result is limited by the noise in the physical quantity.

For an optical field with mean amplitude $\bar{A}$ and noise fluctuation $\delta A$ (RMS), the signal-to-noise ratio is:

$$\text{SNR} = \frac{\bar{A}^2}{\langle(\delta A)^2\rangle}$$

The *effective number of bits* (ENOB) is (from Chapter 9):

$$\text{ENOB} = \frac{1}{2}\log_2(\text{SNR}) - 0.5$$

Shot noise (the quantum limit, from Section 5.1.2) sets the minimum noise $\langle(\delta A)^2\rangle_{\text{min}} = 1/\bar{n}$ (where $\bar{n}$ is the mean photon number). The maximum ENOB is:

$$\text{ENOB}_{\text{max}} = \frac{1}{2}\log_2(\bar{n}) - 0.5$$

For $\bar{n} = 10^6$ photons/detection (milliwatt-level power at 1 Gsps):
$$\text{ENOB}_{\text{max}} = \frac{1}{2}\log_2(10^6) - 0.5 = 10 - 0.5 = 9.5 \text{ bits}$$

This is the theoretical maximum. In practice, additional noise (laser RIN, thermal noise in the TIA, phase noise in the MZI) reduces this to 5–8 bits in demonstrated photonic matrix processors.

### Is 6–8 Bits Adequate?

For neural network inference: yes, in most cases. This is one of the key results from the deep learning hardware literature:

**Post-training quantization**: A neural network trained in FP32 or FP16 can typically be quantized to INT8 (8 bits) with < 1% accuracy loss for image classification on ImageNet. For language models, INT4 (4 bits) is viable for some tasks [1].

**Training precision**: 8-bit training (with appropriate scaling) is demonstrated for large models (FP8 training in H100 TensorFloat-32). FP16 (16 bits) is standard.

**Physical reasoning**: Neural networks are surprisingly tolerant of weight precision because the network has redundancy (many neurons contribute to each output), and the training process implicitly adapts to the precision available. The mathematical explanation: if weights are $w_i$ with precision noise $\delta w_i \sim \mathcal{N}(0, 2^{-2B})$ for $B$-bit representation, the output noise from an $N$-neuron layer is:

$$\sigma_y^2 = \sum_i (\delta w_i)^2 x_i^2 \leq N \cdot 2^{-2B} \cdot \|\mathbf{x}\|_\infty^2$$

For $N = 1000$, $B = 6$: $\sigma_y < \sqrt{1000} \times 2^{-6} \approx 0.49$ relative to unit-norm input. This is significant but recoverable by the next activation function (which clips or saturates extreme values).

**Scientific computing**: 6–8 bits is grossly inadequate for many scientific computing applications (iterative solvers for PDEs, molecular dynamics, quantum chemistry) that require 14–16 digits of precision. This is a fundamental limitation that prevents photonic processors from targeting these applications.

---

## 12.1.2.2 The Precision-Energy Tradeoff

### Why Higher Precision Costs More Energy

The energy per analog computing operation scales with precision because higher precision requires:
1. Higher SNR, which requires more optical power (more photons per measurement)
2. More careful calibration of each component
3. Longer averaging times (if precision is improved by averaging multiple measurements)

The energy scaling: for shot-noise-limited detection at $B$-bit precision, the required photon number per output is $\bar{n} = 2^{2(B+0.5)} \approx 4^B$. The energy per output sample:

$$E_{\text{sample}} = 4^B \times \hbar\omega$$

For $B = 8$: $E_{\text{sample}} = 65{,}536 \times 1.5\times10^{-19} \approx 10$ aJ
For $B = 16$: $E_{\text{sample}} = 4.3\times10^9 \times 1.5\times10^{-19} \approx 640$ aJ
For $B = 32$: $E_{\text{sample}} \approx 3 \times 10^{10}$ aJ = 30 nJ

The energy grows as $4^B$: each additional bit requires 4× more energy. This exponential scaling is the fundamental reason that analog computing loses to digital at high precision.

**The crossover**: At 8 bits, the photon energy is ~10 aJ/sample — still much less than the laser energy (100–300 fJ/bit for a ring modulator system including laser). At 16 bits, the photon energy is 640 aJ — still less than laser energy. The photon energy only becomes the bottleneck at precisions > 20 bits, which are far beyond what analog optical systems can achieve anyway.

The practical ENOB ceiling for optical systems is ~8–10 bits, set by:
- Phase noise in the MZI phase shifters (~0.01 radians RMS from heater power fluctuations → ~0.3% intensity noise)
- Laser relative intensity noise (RIN < -140 dB/Hz for good DFB, but accumulates over the measurement)
- Fabrication imperfections (MZI splitting ratio deviations, waveguide width variation)

---

## 12.1.2.3 Error Correction for Analog Precision Extension

### The Idea

If an 8-bit analog optical processor can represent weights to ~8-bit precision, can we extend the effective precision to 16 bits by combining multiple 8-bit operations?

The answer is yes, at the cost of 2× more computation. The approach: represent a 16-bit weight $w$ as $w = w_{\text{MSB}} \times 2^8 + w_{\text{LSB}}$, where $w_{\text{MSB}}$ and $w_{\text{LSB}}$ are each 8-bit values. Compute $W_{\text{MSB}} \mathbf{x}$ and $W_{\text{LSB}} \mathbf{x}$ separately (two optical passes), then combine digitally:

$$W\mathbf{x} = 2^8 \times W_{\text{MSB}}\mathbf{x} + W_{\text{LSB}}\mathbf{x}$$

This gives 16-bit precision with 2 optical passes (and a final digital addition). For 32-bit: 4 passes.

The energy cost grows linearly with the number of passes, not as $4^B$. This is the advantage of the digital decomposition: the energy scales as $B$ (number of passes) rather than $4^B$ (for analog). At 32 bits, digital decomposition requires 4 optical passes × 8 bits each, while a direct analog approach would require $4^{32} \approx 10^{19}$ times more energy — completely impractical.

**The disadvantage**: Multiple passes require more time (4× latency for 4 passes). The advantage of optical speed (single-pass time $\sim 1$ ns for a 64-port mesh) is reduced by a factor equal to the number of passes.

---

## References

[1] Dettmers, T., et al. (2022). "LLM.int8(): 8-bit matrix multiplication for transformers at scale." *arXiv:2208.07339*. [Demonstrates that large language models can be quantized to INT8 with negligible accuracy loss using mixed-precision decomposition.]

[2] Cheng, Y., et al. (2018). "Model compression and acceleration for deep neural networks." *IEEE Signal Processing Magazine*, 35(1), 126–136. [Review of quantization approaches for neural network compression; provides context for why 6–8 bits is adequate for inference.]

[3] Hamerly, R., et al. (2022). "Accurate self-configuration of rectangular multiport interferometers." *Physical Review Applied*, 18, 024019. [Analysis of calibration and precision limits in optical matrix processors; derives the ENOB bounds in the context of MZI mesh imperfections.]
