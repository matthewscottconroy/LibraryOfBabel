# Chapter 31 Exercises

**Exercise 31.1.** *(Density matrix basics)* A qubit density matrix has the form $\rho = \frac{1}{2}(I + \vec{r}\cdot\vec{\sigma})$ where $\vec{r} \in \mathbb{R}^3$ with $|\vec{r}| \leq 1$ (the Bloch vector) and $\vec{\sigma} = (\sigma_x, \sigma_y, \sigma_z)$.
(a) Show that a valid density matrix (Hermitian, positive semidefinite, trace 1) corresponds exactly to $|\vec{r}| \leq 1$.
(b) For the Lindblad equation with $H = \omega\sigma_z/2$ and $L = \sqrt{\gamma}\sigma_-$, write down the ODE for $\vec{r}(t) = (\langle\sigma_x\rangle, \langle\sigma_y\rangle, \langle\sigma_z\rangle)$. Find the steady state $\vec{r}_{\text{ss}}$ and the relaxation rate.
(c) This qubit driven by a time-varying input $u(t)$ that modulates $\omega$: $\omega \to \omega + \alpha u(t)$. Show that the time-averaged output $\langle\sigma_z\rangle(t)$ is a fading-memory functional of $u$.

**Exercise 31.2.** *(Liouvillian gap)* For an $N$-qubit system with Lindblad operators $L_k$, the Liouvillian $\mathcal{L}$ acts on $N^2 \times N^2$ matrices (as a superoperator on density matrices). The gap $\Delta_{\mathcal{L}}$ determines the fading memory rate.
(a) For a single qubit with dephasing $L = \sqrt{\gamma}\sigma_z$: find all eigenvalues of $\mathcal{L}$ and compute $\Delta_{\mathcal{L}}$.
(b) For two qubits with independent dephasing on each qubit: find $\Delta_{\mathcal{L}}$ and show that the two-qubit coherences decay at rate $2\gamma$ (faster than single-qubit coherences).
(c) Why does a faster Liouvillian gap mean a shorter memory? Relate $\Delta_{\mathcal{L}}$ to the weight sequence $w_k$ in the Boyd-Chua framework.

**Exercise 31.3.** *(Quantum observables as polynomial functionals)* Consider a 2-qubit quantum reservoir driven by input $u(t)$ via $H_{\text{input}}(t) = u(t)\sigma_z^{(1)}$. The state evolves as $\rho(t+1) = \mathcal{E}_{u(t)}(\rho(t))$ where $\mathcal{E}_u$ is the discretized Lindblad evolution.
(a) Show that $\text{tr}(O\rho(t))$ for any fixed observable $O$ is a linear functional of $\rho(t)$, hence (by induction) a polynomial functional of the input history $(u(t), u(t-1), \ldots, u(0))$.
(b) Show that the algebra of such polynomial functionals (as $O$ ranges over Pauli operators and $t \to \infty$) separates points in the input space, establishing quantum universality via Stone-Weierstrass.

**Exercise 31.4.** *(Simulation cost)* An $N$-qubit quantum reservoir can be classically simulated by tracking the $4^N$-dimensional vector of Pauli expectation values $v_k = \text{tr}(P_k \rho)$ for all $4^N$ Pauli strings $P_k$.
(a) Write down the classical simulation update rule for $v$ under the Lindblad equation.
(b) How does the simulation cost scale with $N$? At what $N$ does exact classical simulation become infeasible (say, $> 10^{12}$ operations per time step)?
(c) For the NISQ experiments described in Section 31.3 ($N = 4-10$), is exact classical simulation feasible? What does this imply about quantum advantage claims?

**Exercise 31.5.** *(NISQ experiment design)* Design a quantum reservoir computing experiment on IBM Quantum (use the IBM Quantum Experience API or Qiskit) with $N = 5$ qubits for the task of predicting a chaotic time series. Specify:
(a) The input encoding circuit.
(b) The fixed reservoir circuit (specify the gates).
(c) The observables measured.
(d) The training procedure.
Run the experiment (or simulate with noise) and compare the NMSE to a classical ESN with 5 units and with 25 units (matching the number of features from the quantum system, assuming 2-qubit correlators are measured).

**Exercise 31.6.** *(Quantum advantage argument and rebuttal)* Write a 500-word argument FOR quantum advantage in reservoir computing, citing the Hilbert space dimension argument. Then write a 500-word rebuttal citing the classical simulation argument (Section 31.1.2). Identify the key assumption in each argument, and describe what experiment would resolve the dispute.

**Exercise 31.7.** *(Research problem — optimal decoherence rate)* Theorem 31.2.1 says quantum reservoirs with sufficient qubits and "generic" Hamiltonian coupling achieve universality. But universality is an existence result; the performance of a specific quantum reservoir depends on the decoherence rate $\gamma$.
(a) For a specific task (e.g., NARMA-10), simulate a quantum reservoir (2-qubit toy model) at varying decoherence rates $\gamma \in [0.01, 10]$. Plot NMSE vs. $\gamma$.
(b) Is there an optimal $\gamma$? If so, how does it relate to the input correlation time?
(c) Conjecture an optimal decoherence rate as a function of the input's characteristic frequency, and test your conjecture numerically.
