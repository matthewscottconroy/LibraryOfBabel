# 42.5 Quantum Error Correction as Dynamical Stability

A quantum error correcting code is a subspace $\mathcal{C} \subseteq \mathcal{H}^{\otimes n}$ that is protected against a group of errors $\{E_k\}$. The code space $\mathcal{C}$ is an attractor: after errors, the error correction map $\mathcal{R}$ pulls the corrupted state back to $\mathcal{C}$.

**Definition 42.5.1.** A *quantum error correcting code* (QECC) is a subspace $\mathcal{C} \subseteq \mathcal{H}^{\otimes n}$ (the "code space") that is stable under a group of errors $\{E_k\}$.

**Connection to Dynamical Systems:** The code space $\mathcal{C}$ is the analogue of an *attractor* in a dynamical system. The *recovery map* $\mathcal{R}$ (error correction circuit) is a contraction mapping that maps perturbed states back to $\mathcal{C}$.

**Theorem 42.5.2 (Knill-Laflamme Conditions as Lyapunov Stability).** A code $\mathcal{C}$ corrects errors $\{E_k\}$ iff the conditions $\langle\psi|E_k^\dagger E_l|\phi\rangle = c_{kl}\langle\psi|\phi\rangle$ hold. This is equivalent to saying:
$$\mathcal{C} \text{ is invariant under } \{E_k^\dagger E_k\} \text{ up to a "gauge" } (c_{kl}).$$

The error correction conditions say $\mathcal{C}$ is "Lyapunov stable" with respect to the error group.

The Knill-Laflamme conditions are the quantum error correction analogue of Lyapunov stability. A code is error-correcting if the code space is stable in the "direction" of the errors — the errors don't mix different codewords. This is precisely stability in the Lyapunov sense.

**Topological Quantum Error Correction:**

**Definition 42.5.3 (Toric Code — Kitaev, 1997).** The *toric code* is defined on a 2D torus with qubits on edges. Stabilizers are vertex operators $A_v = \prod_{e\ni v} X_e$ and plaquette operators $B_p = \prod_{e\in\partial p} Z_e$. The code space is the $+1$ eigenspace of all stabilizers.

**Theorem 42.5.4.** The toric code has distance $d = O(\sqrt{n})$ (for $n$ physical qubits) and encodes 2 logical qubits. Errors correspond to anyonic excitations; error correction corresponds to bringing anyons together to annihilate them (topological operations).

**Dynamical Interpretation:** The toric code dynamics under noise is a 2D classical statistical mechanics model (a random-bond Ising model on the dual lattice). The threshold theorem for the toric code is equivalent to a phase transition in this statistical model.

The threshold theorem says: if the error rate per qubit is below a critical threshold, the toric code can correct errors indefinitely. Above the threshold, errors percolate and the code fails. This is precisely a percolation phase transition — the same kind of phase transition we saw in Chapter 10. The quantum error correction threshold is a classical statistical mechanics phase transition in disguise.
