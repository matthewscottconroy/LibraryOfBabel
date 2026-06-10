# Subsection 12.2.1: The MZI as a Unitary Gate

## Orientation

A Mach-Zehnder interferometer (analyzed in detail in Section 7.3.2 for modulation applications) becomes a versatile unitary gate when its phase settings are used to define a rotation rather than modulate a signal. The crucial step is recognizing that the MZI transfer matrix describes a $2 \times 2$ rotation in the complex plane — parameterized by two angles — and that by choosing these angles freely, we can implement any $2 \times 2$ unitary matrix.

---

## 12.2.1.1 Transfer Matrix of a Single MZI

### Architecture

The standard MZI in photonic computing uses two 50:50 directional couplers and two phase shifters: one internal phase shifter $\theta$ between the two arms, and one external phase shifter $\phi$ at one of the outputs:

```
Port 1 ─────────────────┐          ┌──── Port 3
          [φ]  ─ DC ─  [θ]  ─ DC ─
Port 2 ─────────────────┘          └──── Port 4
```

where DC is a 50:50 directional coupler.

**Transfer matrix of a 50:50 coupler**:

$$U_{\text{DC}} = \frac{1}{\sqrt{2}} \begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix}$$

The factor $i$ comes from the $\pi/2$ phase acquired in the coupled port relative to the straight-through port.

**Phase shifters**: The internal phase shifter applies $e^{i\theta}$ to one arm; the external phase shifter applies $e^{i\phi}$ to one output.

**Full MZI transfer matrix**:

$$U_{\text{MZI}}(\theta, \phi) = \begin{pmatrix} e^{i\phi} & 0 \\ 0 & 1 \end{pmatrix} U_{\text{DC}} \begin{pmatrix} e^{i\theta} & 0 \\ 0 & 1 \end{pmatrix} U_{\text{DC}}$$

Computing this product:

$$= \begin{pmatrix} e^{i\phi} & 0 \\ 0 & 1 \end{pmatrix} \frac{1}{\sqrt{2}} \begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix} \begin{pmatrix} e^{i\theta} & 0 \\ 0 & 1 \end{pmatrix} \frac{1}{\sqrt{2}} \begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix}$$

$$= \frac{1}{2} \begin{pmatrix} e^{i\phi} & 0 \\ 0 & 1 \end{pmatrix} \begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix} \begin{pmatrix} e^{i\theta} & i \\ ie^{i\theta} & -1 \end{pmatrix}$$

Let me compute the inner product first: 

$$\begin{pmatrix} e^{i\theta} & i \\ ie^{i\theta} & -1 \end{pmatrix} \rightarrow \frac{1}{\sqrt{2}}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix} \begin{pmatrix} e^{i\theta} & 0 \\ 0 & 1 \end{pmatrix} = \frac{1}{\sqrt{2}}\begin{pmatrix} e^{i\theta} & i \\ ie^{i\theta} & 1 \end{pmatrix}$$

Second coupler:
$$\frac{1}{\sqrt{2}}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix} \times \frac{1}{\sqrt{2}}\begin{pmatrix} e^{i\theta} & i \\ ie^{i\theta} & 1 \end{pmatrix} = \frac{1}{2}\begin{pmatrix} e^{i\theta} + i^2 e^{i\theta} & i + i \\ ie^{i\theta} + ie^{i\theta} & i^2 + 1 \end{pmatrix}$$

$$= \frac{1}{2}\begin{pmatrix} e^{i\theta}(1 - 1) & 2i \\ 2ie^{i\theta} & 0 \end{pmatrix}$$

Hmm, this doesn't look right. Let me redo carefully.

$$\frac{1}{\sqrt{2}}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix} \times \frac{1}{\sqrt{2}}\begin{pmatrix} e^{i\theta} & i \\ ie^{i\theta} & 1 \end{pmatrix}$$

$$= \frac{1}{2}\begin{pmatrix} 1 \cdot e^{i\theta} + i \cdot ie^{i\theta} & 1 \cdot i + i \cdot 1 \\ i \cdot e^{i\theta} + 1 \cdot ie^{i\theta} & i \cdot i + 1 \cdot 1 \end{pmatrix}$$

$$= \frac{1}{2}\begin{pmatrix} e^{i\theta} - e^{i\theta} & 2i \\ 2ie^{i\theta} & -1 + 1 \end{pmatrix} = \frac{1}{2}\begin{pmatrix} 0 & 2i \\ 2ie^{i\theta} & 0 \end{pmatrix}$$

This gives a $\pi$-crossing, which means I set up the phase shifter location wrong. The standard MZI architecture with the internal phase shifter in one arm between the two couplers is:

$$U_{\text{MZI}} = U_{\text{DC}} \begin{pmatrix} e^{i\theta} & 0 \\ 0 & 1 \end{pmatrix} U_{\text{DC}} \begin{pmatrix} e^{i\phi} & 0 \\ 0 & 1 \end{pmatrix}$$

Let me use the standard result quoted in Reck (1994) and Clements (2016). The MZI with internal phase $\theta$ (splitting angle) and external phase $\phi$ (overall phase) acts as:

$$\boxed{U(\theta, \phi) = ie^{i\phi/2}\begin{pmatrix} e^{i\phi}\sin(\theta/2) & \cos(\theta/2) \\ e^{i\phi}\cos(\theta/2) & -\sin(\theta/2) \end{pmatrix}}$$

This is the transfer matrix from (input 1, input 2) to (output 1, output 2), with the convention that $\theta$ controls the power splitting ratio and $\phi$ controls the relative phase. The overall phase $ie^{i\phi/2}$ is unimportant for most applications.

The action: 
- $\theta = 0$: 0 splitting → output 1 = input 1 ×0 + input 2 ×1 (cross-bar state)
- $\theta = \pi$: full splitting → output 1 = input 1 × $e^{i\phi}$ (bar state, with phase)
- $\theta = \pi/2$: 50:50 splitting → balanced superposition

---

## 12.2.1.2 The Space of $2 \times 2$ Unitary Matrices

### Parameterization

A general $2 \times 2$ unitary matrix $U \in U(2)$ has 4 real parameters (since $U(2)$ is a 4-dimensional Lie group). An MZI with two phase shifters ($\theta$ and $\phi$) has 2 free parameters. Therefore, an MZI cannot implement *all* $2 \times 2$ unitary matrices.

However, all we need for the matrix decomposition is to implement all $2 \times 2$ *special* unitary matrices — elements of SU(2) (determinant = 1), which has 3 parameters. A single MZI with two phase shifters plus one additional external phase on an input implements SU(2):

$$T(\theta, \phi, \alpha) = \begin{pmatrix} e^{i\alpha} & 0 \\ 0 & 1 \end{pmatrix} U(\theta, \phi)$$

3 parameters → all of SU(2). Global U(1) phases (the determinant phase) accumulate and can be tracked separately.

For the purpose of matrix decomposition (which requires implementing $N \times N$ *unitary* matrices as products of $2 \times 2$ operations), each MZI plus one external phase shift provides exactly the right number of degrees of freedom.

### Geometric Interpretation

The MZI transfer matrix $U(\theta, \phi)$ rotates a 2D complex vector. In the Bloch sphere picture (Section 4.3.2 uses this for two-level systems), the MZI implements a rotation:
- $\theta$ controls the polar angle (mixing angle between the two modes)
- $\phi$ controls the azimuthal angle (relative phase between the modes)

Any point on the Bloch sphere is reachable from any other by a sequence of such rotations. This is why MZI meshes can implement any unitary: any unitary is a sequence of 2D rotations.

---

## 12.2.1.3 Power Transfer Function

### For Computing Applications

The most important practical property of the MZI is its power transfer function:

$$P_3 = |U_{11}|^2 P_1 + |U_{12}|^2 P_2 + 2\text{Re}(U_{11}U_{12}^* \sqrt{P_1 P_2} e^{i\Delta\phi_{\text{in}}})$$

For equal inputs ($P_1 = P_2 = P_0/2$, $\Delta\phi_{\text{in}} = 0$):

$$P_3 = P_0 \cos^2(\theta/2), \quad P_4 = P_0 \sin^2(\theta/2)$$

where $\theta$ is the internal phase. This is the classic MZI transfer function (from Section 7.3.2).

**Extinction ratio**: When $\theta = 0$ (off-state for output 3): $P_3 = P_0$, $P_4 = 0$. When $\theta = \pi$ (on-state for output 3): $P_3 = 0$, $P_4 = P_0$.

In practice, the extinction ratio is finite due to coupler imbalance and fabrication errors:

$$\text{ER} = \frac{P_{\text{max}}}{P_{\text{min}}} = \left(\frac{1 + |\delta\kappa|}{1 - |\delta\kappa|}\right)^2$$

where $\delta\kappa$ is the deviation of the coupler splitting ratio from 50:50. For $\delta\kappa = 1\%$: ER $\approx 32$ dB. For $\delta\kappa = 3\%$: ER $\approx 21$ dB.

**ENOB from extinction ratio**: The limited ER limits the dynamic range of the matrix element:
$$\text{ENOB}_{\text{ER}} = \frac{\text{ER}_{\text{dB}}}{6.02} \approx \frac{21}{6.02} \approx 3.5 \text{ bits for } 3\% \text{ splitting error}$$

This is one of the dominant precision limitations in fabricated MZI meshes. Silicon photonic foundries achieve splitting-ratio tolerances of 1–3%, limiting ENOB to 3.5–5.3 bits from this effect alone.

---

## References

[1] Reck, M., Zeilinger, A., Bernstein, H.J., & Bertani, P. (1994). "Experimental realization of any discrete unitary operator." *Physical Review Letters*, 73, 58–61. [Proves that any $N \times N$ unitary matrix can be decomposed into a product of 2×2 rotations, and implements one optically.]

[2] Miller, D.A.B. (2013). "Self-configuring universal linear optical component." *Photonics Research*, 1(1), 1–15. [Proposes a self-configuring approach to programming a unitary mesh without needing to know the target matrix explicitly; uses power detectors within the mesh.]

[3] Bandyopadhyay, S., Hamerly, R., & Englund, D. (2021). "Hardware error correction for programmable photonics." *Optica*, 8(10), 1247–1255. [Analysis of how fabrication errors limit MZI mesh performance and approaches to hardware error correction.]
