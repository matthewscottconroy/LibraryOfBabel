# 21.1 Quantum States and Operations

## 21.1.1 Density Matrices

The state of a quantum system is described not by a single vector but by a *density matrix* — a generalization that handles both pure quantum states and classical uncertainty about which state the system is in.

**Definition 21.1.1 (Quantum State).** A *quantum state* of a $d$-dimensional system is a *density matrix* $\rho \in M_d(\mathbb{C})$ with:
- $\rho \geq 0$ (positive semidefinite)
- $\text{Tr}[\rho] = 1$ (normalized)

*Pure states*: $\rho = |\psi\rangle\langle\psi|$ for some unit vector $|\psi\rangle \in \mathbb{C}^d$. Pure states are perfectly known quantum states.

*Mixed states*: convex combinations $\rho = \sum_i p_i |\psi_i\rangle\langle\psi_i|$ of pure states. Mixed states represent classical uncertainty about the quantum state, or reduced states of an entangled system (as we will see).

The density matrix formalism is necessary for two reasons. First, in practice, we are often uncertain which quantum state a system is in (preparation uncertainty). Second, even when the global state of a composite system is pure (perfectly known), the reduced state of a subsystem may be mixed — this is the phenomenon of entanglement.

**Definition 21.1.2 (Purification).** Every mixed state $\rho_A$ of system $A$ can be written as the reduced state of a pure state $|\psi\rangle_{AB}$ on a larger system $AB$:
$$\rho_A = \text{Tr}_B[|\psi\rangle\langle\psi|_{AB}].$$
This is the *purification* of $\rho_A$. The purification is not unique but is unique up to unitaries on $B$.

Purification is a powerful tool: any question about a mixed state can be "lifted" to a question about a pure state on a larger system. Entanglement with the environment is what makes a state mixed.

## 21.1.2 Quantum Channels

Classical channels are described by conditional probabilities $p(y|x)$. The quantum analogue is a *completely positive trace-preserving (CPTP) map*.

**Definition 21.1.3 (Quantum Channel).** A *quantum channel* is a CPTP map $\mathcal{E}: \mathcal{D}(H_A) \to \mathcal{D}(H_B)$, where $\mathcal{D}(H)$ is the set of density matrices on $H$.

*Complete positivity* is a stronger condition than positivity: not only does $\mathcal{E}$ map positive matrices to positive matrices, but so does $\mathcal{E} \otimes I$ for any auxiliary system. This is necessary to handle entangled inputs.

Every CPTP map has a *Kraus representation*:
$$\mathcal{E}(\rho) = \sum_k K_k \rho K_k^\dagger, \quad \sum_k K_k^\dagger K_k = I.$$

The operators $\{K_k\}$ are *Kraus operators*. Intuitively, the channel "secretly" applies one of the operations $K_k$ (chosen randomly, with probabilities related to the Kraus operators) and you don't know which. Tracing out the "which Kraus operator" information gives the CPTP map.

**Examples:**
- Unitary evolution: $\mathcal{E}(\rho) = U\rho U^\dagger$ — a single Kraus operator, the noiseless case.
- Bit-flip channel: $\mathcal{E}(\rho) = (1-p)\rho + p X\rho X$ where $X = \begin{pmatrix}0&1\\1&0\end{pmatrix}$ — with probability $p$, the qubit is flipped.
- Depolarizing channel: $\mathcal{E}(\rho) = (1-p)\rho + p I/2$ — with probability $p$, the state is replaced by the maximally mixed state.
- Amplitude damping: models spontaneous emission — a two-level atom decays from excited to ground state.

## 21.1.3 Composite Systems and Entanglement

**Definition 21.1.4 (Bipartite States).** The state space of a bipartite system $AB$ is $H_A \otimes H_B$. A state $\rho_{AB}$ is:
- *Separable*: $\rho_{AB} = \sum_i p_i \rho_A^i \otimes \rho_B^i$ — classically correlated, the state is a mixture of product states.
- *Entangled*: not separable — the state has quantum correlations with no classical analogue.

The maximally entangled two-qubit states are the *Bell states*:
$$|\Phi^\pm\rangle = \frac{|00\rangle \pm |11\rangle}{\sqrt{2}}, \quad |\Psi^\pm\rangle = \frac{|01\rangle \pm |10\rangle}{\sqrt{2}}.$$

These four states form an orthonormal basis for two-qubit space ($\mathbb{C}^2 \otimes \mathbb{C}^2$) and are the canonical examples of entanglement.

Entanglement is not just a curiosity. It is a *resource* — a property of quantum states that enables tasks impossible with separable states. Quantum teleportation uses an entangled pair to transmit an unknown quantum state using only classical communication. Dense coding uses entanglement to transmit two bits with one qubit. Entanglement distillation converts multiple copies of a weakly entangled state into fewer copies of maximally entangled Bell pairs. All of these are operational manifestations of entanglement as a resource, and Section 21.4 develops the theory.
