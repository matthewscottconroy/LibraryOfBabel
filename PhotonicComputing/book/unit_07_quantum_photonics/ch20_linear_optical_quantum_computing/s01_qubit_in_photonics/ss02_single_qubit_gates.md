# 20.1.2 Single-Qubit Gates with Linear Optics

## Mode Transformations Are Qubit Rotations

A linear optical element acting on modes $a, b$ implements a unitary on the creation operators,

$$\begin{pmatrix}\hat{a}^\dagger \\ \hat{b}^\dagger\end{pmatrix} \to U^*\begin{pmatrix}\hat{a}^\dagger \\ \hat{b}^\dagger\end{pmatrix}, \qquad U \in \mathrm{U}(2).$$

Because the dual-rail qubit state is $(\alpha\,\hat{a}^\dagger + \beta\,\hat{b}^\dagger)|\text{vac}\rangle$, the same matrix $U$ acts directly on the qubit amplitudes $(\alpha, \beta)$: **for a single photon in two modes, the mode unitary *is* the qubit unitary.** Single-qubit logic in photonics is therefore not merely easy — it is the entire content of classical interferometer design, inherited wholesale from Unit 3.

The two primitive elements:

**Phase shifter** (phase $\phi$ on rail $b$ — a heater, a Pockels section, or simply a longer waveguide):

$$P(\phi) = \begin{pmatrix}1 & 0 \\ 0 & e^{i\phi}\end{pmatrix}$$

Up to a global phase this is a Z-axis rotation $R_z(\phi)$; with $\phi = \pi$ it is the Pauli $Z$ gate.

**Beam splitter** (transmissivity $\cos^2\theta$, with the symmetric phase convention):

$$B(\theta) = \begin{pmatrix}\cos\theta & i\sin\theta \\ i\sin\theta & \cos\theta\end{pmatrix}$$

an X-axis rotation $R_x(2\theta)$ up to phases. A 50/50 splitter ($\theta = \pi/4$) is a Hadamard-class gate: it maps $|0\rangle_L \to (|0\rangle_L + i|1\rangle_L)/\sqrt{2}$. (The textbook Hadamard, with real matrix elements, is a 50/50 splitter dressed with $\pm\pi/2$ phases; experimentalists rarely bother with the distinction, absorbing convention phases into calibration.)

**Pauli X** is a waveguide crossing — physically swap the rails. In polarization encoding the same gates are waveplates: a half-wave plate at angle $\theta$ gives a rotation by $2\theta$, a quarter-wave plate supplies the $\pi/2$ phases.

## Universality: The MZI

Composing the primitives gives all of SU(2). The canonical arrangement is the Mach-Zehnder interferometer with an internal phase $\phi$ and an output phase $\varphi$:

$$U_{MZI}(\phi, \varphi) = P(\varphi)\,B(\pi/4)\,P(\phi)\,B(\pi/4)$$

Carrying out the multiplication (Exercise M20.1) yields, up to a global phase,

$$U_{MZI} \sim \begin{pmatrix} \sin(\phi/2) & \cos(\phi/2) \\ e^{i\varphi}\cos(\phi/2) & -e^{i\varphi}\sin(\phi/2)\end{pmatrix},$$

an arbitrary SU(2) element as $\phi, \varphi$ range over $[0, 2\pi)$: **two phase shifters and two fixed 50/50 couplers make every single-qubit gate.** The MZI mesh architectures of Chapter 7 — Reck's triangular decomposition (any $N$-mode unitary from $N(N-1)/2$ MZIs) and Clements' rectangular refinement — are thus, in quantum language, arbitrary single-qubit gate arrays plus arbitrary multi-*mode* (not multi-qubit!) interference networks. The same silicon mesh that multiplies matrices in Unit 5 executes quantum single-qubit logic without modification.

## How Good Are These Gates?

Startlingly good — the best gates in quantum computing, on several axes:

- **Fidelity:** process fidelities of 99.9%+ are routine for integrated MZI gates; the universal six-mode processor of Carolan et al. (2015) reported average gate fidelities above 99% across thousands of configurations, limited by calibration rather than physics. Unlike matter qubits there is no stochastic decoherence during the gate — errors are coherent (fabrication phase offsets, imperfect splitting ratios) and therefore calibratable and correctable in the mesh itself.
- **Stability:** on-chip path interferometers hold their phase passively for days; polarization gates (waveplates) are similarly static.
- **Speed:** traversal time of an integrated MZI is ~1–10 ps. The gate is never the bottleneck; the photon's schedule is.

Two genuine imperfections matter downstream. First, **insertion loss** (0.1–0.5 dB per MZI in silicon, less in Si₃N₄): harmless to fidelity conditioned on the photon surviving, but it feeds the loss budget that Section 20.5 shows is the real enemy. Second, **mode mismatch**: a gate can be perfect on the qubit subspace while the photon's spectral-temporal wavepacket, which the gate does not touch, fails to match its interference partner's — degrading the *two*-photon interference that all entangling operations rely on. Single-qubit perfection is necessary but nowhere near sufficient.

## Measurement Completes the Single-Qubit Toolkit

A computational-basis measurement is a detector on each rail. Arbitrary-basis measurement is a gate followed by detectors — e.g., $X$-basis measurement is a 50/50 splitter then click discrimination. With Chapter 19's SNSPDs (>98% efficiency, ~15 ps jitter), single-qubit state preparation, rotation, and readout are all effectively solved problems.

This is the seductive half of photonic quantum computing: state preparation, all of SU(2), and measurement, at room temperature, in a foundry process, with fidelities matter platforms envy. Everything difficult is concentrated in one place — making two photons talk to each other — and that is where we turn next.
