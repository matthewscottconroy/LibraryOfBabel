# 17.1.1 State Vectors and Hilbert Space

## The First Postulate

**Postulate 1.** *The state of an isolated quantum system is completely described by a unit vector $|\psi\rangle$ in a complex Hilbert space $\mathcal{H}$ — a complex vector space equipped with an inner product.*

The notation is Dirac's. A **ket** $|\psi\rangle$ is a column vector; a **bra** $\langle\phi|$ is the conjugate-transpose row vector; their pairing $\langle\phi|\psi\rangle \in \mathbb{C}$ is the inner product. The inner product satisfies $\langle\phi|\psi\rangle = \langle\psi|\phi\rangle^*$, and the norm of a state is $\||\psi\rangle\| = \sqrt{\langle\psi|\psi\rangle}$. Physical states are normalized:

$$\langle\psi|\psi\rangle = 1$$

The **outer product** $|\psi\rangle\langle\phi|$ is an operator (a matrix): it maps $|\chi\rangle$ to $|\psi\rangle\langle\phi|\chi\rangle$, i.e., the vector $|\psi\rangle$ scaled by the number $\langle\phi|\chi\rangle$. Outer products of a state with itself, $|\psi\rangle\langle\psi|$, are projectors onto that state, and they are the bridge to density matrices below.

## The Qubit: A Photon's Polarization

The smallest interesting Hilbert space is two-dimensional. Choose an orthonormal basis $\{|0\rangle, |1\rangle\}$; the general normalized state is

$$|\psi\rangle = \alpha|0\rangle + \beta|1\rangle, \qquad |\alpha|^2 + |\beta|^2 = 1, \quad \alpha, \beta \in \mathbb{C}$$

This is a **qubit**. For a single photon, nature provides the two-dimensional space for free: the two transverse polarizations of a given spatial mode. Writing $|H\rangle$ and $|V\rangle$ for horizontal and vertical polarization,

$$|D\rangle = \frac{|H\rangle + |V\rangle}{\sqrt{2}}, \qquad |A\rangle = \frac{|H\rangle - |V\rangle}{\sqrt{2}}, \qquad |L/R\rangle = \frac{|H\rangle \pm i|V\rangle}{\sqrt{2}}$$

are the diagonal, antidiagonal, and circular polarization states. Formally this is the Jones calculus of Section 2.4 — the same two-component complex vectors, the same wave plates as $2\times 2$ unitaries. The physical content is radically different: a Jones vector describes a classical field carrying many photons, while $|\psi\rangle$ here describes *one photon*, and $|\alpha|^2$, $|\beta|^2$ are probabilities of detection events, not fractions of a beam's power. The same algebra describes a photon delocalized over two paths (**dual-rail encoding**, Chapter 20) or two time bins (Chapter 22); polarization, path, and time-bin qubits are unitarily interchangeable encodings of the same abstract two-level system.

Two states that differ by a **global phase**, $|\psi\rangle$ and $e^{i\gamma}|\psi\rangle$, are physically identical: no measurement can distinguish them, since probabilities involve squared magnitudes. *Relative* phase, by contrast, is everything: $|D\rangle$ and $|A\rangle$ differ only by the sign of $\beta$ and are orthogonal.

## Superposition Is Not Ignorance

The state $|D\rangle = (|H\rangle + |V\rangle)/\sqrt{2}$ does *not* mean "the photon is H or V and we don't know which." A diagonal polarizer transmits a $|D\rangle$ photon with certainty; it transmits a photon that is randomly H or V half the time. Superposition carries phase; ignorance does not. Making this distinction quantitative requires the density matrix.

## Density Matrices: Pure and Mixed States

For a pure state, define the **density operator**

$$\rho = |\psi\rangle\langle\psi|$$

For an ensemble in which state $|\psi_i\rangle$ occurs with classical probability $p_i$,

$$\rho = \sum_i p_i\, |\psi_i\rangle\langle\psi_i|, \qquad \sum_i p_i = 1$$

Any valid $\rho$ is Hermitian, positive semidefinite, and has unit trace, $\mathrm{Tr}\,\rho = 1$. Expectation values of any observable $\hat{A}$ (Section 17.1.2) become $\langle \hat{A}\rangle = \mathrm{Tr}(\rho \hat{A})$, which reproduces $\langle\psi|\hat{A}|\psi\rangle$ for pure states and averages over the ensemble for mixed ones.

The **purity** $\mathcal{P} = \mathrm{Tr}(\rho^2)$ discriminates the cases: $\mathcal{P} = 1$ if and only if $\rho$ is pure; $\mathcal{P} = 1/d$ for the maximally mixed state $\rho = \mathbb{1}/d$ in dimension $d$.

**Worked example.** Compare the diagonal *superposition* with the 50/50 *mixture* of $|H\rangle$ and $|V\rangle$:

$$\rho_D = |D\rangle\langle D| = \frac{1}{2}\begin{pmatrix} 1 & 1 \\ 1 & 1 \end{pmatrix}, \qquad \rho_{\text{mix}} = \frac{1}{2}|H\rangle\langle H| + \frac{1}{2}|V\rangle\langle V| = \frac{1}{2}\begin{pmatrix} 1 & 0 \\ 0 & 1 \end{pmatrix}$$

in the $\{|H\rangle, |V\rangle\}$ basis. Both give probability $1/2$ for transmission through an H-polarizer. But behind a *diagonal* polarizer, the transmission probabilities are $\langle D|\rho_D|D\rangle = 1$ versus $\langle D|\rho_{\text{mix}}|D\rangle = 1/2$. The off-diagonal elements of $\rho$ — the **coherences** — carry the relative phase, and measuring in a rotated basis reveals them. Purities: $\mathrm{Tr}(\rho_D^2) = 1$ (pure), $\mathrm{Tr}(\rho_{\text{mix}}^2) = 1/2$ (maximally mixed). The state $\rho_{\text{mix}}$ is precisely *unpolarized* light at the single-photon level: no polarizer orientation transmits it with anything but probability $1/2$.

## The Bloch Sphere

Every qubit density matrix can be written

$$\rho = \frac{1}{2}\left(\mathbb{1} + \mathbf{r}\cdot\hat{\boldsymbol{\sigma}}\right), \qquad \mathbf{r} \in \mathbb{R}^3, \quad |\mathbf{r}| \leq 1$$

where $\hat{\boldsymbol{\sigma}} = (\hat{\sigma}_x, \hat{\sigma}_y, \hat{\sigma}_z)$ are the Pauli operators (Section 17.1.2). Pure states live on the surface of the unit sphere ($|\mathbf{r}| = 1$); mixed states fill the interior; the maximally mixed state sits at the center. For polarization qubits, the Bloch sphere *is* the Poincaré sphere of Section 2.4, with $|\mathbf{r}|$ the degree of polarization — a satisfying closure of the classical-quantum correspondence, and a reminder that the quantum formalism was already hiding inside classical polarization optics.

## Where Mixed States Come From

Three sources of mixedness dominate quantum photonics, and all three will recur:

1. **Classical randomness in preparation** — e.g., a source emitting $|H\rangle$ or $|V\rangle$ with a fluctuating pump.
2. **Loss and decoherence in transmission** — a fiber whose birefringence wanders in time scrambles polarization coherences.
3. **Entanglement with an unobserved system** — the deepest source: as Section 17.4.1 shows, each half of an entangled pure state is, on its own, a mixed state. Mixedness of the part is the fingerprint of entanglement of the whole.

The Hilbert-space dimension grows fast: one qubit needs $\mathbb{C}^2$, but $n$ qubits need $\mathbb{C}^{2^n}$ (Section 17.4.1), and a single optical mode already needs the infinite-dimensional space spanned by the photon-number states of Section 17.2.3. That exponential growth is the resource quantum computing spends — and the burden any classical simulation of it must carry.
