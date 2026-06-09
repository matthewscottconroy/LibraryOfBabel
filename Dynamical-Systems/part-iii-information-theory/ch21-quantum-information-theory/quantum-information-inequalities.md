# 21.6 Quantum Information Inequalities

The technical heart of quantum information theory is a single inequality: strong subadditivity of von Neumann entropy (SSA). Everything else — Schumacher's theorem, the quantum channel capacity bounds, the entanglement theory — either relies on SSA directly or follows from its equivalent reformulation as monotonicity of quantum relative entropy.

**Theorem 21.6.1 (Strong Subadditivity — Lieb-Ruskai 1973).** For any tripartite quantum state $\rho_{ABC}$:
$$S(AB) + S(BC) \geq S(B) + S(ABC),$$
where we write $S(AB)$ for $S(\rho_{AB})$, etc.

Equivalently: the *quantum conditional mutual information* is nonnegative:
$$I(A; C \mid B) = S(AB) + S(BC) - S(ABC) - S(B) \geq 0.$$

In the classical case, this is obvious: $I(A; C \mid B) = H(A \mid B) - H(A \mid BC) \geq 0$ because conditioning reduces entropy. In the quantum case, the analogous statement is not at all obvious, because the quantum conditional entropy $S(A \mid B) = S(AB) - S(B)$ can be negative (for entangled states). The fact that $I(A; C \mid B) \geq 0$ nonetheless is deep.

**Proof (sketch).** The cleanest proof uses the *Lieb concavity theorem*: the map $(A, B) \mapsto \text{Tr}[K^\dagger A^t K B^{1-t}]$ is jointly concave for $t \in [0,1]$ and all fixed operators $K$. This implies the joint convexity of quantum relative entropy, which implies strong subadditivity. The original Lieb-Ruskai proof used complex interpolation to prove the Lieb concavity theorem; Petz later gave a proof via the Peierls-Bogoliubov inequality.

**Corollary 21.6.2 (Monotonicity of Quantum Relative Entropy).** For any CPTP map $\mathcal{E}$:
$$D(\mathcal{E}(\rho) \| \mathcal{E}(\sigma)) \leq D(\rho \| \sigma).$$

This is the *quantum data processing inequality*: quantum channels cannot increase the relative entropy (distinguishability) between states. It is the quantum analogue of the classical data processing inequality from Section 16.6, and it implies SSA (and is equivalent to it).

All the useful properties of quantum entropy — the concavity of von Neumann entropy, the monotonicity of quantum relative entropy, the channel capacity bounds — follow from SSA or its equivalent. This is why the Lieb-Ruskai result is the cornerstone of the field.

**What makes the quantum case hard:** In the classical case, all the entropy inequalities follow easily from the nonnegativity of KL divergence (Gibbs' inequality), which is itself a one-line Jensen's inequality argument. In the quantum case, the matrices do not commute, and the spectral decomposition of $\rho_{ABC}$ is not simply related to those of $\rho_{AB}$ and $\rho_{BC}$. The non-commutativity is what makes SSA non-trivial and why its proof required sophisticated complex analysis.

**Equality conditions:** SSA is an equality iff the system $ABC$ is in a *quantum Markov chain* state — a state where $C$ is independent of $A$ given $B$ in the quantum sense. The characterization of equality cases (Hayden-Junge-Wilde-Winter) uses an explicit reconstruction map and has applications to quantum error correction.

**Looking forward:** Quantum information theory is a young field — the major results are from the 1990s and 2000s, and many fundamental questions remain open. The additivity of quantum channel capacities, the structure of multi-party entanglement, the quantum capacity of specific channels, and the information-theoretic foundations of quantum thermodynamics are all active areas of research. What we have covered in this chapter is the core of the subject — the theorems that are known, proved, and understood. The frontier is much larger.
