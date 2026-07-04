# 17.4.2 Bell States

## The Four Maximally Entangled Two-Qubit States

Four particular entangled states organize all of two-qubit quantum information:

$$|\Phi^\pm\rangle = \frac{|00\rangle \pm |11\rangle}{\sqrt{2}}, \qquad |\Psi^\pm\rangle = \frac{|01\rangle \pm |10\rangle}{\sqrt{2}}$$

They are mutually orthonormal and complete: the **Bell basis** of the two-qubit space. Any two-qubit state can be expanded in Bell states, and the four are interconverted by *local* Pauli operations on one qubit alone ($\mathbb{1}\otimes\hat{\sigma}_x$ maps $|\Phi^+\rangle \to |\Psi^+\rangle$, $\mathbb{1}\otimes\hat{\sigma}_z$ maps $|\Phi^+\rangle\to|\Phi^-\rangle$, etc.) — a fact that quantum dense coding and teleportation exploit directly: two classical bits of choice, encoded by one party's local Pauli, select among four orthogonal (hence perfectly distinguishable) joint states.

Each Bell state is **maximally entangled**: its reduced states are maximally mixed ($\rho_A = \rho_B = \mathbb{1}/2$, Schmidt coefficients $1/\sqrt{2}, 1/\sqrt{2}$), so each carries exactly one **ebit** — the unit of entanglement (17.4.4). No two-qubit state is more entangled.

## Correlations in Every Basis

The defining operational property of Bell states is correlation that survives basis changes. Take $|\Phi^+\rangle$ with polarization encoding ($|0\rangle = |H\rangle$, $|1\rangle = |V\rangle$):

- **H/V basis**: outcomes are random ($1/2$ each) but always *equal* — HH or VV, never HV.
- **Diagonal basis**: rewriting with $|H\rangle = (|D\rangle + |A\rangle)/\sqrt{2}$, $|V\rangle = (|D\rangle - |A\rangle)/\sqrt{2}$ gives
$$|\Phi^+\rangle = \frac{|DD\rangle + |AA\rangle}{\sqrt{2}}$$
— the *same form*: perfectly correlated again. (The cross terms $|DA\rangle, |AD\rangle$ cancel; a classical "HH or VV, 50/50" mixture would instead show only 50% correlation diagonally — Exercise 17.8.)
- **General linear basis at angle $\theta$**: the joint probability that both photons pass polarizers at angles $\theta_1, \theta_2$ is $P(\theta_1,\theta_2) = \frac{1}{2}\cos^2(\theta_1 - \theta_2)$, and the correlation function is $E(\theta_1,\theta_2) = \cos 2(\theta_1 - \theta_2)$: dependence only on the *difference* of settings, with perfect correlation at equal angles.

The singlet $|\Psi^-\rangle$ is the aristocrat of the family: antisymmetric under exchange, it is *rotationally invariant* — the same state in every polarization basis — and gives perfect *anti*-correlation at any common angle, $E = -\cos 2(\theta_1-\theta_2)$. These stronger-than-classical multi-basis correlations are exactly what the CHSH inequality (17.4.3) quantifies.

Measuring one qubit of $|\Phi^+\rangle$ in any basis leaves the other in a *pure, definite* state correlated with the outcome (e.g., outcome $|D\rangle$ on A collapses B to $|D\rangle$) — instantaneously, at any distance. No signal is sent: B's local statistics remain $\mathbb{1}/2$ regardless (17.4.1). But the correlation structure is the engine of **teleportation** (an unknown qubit + one shared ebit + two classical bits = the qubit transferred), **entanglement swapping** (Chapter 22's repeaters), and **device-independent QKD**.

## Photonic Encodings of Bell States

Photon pairs are the dominant carriers of Bell states in practice, with three standard encodings:

| Encoding | $|0\rangle / |1\rangle$ | Generation | Strengths / weaknesses |
|---|---|---|---|
| **Polarization** | $|H\rangle / |V\rangle$ | Type-II SPDC (Section 18.3.1); crossed-crystal Type-I | Easy single-qubit gates (wave plates); fiber birefringence scrambles it |
| **Path (dual-rail)** | photon in waveguide $a$ / $b$ | On-chip SPDC/SFWM + directional coupler | Natural for photonic integrated circuits; interferometric stability required |
| **Time-bin** | early / late wavepacket | Pulsed pump + unbalanced interferometer | Survives long fiber links (no polarization drift) — the choice for QKD backbones |

A polarization Bell state from a Type-II SPDC source has the form $(|H\rangle_s|V\rangle_i + e^{i\phi}|V\rangle_s|H\rangle_i)/\sqrt{2}$ — a $|\Psi\rangle$-type state with the relative phase $\phi$ set (and tuned) by crystal birefringence and compensation plates. State-of-the-art sources produce entangled pairs at $>10^6$ detected pairs/s with fidelity to the ideal Bell state exceeding 99%, verified by quantum state tomography (17.1.2's Pauli-correlation measurements, extended to two qubits).

## The Bell-State Measurement Problem

Using Bell states requires not only preparing but also *measuring* in the Bell basis — projecting two incoming qubits onto $\{|\Phi^\pm\rangle, |\Psi^\pm\rangle\}$. Here photonics hits a structural wall: **with linear optics and photon counting alone (no ancillas), a complete Bell-state measurement on two photonic qubits is impossible.** The standard scheme — interfere the two photons on a 50/50 beam splitter and analyze the outputs — is a beautiful application of exchange symmetry: the antisymmetric singlet $|\Psi^-\rangle$ is the *only* Bell state whose photons exit different ports (it alone changes sign under exchange, forcing spatial antibunching against the Hong-Ou-Mandel tendency of Section 18.2.2), while $|\Psi^+\rangle$ is flagged by two same-port photons with orthogonal polarizations; but $|\Phi^+\rangle$ and $|\Phi^-\rangle$ produce identical signatures and cannot be told apart. Success probability: at most $1/2$ (the bound proved by Calsamiglia & Lütkenhaus, 2001).

This "50% BSM ceiling" propagates through the whole field: teleportation and entanglement swapping succeed at most half the time per attempt; fusion gates in fusion-based quantum computing (Chapter 20) inherit the same bound; and boosting beyond 50% costs ancilla photons or nonlinearity. It is a recurring theme of this unit that photonic quantum computing's grand architectures are, at bottom, strategies for paying — or dodging — exactly this toll.
