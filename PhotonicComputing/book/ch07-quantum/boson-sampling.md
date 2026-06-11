# Quantum Photonics: Single Photons, Entanglement, and Boson Sampling

> *A photon is a remarkably good qubit. It travels at the speed of light, interacts weakly with its environment (meaning it decoheres slowly), and carries information in multiple degrees of freedom: polarization, spatial mode, temporal mode, frequency. The challenge is making photons interact with each other — which they don't do in vacuum. This chapter covers the physics of quantum photonic systems, from the generation of single photons to the computational hardness result underlying boson sampling.*

---

## 1. Single-Photon Sources

### 1.1 What Makes a Good Single-Photon Source?

A perfect single-photon source emits exactly one photon on demand, into a well-defined optical mode, with high efficiency. The key figures of merit:

- **Single-photon purity $g^{(2)}(0)$:** The second-order correlation function at zero delay. For a true single-photon source, $g^{(2)}(0) = 0$. A coherent (laser) source has $g^{(2)}(0) = 1$. A thermal source has $g^{(2)}(0) = 2$. Values $g^{(2)}(0) < 0.5$ are taken as the threshold for "quantum" behavior.

- **Indistinguishability $M$:** How identical consecutive photons are. For quantum interference (the Hong-Ou-Mandel effect, see below), photons must be indistinguishable: same frequency, polarization, spatial mode, and temporal wavepacket. $M = 1$ means perfectly indistinguishable; $M < 1$ arises from dephasing (spectral wandering, phonon sidebands).

- **Brightness (efficiency) $\eta$:** The probability that a trigger pulse produces a photon in the collection mode. For boson sampling and photonic quantum computing, we need $\eta \to 1$.

### 1.2 Quantum Dot Single-Photon Sources

A semiconductor quantum dot (QD) is a nanoscale region ($\sim 20$ nm) of lower-bandgap material embedded in a higher-bandgap matrix. Quantum confinement creates discrete electronic energy levels — effectively an artificial atom.

**Operating principle:** An excitation pulse (laser or electrical pulse) creates an exciton (bound electron-hole pair) in the QD. The exciton decays radiatively by emitting one photon whose energy equals the transition energy $\hbar\omega_0 = E_\text{gap} + E_\text{confinement}$.

Because the QD can only hold one exciton at a time (the second exciton would require significantly higher energy due to Coulomb repulsion), only one photon is emitted per excitation. Hence $g^{(2)}(0) \approx 0$.

**Photonic crystal cavity coupling:** The spontaneous emission rate of the QD is enhanced by the Purcell effect when the QD is placed in a photonic crystal cavity or pillar microcavity resonant with the transition. The Purcell factor:

$$F_P = \frac{3}{4\pi^2}\left(\frac{\lambda}{n}\right)^3 \frac{Q}{V}$$

where $Q$ is the cavity quality factor and $V$ is the mode volume. For $F_P \gg 1$, the emission rate into the cavity mode exceeds the total free-space emission rate, giving near-unity collection efficiency. Demonstrated values: $F_P > 50$, $\eta > 0.95$, $M > 0.99$ in state-of-the-art GaAs quantum dot systems.

**Limitation:** Quantum dots are spectrally inhomogeneous — different dots have different transition frequencies (due to size and composition fluctuations). Tuning via strain piezoelectric or Stark effect can compensate, but multi-emitter experiments require careful matching.

### 1.3 Spontaneous Parametric Down-Conversion (SPDC)

SPDC is a nonlinear optical process in which a single pump photon ($\omega_p$) spontaneously splits into two lower-energy photons, signal ($\omega_s$) and idler ($\omega_i$), in a crystal with second-order nonlinearity ($\chi^{(2)}$).

**Energy and momentum conservation:**

$$\omega_p = \omega_s + \omega_i \quad \text{(energy)}$$
$$\mathbf{k}_p = \mathbf{k}_s + \mathbf{k}_i \quad \text{(phase matching)}$$

For degenerate SPDC: $\omega_s = \omega_i = \omega_p/2$.

**The two-photon state:**

The SPDC output is not a single photon but a quantum state of the electromagnetic field. In the low-gain limit (weak pump), the state is:

$$|\psi\rangle = \sqrt{1-|\xi|^2}\left(|0,0\rangle + \xi |1_s, 1_i\rangle + \xi^2 |2_s, 2_i\rangle + \cdots\right)$$

where $\xi$ is proportional to the pump amplitude and $\chi^{(2)}$, and $|n_s, n_i\rangle$ denotes $n$ photons in each mode. For small $|\xi|$: predominantly vacuum, occasionally one pair ($\xi$), rarely two pairs ($\xi^2$).

**Why SPDC is not a perfect single-photon source:** The multi-pair term ($\xi^2$) contributes to $g^{(2)}(0) > 0$. For boson sampling, SPDC sources are commonly used with $\xi^2 \ll \xi$ (low pump power), accepting low brightness in exchange for high purity. There is a fundamental trade-off between brightness and multi-photon contamination in SPDC sources.

**Hong-Ou-Mandel effect:** If two identical photons (same frequency, polarization, spatial mode) arrive simultaneously at the two input ports of a 50/50 beamsplitter, they always exit together (bunching) — they are never found in different output ports. The HOM dip in coincidence rate as a function of delay is the signature of photon indistinguishability. This is a purely quantum phenomenon (no classical analogue) and the basis of photonic quantum gates.

---

## 2. Linear Optical Quantum Computing: The KLM Theorem

### 2.1 The No-Go Intuition

Photons don't naturally interact with each other. A linear optical network (beamsplitters, phase shifters) can only implement linear transformations on the optical field amplitudes — which is unitary evolution of the mode operators. Two-qubit gates (like CNOT) require a nonlinear interaction between photons.

The question Knill, Laflamme, and Milburn (KLM, 2001) asked: can we perform universal quantum computation using only linear optics, single-photon sources, and single-photon detectors (which are nonlinear in the quantum sense — they perform a projective measurement)?

### 2.2 The KLM Theorem

**Theorem (Knill, Laflamme, Milburn, 2001):** Efficient universal quantum computation is possible with linear optics, single-photon sources, and photon-number-resolving detectors, using **ancilla photons** and **feed-forward** (conditional operations based on measurement outcomes).

**Key idea:** Measurement in quantum mechanics is inherently nonlinear — it projects the quantum state. By measuring ancilla photons in a carefully designed circuit and then applying corrective linear optic operations conditional on the measurement outcome, effective two-photon gates can be implemented.

**The cost:** The KLM scheme requires quantum gates that succeed only with probability $p < 1$. Non-deterministic gates can be made near-deterministic by using teleportation — but teleportation requires entangled resource states, which require more single photons. The resource overhead scales polynomially with the desired gate success probability.

**Practical implication:** The KLM theorem established that linear optical quantum computing (LOQC) is theoretically possible but resource-intensive. Fault-tolerant LOQC with the overhead acceptable for practical computation requires improvements in single-photon efficiency ($\eta > 0.999$), detector efficiency ($> 0.999$), and photon indistinguishability ($M > 0.999$) not yet achieved.

---

## 3. Boson Sampling

### 3.1 The Problem

Boson sampling, proposed by Aaronson and Arkhipov (2011), is a computational problem that is:

1. **Easy to implement physically** — it is just scattering $n$ photons through an $m$-mode linear optical network.
2. **Believed to be classically hard** — simulating the output distribution requires computing permanents of complex matrices, which is #P-hard.

The boson sampling problem: given a random $m\times m$ unitary $U$ (where $m \gg n$), and $n$ single photons in a specific input configuration, sample from the output photon-number distribution.

### 3.2 Why It Is Hard: The Permanent

The probability amplitude for a specific output configuration $\mathbf{s} = (s_1, s_2, \ldots, s_m)$ (where $s_j$ is the number of photons in output mode $j$) given input configuration $\mathbf{r} = (r_1, \ldots, r_m)$ is:

$$\langle \mathbf{s} | U^{\otimes n} | \mathbf{r} \rangle = \frac{\text{Perm}(U_{\mathbf{r},\mathbf{s}})}{\sqrt{r_1! \cdots r_m! \cdot s_1! \cdots s_m!}}$$

where $U_{\mathbf{r},\mathbf{s}}$ is the $n\times n$ submatrix of $U$ formed by selecting rows corresponding to input modes and columns to output modes (with repetitions for multiply occupied modes), and $\text{Perm}(A)$ is the **permanent** of the matrix:

$$\text{Perm}(A) = \sum_{\sigma \in S_n} \prod_{i=1}^n A_{i,\sigma(i)}$$

The permanent looks like the determinant (which appears in fermionic scattering) but without the sign factors. The determinant can be computed in $O(n^3)$ using Gaussian elimination. The permanent is #P-hard — believed to require exponential time on classical computers.

**Intuition for the hardness:** The permanent counts the number of perfect matchings in a bipartite graph. The sign cancellations that make the determinant tractable (Leibniz formula with $(-1)^\sigma$ signs) are absent. Bosons (symmetric under exchange) contribute all permutations with equal sign; the resulting interference is complex and hard to simulate.

**The Clifton-Brod theorem:** Unless the polynomial hierarchy collapses to its third level (considered extremely unlikely), boson sampling with $n$ photons in $m \gg n^2$ modes cannot be simulated classically in polynomial time.

### 3.3 Approximate Hardness

The Aaronson-Arkhipov hardness result applies to exact sampling. For approximate boson sampling (sampling from a distribution within total variation distance $\epsilon$ of the true distribution), the hardness holds under a plausible conjecture about the #P-hardness of approximating permanents of random Gaussian matrices (the "permanent-of-Gaussians" conjecture).

This conjecture has not been proven, and this is an active area of theoretical computer science.

### 3.4 Experimental Demonstrations

Boson sampling experiments have scaled rapidly:

| Year | Group | Photons ($n$) | Modes ($m$) | Platform |
|---|---|---|---|---|
| 2013 | Oxford/Vienna/Queensland | 3 | 6 | Bulk optics |
| 2019 | Google/Jiuzhang | 5 | 9 | Integrated |
| 2020 | USTC (Jiuzhang 1.0) | 50-76 | 100 | Bulk SPDC |
| 2021 | USTC (Jiuzhang 2.0) | 113 | 144 | Bulk SPDC |
| 2022 | USTC (Borealis) | 216 | ~700 | Fiber + time-bin |

The Jiuzhang 1.0 result (2020, Pan group) reported output sampling rates $10^{14}\times$ faster than classical supercomputer simulation — a claimed quantum computational advantage. However, the theoretical hardness arguments for SPDC-based Gaussian boson sampling (used by Jiuzhang) are more subtle than for the original Aaronson-Arkhipov formulation, and classical simulation algorithms have continued to improve.

**Gaussian Boson Sampling (GBS):** Instead of Fock-state inputs ($n$ photons in specific modes), GBS uses squeezed states as inputs. The output probability distribution is related to the Hafnian (a generalization of the permanent) rather than the permanent itself. The Hafnian is also #P-hard, and GBS may be more efficient to implement experimentally using SPDC.

---

## 4. Photonic Entanglement Generation

### 4.1 Polarization Entanglement from SPDC

Type-II SPDC (or type-I in a pair of crystals) produces polarization-entangled photon pairs:

$$|\Phi^+\rangle = \frac{1}{\sqrt{2}}(|H\rangle_s|H\rangle_i + |V\rangle_s|V\rangle_i)$$

This is a **Bell state** — maximally entangled. The photons are correlated in polarization: if you measure one as $H$, the other is $H$; if one is $V$, the other is $V$, regardless of the spatial separation of the photons when measured.

Bell inequality violation (Clauser-Horne-Shimony-Holt):

$$S = |E(\mathbf{a},\mathbf{b}) - E(\mathbf{a},\mathbf{b}') + E(\mathbf{a}',\mathbf{b}) + E(\mathbf{a}',\mathbf{b}')| \leq 2$$

Quantum mechanics predicts $S = 2\sqrt{2} \approx 2.83 > 2$. Experiments confirm $S > 2$, ruling out local hidden variable theories.

### 4.2 Cluster State Generation

For measurement-based quantum computing (one-way QC), the resource is a **cluster state** — a highly entangled multi-photon state where all parties are entangled in a specific graph pattern.

A 2D cluster state enables universal quantum computation: single-qubit measurements on the cluster implement quantum gates, with the direction of measurement determining the gate applied.

Photonic cluster state generation:
1. Prepare single photons (from QD sources or SPDC heralding)
2. Apply fusion gates (partial Bell measurements) to merge independent photon pairs into a cluster
3. Type-II fusion: success probability $p = 1/2$ (linear optics)
4. Resource overhead: $\sim 10$–100 ancilla photons per logical qubit per gate

### 4.3 Graph States and Raussendorf-Briegel Model

The Raussendorf-Briegel (2001) model of one-way quantum computation: start with a cluster state $|G\rangle$ associated with a graph $G = (V, E)$. The state is:

$$|G\rangle = \prod_{(u,v)\in E} CZ_{uv} |+\rangle^{\otimes |V|}$$

where $CZ_{uv}$ is a controlled-Z gate between vertices $u$ and $v$, and $|+\rangle = (|0\rangle + |1\rangle)/\sqrt{2}$.

Measurement of qubit $v$ in the basis $\{|+\theta\rangle, |-\theta\rangle\}$ where $|\pm\theta\rangle = (|0\rangle \pm e^{i\theta}|1\rangle)/\sqrt{2}$ implements a single-qubit rotation by angle $\theta$ on the neighboring qubits, with a byproduct Pauli correction depending on the measurement outcome.

---

## 5. Continuous-Variable Quantum Information

### 5.1 Quadrature Variables

Instead of discrete photon numbers (Fock states), quantum information can be encoded in the continuous eigenvalues of the quadrature operators:

$$\hat{x} = \frac{1}{2}(\hat{a} + \hat{a}^\dagger), \qquad \hat{p} = \frac{1}{2i}(\hat{a} - \hat{a}^\dagger)$$

where $\hat{a}, \hat{a}^\dagger$ are the photon annihilation and creation operators. These satisfy $[\hat{x}, \hat{p}] = i/2$ (leading to the uncertainty relation $\Delta x \cdot \Delta p \geq 1/4$).

Gaussian states (coherent states, squeezed states, thermal states) are fully characterized by their first and second moments (displacement vector and covariance matrix). Gaussian operations (linear optics + squeezing) map Gaussian states to Gaussian states.

### 5.2 Squeezed States and Quantum Advantage

Squeezing reduces the quantum noise in one quadrature below the vacuum noise, at the expense of increased noise in the conjugate quadrature:

$$|\text{squeezed}\rangle: \quad \Delta x = \frac{1}{2}e^{-r}, \quad \Delta p = \frac{1}{2}e^{+r}$$

where $r$ is the squeezing parameter. The product $\Delta x \cdot \Delta p = 1/4$ is saturated (minimum uncertainty state).

Squeezed states are essential for continuous-variable quantum key distribution (CV-QKD), continuous-variable quantum teleportation (Furusawa et al., 1998), and continuous-variable boson sampling (GBS). Squeezing of 15 dB ($r \approx 1.7$) has been demonstrated in OPO cavities.

### 5.3 CV Quantum Gates

A universal set of CV quantum gates:
- **Displacement $D(\alpha)$:** Shifts the state in phase space: $|x\rangle \to |x + \text{Re}(\alpha)\rangle$. Implemented by a laser field driving the mode.
- **Phase rotation $R(\theta)$:** Rotates phase space: $\hat{a} \to e^{i\theta}\hat{a}$. A phase shifter.
- **Squeezing $S(r)$:** Implemented by an optical parametric amplifier.
- **Beamsplitter $B(\theta)$:** Mixes two modes. Linear optical element.
- **Cubic phase gate $V(\gamma)$:** $|\psi\rangle \to e^{i\gamma\hat{x}^3}|\psi\rangle$. Non-Gaussian; needed for universality. Very difficult to implement physically.

The first four gates are Gaussian and do not provide quantum advantage over classical simulation (Gottesman-Knill theorem for CV systems). The cubic phase gate makes the set universal.

---

## 6. Error Correction in Photonic Systems

### 6.1 Photon Loss: The Dominant Error

Unlike superconducting qubits (where decoherence is depolarizing noise), the dominant error in photonic quantum computing is **photon loss**: a photon is absorbed or scattered before reaching the detector.

Photon loss can be modeled as a beamsplitter with transmission $\eta$:

$$\hat{a} \to \sqrt{\eta}\hat{a} + \sqrt{1-\eta}\hat{b}_\text{env}$$

where $\hat{b}_\text{env}$ is an environmental mode. For $\eta < 1$, the photon can be lost into the environment.

Sources of photon loss:
- Waveguide propagation loss: 0.1–5 dB/cm (silicon), 0.003 dB/cm (ultrapure silica fiber)
- Fiber-chip coupling: 1–3 dB per interface
- MZI insertion loss: 0.5–2 dB per MZI
- Detector efficiency: $\eta_d < 1$ (superconducting nanowire single-photon detectors: $\eta_d > 0.95$)

### 6.2 Gottesman-Kitaev-Preskill (GKP) Encoding

The GKP code encodes a qubit in the quadrature space of a harmonic oscillator. The code words are approximate grid states:

$$|0_L\rangle \propto \sum_{n=-\infty}^{\infty} |2n\sqrt{\pi}\rangle_x, \qquad |1_L\rangle \propto \sum_{n=-\infty}^{\infty} |(2n+1)\sqrt{\pi}\rangle_x$$

The code is designed to correct small displacements in phase space (which model photon loss and Gaussian noise). GKP codes are well-suited to CV photonic systems.

**Error correction condition:** A displacement error $\delta x < \sqrt{\pi}/2$ in $x$ or $\delta p < \sqrt{\pi}/2$ in $p$ can be corrected. Larger errors are catastrophic.

### 6.3 Percolation-Based Fault-Tolerant LOQC

The leading architecture for fault-tolerant linear optical quantum computing uses **photonic fusion networks** and percolation:

1. Generate small entangled resource states (e.g., 4-photon GHZ states) using probabilistic fusion gates
2. Attempt to fuse resource states into a cluster state; each fusion succeeds with probability $p_\text{fusion} = 1/2$
3. Above a percolation threshold $p_c$, a spanning cluster forms with high probability — enabling universal computation despite individual gate failures
4. Threshold: $p_c \approx 0.5$ for a 3D cluster (above $p_\text{fusion} = 0.5$) — marginally achievable

The Photonic Inc. and PsiQuantum architectures (2024) both follow variants of this approach, targeting fault-tolerant quantum computing in Si photonics using single-photon sources and SNSPDs.

---

## 7. Worked Example: Hong-Ou-Mandel Visibility

**Setup:** Two photons, one in mode $a$ and one in mode $b$, meet at a 50/50 beamsplitter. The beamsplitter transformation is:

$$\hat{a}^\dagger \to \frac{1}{\sqrt{2}}(\hat{c}^\dagger + i\hat{d}^\dagger), \qquad \hat{b}^\dagger \to \frac{1}{\sqrt{2}}(i\hat{c}^\dagger + \hat{d}^\dagger)$$

**Input state:** $|\psi_\text{in}\rangle = \hat{a}^\dagger \hat{b}^\dagger |0\rangle = |1_a, 1_b\rangle$.

**After the beamsplitter:**

$$\hat{a}^\dagger \hat{b}^\dagger |0\rangle \to \frac{1}{\sqrt{2}}(\hat{c}^\dagger + i\hat{d}^\dagger)\cdot\frac{1}{\sqrt{2}}(i\hat{c}^\dagger + \hat{d}^\dagger)|0\rangle$$

$$= \frac{1}{2}(i(\hat{c}^\dagger)^2 + \hat{c}^\dagger\hat{d}^\dagger - \hat{c}^\dagger\hat{d}^\dagger + i(\hat{d}^\dagger)^2)|0\rangle = \frac{i}{2}((\hat{c}^\dagger)^2 + (\hat{d}^\dagger)^2)|0\rangle$$

$$= \frac{i}{\sqrt{2}}(|2,0\rangle + |0,2\rangle)$$

The cross terms $\hat{c}^\dagger\hat{d}^\dagger|0\rangle$ cancel exactly! The probability of detecting one photon in each output is zero. Both photons exit together into the same port.

This is the HOM effect. The visibility of the HOM dip:

$$V = \frac{R_\text{classical} - R_\text{coincidence}}{R_\text{classical}} = 1 \quad \text{(perfectly indistinguishable photons)}$$

For partially distinguishable photons (indistinguishability $M < 1$):

$$V = M \cdot \eta_\text{source}^2 \cdot \eta_\text{detector}^2$$

Measured HOM visibilities: $V > 0.99$ for the best quantum dot sources.

---

## 8. Exercises

**8.1** (Easy) What is $g^{(2)}(0)$ for: (a) a perfect single-photon source, (b) a coherent state (laser), (c) a thermal state? What physical distinction does $g^{(2)}(0) < 1$ indicate?

**8.2** (Easy) For SPDC in the low-gain regime with $|\xi| = 0.1$, estimate the probability of generating: (a) a single pair, (b) two pairs simultaneously. Comment on the effect of the two-pair term on $g^{(2)}(0)$.

**8.3** (Medium) Calculate the permanent of the $2\times2$ matrix $A = \begin{pmatrix}a & b \\ c & d\end{pmatrix}$. Compare with the determinant. For a random complex $3\times3$ matrix, write out the sum for the permanent explicitly (6 terms). Estimate the complexity of computing the permanent of an $n\times n$ matrix by brute force.

**8.4** (Medium) The boson sampling output probability involves the permanent of a submatrix of the unitary $U$. For $n = 3$ photons in $m = 6$ modes, how many distinct output configurations are there? How does this scale with $n$ and $m$?

**8.5** (Medium) In the HOM effect, suppose one photon has a spectral bandwidth $\Delta\omega$ (Gaussian envelope) and the other is monochromatic. Show that the HOM visibility depends on the photon overlap integral. For a Gaussian wavepacket, derive the visibility as a function of temporal delay $\tau$ between the photon arrival times.

**8.6** (Hard) The KLM CNOT gate uses a non-deterministic linear-optical gate with success probability $p$. To achieve near-deterministic CNOT via gate teleportation, one needs an entangled ancilla prepared off-line. Show that if individual gate success probability is $p = 1/2$, the resource overhead (number of ancilla photons) to achieve $k$ CNOT gates with failure probability $< \epsilon$ scales as $O(k \log(1/\epsilon))$.

**8.7** (Hard) Derive the GBS output probability for a two-mode squeezed vacuum state $|\text{TMSV}\rangle = \sqrt{1-\lambda^2}\sum_{n=0}^\infty \lambda^n |n,n\rangle$ entering a 50/50 beamsplitter. What is the probability of detecting $n$ photons in each output port?

---

## 9. Further Reading

- **KLM Original Paper:** Knill, Laflamme, Milburn, "A scheme for efficient quantum computation with linear optics," *Nature* 409, 46 (2001)
- **Boson Sampling:** Aaronson & Arkhipov, "The Computational Complexity of Linear Optics," *STOC* 2011; also arXiv:1011.3245
- **GBS Experiments:** Zhong et al. (USTC), "Quantum computational advantage using photons," *Science* 370, 1460 (2020)
- **GKP Codes:** Gottesman, Kitaev, Preskill, "Encoding a qubit in an oscillator," *Phys. Rev. A* 64, 012310 (2001)
- **Fusion-Based QC:** Bartolucci et al., "Fusion-based quantum computation," *Nature Commun.* 14, 912 (2023)
- **Quantum Dot Sources:** Tomm et al., "A bright and fast source of coherent single photons," *Nature Nanotechnol.* 16, 399 (2021)
- **Review:** O'Brien, "Optical quantum computing," *Science* 318, 1567 (2007)
