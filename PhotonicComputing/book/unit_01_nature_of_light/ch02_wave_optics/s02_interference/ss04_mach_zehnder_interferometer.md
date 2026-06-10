# 2.2.4 — The Mach-Zehnder Interferometer

## The Architecture

The Mach-Zehnder interferometer (MZI) was developed independently by Ludwig Mach and Ludwig Zehnder in the 1890s as a two-path interferometer for measuring refractive index changes in transparent media [1, 2]. In its original free-space form, a beam splitter divides the input beam into two paths; the paths may pass through different samples; a second beam splitter recombines them. The interference pattern at the output encodes the differential phase shift introduced between the two arms.

The integrated-photonics version is a waveguide device: a Y-junction or directional coupler splits the input waveguide into two arms, which propagate in parallel for some length $L$, then recombine at a second coupler. Phase modulators (electro-optic or thermo-optic) in one or both arms control the phase difference between the paths.

The MZI is the elementary building block of photonic neural networks. An MZI mesh — a 2D array of interconnected MZIs — can implement any unitary matrix transformation on the optical field [3, 4]. Understanding the MZI at the wave level is therefore understanding the fundamental computational unit of coherent photonic processors.

## Wave Analysis: The Transfer Function

Let the input field amplitude be $E_\text{in}$. A 50:50 beam splitter divides this into two equal amplitude paths:

$$E_\text{upper} = \frac{E_\text{in}}{\sqrt{2}}, \qquad E_\text{lower} = \frac{E_\text{in}}{\sqrt{2}}$$

The upper arm accumulates phase $\phi_1$; the lower arm accumulates phase $\phi_2$. After the second 50:50 beam splitter, the two output ports receive:

$$E_\text{out1} = \frac{1}{2}\left(e^{i\phi_1} + e^{i\phi_2}\right) E_\text{in}$$

$$E_\text{out2} = \frac{1}{2}\left(e^{i\phi_1} - e^{i\phi_2}\right) E_\text{in}$$

(The sign difference comes from the $\pi/2$ phase shift that the evanescent coupler introduces in the cross-coupled output; see below.) The output intensities are:

$$I_\text{out1} = |E_\text{out1}|^2 = \frac{I_\text{in}}{2}(1 + \cos\Delta\phi)$$

$$I_\text{out2} = |E_\text{out2}|^2 = \frac{I_\text{in}}{2}(1 - \cos\Delta\phi)$$

where $\Delta\phi = \phi_1 - \phi_2$ is the differential phase. Note that $I_\text{out1} + I_\text{out2} = I_\text{in}$ — energy is conserved.

When $\Delta\phi = 0$: all power exits port 1 (constructive interference). When $\Delta\phi = \pi$: all power exits port 2 (destructive interference in port 1, constructive in port 2). The MZI is a continuously controllable power splitter.

## The Beam Splitter as a Unitary Matrix

The 50:50 beam splitter (directional coupler) relates the four complex field amplitudes at its two input and two output ports. For a lossless symmetric coupler, the transformation matrix must be unitary (energy conservation) and symmetric. The standard form is:

$$\begin{pmatrix} E_\text{through} \\ E_\text{cross} \end{pmatrix} = \frac{1}{\sqrt{2}}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix}\begin{pmatrix} E_{1} \\ E_{2} \end{pmatrix}$$

The $i$ (imaginary unit, i.e., $\pi/2$ phase shift) in the off-diagonal element is a consequence of energy conservation and time-reversal symmetry [5]. It is not a convention; it is a physical requirement. If the through-coupling is real ($t$), the cross-coupling must satisfy $|r|^2 + |t|^2 = 1$ and $t^* r + r^* t = 0$ (from unitarity), which forces the cross-coupling to be $90°$ out of phase with the through-coupling.

The physical origin: in a directional coupler, the cross-coupled wave is produced by evanescent coupling, which introduces a $\pi/2$ phase lag relative to the through-path. This is not a loss of energy — the energy that would have been in the through port is transferred to the cross port with a phase shift. The $i$ factor is the wave-mechanical signature of this energy transfer.

## The MZI as a Unitary 2×2 Matrix

The complete MZI (input coupler + phase shifts + output coupler) acts on a two-dimensional vector of complex field amplitudes $(E_\text{upper}, E_\text{lower})^T$ via a unitary $2 \times 2$ matrix. With a phase shift $\theta$ in the upper arm and $\phi$ in the lower:

$$U_\text{MZI}(\theta, \phi) = \frac{1}{\sqrt{2}}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix} \begin{pmatrix} e^{i\theta} & 0 \\ 0 & e^{i\phi} \end{pmatrix} \frac{1}{\sqrt{2}}\begin{pmatrix} 1 & i \\ i & 1 \end{pmatrix}$$

$$= \frac{e^{i(\theta+\phi)/2}}{2}\begin{pmatrix} ie^{i\Delta\phi/2} + ie^{-i\Delta\phi/2} & -e^{i\Delta\phi/2} + e^{-i\Delta\phi/2} \\ -e^{i\Delta\phi/2} + e^{-i\Delta\phi/2} & ie^{i\Delta\phi/2} + ie^{-i\Delta\phi/2} \end{pmatrix}$$

$$= e^{i(\theta+\phi)/2}\begin{pmatrix} i\cos(\Delta\phi/2) & \sin(\Delta\phi/2) \\ \sin(\Delta\phi/2) & i\cos(\Delta\phi/2) \end{pmatrix}$$

where $\Delta\phi = \theta - \phi$. Up to a global phase $e^{i(\theta+\phi)/2}$, this is a rotation in $\text{SU}(2)$ (the group of $2\times 2$ unitary matrices with determinant 1) parametrized by $\Delta\phi/2$.

**Key insight**: The MZI implements a continuously tunable beam splitter. By choosing $\Delta\phi$, the ratio of output powers can be set to anything from 0:1 to 1:0. Critically, the transformation is *unitary* — it preserves the total optical power. This is the physical mechanism by which photonic neural networks implement unitary (or quasi-unitary) matrix transformations.

## MZI Meshes and Universal Unitary Matrices

A key theoretical result (Reck et al., 1994 [3]; Clements et al., 2016 [4]) states that any $N \times N$ unitary matrix can be decomposed into a product of $N(N-1)/2$ two-dimensional unitary rotations, each implemented by a single MZI. Therefore, a triangular or rectangular mesh of $N(N-1)/2$ MZIs, with appropriate phase settings, can implement any $N \times N$ unitary transformation on a vector of $N$ optical field amplitudes.

This is the theorem underlying photonic matrix multiplication:
1. Any linear (not just unitary) map on a complex vector space can be decomposed via singular value decomposition (SVD): $W = U\Sigma V^\dagger$, where $U$ and $V$ are unitary and $\Sigma$ is diagonal.
2. The unitary parts ($U$ and $V^\dagger$) can be implemented with MZI meshes.
3. The diagonal part ($\Sigma$) is implemented with amplitude modulators (e.g., ring resonator weight banks or variable optical attenuators).
4. The cascade $V^\dagger \to \Sigma \to U$ performs arbitrary linear transformation $W$.

This is the operating principle of the Shen et al. photonic neural network [6] and its successors. The MZI mesh is thus not merely a component — it is the physical implementation of a fundamental mathematical operation: unitary matrix multiplication.

**Phase sensitivity and precision**: Each MZI requires a phase setting accurate to a few milliradians for acceptable matrix error. A silicon thermo-optic phase shifter achieves this precision, but requires power ($\sim 10$ mW per phase shifter for $\pi$ rad). An electro-optic phase shifter (carrier injection or Pockels effect) uses less power but may have limited phase range.

## The MZI as a Photonic Neuron

In the context of photonic neural networks, each column of a MZI mesh followed by nonlinear optical or opto-electronic elements constitutes a neural network layer. The MZI mesh performs the linear (weight) transformation; nonlinearity is provided by saturable absorbers, laser amplifiers, electro-optic elements, or by detection and re-encoding.

The energy efficiency argument for photonic computing depends partly on the MZI. An optical matrix-vector multiplication using an MZI mesh consumes energy only in the phase shifter and detection; the optical propagation itself is essentially lossless (photons travel at the speed of light and do not generate ohmic heat). For very large matrices ($N \gg 1$), this energy advantage over electronic matrix multiplication grows with $N$.

However, this argument is more nuanced than it appears. The energy cost of the analog-to-digital converters (ADCs) and digital-to-analog converters (DACs) required to interface the optical processor with digital electronics often dominates. A full analysis of energy efficiency requires counting all conversion costs — a topic revisited in Unit V.

## Imperfections and Calibration

Real MZIs deviate from ideal behavior due to:
- **Fabrication errors**: Waveguide width and thickness variations cause phase errors of $\sim 0.1$–$1$ rad across a chip.
- **Coupler imperfections**: Directional couplers are not exactly 50:50 (splitting ratio varies with wavelength and fabrication).
- **Thermal crosstalk**: Heating one phase shifter slightly changes adjacent waveguide temperatures.
- **Insertion loss**: Each waveguide section and coupler introduces small losses ($\sim 0.01$–$0.1$ dB), which accumulate over a large mesh.

Calibration algorithms (which measure the actual output of the mesh for known inputs and correct the phase settings) are an active research area in photonic computing.

## Summary

- The MZI splits, phase-shifts, and recombines two optical paths; the output power distribution is $I_\text{out} = I_\text{in}(1 \pm \cos\Delta\phi)/2$.
- A beam splitter acts as a unitary $2\times 2$ matrix with the key $\pi/2$ phase relation between through and cross ports.
- The MZI implements a continuously tunable unitary $2\times 2$ rotation in field space.
- An MZI mesh of $N(N-1)/2$ elements can implement any $N\times N$ unitary matrix (Reck/Clements decomposition).
- Combined with amplitude control (SVD decomposition), the MZI mesh performs arbitrary linear transformations — the computational core of photonic neural networks.

---

*References*

[1] Zehnder, L. (1891). Ein neuer Interferenzrefraktor. *Zeitschrift für Instrumentenkunde*, 11, 275–285.

[2] Mach, L. (1892). Über einen Interferenzrefraktor. *Zeitschrift für Instrumentenkunde*, 12, 89–93.

[3] Reck, M., Zeilinger, A., Bernstein, H.J., & Bertani, P. (1994). Experimental realization of any discrete unitary operator. *Physical Review Letters*, 73(1), 58–61. [DOI: 10.1103/PhysRevLett.73.58]

[4] Clements, W.R., Humphreys, P.C., Metcalf, B.J., Kolthammer, W.S., & Walmsley, I.A. (2016). Optimal design for universal multiport interferometers. *Optica*, 3(12), 1460–1465. [DOI: 10.1364/OPTICA.3.001460]

[5] Loudon, R. (2000). *The Quantum Theory of Light*, 3rd ed. Chapter 6. Oxford University Press. [Derives the $\pi/2$ phase relation from energy conservation and time-reversal symmetry.]

[6] Shen, Y., Harris, N.C., Skirlo, S., Prabhu, M., Baehr-Jones, T., Hochberg, M., Sun, X., Zhao, S., Larochelle, H., Englund, D., & Soljačić, M. (2017). Deep learning with coherent nanophotonic circuits. *Nature Photonics*, 11(7), 441–446. [DOI: 10.1038/nphoton.2017.93] [The landmark paper demonstrating MZI mesh photonic neural network.]
