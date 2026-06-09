# Exercises — Chapter 21

The exercises progress from basic computation to research-adjacent territory. Exercise 21.6 connects to an open problem.

**Exercise 21.1.** Compute the von Neumann entropy of $\rho = p|0\rangle\langle 0| + (1-p)|1\rangle\langle 1|$ for all $p \in [0,1]$. When is $\rho$ a pure state? What is $S(\rho)$ at $p = 1/2$ (the maximally mixed state)?

**Exercise 21.2.** Show that the Bell state $|\Phi^+\rangle = (|00\rangle + |11\rangle)/\sqrt{2}$ is a pure state of the bipartite system $AB$ with $S(\rho_{AB}) = 0$ but $S(\rho_A) = S(\rho_B) = 1$ (maximum entropy for a qubit). This demonstrates that $S(\rho_A) > S(\rho_{AB})$ is possible — unlike classical entropy.

**Exercise 21.3.** Verify the Holevo bound for the following scenario: Alice sends one of two states $\rho_0 = |0\rangle\langle 0|$ or $\rho_1 = |+\rangle\langle +| = (|0\rangle + |1\rangle)(\langle 0| + \langle 1|)/2$ with equal probability. Compute $\chi$ and compare to the classical capacity of the resulting ensemble.

**Exercise 21.4.** Verify the Knill-Laflamme conditions for the 3-qubit repetition code that encodes $|0\rangle \to |000\rangle$ and $|1\rangle \to |111\rangle$. Can it correct phase-flip errors?

**Exercise 21.5.** (Strong Subadditivity) For the tripartite state $\rho_{ABC} = |GHZ\rangle\langle GHZ|$ where $|GHZ\rangle = (|000\rangle + |111\rangle)/\sqrt{2}$: compute $S(A)$, $S(B)$, $S(C)$, $S(AB)$, $S(BC)$, $S(ABC)$, and verify SSA.

**Exercise 21.6.** (Research Connection) Quantum channels are models of noisy quantum dynamics. The quantum capacity $Q(\mathcal{E})$ is the rate of reliable quantum information transmission. For a unitary channel $\mathcal{E}(\rho) = U\rho U^\dagger$ (noiseless): $Q = 1$. For the completely depolarizing channel $\mathcal{E}(\rho) = I/d$: $Q = 0$. What is the quantum capacity of the quantum erasure channel that, with probability $\varepsilon$, replaces $\rho$ with a "known erasure" state?
