# Subsection 12.2.2: The Reck Decomposition

## Orientation

The Reck decomposition (Reck, Zeilinger, Bernstein, Bertani, 1994) is the foundational theoretical result for optical matrix computing: it proves that any $N \times N$ unitary matrix can be implemented by a triangular mesh of $N(N-1)/2$ MZIs arranged in a specific pattern. Understanding this decomposition from first principles — not just accepting the result — is essential for designing, programming, and debugging photonic matrix processors.

---

## 12.2.2.1 The Mathematical Foundation: Givens Rotations

### Givens Rotation

A Givens rotation $G(p, q, \theta)$ for $p < q$ is an $N \times N$ matrix that is the identity everywhere except in the $(p,p)$, $(p,q)$, $(q,p)$, $(q,q)$ positions:

$$G(p, q, \theta) = \begin{pmatrix} I_{p-1} & & & \\ & \cos\theta & \cdots & -\sin\theta & \\ & \vdots & I_{q-p-1} & \vdots & \\ & \sin\theta & \cdots & \cos\theta & \\ & & & & I_{N-q} \end{pmatrix}$$

Multiplying on the left by $G(p,q,\theta)$ rotates rows $p$ and $q$; multiplying on the right rotates columns $p$ and $q$.

**Connection to MZI**: The $2 \times 2$ submatrix of $G(p,q,\theta)$ acting on modes $p$ and $q$ is exactly the MZI transfer matrix (up to phase factors):

$$\begin{pmatrix} \cos\theta & -\sin\theta \\ \sin\theta & \cos\theta \end{pmatrix} = \begin{pmatrix} \cos\theta & -\sin\theta \\ \sin\theta & \cos\theta \end{pmatrix}$$

(Real-valued case; the complex generalization allows $\cos\theta \rightarrow e^{i\phi}\cos\theta$ etc.) A Givens rotation is what an MZI computes when acting on two specific modes of an $N$-mode system.

### Decomposing a Unitary Matrix

**Theorem (Reck 1994)**: Any $N \times N$ unitary matrix $U$ can be written as a product of $N(N-1)/2$ Givens rotations (each parameterized by two angles $\theta$ and $\phi$) and a diagonal phase matrix:

$$U = D \cdot G_{1,2}(\theta_{1,2}, \phi_{1,2}) \cdot G_{2,3} \cdots G_{N-1,N}(\theta_{N-1,N}, \phi_{N-1,N})$$

where $D = \text{diag}(e^{i\psi_1}, \ldots, e^{i\psi_N})$ contains the remaining diagonal phases.

**Proof sketch**: 
1. Start with an arbitrary unitary $U$.
2. Multiply on the right by a Givens rotation $G_{1,2}(\theta, \phi)$ to zero out element $U_{12}$.
3. Multiply by $G_{1,3}$ to zero out $U_{13}$... and so on until the first column of the accumulated product has the form $(e^{i\psi}, 0, 0, \ldots, 0)^T$.
4. Repeat for each column until the product is diagonal.

Since each step multiplies by a unitary (Givens rotation), the result is unitary at every step. The sequence of Givens rotations applied is the decomposition.

---

## 12.2.2.2 The Reck Mesh Topology

### Triangular Arrangement

The Reck decomposition arranges MZIs in a triangular mesh. For $N = 4$:

```
Mode 1: ──────────────────────────────── 
Mode 2: ──[MZI]──────────────────────── 
Mode 3: ──[MZI]──[MZI]─────────────────
Mode 4: ──[MZI]──[MZI]──[MZI]──────────
```

More precisely:

```
       Stage 1    Stage 2    Stage 3
M1: ───────────────────────────────────
M2: ──[T₁₂]───────────────────────────
M3: ──[T₁₃]──[T₂₃]────────────────────
M4: ──[T₁₄]──[T₂₄]──[T₃₄]────────────
```

Wait, the Reck scheme for $N = 4$ goes column by column, zeroing elements in each column:

Column 1: Apply MZI(1,2), MZI(1,3), MZI(1,4) to zero $u_{12}$, $u_{13}$, $u_{14}$

The physical arrangement is a triangular mesh where the number of MZIs in each column increases from 1 (for the last column pair) to $N-1$ (for the first column pair).

For $N$ modes:
- Total MZIs: $N(N-1)/2$
- Depth (number of sequential MZI layers): $2N - 3$ (the bottleneck is the longest diagonal path)
- The triangular shape means the physical chip is roughly triangular

---

## 12.2.2.3 Programming the Reck Mesh

### Algorithm

Given a target unitary matrix $U$, the phase angles $(\theta_{ij}, \phi_{ij})$ for each MZI are found by the following procedure:

1. For columns $j = 1, 2, \ldots, N-1$:
   - For rows $i = N, N-1, \ldots, j+1$:
     - Find $(\theta_{ij}, \phi_{ij})$ such that element $(i, j)$ of the accumulated product becomes zero.
     - This is a 2-element system: the phase angle $\phi_{ij}$ sets the argument of the target element, and $\theta_{ij}$ sets its magnitude to zero.

2. After all off-diagonal elements are zeroed, the remaining diagonal phase matrix $D$ gives the external phases.

**Explicit formulas**: For the $(i,j)$ element $u_{ij} = |u_{ij}|e^{i\psi_{ij}}$, the MZI parameters are:

$$\theta_{ij} = -2\arctan\left(\frac{|u_{ij}|}{|u_{i-1,j}|}\right)$$
$$\phi_{ij} = \arg(u_{ij}) - \arg(u_{i-1,j}) + \pi/2$$

(These are approximate; the exact formulas depend on the MZI convention and the order of operations.)

### Calibration Requirement

In practice, the manufactured MZI has imperfect 50:50 couplers. The actual transfer function deviates from the ideal, causing a systematic error in each element of the implemented matrix. Calibration corrects for this: by measuring the actual response of each MZI to a known input and adjusting the programmed angles accordingly, the effective matrix can be corrected.

Calibration requires access to the internal fields — either through additional monitor ports in the mesh or by inference from the output statistics. For an $N = 32$ mesh with $\sim 500$ MZIs, calibration takes $\sim 500 \times T_{\text{measurement}} \approx 1$–5 seconds at 10–50 ms per MZI measurement.

---

## 12.2.2.4 Performance Limitations

### Loss Accumulation

Each MZI has insertion loss $L_{\text{MZI}}$ (dB). For a signal traversing $d = 2N-3$ MZI layers, the total loss:

$$L_{\text{total}} = (2N-3) \times L_{\text{MZI, dB}}$$

For $N = 32$, $L_{\text{MZI}} = 0.5$ dB: $L_{\text{total}} = 61 \times 0.5 = 30.5$ dB. The signal at the output is 1000× weaker than at the input! This is a severe problem: the optical power budget requires either:
(a) A very high input power (pushing modulators beyond their linear range), or
(b) Optical amplifiers within the mesh (adding noise and complexity).

This loss accumulation is one of the two primary reasons the Reck decomposition is impractical for large $N$. The Clements decomposition (Subsection 12.2.3) addresses this by reducing the depth from $2N-3$ to $N$.

### Phase Error Sensitivity

The output matrix element accuracy depends on the phase setting accuracy. For a random phase error $\delta\phi$ on each MZI (Gaussian with standard deviation $\sigma_\phi$):

$$\|U_{\text{actual}} - U_{\text{ideal}}\|_F \approx \sqrt{N(N-1)/2} \cdot \sigma_\phi$$

For $N = 32$ and $\sigma_\phi = 0.01$ rad (achievable with 8-bit DAC control of thermo-optic heaters):
$$\|U - U_{\text{ideal}}\|_F \approx \sqrt{496} \times 0.01 \approx 0.22$$

The Frobenius norm of the error is ~0.22 in a matrix with $N = 32$ elements per row. The per-element error is $0.22/\sqrt{32} \approx 0.04$, corresponding to about 4.6 bits of matrix element precision. This is consistent with the ENOB analysis in Subsection 12.1.2.

---

## References

[1] Reck, M., Zeilinger, A., Bernstein, H.J., & Bertani, P. (1994). "Experimental realization of any discrete unitary operator." *Physical Review Letters*, 73, 58–61. [The original decomposition; demonstrated experimentally with 3 MZIs implementing a $3 \times 3$ unitary.]

[2] Mower, J., Harris, N.C., Steinbrecher, G.R., Lahini, Y., & Englund, D. (2015). "High-fidelity quantum state evolution in imperfect photonic integrated circuits." *Physical Review A*, 92, 032322. [Analysis of how fabrication imperfections affect the implemented unitary; derives the error sensitivity formulas.]

[3] Pai, S., et al. (2019). "Matrix optimization on universal unitary photonic devices." *Advanced Photonics*, 1(6), 066001. [Optimization-based programming of MZI meshes; demonstrates how to correct for fabrication imperfections during programming.]
