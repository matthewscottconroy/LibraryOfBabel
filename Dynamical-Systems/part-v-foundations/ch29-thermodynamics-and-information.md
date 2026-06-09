# Chapter 29 — Thermodynamics and Information

> *Maxwell's demon was defeated by Landauer: erasing one bit of information costs $kT\ln 2$ joules. Entropy is physical — Boltzmann's entropy, Gibbs's entropy, and Shannon's entropy are the same thing in different units. Thermodynamics is information theory at finite temperature.*

**Prerequisites:** Chapter 16 (Shannon entropy), Chapter 17 (entropy generalizations), Chapter 22 (entropy in dynamical systems).

---

## 29.1 The Boltzmann-Gibbs-Shannon Connection

### 29.1.1 Statistical Mechanics Entropy

**Definition 29.1.1 (Boltzmann Entropy).** For a macrostate with $\Omega$ microstates (each equally likely):
$$S_B = k_B \ln \Omega,$$
where $k_B = 1.38 \times 10^{-23}$ J/K is Boltzmann's constant.

**Definition 29.1.2 (Gibbs Entropy).** For a probability distribution $p = (p_1, \ldots, p_n)$ over microstates:
$$S_G = -k_B \sum_i p_i \ln p_i.$$

**Theorem 29.1.3 (Shannon = Gibbs).** $S_G = k_B \cdot H(p)/\log e$, where $H(p) = -\sum_i p_i \log_2 p_i$ is Shannon entropy in bits. The conversion: 1 bit $= k_B \ln 2$ joules/kelvin.

**Maximum Entropy Principle:** Among all distributions over $n$ microstates, the uniform distribution maximizes $S_G$. The Gibbs distribution $p_i \propto e^{-E_i/k_BT}$ maximizes $S_G$ subject to fixed mean energy $\langle E \rangle$.

### 29.1.2 Free Energy and Information

**Definition 29.1.4.** The *Helmholtz free energy* is $F = U - TS$ where $U = \langle E \rangle$ is the internal energy, $T$ is temperature, $S$ is Gibbs entropy. At equilibrium, $F$ is minimized.

**Theorem 29.1.5 (Free Energy = Negative Relative Entropy).** For a system with Gibbs equilibrium distribution $p^* \propto e^{-E_i/k_BT}$ and actual distribution $q$:
$$F(q) = F(p^*) + k_BT \cdot D_{KL}(q \| p^*).$$

So the *excess free energy* is $k_BT$ times the KL divergence from equilibrium. Minimizing free energy = minimizing KL divergence.

---

## 29.2 Landauer's Principle

**Theorem 29.2.1 (Landauer's Principle, 1961).** Erasing one bit of information in a system at temperature $T$ requires dissipating at least $k_BT\ln 2$ joules of energy as heat.

*(physical argument)* A bit stores 1 bit of entropy, $S = k_B\ln 2$. Erasing the bit (mapping both states to one) reduces the system entropy by $k_B\ln 2$. By the second law, this entropy must be transferred to the environment as heat $Q \geq k_BT\ln 2$.

**Maxwell's Demon Resolution:** Maxwell's demon can reduce the entropy of a gas by observing and sorting molecules, seeming to violate the second law. Resolution: the demon must erase its memory (a classical storage device) to operate cyclically. The erasure cost exactly compensates the entropy reduction.

**Theorem 29.2.2 (Szilard Engine).** A Szilard engine (a one-molecule gas) can extract $k_BT\ln 2$ of work from a single bit of information. The information erasure cost $k_BT\ln 2$ makes the cycle thermodynamically consistent with the second law.

---

## 29.3 The Jarzynski Equality and Fluctuation Theorems

**Theorem 29.3.1 (Jarzynski Equality, 1997).** For a system driven from equilibrium state $A$ to equilibrium state $B$ by an arbitrary protocol:
$$\langle e^{-W/k_BT} \rangle = e^{-\Delta F/k_BT},$$
where $W$ is the work done on the system (random variable) and $\Delta F = F_B - F_A$ is the free energy difference.

**Corollary 29.3.2 (Second Law as Jensen's Inequality).** By Jensen's inequality $\langle e^{-W/k_BT}\rangle \geq e^{-\langle W\rangle/k_BT}$, so:
$$\langle W \rangle \geq \Delta F,$$
which is the second law (average work $\geq$ free energy change).

**Theorem 29.3.3 (Crooks Fluctuation Theorem, 1999).** For forward and reverse protocols connecting $A \leftrightarrow B$:
$$\frac{P_F(W)}{P_R(-W)} = e^{(W - \Delta F)/k_BT}.$$

The ratio of probabilities of observing work $W$ forward and $-W$ in reverse gives the entropy production.

**Information-Theoretic Formulation:** The Jarzynski equality is the statement that the KL divergence between the forward and reverse path distributions equals the average entropy production:
$$D_{KL}(P_F \| P_R) = \langle \Sigma \rangle / k_B,$$
where $\Sigma$ is the total entropy production (system + environment).

---

## 29.4 Thermodynamic Formalism

**Definition 29.4.1 (Transfer Matrix / Partition Function).** For an SFT with transition matrix $A$ and potential $\phi: X \to {\mathbb R}$, the *partition function* at inverse temperature $\beta$ is:
$$Z_n(\beta) = \sum_{x: f^n(x) = x} e^{\beta\sum_{k=0}^{n-1}\phi(f^k(x))}.$$

**Theorem 29.4.2.** The *free energy* $F(\beta) = \lim_{n\to\infty}\frac{1}{n}\log Z_n(\beta) = P(f, \beta\phi)$ (the topological pressure).

**Statistical Mechanics Analogy:**

| Thermodynamics | Thermodynamic Formalism |
|---|---|
| States (microstates) | Periodic orbits |
| Energy $E_i$ | $-\sum_{k=0}^{n-1}\phi(f^k(x))$ (action) |
| Inverse temperature $\beta$ | Parameter $\beta$ |
| Partition function $Z$ | $\sum_{\text{Per}_n} e^{\beta S_n(x)}$ |
| Free energy $-\frac{1}{\beta}\log Z$ | $-P(f, \beta\phi)$ |
| Gibbs state | Equilibrium state (SRB measure) |
| Phase transition | Non-differentiability of $P(\beta\phi)$ |

**Theorem 29.4.3 (Phase Transitions in Dynamics).** The pressure function $\beta \mapsto P(f, \beta\phi)$ is convex and continuous. A *phase transition* occurs when $P$ is not differentiable at some $\beta^*$ — i.e., there are multiple tangent measures (equilibrium states) at $\beta^*$.

**Example 29.4.4 (Hofbauer Tower).** For piecewise monotone interval maps, the existence of phase transitions at $\beta = 1$ (the inverse temperature corresponding to the natural measure) is related to the number of SRB measures and the decay of correlations.

---

## 29.5 Entropy Production and Irreversibility

**Definition 29.5.1.** For a Markov chain with transition matrix $P$ and stationary distribution $\pi$, the *entropy production rate* is:
$$\dot{\sigma} = \sum_{i,j} \pi_i P_{ij} \log\frac{P_{ij}}{P_{ji}} \geq 0.$$

The system is *reversible* (in detailed balance) iff $\dot{\sigma} = 0$, i.e., $\pi_i P_{ij} = \pi_j P_{ji}$.

**Theorem 29.5.2 (Second Law for Markov Chains).** The relative entropy $D_{KL}(\mu_t \| \pi)$ of the distribution at time $t$ from the stationary distribution is monotonically decreasing:
$$\frac{d}{dt} D_{KL}(\mu_t \| \pi) \leq 0,$$
with equality iff $\mu_t = \pi$ (at stationarity).

**Dynamical Systems Formulation:** For a measure-preserving system $(X, \mu, f)$, entropy production is zero — the system is at "infinite temperature equilibrium." For a dissipative system with an SRB measure $\mu_{SRB}$, the entropy production rate equals the sum of negative Lyapunov exponents (measuring the "phase space contraction").

---

## Exercises

**Exercise 29.1.** Verify that the Gibbs distribution $p_i = Z^{-1}e^{-\beta E_i}$ maximizes the Gibbs entropy $S = -k_B\sum_i p_i\ln p_i$ subject to the constraint $\sum_i p_i E_i = U$ (fixed mean energy). Use Lagrange multipliers.

**Exercise 29.2.** (Jarzynski) A spring is compressed by work $W$. If the equilibrium free energy change is $\Delta F = 2$ J and the temperature is $T = 300$ K, what is the minimum possible average work? Simulate 100 trajectories and verify Jarzynski numerically (use exponential distribution for $W$ with rate $1/k_BT$).

**Exercise 29.3.** Compute the entropy production rate for the Markov chain with transition matrix $P = \begin{pmatrix}0.9 & 0.1 \\ 0.3 & 0.7\end{pmatrix}$. Is it in detailed balance?

**Exercise 29.4.** (Thermodynamic Formalism) For the full 2-shift with potential $\phi(x) = -t\cdot x_0$ (where $x_0 \in \{0,1\}$ is the 0th symbol): compute $P(f, \phi)$ and find the equilibrium state for each $t \in {\mathbb R}$.

---

## Chapter Notes

Jaynes's *Information Theory and Statistical Mechanics* (1957) established the information-theoretic foundations of statistical mechanics. Landauer's original paper is *Irreversibility and Heat Generation in the Computing Process* (IBM Journal, 1961). The experimental verification of Landauer's principle is in Bérut et al. (Nature, 2012).

The Jarzynski equality and Crooks fluctuation theorem are in Jarzynski (1997) and Crooks (1999); a pedagogical treatment is in Seifert's *Stochastic Thermodynamics, Fluctuation Theorems, and Molecular Machines* (2012).

Thermodynamic formalism as developed by Ruelle and Bowen is in Ruelle's *Thermodynamic Formalism* (2004, 2nd ed.). The connection to statistical mechanics is explicit in Chapter 2 of Ruelle's book.
