# 42.6 Variational Quantum Algorithms and Optimization

Variational quantum algorithms are hybrid classical-quantum algorithms that use a quantum circuit to evaluate a cost function and a classical optimizer to find the optimal parameters. The most famous is the Quantum Approximate Optimization Algorithm (QAOA), designed to solve combinatorial optimization problems.

**Definition 42.6.1 (QAOA — Quantum Approximate Optimization Algorithm).** QAOA is a variational algorithm with circuit:
$$|\boldsymbol{\gamma},\boldsymbol{\beta}\rangle = \prod_{k=1}^p e^{-i\beta_k H_B}e^{-i\gamma_k H_C}|+\rangle^{\otimes n},$$
where $H_C$ is the cost Hamiltonian (encoding the optimization problem), $H_B = \sum_i X_i$ is the mixing Hamiltonian, and $(\boldsymbol{\gamma}, \boldsymbol{\beta})$ are variational parameters optimized classically.

**Connection to Dynamical Systems:** QAOA is a discrete-time dynamical system in parameter space. The optimization landscape $E(\boldsymbol{\gamma}, \boldsymbol{\beta}) = \langle\boldsymbol{\gamma},\boldsymbol{\beta}|H_C|\boldsymbol{\gamma},\boldsymbol{\beta}\rangle$ is a function on a manifold, and classical gradient descent on this function is the "outer loop." The quantum circuit is the "inner loop" evaluating $E$.

The parameter space for QAOA of depth $p$ is $\mathbb{R}^{2p}$ — $p$ pairs $(\gamma_k, \beta_k)$. The energy landscape $E: \mathbb{R}^{2p} \to \mathbb{R}$ is a smooth function (the expectation value of a Hermitian operator in a state that varies smoothly with the parameters). Gradient descent on this landscape is a classical dynamical system: a flow on $\mathbb{R}^{2p}$ with fixed points at local minima of $E$.

**Theorem 42.6.2 (QAOA as Trotterized Adiabatic Evolution).** For large $p$, QAOA approximates the adiabatic evolution from $H_B$ to $H_C$. The QAOA parameters $\gamma_k \approx t_k/p$ and $\beta_k \approx (T-t_k)/p$ match the adiabatic interpolation time schedule.

This connection is exact in the limit $p \to \infty$: infinite-depth QAOA is exactly adiabatic quantum computation. At finite depth, QAOA is a Trotterization of the adiabatic path. The depth $p$ controls the quality of the approximation, and for hard optimization problems (small spectral gap), you need $p = \Omega(1/\Delta_{\min})$ — exponential depth in the worst case.

The landscape of variational quantum algorithms is itself a dynamical system, and understanding the basins of attraction (which initial parameters lead to the global minimum?), the presence of "barren plateaus" (flat regions where gradients vanish), and the complexity of the optimization are active research problems that draw on the full toolkit of dynamical systems theory.
