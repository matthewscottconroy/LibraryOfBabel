# 21.4 Entanglement Theory

Entanglement is the defining feature of quantum mechanics that has no classical counterpart. Two particles can share a quantum state in which their properties are correlated in ways that cannot be explained by any classical probability distribution over local states. But entanglement is not just a philosophical curiosity — it is a resource, and information theory lets us quantify how much entanglement a state contains and what you can do with it.

## 21.4.1 Entanglement Measures

For bipartite pure states, there is a unique natural measure of entanglement:

**Definition 21.4.1 (Entanglement Entropy).** For a bipartite pure state $|\psi\rangle_{AB}$ with Schmidt decomposition $|\psi\rangle = \sum_i \sqrt{\lambda_i} |i\rangle_A |i\rangle_B$, the *entanglement entropy* is:
$$E(|\psi\rangle) = S(\rho_A) = -\sum_i \lambda_i \log \lambda_i,$$
where $\rho_A = \text{Tr}_B[|\psi\rangle\langle\psi|]$ is the reduced state of $A$.

The Schmidt decomposition is the quantum analogue of the singular value decomposition: every bipartite pure state $|\psi\rangle_{AB}$ can be written as $\sum_i \sqrt{\lambda_i} |i\rangle_A|i\rangle_B$ for orthonormal bases $\{|i\rangle_A\}$ and $\{|i\rangle_B\}$. The Schmidt coefficients $\sqrt{\lambda_i}$ determine how entangled the state is.

For product states ($|\psi\rangle = |\phi\rangle_A \otimes |\chi\rangle_B$), there is only one Schmidt coefficient ($\lambda_1 = 1$), so $E = 0$ — no entanglement. For the Bell states, the Schmidt coefficients are both $1/\sqrt{2}$, giving $E = 1$ ebit (the maximum for two qubits). More entanglement corresponds to more "mixed" reduced states.

**Theorem 21.4.2 (Uniqueness for Pure States).** For pure states, entanglement entropy is the unique entanglement measure (up to normalization) satisfying: monotonicity under LOCC (local operations and classical communication), continuity, and normalization.

LOCC is the class of operations that cannot create entanglement: you can perform any quantum operations on your local system and communicate classically, but you cannot use the quantum channel for signaling. Any reasonable entanglement measure must be nonincreasing under LOCC — you can't create entanglement for free. The uniqueness theorem says entanglement entropy is the only function with these natural properties.

**Entanglement of Formation:** For mixed states, the situation is more complex. The *entanglement of formation* extends the definition to mixed states:
$$E_F(\rho_{AB}) = \min_{\{p_i, |\psi_i\rangle\}} \sum_i p_i E(|\psi_i\rangle),$$
where the minimum is over all pure state decompositions $\rho_{AB} = \sum_i p_i |\psi_i\rangle\langle\psi_i|$.

This is the minimum average entanglement needed to prepare $\rho_{AB}$ by mixing pure states — the "cost" of the entanglement in $\rho$.

## 21.4.2 Entanglement Distillation and Dilution

The operational meaning of entanglement entropy becomes clearest through the conversion theorems. Bell pairs (maximally entangled two-qubit states) are the "gold standard" of entanglement — maximally useful, universally applicable. The question is: how do arbitrary entangled states convert to and from Bell pairs?

**Theorem 21.4.3 (Bennett et al., Hayden et al.).** For a bipartite pure state $|\psi\rangle^{\otimes n}$:
- *Distillation*: by LOCC operations, one can extract $\approx nE(|\psi\rangle)$ maximally entangled Bell pairs.
- *Dilution*: $\approx nE(|\psi\rangle)$ Bell pairs suffice to create $n$ copies of $|\psi\rangle$ by LOCC.

The entanglement entropy $E(|\psi\rangle)$ is thus both the rate of Bell pair extraction (distillation) and the rate of Bell pair consumption (dilution). It is the "exchange rate" between arbitrary pure entangled states and the Bell pair currency.

This is a profound result: entanglement entropy is not just a mathematical quantity — it is the unique number that tells you how many Bell pairs you can get from a state and how many you need to make it. The analogy with Shannon entropy is exact: just as Shannon entropy is both the compression rate of a classical source and the expansion rate for reliable transmission, entanglement entropy is both the distillation rate and the dilution rate for quantum entanglement.

For mixed states, the situation is more complex and generally not reversible: the distillable entanglement $E_D$ (rate of Bell pair extraction) can be strictly less than the entanglement cost $E_C$ (rate of Bell pair consumption). This irreversibility of mixed-state entanglement is a fundamental quantum phenomenon with no classical analogue.
