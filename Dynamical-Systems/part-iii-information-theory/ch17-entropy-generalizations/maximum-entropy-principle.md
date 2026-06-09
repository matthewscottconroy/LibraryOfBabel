# 17.5 The Maximum Entropy Principle

Suppose you know something about a random variable — its mean, or its variance, or the probabilities of certain events — but you don't know its full distribution. What distribution should you assume?

E.T. Jaynes answered this question in two influential papers from 1957, drawing on an idea from statistical mechanics: choose the distribution that maximizes entropy subject to what you know. This is the *maximum entropy principle* (MaxEnt), and it turns out to be the unique rational choice under very general conditions.

**Definition 17.5.1 (Maximum Entropy Distribution — Jaynes).** Given moment constraints $E[g_k(X)] = c_k$ for $k = 1, \ldots, m$, the *maximum entropy distribution* is the one maximizing $H(X)$ (or $h(X)$ in the continuous case) subject to the constraints.

Why maximize entropy? Because entropy measures uncertainty, and choosing the maximum entropy distribution means you are assuming as little as possible beyond what the constraints force. Any distribution with lower entropy would make implicit assumptions about the data that are not justified by your constraints. MaxEnt is, in this sense, the least-presumptuous choice.

The solution has an elegant form:

**Theorem 17.5.2 (Gibbs / Jaynes).** The maximum entropy distribution subject to moment constraints $E[g_k(X)] = c_k$ has the *exponential family* form:
$$p^*(x) = \frac{1}{Z(\lambda)} \exp\left(-\sum_k \lambda_k g_k(x)\right),$$
where $Z(\lambda) = \int \exp(-\sum_k \lambda_k g_k)\,dx$ is the *partition function* (normalization) and $\lambda_k$ are Lagrange multipliers determined by the constraints.

This is proved by Lagrangian optimization: write the entropy as $h = -\int f\log f\,dx$, add Lagrange multipliers for each constraint, differentiate, and set to zero. The exponential form is the unique solution.

The examples are illuminating:

**Examples:**
- *No constraints (finite support)*: uniform distribution — no structure at all, maximum entropy.
- *Constraint $E[X] = \mu$ only* (on $[0, \infty)$): exponential distribution $p(x) = \lambda e^{-\lambda x}$ with $\lambda = 1/\mu$.
- *Constraints $E[X] = \mu$, $\text{Var}(X) = \sigma^2$*: Gaussian $N(\mu, \sigma^2)$ — explaining why the Gaussian is so ubiquitous.
- *Energy constraint $E[H(x)] = \bar{E}$ in statistical mechanics*: Boltzmann distribution $p(x) \propto e^{-H(x)/kT}$.

That last example is the connection to physics. In statistical mechanics, the equilibrium distribution of a physical system at temperature $T$ is the maximum entropy distribution subject to the constraint that the average energy is $\bar{E}$. This is the Boltzmann-Gibbs distribution, and it was derived from maximum entropy long before Jaynes — though Jaynes' contribution was to identify *why* it is the right distribution: not because of any particular physical argument, but because it is the least-biased inference given the energy constraint.

MaxEnt is not without controversy. Jaynes interpreted it as the basis of a full Bayesian epistemology: all of statistics is inference under constraints, and entropy is the universal measure of uncertainty. Critics argue that the choice of constraints is itself subjective, and different choices of what to condition on can give radically different distributions. This debate is ongoing in the foundations of statistics and statistical mechanics.

For our purposes, MaxEnt has two clean mathematical contributions: (1) it explains why exponential families (Gaussians, exponentials, Poissons, Bernoullis) dominate statistical practice — they are the maximum entropy distributions for natural moment constraints; and (2) it connects information theory to equilibrium statistical mechanics via the Boltzmann distribution, a connection we will develop further in Chapter 20 when we discuss the information geometry of exponential families.
