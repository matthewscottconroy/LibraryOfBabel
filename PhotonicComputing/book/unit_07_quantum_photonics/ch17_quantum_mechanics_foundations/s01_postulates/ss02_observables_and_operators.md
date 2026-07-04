# 17.1.2 Observables and Operators

## The Second Postulate

**Postulate 2.** *Every measurable physical quantity (observable) $A$ is represented by a Hermitian operator $\hat{A} = \hat{A}^\dagger$ on $\mathcal{H}$. The only possible outcomes of a measurement of $A$ are the eigenvalues of $\hat{A}$.*

Hermiticity ($\hat{A}^\dagger = \hat{A}$, where $\dagger$ denotes conjugate transpose) guarantees two properties we need physically. First, all eigenvalues are real — as measurement results must be. Second, by the **spectral theorem**, the eigenvectors of $\hat{A}$ form a complete orthonormal basis, so $\hat{A}$ can be written in its spectral decomposition:

$$\hat{A}|a_n\rangle = a_n|a_n\rangle, \qquad \hat{A} = \sum_n a_n\,|a_n\rangle\langle a_n|, \qquad \langle a_m | a_n \rangle = \delta_{mn}$$

Completeness is the **resolution of the identity**, $\sum_n |a_n\rangle\langle a_n| = \mathbb{1}$ — the statement that any state can be expanded in measurement outcomes, which is what makes the Born rule of the next subsection well-defined for every observable.

## Expectation Values

If the system is in state $|\psi\rangle$, repeated measurements of $A$ on identically prepared systems yield outcome $a_n$ with probability $P(a_n) = |\langle a_n|\psi\rangle|^2$ (anticipating 17.1.3). The mean of these outcomes is the **expectation value**:

$$\langle \hat{A} \rangle = \sum_n a_n P(a_n) = \langle\psi|\hat{A}|\psi\rangle \qquad \text{or} \qquad \langle \hat{A}\rangle = \mathrm{Tr}(\rho \hat{A}) \text{ for mixed states}$$

with variance $\sigma_A^2 = \langle \hat{A}^2\rangle - \langle \hat{A}\rangle^2$. Note carefully what an expectation value is *not*: it is not the value of any single measurement (which is always an eigenvalue), and for a single photon it is not a weak, continuous reading — it is a statistical average over an ensemble of detection events. Quantum photonic experiments are repetition machines: state preparation and measurement, millions of times per second, with statistics assembled from clicks.

## The Pauli Operators: Polarization Observables

For the polarization qubit, the natural observables are the Pauli operators. In the $\{|H\rangle, |V\rangle\}$ basis:

$$\hat{\sigma}_z = \begin{pmatrix} 1 & 0 \\ 0 & -1 \end{pmatrix}, \qquad \hat{\sigma}_x = \begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix}, \qquad \hat{\sigma}_y = \begin{pmatrix} 0 & -i \\ i & 0 \end{pmatrix}$$

Each is Hermitian with eigenvalues $\pm 1$. Their eigenbases are exactly the three polarization bases of 17.1.1: $\hat{\sigma}_z$ has eigenvectors $|H\rangle, |V\rangle$; $\hat{\sigma}_x$ has $|D\rangle, |A\rangle$; $\hat{\sigma}_y$ has $|L\rangle, |R\rangle$. Measuring $\hat{\sigma}_z$ means a polarizing beam splitter with a detector on each port; measuring $\hat{\sigma}_x$ or $\hat{\sigma}_y$ means preceding it with a half- or quarter-wave plate. The expectation values $\langle\hat{\sigma}_x\rangle, \langle\hat{\sigma}_y\rangle, \langle\hat{\sigma}_z\rangle$ are precisely the normalized Stokes parameters of classical polarimetry (Section 2.4), now assembled photon by photon — and they are the components of the Bloch vector $\mathbf{r}$, so measuring all three Paulis on an ensemble reconstructs $\rho$ completely. This is **quantum state tomography** in its simplest instance.

**Worked example.** Prepare $|\psi\rangle = \cos\theta\,|H\rangle + \sin\theta\,|V\rangle$ (linear polarization at angle $\theta$). Then

$$\langle\hat{\sigma}_z\rangle = \cos^2\theta - \sin^2\theta = \cos 2\theta, \qquad \langle\hat{\sigma}_x\rangle = 2\cos\theta\sin\theta = \sin 2\theta, \qquad \langle\hat{\sigma}_y\rangle = 0$$

and $\sigma_{\sigma_z}^2 = 1 - \cos^2 2\theta = \sin^2 2\theta$. At $\theta = 45°$ the $\hat{\sigma}_z$ outcome is maximally uncertain (variance 1) while $\hat{\sigma}_x$ is certain (variance 0): definite in one basis, random in another. The doubling $\theta \to 2\theta$ is the familiar Poincaré-sphere doubling of polarization angles.

## Commutators and Compatibility

Define the **commutator**

$$[\hat{A}, \hat{B}] = \hat{A}\hat{B} - \hat{B}\hat{A}$$

If $[\hat{A}, \hat{B}] = 0$, the operators share a complete set of eigenvectors: both quantities can be simultaneously definite, and measuring one does not disturb the statistics of the other. If $[\hat{A}, \hat{B}] \neq 0$, no state is simultaneously an eigenstate of both (except possibly special cases in the kernel of the commutator), and the observables are **incompatible**. The Paulis are the standard example:

$$[\hat{\sigma}_x, \hat{\sigma}_y] = 2i\hat{\sigma}_z \quad \text{(and cyclic permutations)}$$

No polarization state has definite values in more than one of the three bases — the geometric fact that the three Poincaré axes are mutually orthogonal, promoted to a theorem. This incompatibility is not a nuisance; it is a resource. The security of BB84 quantum key distribution (Chapter 22) rests entirely on encoding bits in two mutually incompatible polarization bases, so that an eavesdropper who measures in the wrong basis necessarily disturbs the state.

## The Uncertainty Principle

For any state and any two observables, the Robertson uncertainty relation holds:

$$\sigma_A\,\sigma_B \;\geq\; \frac{1}{2}\left|\langle[\hat{A}, \hat{B}]\rangle\right|$$

The proof is two lines of Cauchy-Schwarz applied to $(\hat{A} - \langle A\rangle)|\psi\rangle$ and $(\hat{B} - \langle B\rangle)|\psi\rangle$. For position and momentum, $[\hat{x}, \hat{p}] = i\hbar$ gives the Heisenberg relation $\sigma_x \sigma_p \geq \hbar/2$. For the field quadratures $\hat{X}_1, \hat{X}_2$ of a light mode — the observables homodyne detectors measure — Section 17.3 will give $[\hat{X}_1, \hat{X}_2] = i/2$ and hence

$$\Delta X_1\, \Delta X_2 \geq \frac{1}{4}$$

Vacuum saturates this bound with $\Delta X_1 = \Delta X_2 = 1/2$; that irreducible vacuum noise is the origin of shot noise in coherent detection. Squeezed states (17.3.3) also saturate it, but asymmetrically — quieter in one quadrature, noisier in the other. The uncertainty principle thus sets the noise floor of every analog photonic computation and every interferometric measurement, and Section 18.3 shows how squeezing lets LIGO engineer *which* quadrature carries the noise.

Two cautions. First, the bound is state-dependent through $\langle[\hat{A},\hat{B}]\rangle$; for the Paulis it reads $\sigma_{\sigma_x}\sigma_{\sigma_y} \geq |\langle\hat{\sigma}_z\rangle|$, which is vacuous for states in the equatorial plane. Second, the relation constrains the statistics of separately prepared ensembles; it is not by itself a statement about the disturbance one sequential measurement inflicts on the next — that is the subject of the Born rule and state collapse, to which we now turn.
