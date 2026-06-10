# 2.4.2 — Jones Calculus

## The Jones Vector

The Jones calculus, introduced by R. Clark Jones in 1941 [1], provides a compact algebraic framework for describing the polarization state of fully coherent (polarized) light and the action of optical elements on it.

A polarization state is represented as a two-component complex column vector, the *Jones vector*:

$$\mathbf{J} = \begin{pmatrix} E_x \\ E_y \end{pmatrix}$$

where $E_x = |E_x|e^{i\phi_x}$ and $E_y = |E_y|e^{i\phi_y}$ are the complex amplitudes of the two field components. The physical electric field is $\text{Re}[\mathbf{J} e^{-i\omega t}]$.

**Normalization**: Jones vectors are usually normalized so that $|E_x|^2 + |E_y|^2 = 1$ (representing polarization state only, not amplitude). Overall phase factors are often omitted because they do not affect observable intensities.

**Standard Jones vectors**:

| Polarization state | Jones vector |
|-------------------|--------------|
| Horizontal linear ($x$) | $\begin{pmatrix} 1 \\ 0 \end{pmatrix}$ |
| Vertical linear ($y$) | $\begin{pmatrix} 0 \\ 1 \end{pmatrix}$ |
| Linear at $+45°$ | $\frac{1}{\sqrt{2}}\begin{pmatrix} 1 \\ 1 \end{pmatrix}$ |
| Linear at $-45°$ | $\frac{1}{\sqrt{2}}\begin{pmatrix} 1 \\ -1 \end{pmatrix}$ |
| Right circular (RCP) | $\frac{1}{\sqrt{2}}\begin{pmatrix} 1 \\ -i \end{pmatrix}$ |
| Left circular (LCP) | $\frac{1}{\sqrt{2}}\begin{pmatrix} 1 \\ i \end{pmatrix}$ |

**Orthogonality**: Two Jones vectors $\mathbf{J}_1$ and $\mathbf{J}_2$ are orthogonal if $\mathbf{J}_1^\dagger \mathbf{J}_2 = 0$ (the inner product vanishes). $H$ and $V$ are orthogonal, $R$ and $L$ are orthogonal, $D$ and $A$ are orthogonal. Orthogonal polarizations cannot interfere.

## Jones Matrices

An optical element that acts linearly on the polarization state (a wave plate, polarizer, beam splitter, phase retarder) is represented by a $2 \times 2$ complex matrix — the *Jones matrix* $\mathsf{M}$. The output polarization state is:

$$\mathbf{J}_\text{out} = \mathsf{M} \cdot \mathbf{J}_\text{in}$$

For a cascade of $N$ elements with Jones matrices $\mathsf{M}_1, \mathsf{M}_2, \ldots, \mathsf{M}_N$:

$$\mathbf{J}_\text{out} = \mathsf{M}_N \cdots \mathsf{M}_2 \mathsf{M}_1 \cdot \mathbf{J}_\text{in}$$

(rightmost matrix acts first). The Jones matrix of the complete system is the product.

**Standard Jones matrices**:

**Horizontal linear polarizer**:
$$\mathsf{M}_H = \begin{pmatrix} 1 & 0 \\ 0 & 0 \end{pmatrix}$$

**Vertical linear polarizer**:
$$\mathsf{M}_V = \begin{pmatrix} 0 & 0 \\ 0 & 1 \end{pmatrix}$$

**Linear polarizer at angle $\psi$**:
$$\mathsf{M}_\psi = \begin{pmatrix} \cos^2\psi & \sin\psi\cos\psi \\ \sin\psi\cos\psi & \sin^2\psi \end{pmatrix}$$

**Phase retarder** (fast axis along $x$, retardation $\Gamma$): introduces phase delay $\Gamma$ between the $y$ and $x$ components:
$$\mathsf{M}_\Gamma = \begin{pmatrix} 1 & 0 \\ 0 & e^{i\Gamma} \end{pmatrix} = e^{i\Gamma/2}\begin{pmatrix} e^{-i\Gamma/2} & 0 \\ 0 & e^{i\Gamma/2} \end{pmatrix}$$

(The second form, factoring out a global phase, is often more convenient.)

**Half-wave plate** ($\Gamma = \pi$, fast axis at $\psi$): rotates linear polarization by $2\psi$:
$$\mathsf{M}_{\lambda/2}(\psi) = \begin{pmatrix} \cos 2\psi & \sin 2\psi \\ \sin 2\psi & -\cos 2\psi \end{pmatrix}$$

**Quarter-wave plate** ($\Gamma = \pi/2$, fast axis at $\psi = 0$): converts linear to elliptical/circular:
$$\mathsf{M}_{\lambda/4}(0) = \frac{1}{\sqrt{2}}\begin{pmatrix} 1 & 0 \\ 0 & i \end{pmatrix} \cdot \sqrt{2} = \begin{pmatrix} 1 & 0 \\ 0 & i \end{pmatrix}$$

(More precisely, with appropriate global phase, $e^{-i\pi/4}\begin{pmatrix} 1 & 0 \\ 0 & i \end{pmatrix}$.)

**Rotation matrix** (rotating the coordinate system by angle $\theta$, or equivalently, rotating the optical element):

$$\mathsf{R}(\theta) = \begin{pmatrix} \cos\theta & \sin\theta \\ -\sin\theta & \cos\theta \end{pmatrix}$$

A wave plate with fast axis at angle $\psi$ has Jones matrix $\mathsf{R}(-\psi) \cdot \mathsf{M}_\Gamma \cdot \mathsf{R}(\psi)$.

## Worked Example: Quarter-Wave Plate and Circular Polarization

Input: horizontally polarized light $\mathbf{J} = \begin{pmatrix} 1 \\ 0 \end{pmatrix}$.

Quarter-wave plate with fast axis at $45°$:
$$\mathsf{M} = \mathsf{R}(-45°)\begin{pmatrix} 1 & 0 \\ 0 & i \end{pmatrix}\mathsf{R}(45°) = \frac{1}{2}\begin{pmatrix} 1+i & 1-i \\ 1-i & 1+i \end{pmatrix}$$

Output:
$$\mathbf{J}_\text{out} = \frac{1}{2}\begin{pmatrix} 1+i \\ 1-i \end{pmatrix} = \frac{1}{\sqrt{2}}\begin{pmatrix} 1 \\ -i \end{pmatrix} \cdot \frac{1+i}{\sqrt{2}}$$

Up to an overall phase $\frac{1+i}{\sqrt{2}} = e^{i\pi/4}$, the output is $\frac{1}{\sqrt{2}}\begin{pmatrix} 1 \\ -i \end{pmatrix}$ — right circular polarization. A quarter-wave plate at $45°$ converts horizontally polarized light to right circular polarization. This is the standard way to generate circular polarization.

## Unitary Jones Matrices and Lossless Elements

For a lossless optical element, $|\mathbf{J}_\text{out}| = |\mathbf{J}_\text{in}|$ (power is conserved). This requires $\mathsf{M}$ to be *unitary*: $\mathsf{M}^\dagger \mathsf{M} = \mathsf{I}$, or equivalently, $\mathsf{M}^{-1} = \mathsf{M}^\dagger$.

Wave plates and beam splitters are unitary. Polarizers are not (they absorb one polarization component). In photonic computing, the MZI coupled with polarization control is unitary (up to insertion loss). The ability to implement arbitrary $\text{SU}(2)$ (single-photon unitary) and $\text{SU}(N)$ (N-photon unitary) transformations using combinations of wave plates (or beam splitters with phase shifters) is the basis of both polarization-encoded quantum gates and classical photonic matrix operations.

The group $\text{SU}(2)$ — the group of $2 \times 2$ unitary matrices with determinant 1 — is isomorphic to the rotation group $\text{SO}(3)$ (with a $2:1$ map). Any polarization transformation by a lossless optical element corresponds to a rotation of the Poincaré sphere (Section 2.4.3). This geometric picture is often more intuitive than the algebraic Jones matrix description.

## The Jones Calculus and Photonic Computing

The Jones calculus is a *linear* description of polarization optics. It is the $2 \times 2$ instance of the same linear algebraic framework used to describe MZI networks ($N \times N$ unitary matrices). In fact, one way to think about a photonic neural network is as a large Jones-like calculation: an $N$-mode optical field is described by an $N$-component complex vector, and each optical element in the network is represented by an $N \times N$ unitary (or lossy) matrix acting on this vector.

The Jones calculus becomes inadequate when the light is partially polarized (incoherent superposition of different polarization states). In that case, one must use the Stokes/Mueller formalism (Section 2.4.3), which describes the time-averaged intensity of each polarization component. For fully coherent laser light in a photonic computing circuit, the Jones calculus is exact and sufficient.

## Summary

- Jones vector: $\mathbf{J} = (E_x, E_y)^T$ — two-component complex representation of polarization state.
- Jones matrix: $2 \times 2$ complex matrix acting on Jones vectors; cascade of elements = product of matrices.
- Key matrices: polarizers (projection operators), retarders/wave plates (phase matrices), rotators.
- Lossless elements → unitary Jones matrices; unitary group $\text{SU}(2)$ $\equiv$ rotations on Poincaré sphere.
- Jones calculus is the $2 \times 2$ instance of the linear algebra underlying photonic computing.

---

*References*

[1] Jones, R.C. (1941). A new calculus for the treatment of optical systems. *Journal of the Optical Society of America*, 31(7), 488–493. [DOI: 10.1364/JOSA.31.000488] [The original paper introducing the Jones vector and matrix formalism.]

[2] Hecht, E. (2017). *Optics*, 5th ed. Pearson. Chapter 8. [Accessible treatment of Jones calculus with worked examples.]
