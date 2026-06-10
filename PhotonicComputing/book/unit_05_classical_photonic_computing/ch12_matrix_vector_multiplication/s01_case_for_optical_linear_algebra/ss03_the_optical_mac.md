# Subsection 12.1.3: The Optical Multiply-Accumulate (MAC)

## Orientation

We have established that optical systems perform matrix-vector products naturally and that the energy argument for them depends strongly on the application context. Now we need to understand what one multiply-accumulate (MAC) operation looks like physically in an optical system — both the coherent (complex field) implementation and the incoherent (intensity) implementation.

---

## 12.1.3.1 The Coherent MAC

### Optical Field as a Complex Number

A monochromatic optical field at a single spatial mode and frequency can be written as:

$$E(t) = \text{Re}[A e^{i(\omega t - kz)}] = \text{Re}[Ae^{i\omega t}]$$

where $A = |A|e^{i\phi}$ is the complex amplitude. In a photonic integrated circuit, the optical field at a given waveguide port at a given time is fully described by this complex number $A$.

A beam splitter (50:50 directional coupler) takes two input fields $A_1, A_2$ and produces two output fields:

$$\begin{pmatrix} B_1 \\ B_2 \end{pmatrix} = \frac{1}{\sqrt{2}} \begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix} \begin{pmatrix} A_1 \\ A_2 \end{pmatrix}$$

The factor $i$ accounts for the $\pi/2$ phase shift in the coupled port.

A phase shifter in one arm multiplies the field by $e^{i\phi}$:
$$B = A e^{i\phi}$$

An MZI (two 50:50 couplers + one phase shifter) transforms:

$$\begin{pmatrix} B_1 \\ B_2 \end{pmatrix} = \begin{pmatrix} \cos(\theta/2) e^{i\phi} & i\sin(\theta/2) e^{i\phi} \\ i\sin(\theta/2) & \cos(\theta/2) \end{pmatrix} \begin{pmatrix} A_1 \\ A_2 \end{pmatrix}$$

(The exact form depends on where the phase shifter is placed.) Each output is a linear combination of the inputs with complex coefficients — a complex-valued multiply-accumulate.

### The Inner Product

For the specific case of optical interference between two fields: at a beam combiner (50:50 coupler), output 1 is:

$$B_1 = \frac{1}{\sqrt{2}}(A_1 + iA_2)$$

The intensity at a photodetector:
$$I_1 = |B_1|^2 = \frac{1}{2}|A_1|^2 + \frac{1}{2}|A_2|^2 + \text{Re}(iA_1^* A_2)$$

The third term is the interference term: $\text{Re}(iA_1^* A_2) = -\text{Im}(A_1^* A_2)$. For balanced detection (output 1 minus output 2):

$$I_1 - I_2 = \text{Re}(iA_1^* A_2 - iA_1 A_2^*) = -2\text{Im}(A_1^* A_2)$$

This is the imaginary part of the inner product $A_1^* A_2$. By adding a $\pi/2$ phase shift to one input, we get the real part. Together, balanced coherent detection gives us both the real and imaginary parts of the complex inner product $A_1^* A_2$ — a complex-valued multiply-accumulate.

**The MAC is performed by optical interference at a beam splitter.** The "multiplication" is the product $A_1^* A_2$ (where one field encodes one operand, the other encodes the second operand). The "accumulation" happens at the photodetector, which integrates the photocurrent over the detector capacitance and converts it to voltage.

---

## 12.1.3.2 The Incoherent MAC

### Weighted Sum of Intensities

In an incoherent system (or a system using optical intensity, not field amplitude), the relevant quantity at a detector is:

$$I_{\text{out}} = \sum_j w_j I_j$$

where $I_j$ are the input intensities and $w_j$ are (non-negative) weights implemented by variable optical attenuators or ring-resonator transmission.

For a ring resonator modulator with transmission $T_j = w_j$ (set by the ring-resonator detuning), a wavelength $\lambda_j$ from a comb source is attenuated to power $P_j = w_j P_{\text{comb},j}$ before being detected. Summing over all wavelengths at a common detector (broadband photodetector):

$$I_{\text{total}} = \sum_j w_j P_{\text{comb},j}$$

If all comb lines have equal power $P_0$:

$$I_{\text{total}} = P_0 \sum_j w_j = P_0 \mathbf{w} \cdot \mathbf{1}$$

For a matrix operation: multiple input signals $x_j$ modulate the comb (one wavelength per input), and the weights $w_{ij}$ are set by the ring transmission for each wavelength-detector pair:

$$y_i = \sum_j w_{ij} x_j P_0$$

This is the weighted sum (MAC) for the incoherent case. The weights $w_{ij}$ must be non-negative (since optical transmission is non-negative). To implement negative weights, two detectors are used with weight$^+$ on one and weight$^-$ on the other, and the output is the electrical difference — the "split-photodetector" trick.

---

## 12.1.3.3 MAC Energy Comparison

| Implementation | Energy/MAC (device) | Notes |
|---------------|---------------------|-------|
| 5 nm CMOS FP16 | 4 fJ | Arithmetic only; 200 fJ with HBM access |
| 5 nm CMOS INT8 | 1 fJ | Arithmetic only |
| Optical coherent (MZI) | ~0.1 aJ (device) | Laser + ADC add 10–100 fJ |
| Optical incoherent (ring WDM) | ~0.1 aJ (device) | Laser + ADC add 10–100 fJ |
| Optical with PCM weights | ~0.1 aJ (device) | Non-volatile; zero static weight power |
| Superconducting nanowire | ~0.01 aJ | Requires cryogenic cooling |

The optical device energy is essentially zero — the photons carry essentially no energy per MAC. The total system energy (including laser, DAC/ADC, weight stabilization) is 10–100 fJ/MAC for current systems. The goal for 2028–2030 is < 10 fJ/MAC including all overhead.

For comparison, a state-of-the-art GPU achieves 4 fJ/MAC (compute-bound with large batches). The photonic processor competes primarily in the memory-bandwidth-limited regime where GPU efficiency is 200+ fJ/MAC.

---

## References

[1] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [The primary paper implementing optical MACs in a silicon photonic MZI mesh.]

[2] Hamerly, R., et al. (2019). "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9, 021032. [Detailed energy per MAC analysis for optical neural networks.]

[3] Tait, A.N., et al. (2017). "Neuromorphic photonic networks using silicon photonic weight banks." *Scientific Reports*, 7, 7430. [The broadcast-and-weight incoherent WDM approach to optical MACs.]
