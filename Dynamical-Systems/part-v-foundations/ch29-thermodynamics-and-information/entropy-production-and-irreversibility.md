# 29.5 Entropy Production and Irreversibility

The second law says entropy increases. But this is a statement about macroscopic averages — the entropy of the universe increases on average. Individual trajectories can locally decrease entropy (a refrigerator makes things colder; a living cell builds order from disorder). The question is: what is the rate of entropy production in a dynamical system, and when is it zero?

For a dynamical system with an invariant measure, entropy production is zero — the system is at "infinite temperature equilibrium," where all configurations are equally likely at all times. For dissipative systems, where phase space contracts toward an attractor, entropy production is positive and quantifiable.

**Definition 29.5.1.** For a Markov chain with transition matrix $P$ and stationary distribution $\pi$, the *entropy production rate* is:
$$\dot{\sigma} = \sum_{i,j} \pi_i P_{ij} \log\frac{P_{ij}}{P_{ji}} \geq 0.$$

The system is *reversible* (in detailed balance) iff $\dot{\sigma} = 0$, i.e., $\pi_i P_{ij} = \pi_j P_{ji}$.

The detailed balance condition $\pi_i P_{ij} = \pi_j P_{ji}$ says that the probability current from state $i$ to state $j$ equals the probability current in the reverse direction, at stationarity. If this holds, the system is time-reversible: the forward and reverse processes are statistically indistinguishable. If it fails, probability "circulates" around cycles, and the system is genuinely irreversible.

The entropy production rate measures this circulation. It is a sum of KL-divergence-like terms comparing forward and reverse transition rates, weighted by the stationary probability. A system far from detailed balance (many cycles, large current asymmetries) has high entropy production.

**Theorem 29.5.2 (Second Law for Markov Chains).** The relative entropy $D_{KL}(\mu_t \| \pi)$ of the distribution at time $t$ from the stationary distribution is monotonically decreasing:
$$\frac{d}{dt} D_{KL}(\mu_t \| \pi) \leq 0,$$
with equality iff $\mu_t = \pi$ (at stationarity).

This is the cleanest formulation of the second law for Markov chains: KL divergence from the stationary distribution is a Lyapunov function for the Markov dynamics. The system converges to stationarity, and the "distance" to equilibrium — measured by KL divergence — decreases monotonically.

This is the same KL divergence we saw in Theorem 29.1.5 (free energy equals $k_BT$ times KL divergence from equilibrium). The second law, the information-theoretic formulation of free energy, and the convergence of Markov chains are all facets of the same mathematical structure.

**Dynamical Systems Formulation:** For a measure-preserving system $(X, \mu, f)$, entropy production is zero — the system is at "infinite temperature equilibrium." For a dissipative system with an SRB measure $\mu_{SRB}$, the entropy production rate equals the sum of negative Lyapunov exponents (measuring the "phase space contraction").

This last statement connects thermodynamics to the Lyapunov spectrum, which we studied in Chapters 13–15. Phase space contraction — the shrinking of volumes under the flow — is the signature of dissipation. The rate of volume contraction (the sum of negative Lyapunov exponents, for an Axiom A attractor) is exactly the entropy production rate. Hot, dissipative, chaotic systems generate entropy at a rate controlled by how quickly nearby orbits are attracted to the attractor.

The circle of ideas in this chapter — Boltzmann, Shannon, Landauer, Jarzynski, Ruelle, Lyapunov — converges on a single deep point: information and heat are the same stuff, measured in different units, and the dynamics of information (how entropy is created, transported, and erased) is the dynamics of physical systems.

In the next chapter, we take a geometric turn. Optimal transport theory, which began with a question about moving dirt, turns out to provide the natural geometry for probability measures — and that geometry connects back to everything we've done here.
