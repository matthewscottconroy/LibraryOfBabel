# Physics of Optical Matrix-Vector Multiplication

> *A single pass of light through a mesh of coupled waveguides can compute the product of an N-dimensional vector with an N×N matrix — in the time it takes light to cross the chip, which is picoseconds. A GPU performing the same operation requires millions of transistor operations and tens of nanoseconds. This is not a software trick. It is physics. Understanding why and how requires connecting Maxwell's equations, linear algebra, and the physics of noise in quantum optical systems.*

---

## 1. The Case for Optical Matrix Multiplication

### 1.1 Why Matrix-Vector Products Dominate Modern Computing

Modern deep neural networks are dominated by two operations: matrix-vector multiplication (MVM) for linear layers and element-wise nonlinearities for activation functions. A single inference pass through GPT-3 requires $\sim 3\times10^{23}$ floating-point multiply-accumulate (MAC) operations. Even highly optimized GPU clusters require hundreds of petaflops of compute.

The energy cost per MAC on current hardware is $\sim 1$–10 fJ (femtojoules). Multiplying an $N\times N$ matrix by an $N$-vector requires $N^2$ MACs. For $N = 512$: $2.6\times10^5$ MACs. At 1 fJ/MAC: 0.26 pJ per inference. This sounds small, but data centers run billions of inferences per second.

Optical systems offer a different energy-time tradeoff: the matrix multiplication happens in $O(1)$ time (the transit time of the chip, $\sim 10$ ps), and the energy is set by the photon budget rather than transistor switching.

### 1.2 The O(1) Claim: What It Means

Strictly speaking, an analog optical matrix-vector product with $N$ inputs requires:

- Input encoding: $O(N)$ modulator operations (one per input element)
- Propagation: $O(1)$ time — all $N^2$ multiplications and $N^2$ additions happen simultaneously in the wave optics
- Output readout: $O(N)$ detector operations

The claim is that the matrix-vector multiplication itself — the $O(N^2)$ work — is done by light propagation at essentially zero time and marginal energy cost. The total energy scales as $O(N)$ (encoding/decoding) rather than $O(N^2)$ (digital computation).

This is not magic — it is the superposition principle of wave optics. Each input mode couples linearly to all output modes, and the coupling coefficients are exactly the matrix elements.

---

## 2. Singular Value Decomposition Architecture

### 2.1 The Shen et al. (2017) Proposal

The landmark paper by Shen et al. (Nature Photonics, 2017) proposed implementing neural network inference using an integrated photonic circuit. Their architecture is based on the singular value decomposition (SVD) of the weight matrix.

Any real $M\times N$ matrix $W$ can be written as:

$$W = U \Sigma V^T$$

where $U$ ($M\times M$) and $V$ ($N\times N$) are orthogonal (unitary) matrices, and $\Sigma$ ($M\times N$) is a diagonal matrix of non-negative singular values $\sigma_1 \geq \sigma_2 \geq \cdots \geq \sigma_{\min(M,N)} \geq 0$.

For a complex weight matrix (as arises in coherent photonic systems): $W = U \Sigma V^\dagger$ where $U, V$ are unitary.

**The physical implementation:**

- $V^\dagger$: a photonic circuit implementing an $N\times N$ unitary matrix (Mach-Zehnder mesh)
- $\Sigma$: $N$ attenuators/amplifiers scaling each mode by $\sigma_i$ (implemented as Mach-Zehnder interferometers with variable splitting)
- $U$: a second photonic circuit implementing an $M\times M$ unitary matrix

The input optical vector $\mathbf{x}$ (encoded as complex field amplitudes in $N$ waveguide modes) is multiplied by $V^\dagger$, then the diagonal $\Sigma$, then $U$, producing the output $W\mathbf{x}$.

### 2.2 Encoding and Decoding

**Input encoding:** A laser supplies CW light. The input vector $x_j$ (real, non-negative) is encoded by modulating the optical amplitude in waveguide $j$: $E_j = \sqrt{x_j} e^{i\phi_j}$. Phase $\phi_j$ is set to zero for purely real operation.

For signed inputs, the bias trick is used: encode $x_j + c$ (all positive) and subtract the DC offset at the output.

**Output readout:** Photodetectors at the output waveguides measure $|E_j^\text{out}|^2 = y_j$ (intensities, non-negative). The matrix multiplication result $y = Wx$ is recovered as differences or ratios of detected powers.

**The coherence requirement:** The $U$ and $V$ meshes must be traversed coherently — the phase relationship between different waveguide modes must be maintained. This requires optical path lengths matched to much better than one wavelength, which is challenging in temperature-sensitive silicon.

---

## 3. The Mach-Zehnder Mesh

### 3.1 Building Blocks

The fundamental building block of a photonic unitary matrix is the **programmable beamsplitter**, implemented as a Mach-Zehnder interferometer (MZI) with two phase shifters:

$$U_\text{MZI}(\theta,\phi) = \begin{pmatrix}e^{i\phi}\cos\theta & -i\sin\theta \\ -ie^{i\phi}\sin\theta & \cos\theta\end{pmatrix}$$

Here $\theta \in [0, \pi/2]$ controls the splitting ratio ($\theta = \pi/4$ gives 50/50) and $\phi \in [0, 2\pi]$ controls the relative phase. Together $(\theta, \phi)$ parameterize any $2\times 2$ unitary matrix (up to a global phase).

**Count of parameters:** A general $N\times N$ unitary matrix $U \in U(N)$ has $N^2$ real parameters. Our $2\times 2$ building block provides 2 parameters ($\theta$, $\phi$) plus potentially one global phase. A mesh of $N(N-1)/2$ MZIs provides exactly the right number of parameters to represent any element of $U(N)$.

### 3.2 The Reck Decomposition

Reck et al. (1994) showed that any $N\times N$ unitary can be decomposed into a product of $T_{mn}(\theta,\phi)$ matrices, each acting nontrivially only on modes $m$ and $n$:

$$U = D \cdot \prod_{m<n} T_{mn}(\theta_{mn}, \phi_{mn})$$

where $D$ is a diagonal phase matrix. The product involves $N(N-1)/2$ MZI units and $N$ phase shifters.

**Physical layout (Reck):** A triangular mesh. For $N = 4$:

```
Input 1 ─────[MZI]─────[MZI]─────[MZI]── Output 1
Input 2 ──[MZI]─────[MZI]──────────────── Output 2
Input 3 ─────[MZI]──────────────────────── Output 3
Input 4 ──────────────────────────────────── Output 4
```

The triangular layout means that inputs 1 and 2 interact in the first column, then (1,3), (1,4), (2,3), (2,4), (3,4). Each interaction is one MZI.

### 3.3 The Clements Decomposition

Clements et al. (2016) proposed a more compact and hardware-efficient layout: a square (rectangular) mesh with **alternating columns** of MZIs, where even columns couple modes $(1,2), (3,4), \ldots$ and odd columns couple $(2,3), (4,5), \ldots$.

**Advantages over Reck:**
1. **Lower depth:** Reck requires $2N-3$ optical depth (number of MZI layers); Clements requires $N$ — an asymptotic factor of 2 improvement.
2. **Equal optical path length:** In Clements, all paths from input to output traverse the same number of MZIs, reducing differential loss and phase errors.
3. **Robustness:** The symmetric structure means fabrication errors are more uniformly distributed.

**The Clements decomposition** achieves $U = D \prod_{k} T_{m_k, n_k}(\theta_k, \phi_k)$ using $N(N-1)/2$ MZIs in a rectangular mesh of depth $N$.

For $N = 4$, the Clements mesh:

```
Layer 1     Layer 2     Layer 3     Layer 4
─[MZI]──────[MZI]──────[MZI]──────[MZI]──
─[MZI]──────[MZI]──────[MZI]──────[MZI]──
─[MZI]──────[MZI]──────[MZI]──────[MZI]──
─[MZI]──────[MZI]──────[MZI]──────[MZI]──
```

(Not all MZIs are active simultaneously; the pattern alternates.)

### 3.4 Decomposition Algorithm

Given a target unitary $U$, compute the Clements decomposition:

1. Initialize the algorithm with $U^{(0)} = U$.
2. At each step, find $(\theta, \phi)$ such that $T_{mn}^{-1}(\theta,\phi) U^{(k)}$ zeros out the $(m+1,n)$ element (multiplying from the left zeros a row element; from the right zeros a column element).
3. Alternate between left and right multiplications following the Clements pattern.
4. After $N(N-1)/2$ steps, the result is a diagonal matrix $D$ which is implemented as phase shifters.

The algorithm is $O(N^3)$ classically — the decomposition itself is computed digitally. The optical network then implements the decomposed unitary.

---

## 4. Noise Sources and Their Effect on Inference Accuracy

### 4.1 Taxonomy of Errors

In a physical optical matrix multiplier, several noise sources degrade the accuracy of the matrix-vector product:

1. **Phase errors $\delta\theta, \delta\phi$:** Fabrication imperfections and thermal drift cause the actual MZI settings to differ from target. Phase errors are the dominant noise source in silicon photonic implementations.

2. **Optical shot noise:** The quantum nature of light — photodetection is a Poisson process. The signal-to-noise ratio is $\text{SNR} = \bar{n}$ where $\bar{n}$ is the mean photon number per detection interval.

3. **Thermal noise (Johnson-Nyquist):** Electrical noise in the photodetector and transimpedance amplifier.

4. **Intensity noise (RIN):** Laser relative intensity noise, typically $< -150$ dBc/Hz for semiconductor lasers.

5. **Crosstalk:** Parasitic coupling between non-intended waveguide modes.

### 4.2 Phase Error Model

Consider an MZI programmed to implement splitting angle $\theta_0$, but the actual angle is $\theta = \theta_0 + \delta\theta$ due to thermal drift or fabrication. The output field amplitude:

$$E_\text{out} = U(\theta_0 + \delta\theta) E_\text{in}$$

To first order in $\delta\theta$:

$$E_\text{out} \approx U(\theta_0) E_\text{in} + \delta\theta \frac{\partial U}{\partial\theta}\bigg|_{\theta_0} E_\text{in}$$

The error term $\delta\theta \frac{\partial U}{\partial\theta} E_\text{in}$ represents the departure from the intended matrix operation.

For a network of $K = N(N-1)/2$ MZIs, each with independent Gaussian phase error $\delta\theta_k \sim \mathcal{N}(0, \sigma^2)$, the mean-square error in the output:

$$\langle \|\delta \mathbf{y}\|^2 \rangle = \sigma^2 \sum_k \left\|\frac{\partial U}{\partial\theta_k} \mathbf{x}\right\|^2$$

For random unitary matrices, this scales approximately as:

$$\langle \|\delta \mathbf{y}\|^2 \rangle \sim K \sigma^2 \|\mathbf{x}\|^2 = \frac{N(N-1)}{2}\sigma^2 \|\mathbf{x}\|^2$$

The relative error in the output scales as $\sqrt{N}\sigma$. For $N = 64$ and $\sigma = 0.01$ rad: relative error $\sim 8\%$, which significantly degrades inference accuracy for deep networks.

### 4.3 Shot Noise and Optical Power Budget

Shot noise is fundamental — it cannot be reduced without increasing optical power or detecting more photons. The number of photons detected per symbol:

$$\bar{n} = \frac{P \cdot T_\text{int}}{h\nu}$$

where $P$ is the optical power, $T_\text{int}$ is the integration time, and $h\nu$ is the photon energy. For SNR $= \bar{n} = 10^4$ (40 dB, needed for 8-bit precision): at $P = 1$ μW and $\lambda = 1550$ nm ($h\nu = 1.28\times10^{-19}$ J):

$$T_\text{int} = \frac{\bar{n} h\nu}{P} = \frac{10^4 \times 1.28\times10^{-19}}{10^{-6}} = 1.28 \text{ ns}$$

So at 1 μW, we need 1.28 ns integration time to achieve 8-bit precision — limiting throughput to $\sim 800$ MHz. Increasing to 1 mW reduces this to 1.28 ps, enabling THz-rate operation. This is the fundamental **optical power vs. throughput vs. precision** tradeoff.

### 4.4 Effect of Noise on Inference Accuracy

For a neural network performing classification, the inference accuracy degrades gracefully with noise — the network is surprisingly robust to analog noise due to the softmax nonlinearity at the output. Empirical results:

- Phase error $\sigma = 0.02$ rad (typical silicon thermo-optic): $\sim 2$–3% accuracy drop on MNIST (from 98.5% to 96%)
- Phase error $\sigma = 0.05$ rad: $\sim 10\%$ accuracy drop
- Shot noise at SNR $= 100$: $< 1\%$ accuracy drop for MNIST

Networks can be made more robust by training with noise (noise-aware training) or by error-correction protocols.

---

## 5. Optical Nonlinearity for Activation Functions

### 5.1 The Problem with Linear Optics

A cascade of linear optical elements (MZI meshes, beam splitters) can only implement linear transformations. Neural networks require nonlinear activation functions — without them, a multilayer network is equivalent to a single linear layer.

The challenge: nonlinear optical effects (e.g., Kerr effect) typically require either very high optical intensities or very long interaction lengths — neither of which is compatible with chip-scale photonic integration.

### 5.2 Options for Optical Nonlinearity

**Option A: Opto-electronic nonlinearity.** Detect the optical signal with a photodiode, apply a nonlinear electronic function (ReLU, sigmoid), then re-encode onto a laser. This is the most practical current approach:

- Photodiode: $O(\text{ns})$ conversion time
- Electronic nonlinearity: $O(100 \text{ ps})$
- Laser modulation: $O(\text{ns})$

Total latency per nonlinear layer: $\sim 1$–5 ns. For deep networks, this dominates over the $\sim 10$ ps optical propagation.

**Option B: Saturable absorber.** A two-level system (quantum dot, carbon nanotube, graphene) absorbs light at low intensity and becomes transparent at high intensity. This is an all-optical sigmoid:

$$T(I) = \frac{T_0}{1 + I/I_\text{sat}}$$

where $I_\text{sat}$ is the saturation intensity. The sigmoidal shape makes this a natural optical activation function. Integration of graphene or quantum dots into silicon waveguides has been demonstrated.

**Option C: Kerr nonlinearity in resonators.** The Kerr effect in a high-Q resonator: $\Delta\omega \propto |E|^2$. For a resonator biased near critical coupling, the transmitted power is a nonlinear function of input power — effectively a Lorentzian-shaped activation function. The key advantage: high Q amplifies the circulating field, reducing the required optical power by $Q/2\pi$.

**Option D: Optical bistability.** A nonlinear resonator with feedback can exhibit bistability — two stable output states for the same input. This implements a binary (step function) activation. Photonic crystal nanocavities with Kerr nonlinearity have shown switching at $\sim 1$ fJ energies.

---

## 6. Energy Efficiency: Comparison with GPU Matrix Multiply

### 6.1 GPU Energy Model

An NVIDIA A100 GPU performing a $1024\times1024$ matrix-matrix multiply (used in transformer attention):
- Peak throughput: 312 TeraFLOPS (BF16)
- Matrix multiply: $2\times1024^3 \approx 2.1\times10^9$ MACs
- Time: $\sim 7$ μs
- Power: 400 W
- Energy: $400 \times 7\times10^{-6} = 2.8$ mJ
- Energy per MAC: $\sim 1.3$ fJ/MAC

### 6.2 Optical MVM Energy Model

For a photonic $1024\times1024$ MVM:
- $K = 1024\times1023/2 \approx 524,000$ MZIs in the Clements mesh
- Thermo-optic phase shifter: 10 mW per MZI for $\pi$ phase shift (static power)
- Total static power: $5.24\times10^5 \times 10\times10^{-3} = 5.24$ kW (!)

This is clearly impractical. The thermo-optic static power is the Achilles' heel of silicon photonic MZI networks.

**Solutions being explored:**
1. **Phase change materials (PCM):** e.g., Ge₂Sb₂Te₅ (GST) has two stable states (amorphous/crystalline) with different refractive indices. Once set, no holding power required. Write energy $\sim 1$–10 pJ per device. Suitable for inference-only (weight-fixed) networks.

2. **Electro-optic phase shifters (Pockels):** TFLN or BTO integrated into silicon circuits. Static power = 0 (Pockels effect requires no DC current). Reconfiguration: $\sim 1$ fJ/bit. 

3. **Analog memory:** Store weights as static charges on floating-gate capacitors controlling nearby waveguide junctions. Static power = 0; refresh energy $\sim 1$ aJ.

### 6.3 Fair Comparison: Energy per Effective Operation

Ignoring static power and counting only the energy for a single inference:

**Photonic (shot noise limited, $P = 1$ mW, $T_\text{int} = 1$ ns):**
- Energy for $N = 64$ matrix-vector multiply: $P \times T_\text{int} = 1$ nJ
- Number of MACs: $64^2 = 4096$
- Energy per MAC: $\sim 0.24$ pJ/MAC

**At higher optical power ($P = 10$ mW, $T_\text{int} = 100$ ps, SNR still $= 10^4$):**
- Energy: $10\times10^{-3} \times 100\times10^{-12} = 1$ pJ
- Energy per MAC: $0.24$ fJ/MAC — comparable to state-of-the-art digital!

But the key advantage emerges at large $N$: the digital computation scales as $O(N^2)$ energy and time; the optical computation scales as $O(N)$ energy (only input/output) with $O(1)$ time. For $N = 10^6$ (future large models): photonic wins by a factor of $10^6$ in energy and $10^6$ in latency.

### 6.4 The Reconfiguration Bottleneck

The main limitation in current photonic neural networks is not the optical computation but the weight update. Training a deep network requires updating weights after every mini-batch ($\sim 10^{6}$ gradient descent steps). Each weight update requires reprogramming the phase shifters:

- Thermo-optic silicon: 100 μs per update × $K = N(N-1)/2$ phase shifters
- For $N = 64$: $\sim 2000$ phase shifters × 100 μs = 0.2 s per training step — catastrophically slow

Current photonic implementations are therefore **inference-only accelerators** trained digitally off-chip and then mapped to photonic hardware. This is a fundamental architectural constraint that motivates research into in-situ training and gradient estimation in photonic networks.

---

## 7. Worked Example: ONN on MNIST

**Setup (Shen et al., 2017):** A 2-layer optical neural network for MNIST digit classification (10 classes, 784-dimensional input).

**Architecture:**
- Input layer: PCA reduction 784 → 56 dimensions (done digitally)
- Layer 1: $56\times56$ optical matrix multiply (Clements mesh, $N=56$, $K=1540$ MZIs)
- Activation: OE nonlinearity (photodetect + ReLU + re-encode)
- Layer 2: $56\times10$ optical matrix multiply (rectangular Clements, $56\times10$)
- Output: detect 10 outputs, take argmax

**Performance:**
- Accuracy: 97.6% (GPU reference: 98.5%)
- Speedup: The optical multiply completes in $<1$ ps; bottleneck is OE conversion ($\sim 1$ ns)
- At $N = 56$: photonic advantage small (only $56^2 = 3136$ MACs per OE conversion)
- Advantage becomes compelling at $N > 10^3$

**Key paper finding:** Even with $\sigma = 0.02$ rad phase errors and shot noise at SNR = 10⁴, the accuracy degrades by only 0.9% relative to the ideal case — demonstrating the noise tolerance of neural network inference.

---

## 8. Exercises

**8.1** (Easy) Verify that the $2\times2$ MZI matrix $U(\theta,\phi)$ is unitary. How many real parameters does a general $2\times2$ unitary have, and how many does the MZI parameterize?

**8.2** (Easy) For $N=3$, how many MZIs are needed for a Reck decomposition? A Clements decomposition? What is the depth (number of layers) of each?

**8.3** (Medium) The singular value decomposition of a $4\times4$ real matrix requires two $4\times4$ unitary matrices and one diagonal matrix. Count the total number of MZIs needed. If each MZI occupies $100\, \mu m \times 100\, \mu m$, what is the total chip area?

**8.4** (Medium) Shot noise analysis: An optical MVM with $N = 64$ inputs operates at $P = 100\, \mu$W total optical power spread equally over 64 modes. The integration time per symbol is $T_\text{int} = 10$ ns. Calculate the SNR per output mode (after summing over the matrix multiplication), assuming perfect matrix and Poissonian photon statistics.

**8.5** (Medium) A thermo-optic phase shifter in silicon requires 10 mW for a $\pi$ phase shift. For a Clements mesh with $N = 32$, calculate (a) the total number of MZIs, (b) the worst-case static power if all MZIs are at $\pi/2$, (c) the power reduction if phase change material (zero static power, 5 pJ write energy per setting) is used instead, assuming 10⁹ inferences before retraining.

**8.6** (Hard) Noise-aware training: Suppose each MZI angle $\theta_k$ has additive Gaussian noise $\sigma = 0.01$ rad during inference. The network was trained assuming perfect hardware. Propose and analyze a noise-aware training procedure that could improve inference accuracy. What loss function modification is appropriate?

**8.7** (Hard) Implement (by hand or in Python/numpy) the Clements decomposition for a random $4\times4$ unitary matrix. Verify that the product of the MZI matrices recovers the original unitary to machine precision.

---

## 9. Further Reading

- **Foundational Paper:** Shen et al., "Deep learning with coherent nanophotonic circuits," *Nature Photon.* 11, 441–446 (2017)
- **SVD Architecture:** Reck et al., "Experimental realization of any discrete unitary operator," *Phys. Rev. Lett.* 73, 58 (1994)
- **Clements Decomposition:** Clements et al., "An optimal design for universal multiport interferometers," *Optica* 3, 1460 (2016)
- **Noise Analysis:** Bandyopadhyay et al., "Hardware error correction for programmable photonics," *Optica* 8, 1247 (2021)
- **Energy Analysis:** Nahmias et al., "Photonic multiply-accumulate operations for neural networks," *IEEE J. Sel. Topics Quantum Electron.* 26, 7701518 (2020)
- **Phase Change Materials:** Feldmann et al., "Parallel convolutional processing using an integrated photonic tensor core," *Nature* 589, 52 (2021)
