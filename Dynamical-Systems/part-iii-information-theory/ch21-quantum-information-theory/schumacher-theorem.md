# 21.2 Quantum Source Coding — Schumacher's Theorem

Shannon's source coding theorem says: you can compress an i.i.d. classical source with entropy $H$ to $H$ bits per symbol, but not less. What is the quantum analogue?

For a quantum source — a device that repeatedly produces the same quantum state $\rho$ — the answer is Schumacher's theorem (1995): the minimum compression rate is $S(\rho)$ qubits per symbol, where $S(\rho)$ is the von Neumann entropy.

**Classical reminder:** Shannon's source coding theorem says an i.i.d. source $X$ with entropy $H$ can be compressed to $H$ bits/symbol.

**Quantum analog:** An i.i.d. quantum source $\rho^{\otimes n}$ can be compressed to $S(\rho)$ qubits/symbol.

**Theorem 21.2.1 (Schumacher Compression, 1995).** For an i.i.d. source producing state $\rho$ with von Neumann entropy $S(\rho) = -\text{Tr}[\rho\log\rho]$:
- *(Achievability)* For any $R > S(\rho)$ and $\varepsilon > 0$: there is a compression scheme using $nR$ qubits per symbol with fidelity $\geq 1 - \varepsilon$ for large $n$.
- *(Converse)* For any $R < S(\rho)$: the fidelity of any compression scheme $\to 0$.

The proof mirrors the classical AEP argument, but in the quantum setting:

**Proof idea:** Use the spectral decomposition $\rho = \sum_i \lambda_i |i\rangle\langle i|$. The eigenvalues $\{\lambda_i\}$ define a classical probability distribution, and by the classical AEP, $n$ i.i.d. copies of this distribution have their probability concentrated on $\approx 2^{nS(\rho)}$ typical eigenstates. The *quantum typical subspace* is the subspace spanned by the tensor products $|i_1 \cdots i_n\rangle$ of eigenstates with typical joint eigenvalue products. This typical subspace has dimension $\approx 2^{nS(\rho)}$.

Project the $n$-copy state $\rho^{\otimes n}$ onto the typical subspace: this "discards" the atypical part (which has exponentially small probability) and compresses the remaining state to $nS(\rho)$ qubits. Transmit the compressed state and recover the original by reversing the projection. The error from discarding the atypical subspace is $\leq \varepsilon$ for large $n$.

The converse: if you use fewer than $nS(\rho)$ qubits, the Hilbert space is too small to accommodate the quantum typical subspace, and fidelity must drop.

Schumacher's theorem is elegant because it shows that von Neumann entropy is not just a mathematical formula — it is an operational compression limit for quantum information, exactly as Shannon entropy is for classical information. The two theorems have the same logical structure and are proved with the same type of argument. The quantum typical subspace plays the role of the classical typical set.

This parallel is not a coincidence: it reflects the deep fact that quantum information theory is the quantum generalization of Shannon's framework, with density matrices playing the role of probability distributions and von Neumann entropy playing the role of Shannon entropy throughout.
