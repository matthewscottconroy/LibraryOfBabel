# Subsection 12.2.4: Programming, Calibration, and Error Analysis

## Orientation

The theoretical elegance of the MZI mesh — that any unitary matrix can be decomposed into a product of $2 \times 2$ rotations — needs to survive contact with real silicon photonic chips. Every MZI on a fabricated chip has imperfect 50:50 couplers, finite extinction ratios, thermally drifting phase angles, and coupling variations. This subsection addresses how to program a desired matrix into a physical mesh, how to correct for these imperfections, and what precision can be achieved in practice.

---

## 12.2.4.1 Forward and Inverse Programming

### Forward Problem

Given: phase angles $\{\theta_{ij}, \phi_{ij}\}$ for all MZIs.
Find: the implemented $N \times N$ unitary matrix $U$.

This is straightforward: compute the matrix product of all individual MZI transfer matrices in order.

$$U = \prod_{l=1}^{L} \left(\prod_{k \in \text{layer } l} U_{\text{MZI}}(\theta_k, \phi_k)\right)$$

This is a product of sparse matrices (each MZI affects only 2 modes), computationally $O(N^2 L) = O(N^3)$ for $L = O(N)$ layers.

### Inverse Problem

Given: a target $N \times N$ unitary matrix $U$.
Find: phase angles $\{\theta_{ij}, \phi_{ij}\}$ for all MZIs.

This is the programming problem. The Clements algorithm (Section 12.2.3) provides an explicit procedure:

For each column $j$ from left and right:
1. Identify the target element to zero.
2. Compute the required MZI phase angles analytically.
3. Apply the MZI to the matrix (updating the remaining target matrix).

**Computational cost**: $O(N^2)$ operations (scanning each element once).

**Phase angle ranges**: Each $\theta \in [0, \pi]$ and each $\phi \in [0, 2\pi)$. The thermo-optic heater voltage is set to achieve the desired phase.

### Calibration of Phase-Voltage Relationship

The thermo-optic phase as a function of heater power (from Section 7.4.1):

$$\Delta\phi = \frac{\pi P}{P_\pi}$$

where $P_\pi$ is the heater power for a $\pi$ phase shift (~10 mW for standard thermo-optic heaters, ~1 mW for suspended waveguides). The relationship is linear; $P_\pi$ must be measured for each MZI individually to account for fabrication variation in heater resistance and thermal resistance.

**Calibration procedure**:
1. For each MZI, sweep heater power from 0 to $P_{\text{max}}$.
2. Measure the output power ratio at the two output ports.
3. Fit the sinusoidal transfer function $P_{\text{out}} = P_{\text{in}} \cos^2(\pi P_h / 2P_\pi + \phi_0)$ to extract $P_\pi$ and $\phi_0$ (phase offset at zero power).
4. Store the calibration map $\{P_\pi^{(k)}, \phi_0^{(k)}\}$ for each MZI $k$.

**Calibration time**: Each MZI requires ~20 power measurements × 50 ms each = 1 second per MZI. For $N(N-1)/2$ MZIs in an $N = 32$ mesh: $496 \times 1 = 496$ seconds ≈ 8 minutes. Acceptable for a chip deployed for many inference runs.

---

## 12.2.4.2 Sources of Error and Their Magnitudes

### 1. Coupler Splitting Ratio Error

Fabrication: waveguide width variation of $\pm 3$ nm → coupling length variation → splitting ratio $\kappa = 0.5 \pm \delta\kappa$, where $\delta\kappa \approx 0.01$–0.03.

Effect on matrix: A coupler with $\kappa = 0.5 + \delta\kappa$ instead of $0.5$ introduces an error in the MZI transfer matrix. The off-diagonal elements of the implemented MZI differ from the target by:

$$\Delta U_{12} \approx \delta\kappa \cdot f(\theta)$$

where $f(\theta)$ is a function of the splitting angle. Worst case: $\Delta U_{12} \approx \delta\kappa$.

**ENOB impact**: For $\delta\kappa = 0.03$, the matrix element error from this source is ~3%, corresponding to $-20\log_{10}(0.03) \approx 30$ dB suppression, or ENOB $\approx 5$ bits.

### 2. Phase Setting Resolution

An 8-bit DAC controlling the heater power has $2^8 = 256$ discrete levels. For a $\pi$ phase range with a heater power range of $[0, P_\pi]$:

$$\delta\phi_{\text{DAC}} = \frac{\pi}{256} \approx 0.012 \text{ rad}$$

Effect on matrix: Phase error $\delta\phi$ causes matrix element error $\delta U \approx \delta\phi$ (in the worst case). For $\delta\phi = 0.012$ rad: ENOB $\approx -\log_2(0.012/\pi) \approx 8$ bits.

With a 12-bit DAC: $\delta\phi = \pi/4096 \approx 0.00077$ rad, ENOB $\approx 12$ bits (but limited by other sources).

### 3. Thermal Drift

Thermo-optic phase drift from temperature changes $\Delta T$:

$$\delta\phi_{\text{thermal}} = \frac{2\pi L (dn/dT)}{\lambda} \Delta T$$

For $L = 100$ μm, $dn/dT = 1.87\times10^{-4}$ K$^{-1}$, $\lambda = 1550$ nm, $\Delta T = 1$ K:

$$\delta\phi = \frac{2\pi \times 10^{-4} \times 1.87\times10^{-4}}{1550\times10^{-9}} \approx 0.076 \text{ rad}$$

For a 10 K temperature change (typical chip temperature variation under workload): $\delta\phi = 0.76$ rad. This is a large phase error — equivalent to ~2 bits of precision — and would completely derail the matrix computation without active thermal stabilization.

**Active correction**: A feedback loop can correct for thermal drift if the drift is slow compared to the feedback bandwidth. For slow (seconds-timescale) drift: proportional-integral controllers on each heater can maintain < 0.01 rad accuracy.

### 4. Waveguide Propagation Loss Non-Uniformity

Different paths through the mesh have different lengths and therefore different losses. A signal routed through 6 MZI layers has $6 \times L_{\text{MZI}}$ loss; a signal routed through 2 layers has $2 \times L_{\text{MZI}}$ loss. This creates a non-uniform loss pattern that is equivalent to multiplying the implemented matrix by a non-uniform diagonal matrix:

$$U_{\text{actual}} = D_{\text{loss}} \cdot U_{\text{ideal}}$$

where $D_{\text{loss}}$ has entries $e^{-\alpha d_i/2}$ ($d_i$ = path length for mode $i$, $\alpha$ = attenuation coefficient). For the Clements mesh, path length uniformity is better than Reck (depth $N$ vs. $2N-3$), but still non-perfect.

**Correction**: Calibrate the loss matrix $D_{\text{loss}}$ and compensate by scaling the input/output amplitudes (at the cost of reduced dynamic range).

---

## 12.2.4.3 Demonstrated Performance

### State of the Art (2017–2024)

**Shen et al. (2017)** — MIT, 4-mode MZI mesh in silicon photonics:
- $N = 4$, 6 MZIs
- Implemented a $4 \times 4$ unitary for vowel-recognition neural network
- Mean fidelity $\|U_{\text{actual}} - U_{\text{target}}\|_F / \|U_{\text{target}}\|_F \approx 0.05$ (5% error)
- Operating speed: 40 Gbps per input mode

**Bandyopadhyay et al. (2021)** — MIT, 8-mode mesh:
- $N = 8$, 28 MZIs
- Hardware error correction applied after programming
- Achieved fidelity improvement from 15% error to 2% error
- ENOB effectively improved from ~3 bits to ~6 bits

**Lightmatter Mars (2022)** — commercial chip:
- $N = 64$ (publicly disclosed)
- Precision: not fully disclosed; estimated ~5–7 bits from inference benchmarks
- 70 TOPS (tera-operations per second) at 200 nm bandwidth

**General trend**: Precision is limited to 5–8 bits by the combination of coupler error, thermal drift, and phase resolution. This is sufficient for neural network inference on many tasks but would require error correction techniques (bit-slicing, as in Section 12.1.2) for higher-precision applications.

---

## References

[1] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [First experimental demonstration of a photonic neural network using MZI mesh; the foundational paper.]

[2] Bandyopadhyay, S., Hamerly, R., & Englund, D. (2021). "Hardware error correction for programmable photonics." *Optica*, 8(10), 1247–1255. [Systematic analysis of MZI mesh error sources and correction methods; primary reference for the error analysis in this subsection.]

[3] Hamerly, R., et al. (2022). "Accurate self-configuration of rectangular multiport interferometers." *Physical Review Applied*, 18, 024019. [Self-configuration algorithm that avoids needing explicit calibration; uses in-situ measurement to find the optimal phase angles.]

[4] Pai, S., et al. (2023). "Experimentally realized in situ backpropagation for deep learning in photonic neural networks." *Science*, 380(6643), 398–404. [In-situ training directly on the optical hardware; avoids the calibration problem by using gradient-based optimization that works despite fabrication imperfections.]
