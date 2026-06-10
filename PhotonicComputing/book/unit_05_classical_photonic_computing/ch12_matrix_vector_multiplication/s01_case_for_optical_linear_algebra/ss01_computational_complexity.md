# Subsection 12.1.1: Computational Complexity and the Optical Advantage

## Orientation

The argument for optical matrix multiplication starts from a simple observation: when light propagates through a linear optical network (a mesh of beam splitters and phase shifters), it is computing a linear transformation. If the network implements the matrix $W$, and the input optical field encodes the vector $\mathbf{x}$, then the output encodes $\mathbf{x}' = W\mathbf{x}$ — automatically, without any explicit computation, in the time it takes light to traverse the device.

Whether this is "faster" or "more efficient" than electronics depends entirely on the cost of encoding the input and reading out the output. In the optical system, those costs involve modulators and detectors (which are electrical devices). We need to account for them carefully.

---

## 12.1.1.1 The Complexity Argument

### Electronic Matrix-Vector Multiplication

For a dense $N \times N$ matrix $W$ multiplied by a vector $\mathbf{x} \in \mathbb{R}^N$:

$$y_i = \sum_{j=1}^{N} W_{ij} x_j, \quad i = 1, \ldots, N$$

This requires $N^2$ multiplications and $N(N-1)$ additions, for a total of $2N^2 - N \approx 2N^2$ floating-point operations. At $F$ FLOPS (floating-point operations per second):

$$T_{\text{electronic}} = \frac{2N^2}{F}$$

For an NVIDIA H100 at $F = 3.9 \times 10^{15}$ FP16 FLOPS/s (tensor core):
- $N = 1024$: $T = 2 \times 1024^2 / (3.9\times10^{15}) \approx 540$ fs
- $N = 4096$: $T \approx 8.6$ ps

Note: GPUs achieve tensor-core throughput for large batched matrix multiplications ($A \times B$), but single vector-matrix products ($W\mathbf{x}$ for one $\mathbf{x}$) are memory-bandwidth bound, not compute-bound. The relevant metric for inference is not peak FLOPS but memory bandwidth, since fetching the $N^2$ weights from memory is the bottleneck:

$$T_{\text{memory}} = \frac{N^2 \times \text{bytes/weight}}{B_{\text{mem}}}$$

For $N = 4096$, FP16 (2 bytes/weight), H100 HBM bandwidth = 3.35 TB/s:
$$T_{\text{memory}} = \frac{4096^2 \times 2}{3.35\times10^{12}} \approx 10 \text{ μs}$$

The GPU is memory-bandwidth-limited for single matrix-vector products, not compute-limited. This is the key: the GPU spends 10 μs fetching weights from memory to perform a computation that takes only 8 ps on the compute units. The compute utilization is less than 0.1%.

### Optical Matrix-Vector Multiplication

In an optical matrix processor, the matrix $W$ is *encoded in the hardware* — as the phase angles of the MZI mesh or the transmission of the ring weight bank. The weights do not need to be fetched from memory; they are *physically implemented* in the device.

**Propagation time**: Light traverses an $N \times N$ MZI mesh of depth $d \approx N$ (Reck: $d = 2N-3$; Clements: $d = N$) with each MZI having length $\sim 100$ μm (for a silicon photonic device):

$$T_{\text{optical}} = \frac{d \times L_{\text{MZI}}}{v_g} \approx \frac{N \times 100\text{ μm}}{(2\times10^8 \text{ m/s})} = \frac{N \times 10^{-4}}{2\times10^8} = 5N \text{ ps}$$

For $N = 64$: $T_{\text{optical}} = 320$ ps.
For $N = 256$: $T_{\text{optical}} = 1.28$ ns.

The optical computation is slower than the electronic *arithmetic*, but far faster than the electronic *memory access*. At $N = 256$, the optical system completes the matrix-vector product in 1.28 ns; the GPU takes 1.28 ns just to fetch 64 weights (at 3.35 TB/s, 2 bytes each: $64 \times 2 / 3.35\times10^{12} = 38$ ps — actually fetching 256 weights takes 153 ps, and $256^2 \times 2 / 3.35\times10^{12} = 39$ ns for the full matrix). So optical wins on speed for this comparison.

---

## 12.1.1.2 The Energy Argument

### Electronic Energy Per MAC

The energy per floating-point multiply-add (MAC) on a modern processor has two components:

**Arithmetic energy**: $E_{\text{arith}} \approx C_{\text{gate}} V_{DD}^2 \times n_{\text{transistors/op}}$

For 7 nm CMOS, an FP16 multiplier requires ~1000 transistors:
$$E_{\text{arith}} \approx 1000 \times 10^{-17} \text{ F} \times (0.65 \text{ V})^2 \approx 4 \text{ fJ/MAC}$$

**Memory access energy**: Each weight must be fetched from some memory. The energy per byte-fetch scales with memory distance:
- Register: ~0.5 fJ/bit
- L1 cache: ~2 fJ/bit  
- L2 cache: ~10 fJ/bit
- HBM (on-package DRAM): ~50 fJ/bit
- DRAM (off-chip): ~200 fJ/bit

For FP16 (2 bytes = 16 bits) weights fetched from HBM: $16 \times 50 = 800$ fJ/weight fetched from HBM.

Since each weight is used for only one MAC (in a single-vector inference), the total energy per MAC is dominated by memory:

$$E_{\text{total/MAC}} \approx 800 \text{ fJ/weight} + 4 \text{ fJ/arithmetic} \approx 804 \text{ fJ/MAC}$$

**Weight-stationary matrix multiplication**: For a batch of $B$ vectors multiplied by the same matrix (common in deep learning inference on batches):
$$E_{\text{per-MAC}} = \frac{800 \text{ fJ} + B \times 4 \text{ fJ}}{B} \approx \frac{800}{B} + 4 \text{ fJ/MAC}$$

At $B = 200$: $E \approx 8$ fJ/MAC. At $B = 10000$: $E \approx 4.08$ fJ/MAC (compute-bound).

For large batches, the electronic compute energy approaches 4 fJ/MAC. For single-inference (batch size 1), it is 200× larger.

### Optical Energy Per MAC

In an optical matrix processor, the weight energy is amortized differently: the weights are stored in the hardware (phase angles of MZIs or temperature of thermo-optic heaters), and the energy to hold them is continuous rather than per-access.

**Per-MAC energy at the optical layer**: The only energy per computation is the optical signal energy. For an $N$-dimensional matrix-vector product performed in one optical pass:

$$E_{\text{opt/MAC}} = \frac{E_{\text{signal}}}{N^2}$$

where $E_{\text{signal}}$ is the total signal energy entering the processor. For a minimum detectable signal:

$$E_{\text{signal, min}} = N_{\text{photons}} \times \hbar\omega$$

where $N_{\text{photons}} \approx 50$ photons for shot-noise-limited detection at BER $= 10^{-6}$ (from Chapter 5). At $\lambda = 1310$ nm:

$$E_{\text{opt/MAC}} = \frac{50 \times \hbar\omega}{N^2} = \frac{50 \times 1.5\times10^{-19}}{N^2} = \frac{7.5\times10^{-18}}{N^2}$$

For $N = 64$: $E_{\text{opt/MAC}} = \frac{7.5\times10^{-18}}{4096} \approx 1.8 \text{ zJ/MAC}$

This is $10^{-21}$ J — 6 orders of magnitude below 4 fJ. The *optical field* carries essentially no energy cost per MAC.

**But the total energy budget includes the laser**: 50 photons at the detector requires, accounting for link loss ($L_{\text{dB}} \approx 5$–10 dB) and laser WPE ($\sim 20\%$):

$$E_{\text{laser/MAC}} = \frac{50 \times \hbar\omega \times 10^{L_{\text{dB}}/10}}{\text{WPE} \times N^2} = \frac{50 \times 1.5\times10^{-19} \times 3}{0.2 \times 4096} \approx 0.27 \text{ aJ/MAC}$$

Even including the laser, the energy per MAC from the optical signal chain is sub-attojoule — vastly below electronic arithmetic.

**The actual energy budget is dominated by ADC and DAC energy**: 
- Each of the $N$ outputs requires an ADC to digitize the optical power: at 8-bit precision and 1 Gsps, a photonic-integrated ADC consumes ~100 fJ/sample. Per $N^2$ MACs: $100 \text{ fJ} \times N / N^2 = 100/N$ fJ/MAC = 1.6 fJ/MAC for $N = 64$.
- The $N$ inputs require driving $N$ modulators: at 10 fJ/bit for a ring modulator at 8-bit DAC: $10 \text{ fJ} / N = 0.16$ fJ/MAC for $N = 64$.
- Thermo-optic phase stabilization of $N(N-1)/2$ MZIs: at 10 mW/MZI (from Section 7.4.1), for an $N=64$ mesh: $64\times63/2 \times 10 \text{ mW} = 20$ W continuous power for the phase shifters.

This last term is the real problem: **the static power of the thermo-optic phase shifters dominates the energy budget by orders of magnitude**.

For a 64-port MZI mesh at 100 Gbps total throughput:
$$E_{\text{static/MAC}} = \frac{20 \text{ W}}{100 \times 10^9 \text{ MACs/s} \cdot 64^2 \text{ MACs/op}} \approx 48 \text{ fJ/MAC}$$

This is 10× higher than the electronic arithmetic energy — and worse still if the throughput is lower.

**Conclusion**: The energy advantage of photonic matrix processors hinges entirely on (1) finding phase-shifter technology with much lower static power (MEMS: <1 μW/element vs. 10 mW; PCM: non-volatile, zero static power), and (2) achieving high throughput (high speed or high parallelism) to amortize any remaining static power.

---

## 12.1.1.3 When Optics Wins

Putting the analysis together, an optical matrix processor is more energy-efficient than a GPU when:

$$\frac{P_{\text{static}}}{T_{\text{total}} \cdot N^2 \cdot B_{\text{rate}}} + E_{\text{DAC/ADC}}/N < E_{\text{GPU/MAC}}$$

For current technology (thermo-optic MZIs):
- $P_{\text{static}} \approx 20$ W for $N = 64$
- $B_{\text{rate}} \approx 25 \times 10^9$ operations/second (at 25 GHz optical bandwidth)
- $E_{\text{GPU/MAC}} \approx 804$ fJ (single inference on H100 from HBM)

Breakeven: 20 W / ($N^2$ × 25 Gbps × 804 fJ) $\approx$ 20 / ($4096 \times 25\times10^9 \times 804\times10^{-15}$) $\approx$ 20 / 82,406 $\approx 0.24$ mJ/MAC — optical loses by $10^9$.

For next-generation MEMS phase shifters ($P_{\text{static}} < 1$ μW/element → $N^2/2 \times 1$ μW = 2 mW total for $N = 64$):
- 2 mW / ($4096 \times 25\times10^9 \times 804\times10^{-15}$) = 2.4 pJ/MAC — still loses to GPU (804 fJ < 2.4 pJ).

The optical processor wins on energy per MAC only at single-inference (batch=1) workloads where the GPU is memory-bandwidth-limited at 800 fJ/MAC and the optical system achieves <100 fJ/MAC. For batch inference, the GPU wins because it can amortize weight memory access cost over the batch. This defines the photonic processor's niche precisely: **single-inference, latency-critical applications where the GPU's memory-bandwidth limitation is the bottleneck**.

---

## References

[1] Hamerly, R., Bernstein, L., Sludds, A., Soljačić, M., & Englund, D. (2019). "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9, 021032. [The paper that most rigorously analyzes when optical neural networks beat GPUs energetically. The analysis here follows this paper's framework.]

[2] Nahmias, M.A., et al. (2020). "Photonic multiply-accumulate operations for neural inference." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518. [Comprehensive analysis of photonic MAC energy; includes ADC/DAC overhead.]

[3] Shainline, J.M., et al. (2019). "Superconducting optoelectronic circuits for neuromorphic computing." *Physical Review Applied*, 7, 034013. [Context: superconducting MAC energy provides the ultimate lower bound on electronic MAC energy for comparison.]
