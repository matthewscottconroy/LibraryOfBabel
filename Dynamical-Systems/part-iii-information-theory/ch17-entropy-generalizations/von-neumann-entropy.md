# 17.7 Von Neumann Entropy

In quantum mechanics, the state of a system is not described by a probability distribution but by a *density matrix* — a positive semidefinite matrix with trace 1. The density matrix generalizes both pure quantum states and classical probability distributions. What is the right notion of entropy for such an object?

John von Neumann answered this question in the 1920s, before Shannon's work, arriving at a formula that turns out to be the quantum analogue of Shannon entropy.

**Definition 17.7.1 (Von Neumann Entropy).** The *von Neumann entropy* of a quantum state $\rho$ (a density matrix, $\rho \geq 0$, $\text{Tr}[\rho] = 1$) is:
$$S(\rho) = -\text{Tr}[\rho \log \rho] = -\sum_i \lambda_i \log \lambda_i,$$
where $\lambda_i$ are the eigenvalues of $\rho$.

If $\rho = \sum_i \lambda_i |\psi_i\rangle\langle\psi_i|$ is the spectral decomposition, then $S(\rho) = H(\lambda_1, \ldots, \lambda_n)$ — the Shannon entropy of the eigenvalue distribution. Von Neumann entropy is thus the Shannon entropy of the "probability distribution of quantum states."

**Properties:**
1. $S(\rho) = 0$ iff $\rho$ is a *pure state* ($\rho = |\psi\rangle\langle\psi|$, rank 1). Pure quantum states have zero entropy: they are perfectly specified.
2. $S(\rho) \leq \log d$ ($d$ = dimension), with equality iff $\rho = I/d$ (the *maximally mixed state*).
3. *Concavity*: $S(\sum_i p_i \rho_i) \geq \sum_i p_i S(\rho_i)$ (mixing quantum states increases entropy).
4. *Unitary invariance*: $S(U\rho U^\dagger) = S(\rho)$.

These are the quantum analogues of Shannon's basic properties, and they hold for the same reasons: entropy measures uncertainty, unitary evolution preserves information, and mixing can only increase uncertainty.

The deepest property of von Neumann entropy is strong subadditivity:

**Theorem 17.7.2 (Strong Subadditivity — Lieb-Ruskai 1973).** For any tripartite quantum state on systems $A$, $B$, $C$:
$$S(\rho_{AB}) + S(\rho_{BC}) \geq S(\rho_{ABC}) + S(\rho_B).$$

This inequality — proved by Lieb and Ruskai using complex interpolation theory — is the cornerstone of quantum information theory. From it follows:
- *Weak subadditivity*: $S(\rho_{AB}) \leq S(\rho_A) + S(\rho_B)$ (entropy of a composite system is at most the sum of its parts).
- *Quantum data processing inequality*: entropy cannot decrease under quantum channels.
- The *quantum channel capacity* bounds that underlie Schumacher's compression theorem and the LSD theorem.

The classical analogue of strong subadditivity is obvious: $H(AB) + H(BC) \geq H(ABC) + H(B)$ is just $I(A;C|B) \geq 0$ (conditional mutual information is nonnegative), which follows immediately from the definition. In the quantum setting, the analogous statement is far from obvious, because density matrices do not commute and the spectral structure of $\rho_{ABC}$ is not simply related to those of $\rho_{AB}$ and $\rho_{BC}$.

**Remark 17.7.3.** Strong subadditivity is equivalent to the *monotonicity of quantum relative entropy*: for any quantum channel $\mathcal{E}$,
$$D(\mathcal{E}(\rho) \| \mathcal{E}(\sigma)) \leq D(\rho \| \sigma).$$
This is the quantum data processing inequality, and it is just as fundamental as its classical counterpart. The proof via Lieb-Ruskai uses the Lieb concavity theorem: the map $(A, B) \mapsto \text{Tr}[K^\dagger A^t K B^{1-t}]$ is jointly concave for $t \in [0,1]$. An elegant alternative proof by Petz uses the Peierls-Bogoliubov inequality.

Von Neumann entropy has one striking property with no classical analogue. For a bipartite pure state $|\psi\rangle_{AB}$:
$$S(\rho_{AB}) = 0 \quad \text{but} \quad S(\rho_A), S(\rho_B) > 0 \text{ in general}.$$

This is impossible classically: if $H(X, Y) = 0$, then both $X$ and $Y$ are deterministic. But in quantum mechanics, a pure bipartite state can have zero global entropy while each subsystem has maximum entropy. This is *entanglement*: the system is perfectly specified globally, but each part is maximally uncertain. Entropy is no longer subadditive in the direction you might expect — knowing the whole does not give you more certainty about the parts.

This quantum peculiarity drives much of quantum information theory, and we will explore it in Chapter 21.
