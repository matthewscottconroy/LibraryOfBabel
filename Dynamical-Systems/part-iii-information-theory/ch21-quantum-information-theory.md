# Chapter 21 — Quantum Information Theory

> *Quantum information theory is the extension of Shannon's theory to quantum mechanics. The key insight: quantum states carry more information than classical states — and the rules for manipulating that information are fundamentally different.*

**Prerequisites:** Chapter 5 (Hilbert spaces, tensor products), Chapter 16 (classical information theory), Chapter 17 (von Neumann entropy).

**What this chapter builds:** Quantum states and channels; quantum compression (Schumacher's theorem); quantum channel capacity; entanglement theory; quantum error correction; and the deep theorem of strong subadditivity as the engine of quantum information theory.

---

## 21.1 Quantum States and Operations

### 21.1.1 Density Matrices

**Definition 21.1.1.** A *quantum state* of a $d$-dimensional system is a *density matrix* $\rho \in M_d({\mathbb C})$ with:
- $\rho \geq 0$ (positive semidefinite)
- $\text{Tr}[\rho] = 1$ (normalized)

*Pure states*: $\rho = |\psi\rangle\langle\psi|$ for some unit vector $|\psi\rangle \in {\mathbb C}^d$.
*Mixed states*: convex combinations $\rho = \sum_i p_i |\psi_i\rangle\langle\psi_i|$ of pure states.

**Definition 21.1.2 (Purification).** Every mixed state $\rho_A$ of system $A$ can be written as the reduced state of a pure state $|\psi\rangle_{AB}$ on a larger system $AB$: $\rho_A = \text{Tr}_B[|\psi\rangle\langle\psi|_{AB}]$. This is the *purification* of $\rho_A$.

### 21.1.2 Quantum Channels

**Definition 21.1.3.** A *quantum channel* is a completely positive trace-preserving (CPTP) map $\mathcal{E}: \mathcal{D}(H_A) \to \mathcal{D}(H_B)$, where $\mathcal{D}(H)$ is the set of density matrices on $H$.

**Kraus Representation:**
$$\mathcal{E}(\rho) = \sum_k K_k \rho K_k^\dagger, \quad \sum_k K_k^\dagger K_k = I.$$

The operators $\{K_k\}$ are *Kraus operators*. Every CPTP map has a Kraus representation.

**Examples:**
- Unitary evolution: $\mathcal{E}(\rho) = U\rho U^\dagger$ (Kraus with single operator $K = U$)
- Bit-flip channel: $\mathcal{E}(\rho) = (1-p)\rho + p X\rho X$ where $X = \begin{pmatrix}0&1\\1&0\end{pmatrix}$
- Depolarizing channel: $\mathcal{E}(\rho) = (1-p)\rho + p I/2$ (with probability $p$, replace by maximally mixed)
- Amplitude damping: models spontaneous emission in a two-level atom

### 21.1.3 Composite Systems and Entanglement

**Definition 21.1.4.** The state space of a bipartite system $AB$ is $H_A \otimes H_B$. A state $\rho_{AB}$ is:
- *Separable*: $\rho_{AB} = \sum_i p_i \rho_A^i \otimes \rho_B^i$ (classically correlated)
- *Entangled*: not separable

**Bell States:**
$$|\Phi^\pm\rangle = \frac{|00\rangle \pm |11\rangle}{\sqrt{2}}, \quad |\Psi^\pm\rangle = \frac{|01\rangle \pm |10\rangle}{\sqrt{2}}.$$

These are maximally entangled states of two qubits.

---

## 21.2 Quantum Source Coding — Schumacher's Theorem

**Classical reminder:** Shannon's source coding theorem says i.i.d. source $X$ with entropy $H$ can be compressed to $H$ bits/symbol.

**Quantum analog:** An i.i.d. quantum source $\{\rho\}^{\otimes n}$ can be compressed to $S(\rho)$ qubits/symbol.

**Theorem 21.2.1 (Schumacher Compression, 1995).** For an i.i.d. source producing state $\rho$ with von Neumann entropy $S(\rho) = -\text{Tr}[\rho\log\rho]$:
- (Achievability) For any $R > S(\rho)$ and $\varepsilon > 0$: there is a compression scheme using $nR$ qubits/symbol with fidelity $\geq 1-\varepsilon$ for large $n$.
- (Converse) For any $R < S(\rho)$: fidelity of any compression scheme $\to 0$.

**Proof idea:** Use the spectral decomposition $\rho = \sum_i \lambda_i |i\rangle\langle i|$. The *quantum typical subspace* is the subspace spanned by eigenvectors corresponding to eigenvalues $\lambda_i$ with $-\frac{1}{n}\log\lambda_{i_1}\cdots\lambda_{i_n} \approx S(\rho)$ (by AEP). Project onto this typical subspace (dimension $\approx 2^{nS(\rho)}$) and transmit.

---

## 21.3 Quantum Channel Capacity

### 21.3.1 Classical Capacity

**Definition 21.3.1.** The *classical capacity* $C(\mathcal{E})$ of a quantum channel $\mathcal{E}$ is the maximum rate (bits/channel use) of reliable classical communication.

**Theorem 21.3.2 (Holevo Bound — Holevo 1973).** For any encoding $\{p_i, \rho_i\}$ (messages with probabilities $p_i$ encoded as states $\rho_i$), the *Holevo $\chi$-information* bounds the accessible information:
$$I(X;Y) \leq \chi = S\left(\sum_i p_i \rho_i\right) - \sum_i p_i S(\rho_i) = S(\rho) - \sum_i p_i S(\rho_i).$$

**Theorem 21.3.3 (HSW Theorem — Hausladen-Schumacher-Westmoreland, Holevo 1997).** The classical capacity of a quantum channel is:
$$C(\mathcal{E}) = \lim_{n\to\infty} \frac{1}{n} \chi(\mathcal{E}^{\otimes n}) = \lim_{n\to\infty} \frac{1}{n} \max_{\{p_i, \rho_i^{(n)}\}} \left[S\left(\mathcal{E}^{\otimes n}\left(\sum_i p_i\rho_i^{(n)}\right)\right) - \sum_i p_i S(\mathcal{E}^{\otimes n}(\rho_i^{(n)}))\right].$$

The limit (regularization over $n$) is needed because $\chi$ may not be additive. *The question of whether $\chi$ is additive was a major open problem, resolved negatively by Hastings (2009): there exist quantum channels for which entangled inputs provide more classical capacity than product inputs.*

### 21.3.2 Quantum Capacity

**Definition 21.3.4.** The *quantum capacity* $Q(\mathcal{E})$ is the maximum rate (qubits/channel use) of reliable quantum communication (transmission of quantum states, preserving quantum coherence).

**Theorem 21.3.5 (LSD Theorem — Lloyd, Shor, Devetak).** The quantum capacity is:
$$Q(\mathcal{E}) = \lim_{n\to\infty} \frac{1}{n} \max_{\rho^{(n)}} I_c(\rho^{(n)}; \mathcal{E}^{\otimes n}),$$
where the *coherent information* is $I_c(\rho; \mathcal{E}) = S(\mathcal{E}(\rho)) - S(\mathcal{E}^c(\rho))$ ($\mathcal{E}^c$ = complementary channel, environment's output).

**Remark 21.3.6.** For a channel with $Q > 0$, reliable quantum communication is possible. Channels with $Q = 0$ cannot reliably transmit quantum information — all quantum coherence is destroyed. Anti-degradable channels and Hadamard channels have $Q = 0$.

---

## 21.4 Entanglement Theory

### 21.4.1 Entanglement Measures

**Definition 21.4.1.** For a bipartite pure state $|\psi\rangle_{AB}$ with Schmidt decomposition $|\psi\rangle = \sum_i \sqrt{\lambda_i} |i\rangle_A |i\rangle_B$, the *entanglement entropy* is:
$$E(|\psi\rangle) = S(\rho_A) = -\sum_i \lambda_i \log \lambda_i$$
where $\rho_A = \text{Tr}_B[|\psi\rangle\langle\psi|]$.

**Theorem 21.4.2.** For pure states, entanglement entropy is the unique entanglement measure (up to normalization) satisfying a natural set of axioms (monotonicity under LOCC, continuity, normalization).

**Entanglement of Formation:** For mixed states $\rho_{AB}$, the *entanglement of formation* is:
$$E_F(\rho_{AB}) = \min_{\{p_i, |\psi_i\rangle\}} \sum_i p_i E(|\psi_i\rangle)$$
where the min is over all pure state decompositions $\rho_{AB} = \sum_i p_i |\psi_i\rangle\langle\psi_i|$.

### 21.4.2 Entanglement Distillation and Dilution

**Theorem 21.4.3 (Bennett et al., Hayden et al.).** For a pure bipartite state $|\psi\rangle^{\otimes n}$:
- *Distillation*: by LOCC operations, one can extract $\approx nE(|\psi\rangle)$ maximally entangled pairs (Bell pairs).
- *Dilution*: $\approx nE(|\psi\rangle)$ Bell pairs suffice to create $n$ copies of $|\psi\rangle$ by LOCC.

The entanglement entropy $E(|\psi\rangle)$ is thus the rate of conversion between arbitrary pure entangled states and Bell pairs — it is the "currency" of pure-state entanglement.

---

## 21.5 Quantum Error Correction

### 21.5.1 Quantum Error Models

**The Problem:** Quantum states are fragile. Decoherence (interaction with environment) destroys quantum information. To build reliable quantum computers, we need quantum error correction.

**Quantum Error Correction Conditions (Knill-Laflamme):**

**Theorem 21.5.1 (Quantum Error Correction Conditions).** A quantum code $\mathcal{C} \subseteq H$ (subspace) can correct errors $\{E_k\}$ iff for all $|\psi\rangle, |\phi\rangle \in \mathcal{C}$:
$$\langle\psi| E_k^\dagger E_l |\phi\rangle = c_{kl} \langle\psi|\phi\rangle$$
for some constant matrix $(c_{kl})$.

*Interpretation:* The errors cannot distinguish states within the code space — they act "uniformly" on the code.

### 21.5.2 Stabilizer Codes

**Definition 21.5.2.** A *stabilizer code* is defined by a group $\mathcal{G}$ of Pauli operators: the code $\mathcal{C} = \{|\psi\rangle : g|\psi\rangle = |\psi\rangle \ \forall g \in \mathcal{G}\}$ is the common $+1$ eigenspace of all stabilizers.

**The 5-qubit Code:** The smallest quantum code correcting any single-qubit error uses 5 physical qubits to encode 1 logical qubit. Stabilized by 4 operators in the Pauli group.

**The Shor Code:** Uses 9 physical qubits, correcting any single-qubit error (including phase errors). First practical quantum error-correcting code.

**Threshold Theorem:**

**Theorem 21.5.3 (Fault-Tolerance Threshold).** If physical error rates are below a threshold $p_{\text{th}} \approx 10^{-3}$ (depends on the code and error model), arbitrarily reliable quantum computation is possible using polynomial overhead.

---

## 21.6 Quantum Information Inequalities

**Theorem 21.6.1 (Strong Subadditivity — Lieb-Ruskai 1973).** For any tripartite quantum state $\rho_{ABC}$:
$$S(AB) + S(BC) \geq S(B) + S(ABC).$$

Equivalently: the conditional mutual information $I(A;C|B) = S(AB) + S(BC) - S(ABC) - S(B) \geq 0$.

**Proof (Petz).** SSA is equivalent to joint convexity of relative entropy: $D(\rho_{12} \| \sigma_{12}) \leq D(\rho_1 \| \sigma_1) + D(\rho_2 \| \sigma_2)$ if $\rho_{12}$ is a product state but $\sigma_{12}$ is not... Actually the cleanest proof is via the *Lieb concavity theorem*: the map $(A, B) \mapsto \text{Tr}[K^\dagger A^t K B^{1-t}]$ is jointly concave for $t \in [0,1]$.

**Corollary 21.6.2 (Monotonicity of Relative Entropy).** For any CPTP map $\mathcal{E}$:
$$D(\mathcal{E}(\rho) \| \mathcal{E}(\sigma)) \leq D(\rho \| \sigma).$$

This is the quantum data processing inequality. All useful properties of quantum entropy follow from this.

---

## Exercises

**Exercise 21.1.** Compute the von Neumann entropy of $\rho = p|0\rangle\langle 0| + (1-p)|1\rangle\langle 1|$ for all $p \in [0,1]$. When is $\rho$ a pure state? What is $S(\rho)$ at $p = 1/2$ (the maximally mixed state)?

**Exercise 21.2.** Show that the Bell state $|\Phi^+\rangle = (|00\rangle + |11\rangle)/\sqrt{2}$ is a pure state of the bipartite system $AB$ with $S(\rho_{AB}) = 0$ but $S(\rho_A) = S(\rho_B) = 1$ (maximum entropy for a qubit). This demonstrates that $S(\rho_A) > S(\rho_{AB})$ is possible — unlike classical entropy.

**Exercise 21.3.** Verify the Holevo bound for the following scenario: Alice sends one of two states $\rho_0 = |0\rangle\langle 0|$ or $\rho_1 = |+\rangle\langle +| = (|0\rangle + |1\rangle)(\langle 0| + \langle 1|)/2$ with equal probability. Compute $\chi$ and compare to the classical capacity of the resulting ensemble.

**Exercise 21.4.** Verify the Knill-Laflamme conditions for the 3-qubit repetition code that encodes $|0\rangle \to |000\rangle$ and $|1\rangle \to |111\rangle$. Can it correct phase-flip errors?

**Exercise 21.5.** (Strong Subadditivity) For the tripartite state $\rho_{ABC} = |GHZ\rangle\langle GHZ|$ where $|GHZ\rangle = (|000\rangle + |111\rangle)/\sqrt{2}$: compute $S(A)$, $S(B)$, $S(C)$, $S(AB)$, $S(BC)$, $S(ABC)$, and verify SSA.

**Exercise 21.6.** (Research Connection) Quantum channels are models of noisy quantum dynamics. The quantum capacity $Q(\mathcal{E})$ is the rate of reliable quantum information transmission. For a unitary channel $\mathcal{E}(\rho) = U\rho U^\dagger$ (noiseless): $Q = 1$. For the completely depolarizing channel $\mathcal{E}(\rho) = I/d$: $Q = 0$. What is the quantum capacity of the quantum erasure channel that, with probability $\varepsilon$, replaces $\rho$ with a "known erasure" state?

---

## Chapter Notes

The standard textbooks are Nielsen-Chuang's *Quantum Computation and Quantum Information* (Chapters 9-12) and Watrous' *The Theory of Quantum Information* (available free online). Wilde's *Quantum Information Theory* is the most recent comprehensive treatment.

Strong subadditivity (Section 21.6) is the cornerstone: its proof by Lieb and Ruskai (1973) used complex interpolation. The modern proof by Petz (1988) via the Lieb concavity theorem is cleaner. An even more elementary proof using the relative entropy was given by Ruskai (2002).

The Hastings counterexample to additivity of minimum output entropy (2009) settled a 20-year-old open problem and showed that quantum channels can have "superadditive" classical capacity — sending entangled inputs achieves more than independent inputs.

For quantum error correction: Gottesman's thesis (*Stabilizer Codes and Quantum Error Correction*) is the foundational reference. The surface code, the leading candidate for practical quantum error correction, is covered in Fowler et al.'s *Surface Codes: Towards Practical Large-Scale Quantum Computation*.
