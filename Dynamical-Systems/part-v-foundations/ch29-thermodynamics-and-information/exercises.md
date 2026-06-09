# Exercises — Chapter 29

These problems build intuition for the thermodynamics-information connection. Exercises 29.1 and 29.3 are primarily computational; 29.2 bridges theory and simulation; 29.4 connects thermodynamic formalism to the symbolic dynamics of Chapter 12.

---

**Exercise 29.1.** Verify that the Gibbs distribution $p_i = Z^{-1}e^{-\beta E_i}$ maximizes the Gibbs entropy $S = -k_B\sum_i p_i\ln p_i$ subject to the constraint $\sum_i p_i E_i = U$ (fixed mean energy). Use Lagrange multipliers.

**Exercise 29.2.** (Jarzynski) A spring is compressed by work $W$. If the equilibrium free energy change is $\Delta F = 2$ J and the temperature is $T = 300$ K, what is the minimum possible average work? Simulate 100 trajectories and verify Jarzynski numerically (use exponential distribution for $W$ with rate $1/k_BT$).

**Exercise 29.3.** Compute the entropy production rate for the Markov chain with transition matrix $P = \begin{pmatrix}0.9 & 0.1 \\ 0.3 & 0.7\end{pmatrix}$. Is it in detailed balance?

**Exercise 29.4.** (Thermodynamic Formalism) For the full 2-shift with potential $\phi(x) = -t\cdot x_0$ (where $x_0 \in \{0,1\}$ is the 0th symbol): compute $P(f, \phi)$ and find the equilibrium state for each $t \in \mathbb{R}$.
