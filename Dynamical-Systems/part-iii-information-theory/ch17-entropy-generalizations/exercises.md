# Exercises — Chapter 17

The exercises below develop both the formulas and the intuition for the entropy family. The later exercises connect to dynamical systems and quantum mechanics.

**Exercise 17.1.** Compute $H_\alpha(X)$ for $\alpha = 0, 1/2, 1, 2, \infty$ for the distribution $(1/2, 1/4, 1/4)$. Verify monotonicity in $\alpha$.

**Exercise 17.2.** Show that for a fair coin $X \sim \text{Bernoulli}(1/2)$: $H_\alpha(X) = 1$ for all $\alpha > 0$. Interpret: uniform distributions have all Rényi entropies equal.

**Exercise 17.3.** Derive the maximum entropy distribution (Theorem 17.5.2) using Lagrange multipliers. Verify for the case of one constraint $E[X] = \mu$ on $[0, \infty)$.

**Exercise 17.4.** Prove that the Gaussian $N(\mu, \sigma^2)$ maximizes differential entropy among distributions with variance $\sigma^2$, using the fact that $D_{\text{KL}}(f \| g_\sigma) \geq 0$ where $g_\sigma$ is the Gaussian density.

**Exercise 17.5.** (Multifractal Connection) For the Bernoulli measure $\mu_p$ on the doubling map, the $q$-th Rényi entropy of the partition $\xi_n = \{[(k-1)/2^n, k/2^n] : k = 1, \ldots, 2^n\}$ is $H_q(\xi_n) = \frac{1}{1-q}\log \sum_k \mu_p(A_k)^q$. Compute this and connect it to the Rényi dimension $D_q$ of $\mu_p$.

**Exercise 17.6.** (Von Neumann) For a qubit $\rho = \begin{pmatrix}p & 0 \\ 0 & 1-p\end{pmatrix}$: compute $S(\rho)$. For the Bell state $\rho_{AB} = |\Phi^+\rangle\langle\Phi^+|$ where $|\Phi^+\rangle = (|00\rangle + |11\rangle)/\sqrt{2}$: compute $S(\rho_{AB})$, $S(\rho_A)$, $S(\rho_B)$, and verify strong subadditivity.
