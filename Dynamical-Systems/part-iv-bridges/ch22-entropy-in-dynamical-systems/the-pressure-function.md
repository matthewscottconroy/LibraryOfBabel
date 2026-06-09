# 22.5 The Pressure Function

The variational principle — $h_{\text{top}} = \sup_\mu h_\mu$ — is a clean duality. The pressure function is its generalization: instead of just maximizing entropy, we maximize entropy plus the expected value of a "potential" function $\phi: X \to \mathbb{R}$. This might seem like a technical extension, but it unlocks a much richer theory and connects directly to equilibrium statistical mechanics.

Think of $\phi$ as an energy function: each point $x$ has energy $\phi(x)$, and the orbit of a point accumulates energy over time. We want to find the invariant measure that best balances high entropy (exploration) against high energy (exploitation). The optimal trade-off is the pressure, and the achieving measure is the equilibrium state.

**Definition 22.5.1.** For a continuous map $f$ on a compact metric space and a continuous function $\phi: X \to {\mathbb R}$ (potential), the *topological pressure* is:
$$P(f, \phi) = \sup_\mu \left[h_\mu(f) + \int \phi\,d\mu\right].$$

This is the *Legendre transform* of the entropy function, viewed as a function of the "inverse temperature" parametrized by $\phi$.

Setting $\phi = 0$ recovers the variational principle: $P(f, 0) = h_{\text{top}}(f)$. The pressure function tells us how entropy trades off against the average energy $\int \phi \, d\mu$ as we vary over invariant measures.

**Theorem 22.5.2 (Variational Principle for Pressure — Walters).** For an Axiom A system:
$$P(f, \phi) = \lim_{n\to\infty} \frac{1}{n} \log \sum_{\text{Per}_n(f)} \exp\left(\sum_{k=0}^{n-1} \phi(f^k(x))\right),$$
where the sum is over periodic points of period $n$.

This formula is remarkable: it expresses the pressure entirely in terms of periodic orbits. The Boltzmann weight $\exp\left(\sum_{k=0}^{n-1} \phi(f^k(x))\right)$ gives each periodic orbit a weight proportional to the total energy accumulated along the orbit. The pressure is the free energy of this "gas" of periodic orbits.

**Equilibrium States:** The measure achieving $h_\mu(f) + \int \phi\,d\mu = P(f,\phi)$ is the *equilibrium state for $\phi$*. For $\phi = 0$: the equilibrium state is the MME. For $\phi = -\log|Df|$ (negative Jacobian): the equilibrium state is the SRB measure (Sinai-Ruelle-Bowen).

The SRB measure is especially important: it's the measure you see when you run the system on a computer starting from generic initial conditions (in Lebesgue measure). The equilibrium state for the potential $-\log|Df|$ is the "physical" measure — the one observed by experiment. This is the thermodynamic formalism connecting mathematics to physics.

**Connection to Statistical Mechanics:** In thermodynamics, the free energy $F = U - TS$ (internal energy minus temperature times entropy) is minimized at equilibrium. The pressure function is $-F/T$; the equilibrium state is the Gibbs distribution for potential $\phi$.

To make this concrete: the Gibbs distribution for energy function $-\phi$ at inverse temperature $\beta$ puts measure proportional to $e^{\beta \phi(x)}$ at each point. The pressure $P(f, \beta\phi)$ is (up to sign) the logarithm of the partition function. As $\beta \to \infty$ (low temperature), the measure concentrates on the highest-energy periodic orbits. As $\beta \to 0$ (high temperature), entropy dominates and we approach the MME.

This is the thermodynamic formalism in dynamical systems: every potential function $\phi$ defines a "temperature," and the equilibrium states are the stationary distributions at that temperature. The pressure function encodes the entire phase diagram of the system.

Phase transitions — where the pressure function fails to be differentiable with respect to $\phi$ — correspond to genuinely different dynamical behaviors coexisting at the same temperature. They are the dynamical analogues of liquid-gas transitions in thermodynamics, and they occur in real dynamical systems (quadratic maps at phase transition parameters, Manneville-Pomeau maps, etc.).
