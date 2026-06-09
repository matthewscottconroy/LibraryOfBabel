# 30.1 The Monge-Kantorovich Problem

The brilliant move in Kantorovich's reformulation of Monge's problem was to replace "maps" with "couplings." Monge's problem says each grain of dirt must go to a specific hole — this is a deterministic transport map $T: X \to Y$ with $T_\# \mu = \nu$ (pushforward condition). Kantorovich's relaxation allows the dirt to be split: each grain can be transported to multiple holes simultaneously, with a probability distribution over destinations. This is a transport plan.

## 30.1.1 Formulation

**Definition 30.1.1 (Transport Plan).** Given probability measures $\mu, \nu$ on metric spaces $X, Y$, a *transport plan* (or *coupling*) is a probability measure $\pi$ on $X \times Y$ with marginals $\pi_X = \mu$ and $\pi_Y = \nu$:
$$\int_Y d\pi(x,y) = d\mu(x), \quad \int_X d\pi(x,y) = d\nu(y).$$

Think of $\pi(A \times B)$ as "the fraction of mass from $A$ that is transported to $B$." The marginal conditions say the total mass leaving any region of $X$ is $\mu$ and the total mass arriving at any region of $Y$ is $\nu$. Every deterministic map $T$ defines a coupling $\pi = (\text{id}, T)_\# \mu$ (the pushforward under the graph map), but couplings are much more general.

**Definition 30.1.2 (Kantorovich Problem).** For a cost function $c: X \times Y \to \mathbb{R}_{\geq 0}$, the *optimal transport cost* is:
$$W_c(\mu, \nu) = \inf_{\pi \in \Pi(\mu,\nu)} \int_{X\times Y} c(x,y)\,d\pi(x,y).$$

The infimum over all transport plans $\Pi(\mu, \nu)$ is the minimal expected cost. The advantage of this formulation: the set of couplings $\Pi(\mu, \nu)$ is a convex set (in fact, a compact subset of probability measures on $X \times Y$), and the objective is linear in $\pi$. This is a linear programming problem.

**Definition 30.1.3 (Wasserstein Distance).** For $p \geq 1$ and metric space $(X,d)$:
$$W_p(\mu, \nu) = \left(\inf_{\pi \in \Pi(\mu,\nu)} \int_{X\times X} d(x,y)^p\,d\pi(x,y)\right)^{1/p}.$$

$W_1$ is called the *Earth mover's distance*; $W_2$ has the richest geometry.

The Earth mover's distance $W_1$ has an attractive interpretation: it is the minimum "effort" to move distribution $\mu$ to distribution $\nu$, where effort equals (mass moved) $\times$ (distance moved). For discrete measures, it's the minimum number of "unit moves" to go from one distribution to the other.

## 30.1.2 Duality — The Kantorovich-Rubinstein Theorem

Linear programming problems have dual problems, and the dual of Kantorovich's problem turns out to be equally important. The duality says: instead of finding the cheapest way to move mass, you can instead find the most adversarial pricing scheme — choosing prices for dirt and holes such that the "profit" is maximized, subject to the prices being consistent with the transport cost.

**Theorem 30.1.4 (Kantorovich-Rubinstein Duality).** For the $W_1$ distance:
$$W_1(\mu, \nu) = \sup_{\|f\|_{\text{Lip}} \leq 1} \left\{\int f\,d\mu - \int f\,d\nu\right\},$$
where the sup is over all 1-Lipschitz functions $f: X \to \mathbb{R}$.

In plain terms: the $W_1$ distance between $\mu$ and $\nu$ is the maximum difference in "test averages" over all functions that don't vary too rapidly. Functions with large Lipschitz constant would be too sensitive to small spatial differences and would overcount the distance; the constraint $\|f\|_{\text{Lip}} \leq 1$ calibrates the sensitivity correctly.

This duality is the basis for Wasserstein GANs in machine learning: training a discriminator to maximize $\int f\, d\mu - \int f\, d\nu$ subject to $f$ being 1-Lipschitz is exactly computing $W_1(\mu, \nu)$.

**Theorem 30.1.5 (General Duality — Kantorovich).** For general cost $c$:
$$W_c(\mu, \nu) = \sup_{\varphi \oplus \psi \leq c} \left\{\int \varphi\,d\mu + \int \psi\,d\nu\right\},$$
where the sup is over all $\varphi \in L^1(\mu)$, $\psi \in L^1(\nu)$ with $\varphi(x) + \psi(y) \leq c(x,y)$.

The dual variables $\varphi, \psi$ are *Kantorovich potentials*.

Strong duality holds: the infimum on the primal side equals the supremum on the dual side, and both are attained (under mild conditions). The optimal $\varphi, \psi$ are the Kantorovich potentials, and they are related to the optimal transport map $T$ by the relation $\varphi(x) + \psi(T(x)) = c(x, T(x))$ — the potentials are "tight" along optimal trajectories.
